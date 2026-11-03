// Mush8 Android - vk.rs
// Minimal Vulkan 1.1 renderer: instance, Android surface, swapchain, clear pass.
use anyhow::{Context, Result, anyhow, bail};
use ash::khr::{android_surface, buffer_device_address, surface, swapchain};
use ash::vk::Handle;
use ash::{Entry, Instance, vk};
use std::ffi::{CStr, c_void};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

mod present_spv {
    include!("gfx_vk_present_spv.rs");
}

#[path = "gfx_vk_gpu.rs"]
mod gpu;

const PRESENT_VERT_SRC: &str = include_str!("../shaders/present.vert");
const PRESENT_FRAG_SRC: &str = include_str!("../shaders/present.frag");

const DBG_VERT: &str = r#"#version 450
layout(location = 0) out vec2 uv;
void main() {
    vec2 p = vec2(float((gl_VertexIndex << 1) & 2), float(gl_VertexIndex & 2));
    uv = p;
    gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}
"#;

const DBG_FRAG: &str = r#"#version 450
#extension GL_EXT_buffer_reference : require
#extension GL_EXT_shader_explicit_arithmetic_types_int64 : require
layout(buffer_reference, std430, buffer_reference_align = 4) buffer FloatRef { float data[]; };
layout(set = 0, binding = 0, std430) readonly buffer ResBuf { uint64_t ptr; } res;
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 color;
void main() {
    vec3 c = FloatRef(res.ptr).data[0].xxx * vec3(1.0, 0.6, 0.3);
    color = vec4(mix(c, vec3(1.0), 1.0 - uv.y * 0.6), 1.0);
}
"#;

#[cfg(mush8_shaderc)]
mod selftest_blob {
    include!("gfx_vk_selftest.rs");
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct GpuTex {
    pub addr: u64,
    pub format: u32,
    pub target: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub levels: u32,
    pub layers: u32,
}

#[derive(Clone)]
pub struct GpuDraw {
    pub target: GpuTex,
    pub depth: Option<GpuTex>,
    pub texture_pool: u64,
    pub sampler_pool: u64,
    pub stages: Vec<(Vec<u8>, u64)>,
    pub attribs: [u32; 16],
    pub streams: [(u32, u32); 16],
    pub vbufs: [(u64, u64); 16],
    pub ubos: [[(u64, u64); 15]; 2],
    pub storage: [[(u64, u64); 16]; 2],
    pub textures: [[u64; 32]; 2],
    pub viewport: Option<[f32; 4]>,
    pub scissor: Option<[i32; 4]>,
    pub flags: u32,
    pub color_state: u32,
    pub channel_mask: u32,
    pub blend: u64,
    pub blend_color: [f32; 4],
    pub alpha_func: u32,
    pub alpha_ref: f32,
    pub mode: u32,
    pub first: u32,
    pub count: u32,
    pub index: Option<(u32, u64)>,
    pub instances: u32,
    pub base_instance: u32,
    pub base_vertex: i32,
}

pub struct GpuCopy {
    pub src: GpuTex,
    pub dst: GpuTex,
    pub src_off: [u32; 3],
    pub dst_off: [u32; 3],
    pub extent: [u32; 3],
}

pub struct GpuDispatch {
    pub blob: Vec<u8>,
    pub constants: u64,
    pub local_size: [u32; 3],
    pub groups: [u32; 3],
    pub ubos: Vec<(u64, u64)>,
    pub storage: Vec<(u64, u64)>,
    pub textures: Vec<u64>,
    pub texture_pool: u64,
}

pub static STAGE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

static STAGE_AT: Mutex<Option<(u32, std::time::Instant)>> = Mutex::new(None);

pub const STAGE_NAMES: &[&str] = &[
    "idle",
    "enter",
    "fence",
    "mirror",
    "textures",
    "ensure-image",
    "fence-b",
    "acquire",
    "dispatches",
    "draws",
    "copies",
    "submit",
    "queue-present",
    "done",
];

pub fn stage(n: u32) {
    STAGE.store(n, std::sync::atomic::Ordering::Relaxed);
    *STAGE_AT.lock().unwrap() = Some((n, std::time::Instant::now()));
}

fn stage_since() -> (u32, u128) {
    let g = STAGE_AT.lock().unwrap();
    match *g {
        Some((n, at)) => (n, at.elapsed().as_secs() as u128),
        None => (0, 0),
    }
}

pub struct GpuFrame {
    pub present: GpuTex,
    pub dispatches: Vec<GpuDispatch>,
    pub clears: Vec<(GpuTex, [f32; 4])>,
    pub draws: Vec<GpuDraw>,
    pub copies: Vec<GpuCopy>,
    pub patches: Vec<(u64, u64, Vec<u8>)>,
    pub depth_clears: Vec<(GpuTex, f32)>,
}

static GPU_FRAME: Mutex<Option<GpuFrame>> = Mutex::new(None);

static LAST_CPU: Mutex<Option<Frame>> = Mutex::new(None);

static GPU_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn gpu_active() -> bool {
    GPU_ACTIVE.load(Ordering::Relaxed)
}

pub fn submit_gpu_frame(f: GpuFrame) {
    *GPU_FRAME.lock().unwrap() = Some(f);
    // Pace the guest to the renderer, like a real vsync.
    let n = SUBMITTED.fetch_add(1, Ordering::Relaxed) + 1;
    let (lock, cv) = presented();
    let guard = lock.lock().unwrap();
    let _ = cv.wait_timeout_while(guard, std::time::Duration::from_millis(50), |done| *done < n);
}

static SUBMITTED: AtomicU64 = AtomicU64::new(0);

fn presented() -> &'static (Mutex<u64>, std::sync::Condvar) {
    static PRESENTED: OnceLock<(Mutex<u64>, std::sync::Condvar)> = OnceLock::new();
    PRESENTED.get_or_init(|| (Mutex::new(0), std::sync::Condvar::new()))
}

pub fn submitted() -> u64 {
    SUBMITTED.load(Ordering::Relaxed)
}

pub fn presented_now() -> u64 {
    *presented().0.lock().unwrap()
}

pub fn wait_presented(seq: u64, timeout: std::time::Duration) {
    let (lock, cv) = presented();
    let g = lock.lock().unwrap();
    let _ = cv.wait_timeout_while(g, timeout, |done| *done < seq);
}

fn mark_presented(serial: u64) {
    let (lock, cv) = presented();
    let mut done = lock.lock().unwrap();
    *done = (*done).max(serial);
    cv.notify_all();
}

pub struct Renderer {
    entry: Entry,
    instance: Instance,
    surface_loader: surface::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    swapchain_loader: swapchain::Device,
    queue: vk::Queue,
    queue_family: u32,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    views: Vec<vk::ImageView>,
    format: vk::Format,
    extent: vk::Extent2D,
    render_pass: vk::RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    image_available: vk::Semaphore,
    render_finished: vk::Semaphore,
    in_flight: vk::Fence,
    desired: (u32, u32),
    start: Instant,
    source_image: vk::Image,
    source_memory: vk::DeviceMemory,
    source_extent: vk::Extent2D,
    source_format: vk::Format,
    staging_buffer: vk::Buffer,
    staging_memory: vk::DeviceMemory,
    staging_ptr: *mut u8,
    staging_size: vk::DeviceSize,
    source_view: vk::ImageView,
    set_layouts: [vk::DescriptorSetLayout; 2],
    desc_pool: vk::DescriptorPool,
    desc_sets: [vk::DescriptorSet; 2],
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    sampler: vk::Sampler,
    flags_buffer: vk::Buffer,
    flags_memory: vk::DeviceMemory,
    flags_ptr: *mut u32,
    bda: Option<buffer_device_address::Device>,
    dbg_set_layout: vk::DescriptorSetLayout,
    dbg_pool: vk::DescriptorPool,
    dbg_set: vk::DescriptorSet,
    dbg_pipeline_layout: vk::PipelineLayout,
    dbg_pipeline: vk::Pipeline,
    dbg_color_buffer: vk::Buffer,
    dbg_color_memory: vk::DeviceMemory,
    dbg_res_buffer: vk::Buffer,
    dbg_res_memory: vk::DeviceMemory,
    dbg_res_ptr: *mut u64,
    gpu: Option<gpu::GpuState>,
    gpu_present_pool: vk::DescriptorPool,
    gpu_present_sets: [vk::DescriptorSet; 2],
    open_pass: Option<(GpuTex, Option<GpuTex>)>,
    state_pipe: vk::Pipeline,
    state_vp: (f32, f32, f32, f32),
    state_sc: (i32, i32, i32, i32),
    state_blend: [f32; 4],
    sample_cache: std::cell::RefCell<std::collections::HashMap<GpuTex, (vk::ImageView, vk::ImageLayout)>>,
    dual_src_blend: bool,
    pipe_cache: vk::PipelineCache,
}

impl Renderer {
    pub fn new(window: *mut c_void, width: u32, height: u32) -> Result<Self> {
        unsafe { Self::create(window, width, height) }
    }

    unsafe fn create(window: *mut c_void, width: u32, height: u32) -> Result<Self> {
        let entry = Entry::load().context("loading libvulkan.so")?;
        let app = vk::ApplicationInfo::default()
            .application_name(c"mush8")
            .engine_name(c"mush8")
            .api_version(vk::API_VERSION_1_1);
        let extensions = [surface::NAME.as_ptr(), android_surface::NAME.as_ptr()];
        let create = vk::InstanceCreateInfo::default()
            .application_info(&app)
            .enabled_extension_names(&extensions);
        let instance = entry.create_instance(&create, None).context("vkCreateInstance")?;
        let surface_loader = surface::Instance::new(&entry, &instance);
        let android = android_surface::Instance::new(&entry, &instance);
        let surface_info = vk::AndroidSurfaceCreateInfoKHR::default().window(window);
        let surface = android.create_android_surface(&surface_info, None).context("vkCreateAndroidSurfaceKHR")?;

        let devices = instance.enumerate_physical_devices().context("enumerate physical devices")?;
        if devices.is_empty() {
            bail!("no Vulkan physical devices");
        }
        let mut chosen = None;
        for pd in devices {
            let props = instance.get_physical_device_properties(pd);
            let queues = instance.get_physical_device_queue_family_properties(pd);
            for (i, q) in queues.iter().enumerate() {
                if !q.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    continue;
                }
                if surface_loader.get_physical_device_surface_support(pd, i as u32, surface).unwrap_or(false) {
                    chosen = Some((pd, i as u32, props.device_name_as_c_str().unwrap_or(c"?").to_string_lossy().into_owned()));
                    break;
                }
            }
            if chosen.is_some() {
                break;
            }
        }
        let (physical_device, queue_family, name) = chosen.ok_or_else(|| anyhow!("no graphics+present queue"))?;
        {
            let exts = instance.enumerate_device_extension_properties(physical_device).unwrap_or_default();
            let mut names: Vec<String> = exts.iter().map(|e| e.extension_name_as_c_str().unwrap_or(c"?").to_string_lossy().into_owned()).collect();
            names.sort();
            let interesting: Vec<&String> = names.iter().filter(|n| n.contains("buffer_device") || n.contains("external_memory") || n.contains("descriptor_index") || n.contains("atomic_float")).collect();
            log::info!("[vk] device extensions: {} total, interesting {:?}", names.len(), interesting);
        }

        let priorities = [1.0f32];
        let queue_info = [vk::DeviceQueueCreateInfo::default().queue_family_index(queue_family).queue_priorities(&priorities)];
        let dev_features = instance.get_physical_device_features(physical_device);
        let dual_src_blend = dev_features.dual_src_blend == vk::TRUE;
        let mut enabled_features = vk::PhysicalDeviceFeatures::default();
        enabled_features.dual_src_blend = dual_src_blend as u32;
        enabled_features.texture_compression_astc_ldr = dev_features.texture_compression_astc_ldr;
        let available = instance.enumerate_device_extension_properties(physical_device).unwrap_or_default();
        let has_bda = available.iter().any(|e| e.extension_name_as_c_str().map(|n| n == buffer_device_address::NAME).unwrap_or(false));
        let mut device_exts: Vec<*const std::ffi::c_char> = vec![swapchain::NAME.as_ptr()];
        if has_bda {
            device_exts.push(buffer_device_address::NAME.as_ptr());
        }
        let mut device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&device_exts)
            .enabled_features(&enabled_features);
        let mut bda_enable = vk::PhysicalDeviceBufferDeviceAddressFeatures::default().buffer_device_address(true);
        if has_bda {
            device_info = device_info.push_next(&mut bda_enable);
        }
        let device = instance.create_device(physical_device, &device_info, None).context("vkCreateDevice")?;
        let queue = device.get_device_queue(queue_family, 0);
        let swapchain_loader = swapchain::Device::new(&instance, &device);
        // Pipeline cache: without it every new NVN pipeline variant is re-JITed
        // by the driver from scratch (hundreds of ms each on PowerVR).
        let cache_data = pipe_cache_path().and_then(|p| std::fs::read(p).ok()).unwrap_or_default();
        if !cache_data.is_empty() {
            log::info!("[vk] pipeline cache: {} KiB from disk", cache_data.len() >> 10);
        }
        let pipe_cache = match unsafe {
            device.create_pipeline_cache(&vk::PipelineCacheCreateInfo::default().initial_data(&cache_data), None)
        } {
            Ok(c) => c,
            Err(e) => {
                log::warn!("[vk] pipeline cache unavailable: {e}");
                vk::PipelineCache::null()
            }
        };
        log::info!("[vk] buffer device address: {}", if has_bda { "available" } else { "missing" });

        let command_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let command_pool = device.create_command_pool(&command_info, None).context("command pool")?;

        let sem_info = vk::SemaphoreCreateInfo::default();
        let image_available = device.create_semaphore(&sem_info, None)?;
        let render_finished = device.create_semaphore(&sem_info, None)?;
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let in_flight = device.create_fence(&fence_info, None)?;

        let set0 = [
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT),
            vk::DescriptorSetLayoutBinding::default()
                .binding(1)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        ];
        let set1 = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
        let layouts = [
            device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&set0), None)?,
            device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&set1), None)?,
        ];
        let pipeline_layout = device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&layouts), None)?;
        let sizes = [
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1),
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1),
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLER).descriptor_count(1),
        ];
        let desc_pool = device.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(2).pool_sizes(&sizes), None)?;
        let desc_sets: [vk::DescriptorSet; 2] =
            device.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(desc_pool).set_layouts(&layouts))?.try_into().unwrap();
        let sampler_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .max_lod(0.0);
        let sampler = device.create_sampler(&sampler_info, None)?;

        let gp_sizes = [
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1),
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1),
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLER).descriptor_count(1),
        ];
        let gpu_present_pool = device.create_descriptor_pool(
            &vk::DescriptorPoolCreateInfo::default().max_sets(2).pool_sizes(&gp_sizes),
            None,
        )?;
        let gpu_present_sets: [vk::DescriptorSet; 2] = device
            .allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(gpu_present_pool).set_layouts(&layouts))?
            .try_into()
            .unwrap();

        let flags_info = vk::BufferCreateInfo::default()
            .size(16)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let flags_buffer = device.create_buffer(&flags_info, None)?;
        let req = device.get_buffer_memory_requirements(flags_buffer);
        let props = instance.get_physical_device_memory_properties(physical_device);
        let mt = (0..props.memory_type_count)
            .find(|&i| {
                req.memory_type_bits & (1 << i) != 0
                    && props.memory_types[i as usize]
                        .property_flags
                        .contains(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
            })
            .ok_or_else(|| anyhow!("no host-visible memory for flags buffer"))?;
        let flags_memory = device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt), None)?;
        device.bind_buffer_memory(flags_buffer, flags_memory, 0)?;
        let flags_ptr = device.map_memory(flags_memory, 0, 16, vk::MemoryMapFlags::empty())? as *mut u32;
        std::ptr::write_bytes(flags_ptr, 0, 4);

        let host_buffer = |size: u64, usage: vk::BufferUsageFlags| -> Result<(vk::Buffer, vk::DeviceMemory, *mut u8)> {
            let info = vk::BufferCreateInfo::default().size(size).usage(usage).sharing_mode(vk::SharingMode::EXCLUSIVE);
            let buffer = device.create_buffer(&info, None)?;
            let req = device.get_buffer_memory_requirements(buffer);
            let mt = (0..props.memory_type_count)
                .find(|&i| {
                    req.memory_type_bits & (1 << i) != 0
                        && props.memory_types[i as usize]
                            .property_flags
                            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
                })
                .ok_or_else(|| anyhow!("no host-visible memory"))?;
            let memory = device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt), None)?;
            device.bind_buffer_memory(buffer, memory, 0)?;
            let ptr = device.map_memory(memory, 0, size, vk::MemoryMapFlags::empty())? as *mut u8;
            Ok((buffer, memory, ptr))
        };
        let bda = has_bda.then(|| buffer_device_address::Device::new(&instance, &device));
        let (dbg_color_buffer, dbg_color_memory, dbg_color_ptr) =
            host_buffer(16, vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)?;
        let (dbg_res_buffer, dbg_res_memory, dbg_res_ptr) =
            host_buffer(16, vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)?;
        for (i, v) in [0.9f32, 0.35, 0.1, 1.0].iter().enumerate() {
            std::ptr::write(dbg_color_ptr.add(i * 4) as *mut f32, *v);
        }
        let color_addr = match &bda {
            Some(b) => b.get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(dbg_color_buffer)),
            None => 0,
        };
        std::ptr::write(dbg_res_ptr as *mut u64, color_addr);
        let dbg_binding = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
        let dbg_set_layout = device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&dbg_binding), None)?;
        let dbg_layouts = [dbg_set_layout];
        let dbg_pipeline_layout = device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&dbg_layouts), None)?;
        let dbg_sizes = [vk::DescriptorPoolSize::default().ty(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1)];
        let dbg_pool = device.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&dbg_sizes), None)?;
        let dbg_set = device.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(dbg_pool).set_layouts(&dbg_layouts))?[0];
        let dbg_buf_info = [vk::DescriptorBufferInfo::default().buffer(dbg_res_buffer).offset(0).range(vk::WHOLE_SIZE)];
        let dbg_write = [vk::WriteDescriptorSet::default()
            .dst_set(dbg_set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&dbg_buf_info)];
        device.update_descriptor_sets(&dbg_write, &[]);
        if has_bda {
            log::info!("[vk] debug color buffer address {color_addr:#x}");
        }

        let mut r = Self {
            entry,
            instance,
            surface_loader,
            surface,
            physical_device,
            device,
            swapchain_loader,
            queue,
            queue_family,
            swapchain: vk::SwapchainKHR::null(),
            images: Vec::new(),
            views: Vec::new(),
            format: vk::Format::UNDEFINED,
            extent: vk::Extent2D { width: width.max(1), height: height.max(1) },
            render_pass: vk::RenderPass::null(),
            framebuffers: Vec::new(),
            command_pool,
            command_buffers: Vec::new(),
            image_available,
            render_finished,
            in_flight,
            desired: (width.max(1), height.max(1)),
            start: Instant::now(),
            source_image: vk::Image::null(),
            source_memory: vk::DeviceMemory::null(),
            source_extent: vk::Extent2D::default(),
            source_format: vk::Format::UNDEFINED,
            staging_buffer: vk::Buffer::null(),
            staging_memory: vk::DeviceMemory::null(),
            staging_ptr: std::ptr::null_mut(),
            staging_size: 0,
            source_view: vk::ImageView::null(),
            set_layouts: layouts,
            desc_pool,
            desc_sets,
            pipeline_layout,
            pipeline: vk::Pipeline::null(),
            sampler,
            flags_buffer,
            flags_memory,
            flags_ptr,
            bda,
            dbg_set_layout,
            dbg_pool,
            dbg_set,
            dbg_pipeline_layout,
            dbg_pipeline: vk::Pipeline::null(),
            dbg_color_buffer,
            dbg_color_memory,
            dbg_res_buffer,
            dbg_res_memory,
            dbg_res_ptr: dbg_res_ptr as *mut u64,
            gpu: None,
            gpu_present_pool,
            gpu_present_sets,
            open_pass: None,
            state_pipe: vk::Pipeline::null(),
            state_vp: (f32::NAN, f32::NAN, f32::NAN, f32::NAN),
            state_sc: (i32::MIN, i32::MIN, i32::MIN, i32::MIN),
            state_blend: [f32::NAN; 4],
            sample_cache: Default::default(),
            dual_src_blend,
            pipe_cache,
        };
        r.create_swapchain(name)?;
        #[cfg(mush8_shaderc)]
        r.shader_selftest();
        Ok(r)
    }

    unsafe fn create_swapchain(&mut self, name: String) -> Result<()> {
        let caps = self.surface_loader.get_physical_device_surface_capabilities(self.physical_device, self.surface)?;
        log::info!("[vk] surface caps: current_extent {}x{}, transform {:?}", caps.current_extent.width, caps.current_extent.height, caps.current_transform);
        let formats = self.surface_loader.get_physical_device_surface_formats(self.physical_device, self.surface)?;
        let modes = self.surface_loader.get_physical_device_surface_present_modes(self.physical_device, self.surface)?;
        let wanted = vk::Format::B8G8R8A8_UNORM;
        let srgb = vk::ColorSpaceKHR::SRGB_NONLINEAR;
        let format = formats
            .iter()
            .find(|f| f.format == wanted && f.color_space == srgb)
            .or_else(|| formats.iter().find(|f| f.format == vk::Format::R8G8B8A8_UNORM && f.color_space == srgb))
            .or_else(|| formats.first())
            .copied()
            .ok_or_else(|| anyhow!("no surface formats"))?;
        let present_mode = if modes.contains(&vk::PresentModeKHR::FIFO) {
            vk::PresentModeKHR::FIFO
        } else {
            modes.first().copied().unwrap_or(vk::PresentModeKHR::FIFO)
        };
        let (dw, dh) = self.desired;
        let extent = if caps.current_extent.width != u32::MAX {
            caps.current_extent
        } else {
            vk::Extent2D {
                width: dw.clamp(caps.min_image_extent.width, caps.max_image_extent.width),
                height: dh.clamp(caps.min_image_extent.height, caps.max_image_extent.height),
            }
        };
        let mut count = caps.min_image_count + 1;
        if caps.max_image_count > 0 {
            count = count.min(caps.max_image_count);
        }
        log::info!("[vk] swapchain: {}x{} transform {:?}", extent.width, extent.height, caps.current_transform);
        let info = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface)
            .min_image_count(count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(self.swapchain);
        let swapchain = self.swapchain_loader.create_swapchain(&info, None).context("create swapchain")?;
        let images = self.swapchain_loader.get_swapchain_images(swapchain)?;
        let mut views = Vec::with_capacity(images.len());
        for &image in &images {
            let view_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format.format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });
            views.push(self.device.create_image_view(&view_info, None)?);
        }

        let attachment = vk::AttachmentDescription::default()
            .format(format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        let color_ref = [vk::AttachmentReference::default().attachment(0).layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let subpass = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_ref)];
        let dependency = [vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)];
        let attachments = [attachment];
        let rp_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpass)
            .dependencies(&dependency);
        let render_pass = self.device.create_render_pass(&rp_info, None).context("create render pass")?;

        let mut framebuffers = Vec::with_capacity(views.len());
        for &view in &views {
            let fb_attachments = [view];
            let fb_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&fb_attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);
            framebuffers.push(self.device.create_framebuffer(&fb_info, None)?);
        }

        let allocate = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(views.len() as u32);
        let command_buffers = self.device.allocate_command_buffers(&allocate)?;

        self.swapchain = swapchain;
        self.images = images;
        self.views = views;
        self.format = format.format;
        self.extent = extent;
        self.render_pass = render_pass;
        self.framebuffers = framebuffers;
        self.command_buffers = command_buffers;
        self.create_pipeline()?;
        if let Err(e) = self.create_debug_pipeline() {
            log::error!("[vk] debug pipeline: {e:#}");
        }
        log::info!(
            "[vk] {name}: {:?} {}x{}, {} images, {:?}",
            self.format,
            extent.width,
            extent.height,
            self.images.len(),
            present_mode
        );
        Ok(())
    }

    unsafe fn destroy_swapchain(&mut self) {
        let _ = self.device.device_wait_idle();
        for &fb in &self.framebuffers {
            self.device.destroy_framebuffer(fb, None);
        }
        self.framebuffers.clear();
        if !self.pipeline.is_null() {
            self.device.destroy_pipeline(self.pipeline, None);
            self.pipeline = vk::Pipeline::null();
        }
        if !self.dbg_pipeline.is_null() {
            self.device.destroy_pipeline(self.dbg_pipeline, None);
            self.dbg_pipeline = vk::Pipeline::null();
        }
        if !self.render_pass.is_null() {
            self.device.destroy_render_pass(self.render_pass, None);
            self.render_pass = vk::RenderPass::null();
        }
        for &v in &self.views {
            self.device.destroy_image_view(v, None);
        }
        self.views.clear();
        self.images.clear();
        if !self.swapchain.is_null() {
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.swapchain = vk::SwapchainKHR::null();
        }
        if !self.command_buffers.is_empty() {
            self.device.free_command_buffers(self.command_pool, &self.command_buffers);
            self.command_buffers.clear();
        }
    }

    #[cfg(mush8_shaderc)]
    unsafe fn shader_selftest(&self) {
        let t0 = Instant::now();
        let t = match nxshader::glsl::translate(selftest_blob::BLOB) {
            Ok(t) => t,
            Err(e) => {
                log::error!("[vk] shader self-test: NVN -> GLSL failed: {e:#}");
                return;
            }
        };
        let kind = match t.stage {
            nxshader::sph::Stage::Vertex => 0,
            nxshader::sph::Stage::Fragment => 1,
            nxshader::sph::Stage::Compute => 2,
            _ => {
                log::warn!("[vk] shader self-test: unsupported stage {:?}", t.stage);
                return;
            }
        };
        match crate::shaderc::compile(kind, &t.source, "selftest.frag") {
            Ok(words) => match self.device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None) {
                Ok(module) => {
                    self.device.destroy_shader_module(module, None);
                    log::info!(
                        "[vk] shader self-test ok: NVN -> GLSL -> SPIR-V -> VkShaderModule, {} words, {} ms",
                        words.len(),
                        t0.elapsed().as_millis()
                    );
                }
                Err(e) => log::error!("[vk] shader self-test: vkCreateShaderModule failed: {e}"),
            },
            Err(e) => log::error!("[vk] shader self-test: shaderc failed: {e:#}"),
        }
    }

    unsafe fn shader_module(&self, _stage: i32, _source: &str, _name: &str, fallback: &[u32]) -> Result<vk::ShaderModule> {
        #[cfg(mush8_shaderc)]
        {
            let t0 = Instant::now();
            match crate::shaderc::compile(_stage, _source, _name) {
                Ok(words) => {
                    log::info!("[vk] shaderc: {_name} compiled in {} ms ({} words)", t0.elapsed().as_millis(), words.len());
                    return self
                        .device
                        .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None)
                        .with_context(|| format!("shader module {_name}"));
                }
                Err(e) => log::error!("[vk] shaderc failed for {_name}: {e:#}"),
            }
        }
        self.device
            .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(fallback), None)
            .with_context(|| format!("shader module {_name} (embedded)"))
    }

    unsafe fn create_pipeline(&mut self) -> Result<()> {
        let vert = self.shader_module(0, PRESENT_VERT_SRC, "present.vert", present_spv::PRESENT_VERT)?;
        let frag = self.shader_module(1, PRESENT_FRAG_SRC, "present.frag", present_spv::PRESENT_FRAG)?;
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert)
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag)
                .name(c"main"),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport_state = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false);
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(std::slice::from_ref(&attachment));
        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .dynamic_state(&dynamic)
            .layout(self.pipeline_layout)
            .render_pass(self.render_pass)
            .subpass(0);
        let result = self.device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
        self.device.destroy_shader_module(vert, None);
        self.device.destroy_shader_module(frag, None);
        let pipelines = result.map_err(|(_, e)| anyhow!("create graphics pipeline: {e}"))?;
        self.pipeline = pipelines[0];
        log::info!("[vk] present pipeline ready (set 0: ResBuf+texture, set 1: sampler)");
        Ok(())
    }

    #[cfg(mush8_shaderc)]
    unsafe fn create_debug_pipeline(&mut self) -> Result<()> {
        if self.bda.is_none() {
            return Ok(());
        }
        let vert_words = crate::shaderc::compile(0, DBG_VERT, "dbg.vert")?;
        let frag_words = crate::shaderc::compile(1, DBG_FRAG, "dbg.frag")?;
        let vert = self.device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&vert_words), None)?;
        let frag = self.device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&frag_words), None)?;
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert)
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag)
                .name(c"main"),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport_state = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false);
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(std::slice::from_ref(&attachment));
        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .dynamic_state(&dynamic)
            .layout(self.dbg_pipeline_layout)
            .render_pass(self.render_pass)
            .subpass(0);
        let result = self.device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
        self.device.destroy_shader_module(vert, None);
        self.device.destroy_shader_module(frag, None);
        self.dbg_pipeline = result.map_err(|(_, e)| anyhow!("create debug pipeline: {e}"))?[0];
        log::info!("[vk] buffer-reference test pipeline ready");
        Ok(())
    }

    #[cfg(not(mush8_shaderc))]
    unsafe fn create_debug_pipeline(&mut self) -> Result<()> {
        Ok(())
    }

    unsafe fn write_descriptors(&self) {
        let buffer = [vk::DescriptorBufferInfo::default().buffer(self.flags_buffer).offset(0).range(vk::WHOLE_SIZE)];
        let image = [vk::DescriptorImageInfo::default()
            .image_view(self.source_view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let sampler = [vk::DescriptorImageInfo::default().sampler(self.sampler)];
        let writes = [
            vk::WriteDescriptorSet::default()
                .dst_set(self.desc_sets[0])
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(&buffer),
            vk::WriteDescriptorSet::default()
                .dst_set(self.desc_sets[0])
                .dst_binding(1)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&image),
            vk::WriteDescriptorSet::default()
                .dst_set(self.desc_sets[1])
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::SAMPLER)
                .image_info(&sampler),
        ];
        self.device.update_descriptor_sets(&writes, &[]);
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        if width == 0 || height == 0 || (width, height) == self.desired {
            return Ok(());
        }
        self.desired = (width, height);
        unsafe {
            self.destroy_swapchain();
            self.create_swapchain("resize".into())?;
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        unsafe {
            self.device.wait_for_fences(&[self.in_flight], true, u64::MAX)?;
            let (index, suboptimal) = match self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available,
                vk::Fence::null(),
            ) {
                Ok(v) => v,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    self.destroy_swapchain();
                    self.create_swapchain("out of date".into())?;
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            };
            self.device.reset_fences(&[self.in_flight])?;
            let cb = self.command_buffers[index as usize];
            self.device.reset_command_buffer(cb, vk::CommandBufferResetFlags::empty())?;
            self.device.begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
            let clear = [vk::ClearValue { color: vk::ClearColorValue { float32: self.clear_color() } }];
            let begin = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[index as usize])
                .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: self.extent })
                .clear_values(&clear);
            self.device.cmd_begin_render_pass(cb, &begin, vk::SubpassContents::INLINE);
            self.device.cmd_end_render_pass(cb);
            self.device.end_command_buffer(cb)?;

            let wait = [self.image_available];
            let stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let signal = [self.render_finished];
            let cbs = [cb];
            let submit = vk::SubmitInfo::default()
                .wait_semaphores(&wait)
                .wait_dst_stage_mask(&stages)
                .command_buffers(&cbs)
                .signal_semaphores(&signal);
            self.device.queue_submit(self.queue, &[submit], self.in_flight)?;
            let swapchains = [self.swapchain];
            let indices = [index];
            let present = vk::PresentInfoKHR::default()
                .wait_semaphores(&signal)
                .swapchains(&swapchains)
                .image_indices(&indices);
            let result = self.swapchain_loader.queue_present(self.queue, &present);
            let _ = suboptimal;
            let needs_recreate = matches!(result, Err(vk::Result::ERROR_OUT_OF_DATE_KHR));
            if needs_recreate {
                self.destroy_swapchain();
                self.create_swapchain("suboptimal".into())?;
            } else if let Err(e) = result {
                return Err(e.into());
            }
        }
        Ok(())
    }

    unsafe fn find_memory_type(&self, bits: u32, props: vk::MemoryPropertyFlags) -> Result<u32> {
        let mem = self.instance.get_physical_device_memory_properties(self.physical_device);
        (0..mem.memory_type_count)
            .find(|&i| bits & (1 << i) != 0 && mem.memory_types[i as usize].property_flags.contains(props))
            .ok_or_else(|| anyhow!("no memory type for {props:?}"))
    }

    unsafe fn destroy_source(&mut self) {
        if !self.staging_ptr.is_null() {
            self.device.unmap_memory(self.staging_memory);
            self.staging_ptr = std::ptr::null_mut();
        }
        if !self.staging_buffer.is_null() {
            self.device.destroy_buffer(self.staging_buffer, None);
            self.staging_buffer = vk::Buffer::null();
        }
        if !self.staging_memory.is_null() {
            self.device.free_memory(self.staging_memory, None);
            self.staging_memory = vk::DeviceMemory::null();
        }
        if !self.source_view.is_null() {
            self.device.destroy_image_view(self.source_view, None);
            self.source_view = vk::ImageView::null();
        }
        if !self.source_image.is_null() {
            self.device.destroy_image(self.source_image, None);
            self.source_image = vk::Image::null();
        }
        if !self.source_memory.is_null() {
            self.device.free_memory(self.source_memory, None);
            self.source_memory = vk::DeviceMemory::null();
        }
        self.source_extent = vk::Extent2D::default();
        self.source_format = vk::Format::UNDEFINED;
        self.staging_size = 0;
    }

    unsafe fn ensure_source(&mut self, width: u32, height: u32) -> Result<()> {
        if !self.source_image.is_null()
            && self.source_extent.width == width
            && self.source_extent.height == height
            && self.source_format == self.format
        {
            return Ok(());
        }
        self.destroy_source();
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(self.format)
            .extent(vk::Extent3D { width, height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        let image = self.device.create_image(&image_info, None)?;
        let req = self.device.get_image_memory_requirements(image);
        let mt = self.find_memory_type(req.memory_type_bits, vk::MemoryPropertyFlags::DEVICE_LOCAL)?;
        let alloc = vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt);
        let memory = self.device.allocate_memory(&alloc, None)?;
        self.device.bind_image_memory(image, memory, 0)?;
        let view_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(self.format)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
        let view = self.device.create_image_view(&view_info, None)?;

        let size = width as u64 * height as u64 * 4;
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let buffer = self.device.create_buffer(&buffer_info, None)?;
        let req = self.device.get_buffer_memory_requirements(buffer);
        let mt = self.find_memory_type(req.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)?;
        let alloc = vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt);
        let mem = self.device.allocate_memory(&alloc, None)?;
        self.device.bind_buffer_memory(buffer, mem, 0)?;
        let ptr = self.device.map_memory(mem, 0, size, vk::MemoryMapFlags::empty())? as *mut u8;

        self.source_image = image;
        self.source_view = view;
        self.source_memory = memory;
        self.write_descriptors();
        self.source_extent = vk::Extent2D { width, height };
        self.source_format = self.format;
        self.staging_buffer = buffer;
        self.staging_memory = mem;
        self.staging_ptr = ptr;
        self.staging_size = size;
        log::info!("[vk] frame source {}x{} {:?}", width, height, self.format);
        Ok(())
    }

    pub fn present_frame(&mut self, width: u32, height: u32, data: &[u8]) -> Result<()> {
        if width == 0 || height == 0 || width > 4096 || height > 4096 {
            return Ok(());
        }
        let len = width as usize * height as usize * 4;
        if data.len() < len {
            return Ok(());
        }
        unsafe {
            self.ensure_source(width, height)?;
            self.device.device_wait_idle()?;
            let dst = std::slice::from_raw_parts_mut(self.staging_ptr, len);
            match self.format {
                vk::Format::B8G8R8A8_UNORM | vk::Format::B8G8R8A8_SRGB => {
                    for i in (0..len).step_by(4) {
                        dst[i] = data[i + 2];
                        dst[i + 1] = data[i + 1];
                        dst[i + 2] = data[i];
                        dst[i + 3] = data[i + 3];
                    }
                }
                vk::Format::R8G8B8A8_UNORM | vk::Format::R8G8B8A8_SRGB => dst.copy_from_slice(&data[..len]),
                other => bail!("unsupported swapchain format {other:?}"),
            }
            self.device.wait_for_fences(&[self.in_flight], true, u64::MAX)?;
            let (index, suboptimal) = match self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available,
                vk::Fence::null(),
            ) {
                Ok(v) => v,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    self.destroy_swapchain();
                    self.create_swapchain("present out of date".into())?;
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            };
            self.device.reset_fences(&[self.in_flight])?;
            let cb = self.command_buffers[index as usize];
            self.device.reset_command_buffer(cb, vk::CommandBufferResetFlags::empty())?;
            self.device.begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
            barrier(
                cb,
                &self.device,
                self.source_image,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::AccessFlags::NONE,
                vk::AccessFlags::TRANSFER_WRITE,
            );
            let region = vk::BufferImageCopy::default()
                .buffer_offset(0)
                .buffer_row_length(width)
                .buffer_image_height(height)
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .mip_level(0)
                        .base_array_layer(0)
                        .layer_count(1),
                )
                .image_offset(vk::Offset3D::default())
                .image_extent(vk::Extent3D { width, height, depth: 1 });
            self.device.cmd_copy_buffer_to_image(
                cb,
                self.staging_buffer,
                self.source_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[region],
            );
            barrier(
                cb,
                &self.device,
                self.source_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::AccessFlags::TRANSFER_WRITE,
                vk::AccessFlags::SHADER_READ,
            );
            let clear = [vk::ClearValue { color: vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } }];
            let begin = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[index as usize])
                .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: self.extent })
                .clear_values(&clear);
            self.device.cmd_begin_render_pass(cb, &begin, vk::SubpassContents::INLINE);
            self.device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
            self.device.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, &self.desc_sets, &[]);
            let (dw, dh, dx, dy) = fit_frame(width, height, self.extent);
            let viewport = vk::Viewport {
                x: dx as f32,
                y: dy as f32,
                width: dw as f32,
                height: dh as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            };
            self.device.cmd_set_viewport(cb, 0, &[viewport]);
            let scissor = vk::Rect2D {
                offset: vk::Offset2D { x: dx, y: dy },
                extent: vk::Extent2D { width: dw as u32, height: dh as u32 },
            };
            self.device.cmd_set_scissor(cb, 0, &[scissor]);
            self.device.cmd_draw(cb, 3, 1, 0, 0);
            self.device.cmd_end_render_pass(cb);
            self.device.end_command_buffer(cb)?;
            let wait = [self.image_available];
            let stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let signal = [self.render_finished];
            let cbs = [cb];
            let submit = vk::SubmitInfo::default()
                .wait_semaphores(&wait)
                .wait_dst_stage_mask(&stages)
                .command_buffers(&cbs)
                .signal_semaphores(&signal);
            self.device.queue_submit(self.queue, &[submit], self.in_flight)?;
            let swapchains = [self.swapchain];
            let indices = [index];
            let present = vk::PresentInfoKHR::default()
                .wait_semaphores(&signal)
                .swapchains(&swapchains)
                .image_indices(&indices);
            let result = self.swapchain_loader.queue_present(self.queue, &present);
            let _ = suboptimal;
            let needs_recreate = matches!(result, Err(vk::Result::ERROR_OUT_OF_DATE_KHR));
            if needs_recreate {
                self.destroy_swapchain();
                self.create_swapchain("present suboptimal".into())?;
            } else if let Err(e) = result {
                return Err(e.into());
            }
        }
        Ok(())
    }

    fn clear_color(&self) -> [f32; 4] {
        let t = self.start.elapsed().as_secs_f32();
        [
            0.5 + 0.5 * (t * 0.7).sin(),
            0.5 + 0.5 * (t * 0.9 + 2.0).sin(),
            0.5 + 0.5 * (t * 1.1 + 4.0).sin(),
            1.0,
        ]
    }

    pub fn summary(&self) -> String {
        format!("[vk] renderer ready: {:?} {}x{} {} images", self.format, self.extent.width, self.extent.height, self.images.len())
    }

    /// Persist the driver pipeline cache so shader variants are not re-JITed
    /// on the next launch. Throttled to at most once every 5 s.
    pub fn save_pipe_cache_throttled(&mut self) {
        if self.pipe_cache == vk::PipelineCache::null() {
            return;
        }
        let Some(path) = pipe_cache_path() else { return };
        {
            static LAST: Mutex<Option<Instant>> = Mutex::new(None);
            let mut last = LAST.lock().unwrap();
            let now = Instant::now();
            if last.is_some_and(|t| now.duration_since(t).as_secs() < 5) {
                return;
            }
            *last = Some(now);
        }
        match unsafe { self.device.get_pipeline_cache_data(self.pipe_cache) } {
            Ok(data) if !data.is_empty() => {
                if let Err(e) = std::fs::write(&path, &data) {
                    log::warn!("[vk] pipeline cache write {}: {e}", path.display());
                } else {
                    log::info!("[vk] pipeline cache saved: {} KiB", data.len() >> 10);
                }
            }
            Ok(_) => {}
            Err(e) => log::warn!("[vk] pipeline cache read back: {e}"),
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            self.destroy_source();
            self.destroy_swapchain();
            self.device.destroy_semaphore(self.image_available, None);
            self.device.destroy_semaphore(self.render_finished, None);
            self.device.destroy_fence(self.in_flight, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_descriptor_pool(self.desc_pool, None);
            for layout in self.set_layouts {
                self.device.destroy_descriptor_set_layout(layout, None);
            }
            self.device.destroy_sampler(self.sampler, None);
            if !self.flags_ptr.is_null() {
                self.device.unmap_memory(self.flags_memory);
            }
            self.device.destroy_buffer(self.flags_buffer, None);
            self.device.free_memory(self.flags_memory, None);
            if let Some(g) = self.gpu.take() {
                for p in g.pipes.values() {
                    self.device.destroy_pipeline(p.pipeline, None);
                }
                for (m, _) in g.modules.values() {
                    self.device.destroy_shader_module(*m, None);
                }
                self.device.destroy_descriptor_pool(g.pool, None);
                for img in g.images.values() {
                    if !img.framebuffer.is_null() {
                        self.device.destroy_framebuffer(img.framebuffer, None);
                    }
                    if !img.clear_pass.is_null() {
                        self.device.destroy_render_pass(img.clear_pass, None);
                    }
                    if !img.load_pass.is_null() {
                        self.device.destroy_render_pass(img.load_pass, None);
                    }
                    self.device.destroy_image_view(img.view, None);
                    self.device.destroy_image(img.image, None);
                    self.device.free_memory(img.memory, None);
                }
                for smp in g.samplers.values() {
                    self.device.destroy_sampler(*smp, None);
                }
                for (image, memory, view) in g.sampled.values() {
                    self.device.destroy_image_view(*view, None);
                    self.device.destroy_image(*image, None);
                    self.device.free_memory(*memory, None);
                }
                for fb in g.framebuffers.values() {
                    self.device.destroy_framebuffer(*fb, None);
                }
                for (clear, load) in g.passes.values() {
                    if !clear.is_null() {
                        self.device.destroy_render_pass(*clear, None);
                    }
                    if !load.is_null() {
                        self.device.destroy_render_pass(*load, None);
                    }
                }
                self.device.destroy_image_view(g.dummy_view, None);
                self.device.destroy_image(g.dummy_image, None);
                self.device.free_memory(g.dummy_memory, None);
                self.device.destroy_sampler(g.dummy_sampler, None);
                for (buffer, memory) in [
                    (g.mirror, g.mirror_memory),
                    (g.zero, g.zero_memory),
                    (g.arena, g.arena_memory),
                ] {
                    self.device.unmap_memory(memory);
                    self.device.destroy_buffer(buffer, None);
                    self.device.free_memory(memory, None);
                }
            }
            self.device.destroy_descriptor_pool(self.gpu_present_pool, None);
            self.device.destroy_pipeline_layout(self.dbg_pipeline_layout, None);
            self.device.destroy_descriptor_pool(self.dbg_pool, None);
            self.device.destroy_descriptor_set_layout(self.dbg_set_layout, None);
            let _ = self.dbg_res_ptr;
            self.device.unmap_memory(self.dbg_res_memory);
            self.device.destroy_buffer(self.dbg_res_buffer, None);
            self.device.free_memory(self.dbg_res_memory, None);
            self.device.unmap_memory(self.dbg_color_memory);
            self.device.destroy_buffer(self.dbg_color_buffer, None);
            self.device.free_memory(self.dbg_color_memory, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
        }
        let _ = &self.entry;
        let _ = &self.instance;
        let _ = &self.physical_device;
        let _ = &self.queue_family;
        let _ = &self.queue;
        let _ = &self.swapchain_loader;
    }
}

#[allow(dead_code)]
fn _assert_cstr() {
    let _: &CStr = surface::NAME;
}

unsafe fn barrier(
    cb: vk::CommandBuffer,
    device: &ash::Device,
    image: vk::Image,
    old: vk::ImageLayout,
    new: vk::ImageLayout,
    src_stage: vk::PipelineStageFlags,
    dst_stage: vk::PipelineStageFlags,
    src_access: vk::AccessFlags,
    dst_access: vk::AccessFlags,
) {
    let b = vk::ImageMemoryBarrier::default()
        .old_layout(old)
        .new_layout(new)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
        .src_access_mask(src_access)
        .dst_access_mask(dst_access);
    device.cmd_pipeline_barrier(cb, src_stage, dst_stage, vk::DependencyFlags::empty(), &[], &[], &[b]);
}

fn fit_frame(w: u32, h: u32, ext: vk::Extent2D) -> (i32, i32, i32, i32) {
    let ar = w as f32 / h as f32;
    let (dw, dh) = if ext.width as f32 / ext.height as f32 > ar {
        ((ext.height as f32 * ar) as u32, ext.height)
    } else {
        (ext.width, (ext.width as f32 / ar) as u32)
    };
    let (dw, dh) = (dw.max(1), dh.max(1));
    (dw as i32, dh as i32, ((ext.width - dw) / 2) as i32, ((ext.height - dh) / 2) as i32)
}

struct Frame {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

static FRAME: Mutex<Option<Frame>> = Mutex::new(None);

pub fn submit_frame(width: u32, height: u32, data: Vec<u8>) {
    if width == 0 || height == 0 || data.len() < width as usize * height as usize * 4 {
        return;
    }
    *FRAME.lock().unwrap() = Some(Frame { width, height, data });
}

static SESSION: Mutex<Option<Session>> = Mutex::new(None);

struct Session {
    stop: Arc<AtomicBool>,
    size: Arc<Mutex<(u32, u32)>>,
    thread: Option<JoinHandle<()>>,
}

impl Session {
    fn shutdown(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        GPU_ACTIVE.store(false, Ordering::Relaxed);
    }
}

pub fn dump_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "android")]
    {
        crate::android::cache_dir()
    }
    #[cfg(not(target_os = "android"))]
    {
        Some(std::path::PathBuf::from("."))
    }
}

fn pipe_cache_path() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "android")]
    {
        crate::android::cache_dir().map(|d| d.join("pipeline.cache"))
    }
    #[cfg(not(target_os = "android"))]
    {
        std::env::var_os("NX_PIPE_CACHE").map(std::path::PathBuf::from)
    }
}

pub fn attach(window: *mut c_void, width: u32, height: u32) {
    log::info!("[vk] attach {width}x{height}");
    detach();
    let stop = Arc::new(AtomicBool::new(false));
    let size = Arc::new(Mutex::new((width.max(1), height.max(1))));
    {
        let stop = stop.clone();
        std::thread::Builder::new().name("vk-watch".into()).spawn(move || {
            let mut last = presented_now();
            let mut quiet = std::time::Instant::now();
            while !stop.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(500));
                let now = presented_now();
                if now != last {
                    last = now;
                    quiet = std::time::Instant::now();
                } else if quiet.elapsed() > Duration::from_secs(3) {
                    let (n, at) = stage_since();
                    log::warn!(
                        "[vk] watch: no present for {}s, submitted {} presented {}, stage {} ({}) for {}s",
                        quiet.elapsed().as_secs(),
                        submitted(),
                        now,
                        n,
                        STAGE_NAMES.get(n as usize).copied().unwrap_or("?"),
                        at
                    );
                    quiet = std::time::Instant::now();
                }
            }
        }).ok();
    }
    let thread = {
        let stop = stop.clone();
        let size = size.clone();
        let window = window as usize;
        std::thread::Builder::new().name("vk".into()).spawn(move || render_loop(window, stop, size)).ok()
    };
    *SESSION.lock().unwrap() = Some(Session { stop, size, thread });
}

pub fn detach() {
    log::info!("[vk] detach");
    if let Some(s) = SESSION.lock().unwrap().take() {
        s.shutdown();
    }
}

pub fn resize(width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }
    if let Some(s) = SESSION.lock().unwrap().as_ref() {
        *s.size.lock().unwrap() = (width, height);
    }
}

fn render_loop(window: usize, stop: Arc<AtomicBool>, size: Arc<Mutex<(u32, u32)>>) {
    let mut renderer = match Renderer::new(window as *mut c_void, 1280, 720) {
        Ok(r) => r,
        Err(e) => {
            log::error!("[vk] init failed: {e:#}");
            unsafe { ndk_sys::ANativeWindow_release(window as *mut _) };
            return;
        }
    };
    log::info!("{}", renderer.summary());
    GPU_ACTIVE.store(true, Ordering::Relaxed);
    while !stop.load(Ordering::Relaxed) {
        let (w, h) = *size.lock().unwrap();
        if let Err(e) = renderer.resize(w, h) {
            log::error!("[vk] resize failed: {e:#}");
            break;
        }
        let gpu = GPU_FRAME.lock().unwrap().take();
        if let Some(f) = gpu {
            let serial = SUBMITTED.load(Ordering::Relaxed);
            if let Err(e) = unsafe { renderer.gpu_present(f) } {
                log::error!("[vk] gpu frame failed: {e:#}");
            }
            mark_presented(serial);
            // With the scene-readback fallback the CPU frame carries the real
            // picture: keep the latest one and present it after every GPU frame
            // so it is the image the display ends up showing.
            if std::env::var_os("NX_SCENE_READBACK").is_some() {
                let fresh = FRAME.lock().unwrap().take();
                if fresh.is_some() {
                    *LAST_CPU.lock().unwrap() = fresh;
                }
                let keep = LAST_CPU.lock().unwrap().take();
                if let Some(cpu) = keep {
                    let _ = renderer.present_frame(cpu.width, cpu.height, &cpu.data);
                    *LAST_CPU.lock().unwrap() = Some(cpu);
                }
            }
            std::thread::sleep(Duration::from_millis(16));
            continue;
        }
        let frame = FRAME.lock().unwrap().take();
        let Some(f) = frame else {
            std::thread::sleep(Duration::from_millis(16));
            continue;
        };
        if let Err(e) = renderer.present_frame(f.width, f.height, &f.data) {
            log::error!("[vk] frame failed: {e:#}");
            break;
        }
        std::thread::sleep(Duration::from_millis(16));
    }
    drop(renderer);
    unsafe { ndk_sys::ANativeWindow_release(window as *mut _) };
    log::info!("[vk] render thread stopped");
}

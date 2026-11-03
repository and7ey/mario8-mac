// Mush8 - gfx_vk_gpu.rs
// GPU replay of NVN draws. Guest memory is mirrored through buffer device
// addresses; every draw renders into an image keyed by the guest render target,
// so later passes can sample the result instead of a dummy texture.
use super::*;
use crate::gfx_vk::GpuCopy;
use anyhow::{Context, Result, anyhow, bail};
use ash::vk;
use std::collections::HashMap;

#[repr(C)]
#[derive(Clone, Copy)]
struct GpuRes {
    cb: [u64; 18],
    vb: [u64; 16],
    vb_stride: [u32; 16],
    vb_divisor: [u32; 16],
    attr_format: [u32; 16],
    attr_stream: [u32; 16],
    attr_offset: [u32; 16],
    base_vertex: i32,
    base_instance: u32,
    flags: u32,
    alpha_ref: f32,
    alpha_func: u32,
    heap_base: u64,
    heap_gpu: u64,
    heap_size: u64,
    zero_gpu: u64,
}

impl Default for GpuRes {
    fn default() -> Self {
        Self {
            cb: [0; 18],
            vb: [0; 16],
            vb_stride: [0; 16],
            vb_divisor: [0; 16],
            attr_format: [0; 16],
            attr_stream: [0; 16],
            attr_offset: [0; 16],
            base_vertex: 0,
            base_instance: 0,
            flags: 0,
            alpha_ref: 0.0,
            alpha_func: 7,
            heap_base: 0,
            heap_gpu: 0,
            heap_size: 0,
            zero_gpu: 0,
        }
    }
}

pub(super) struct GpuImage {
    pub(super) image: vk::Image,
    pub(super) memory: vk::DeviceMemory,
    pub(super) view: vk::ImageView,
    pub(super) clear_pass: vk::RenderPass,
    pub(super) load_pass: vk::RenderPass,
    pub(super) framebuffer: vk::Framebuffer,
    pub(super) load_framebuffer: vk::Framebuffer,
    pub(super) extent: vk::Extent2D,
    pub(super) format: vk::Format,
    pub(super) rendered: bool,
    pub(super) last_frame: u64,
    pub(super) depth: bool,
    pub(super) layout: vk::ImageLayout,
}

pub(super) struct GpuState {
    pub(super) mirror: vk::Buffer,
    pub(super) mirror_memory: vk::DeviceMemory,
    pub(super) mirror_ptr: *mut u8,
    pub(super) heap_base: u64,
    pub(super) heap_size: u64,
    pub(super) mirror_addr: u64,
    pub(super) zero: vk::Buffer,
    pub(super) zero_memory: vk::DeviceMemory,
    pub(super) zero_ptr: *mut u8,
    pub(super) zero_addr: u64,
    pub(super) arena: vk::Buffer,
    pub(super) arena_memory: vk::DeviceMemory,
    pub(super) arena_ptr: *mut u8,
    pub(super) arena_addr: u64,
    pub(super) arena_used: u64,
    pub(super) arena_size: u64,
    pub(super) dummy_image: vk::Image,
    pub(super) dummy_memory: vk::DeviceMemory,
    pub(super) dummy_view: vk::ImageView,
    pub(super) dummy_sampler: vk::Sampler,
    pub(super) modules: HashMap<u64, (vk::ShaderModule, nxshader::glsl::Interface)>,
    pub(super) pipes: HashMap<(u64, u64, u32, u32, u32, u32, u32, u32), GpuPipe>,
    pub(super) compute_pipes: HashMap<u64, ComputePipe>,
    pub(super) pool: vk::DescriptorPool,
    pub(super) frame: u64,
    pub(super) features: HashMap<vk::Format, vk::FormatFeatureFlags>,
    pub(super) images: HashMap<GpuTex, GpuImage>,
    pub(super) sampled: HashMap<GpuTex, (vk::Image, vk::DeviceMemory, vk::ImageView)>,
    pub(super) samplers: HashMap<[u8; 0x60], vk::Sampler>,
    pub(super) skipped: std::collections::HashSet<GpuTex>,
    pub(super) passes: HashMap<(vk::Format, u32), (vk::RenderPass, vk::RenderPass)>,
    pub(super) target_passes: HashMap<(vk::Format, bool), vk::RenderPass>,
    pub(super) framebuffers: HashMap<(GpuTex, Option<GpuTex>), vk::Framebuffer>,
}

pub(super) struct ComputePipe {
    pipe: vk::Pipeline,
    layout: vk::PipelineLayout,
    sets: [vk::DescriptorSetLayout; 2],
    tex_keys: Vec<u32>,
    slots: Vec<u32>,
}

pub(super) struct GpuPipe {
    pub(super) pipeline: vk::Pipeline,
    pub(super) layout: vk::PipelineLayout,
    pub(super) sets: [vk::DescriptorSetLayout; 2],
    pub(super) vtex: Vec<u32>,
    pub(super) ftex: Vec<u32>,
    pub(super) vslots: Vec<u32>,
    pub(super) fslots: Vec<u32>,
    pub(super) vslot_bind: Vec<(u32, u32)>,
    pub(super) fslot_bind: Vec<(u32, u32)>,
}

fn fnv1a(b: &[u8]) -> u64 {
    b.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &x| (h ^ x as u64).wrapping_mul(0x100_0000_01b3))
}

static DRAW_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static UDP_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static BDS_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static DCMD_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static PIPE_MISSES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static DS_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);


#[cfg(mush8_shaderc)]
fn spv_cache(hash: u64) -> Option<(Vec<u32>, nxshader::glsl::Interface)> {
    let dir = std::env::var_os("NX_SPV_CACHE")?;
    let dir = std::path::PathBuf::from(dir);
    let bytes = std::fs::read(dir.join(format!("{hash:016x}.spv"))).ok()?;
    if bytes.len() % 4 != 0 {
        return None;
    }
    let words: Vec<u32> = bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    let txt = std::fs::read_to_string(dir.join(format!("{hash:016x}.iface"))).ok()?;
    let mut lines = txt.lines();
    let stage = match lines.next()?.trim() {
        "Vertex" => nxshader::sph::Stage::Vertex,
        "Fragment" => nxshader::sph::Stage::Fragment,
        _ => return None,
    };
    let _ = stage;
    let nums = |l: &str| -> Vec<u32> {
        l.trim().trim_start_matches('[').trim_end_matches(']').split(',').filter_map(|x| x.trim().parse().ok()).collect()
    };
    let tex = nums(lines.next()?);
    let slots = lines.next().map(nums).unwrap_or_default();
    let mut iface = nxshader::glsl::Interface::default();
    for b in tex {
        iface.textures.insert(b, nxshader::glsl::TexKind::T2D);
    }
    for (i, s) in slots.into_iter().enumerate() {
        iface.samplers.insert(i as u32, s);
    }
    Some((words, iface))
}

fn sorted_keys(map: &std::collections::BTreeMap<u32, nxshader::glsl::TexKind>) -> Vec<u32> {
    map.keys().copied().collect()
}

fn sorted_slots(map: &std::collections::BTreeMap<u32, u32>) -> Vec<u32> {
    let mut v: Vec<u32> = map.values().copied().collect();
    v.sort_unstable();
    v.dedup();
    v
}

fn is_depth_format(nvn: u32) -> bool {
    (49..=54).contains(&nvn)
}

fn f16_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) & 1) as u32;
    let exp = ((h >> 10) & 0x1F) as u32;
    let frac = (h & 0x3FF) as u32;
    let bits = if exp == 0 {
        if frac == 0 {
            sign << 31
        } else {
            let mut e = 127 - 15 + 1;
            let mut f = frac;
            while f & 0x400 == 0 {
                f <<= 1;
                e -= 1;
            }
            (sign << 31) | ((e as u32) << 23) | ((f & 0x3FF) << 13)
        }
    } else if exp == 0x1F {
        (sign << 31) | (0xFF << 23) | (frac << 13)
    } else {
        (sign << 31) | ((exp + 127 - 15) << 23) | (frac << 13)
    };
    f32::from_bits(bits)
}

fn vk_format(nvn: u32) -> Option<vk::Format> {
    Some(match nvn {
        1 => vk::Format::R8_UNORM,
        2 => vk::Format::R8_SNORM,
        3 => vk::Format::R8_UINT,
        4 => vk::Format::R8_SINT,
        5 => vk::Format::R16_SFLOAT,
        6 => vk::Format::R16_UNORM,
        7 => vk::Format::R16_SNORM,
        8 => vk::Format::R16_UINT,
        9 => vk::Format::R16_SINT,
        10 => vk::Format::R32_SFLOAT,
        11 => vk::Format::R32_UINT,
        12 => vk::Format::R32_SINT,
        13 => vk::Format::R8G8_UNORM,
        14 => vk::Format::R8G8_SNORM,
        15 => vk::Format::R8G8_UINT,
        16 => vk::Format::R8G8_SINT,
        17 => vk::Format::R16G16_SFLOAT,
        18 => vk::Format::R16G16_UNORM,
        19 => vk::Format::R16G16_SNORM,
        22 => vk::Format::R32G32_SFLOAT,
        23 => vk::Format::R32G32_UINT,
        24 => vk::Format::R32G32_SINT,
        38 => vk::Format::R8G8B8A8_SNORM,
        39 => vk::Format::R8G8B8A8_UINT,
        40 => vk::Format::R8G8B8A8_SINT,
        42 => vk::Format::R16G16B16A16_UNORM,
        43 => vk::Format::R16G16B16A16_SNORM,
        44 => vk::Format::R16G16B16A16_UINT,
        47 => vk::Format::R32G32B32A32_UINT,
        48 => vk::Format::R32G32B32A32_SINT,
        37 => vk::Format::R8G8B8A8_UNORM,
        41 => vk::Format::R16G16B16A16_SFLOAT,
        45 => vk::Format::R16G16B16A16_SINT,
        46 => vk::Format::R32G32B32A32_SFLOAT,
        49 => vk::Format::S8_UINT,
        50 => vk::Format::D16_UNORM,
        51 | 52 => vk::Format::D32_SFLOAT,
        53 | 54 => vk::Format::D32_SFLOAT_S8_UINT,
        55 | 56 => vk::Format::R8G8B8A8_SRGB,
        60 => vk::Format::B5G6R5_UNORM_PACK16,
        61 => vk::Format::A2B10G10R10_UNORM_PACK32,
        62 => vk::Format::A2B10G10R10_UINT_PACK32,
        63 => vk::Format::R16G16B16A16_SFLOAT,
        64 => vk::Format::R16G16B16A16_SFLOAT,
        121 => vk::Format::ASTC_4X4_UNORM_BLOCK,
        122 => vk::Format::ASTC_5X4_UNORM_BLOCK,
        123 => vk::Format::ASTC_5X5_UNORM_BLOCK,
        124 => vk::Format::ASTC_6X5_UNORM_BLOCK,
        125 => vk::Format::ASTC_6X6_UNORM_BLOCK,
        126 => vk::Format::ASTC_8X5_UNORM_BLOCK,
        127 => vk::Format::ASTC_8X6_UNORM_BLOCK,
        128 => vk::Format::ASTC_8X8_UNORM_BLOCK,
        129 => vk::Format::ASTC_10X5_UNORM_BLOCK,
        130 => vk::Format::ASTC_10X6_UNORM_BLOCK,
        131 => vk::Format::ASTC_10X8_UNORM_BLOCK,
        132 => vk::Format::ASTC_10X10_UNORM_BLOCK,
        133 => vk::Format::ASTC_12X10_UNORM_BLOCK,
        134 => vk::Format::ASTC_12X12_UNORM_BLOCK,
        135 => vk::Format::ASTC_4X4_SRGB_BLOCK,
        136 => vk::Format::ASTC_5X4_SRGB_BLOCK,
        137 => vk::Format::ASTC_5X5_SRGB_BLOCK,
        138 => vk::Format::ASTC_6X5_SRGB_BLOCK,
        139 => vk::Format::ASTC_6X6_SRGB_BLOCK,
        140 => vk::Format::ASTC_8X5_SRGB_BLOCK,
        141 => vk::Format::ASTC_8X6_SRGB_BLOCK,
        142 => vk::Format::ASTC_8X8_SRGB_BLOCK,
        143 => vk::Format::ASTC_10X5_SRGB_BLOCK,
        144 => vk::Format::ASTC_10X6_SRGB_BLOCK,
        145 => vk::Format::ASTC_10X8_SRGB_BLOCK,
        146 => vk::Format::ASTC_10X10_SRGB_BLOCK,
        147 => vk::Format::ASTC_12X10_SRGB_BLOCK,
        148 => vk::Format::ASTC_12X12_SRGB_BLOCK,
        _ => return None,
    })
}

fn target_int_kind(nvn: u32) -> (u32, u32) {
    match nvn {
        3 | 8 | 11 | 15 | 23 | 39 | 44 | 47 | 62 => (1, 0),
        4 | 9 | 12 | 16 | 24 | 40 | 45 | 48 => (0, 1),
        _ => (0, 0),
    }
}

fn sampled_vk_format(nvn: u32) -> Option<vk::Format> {
    match nvn {
        // Guest data is packed B10G11R11/E5B9G9R9; render targets use RGBA16F instead.
        63 => Some(vk::Format::B10G11R11_UFLOAT_PACK32),
        64 => Some(vk::Format::E5B9G9R9_UFLOAT_PACK32),
        _ => vk_format(nvn),
    }
}

fn blend_factor(f: u32, dual_src: bool) -> vk::BlendFactor {
    use vk::BlendFactor as B;
    match f {
        1 => B::ZERO,
        2 => B::ONE,
        3 => B::SRC_COLOR,
        4 => B::ONE_MINUS_SRC_COLOR,
        5 => B::SRC_ALPHA,
        6 => B::ONE_MINUS_SRC_ALPHA,
        7 => B::DST_ALPHA,
        8 => B::ONE_MINUS_DST_ALPHA,
        9 => B::DST_COLOR,
        10 => B::ONE_MINUS_DST_COLOR,
        11 => B::SRC_ALPHA_SATURATE,
        16 if dual_src => B::SRC1_COLOR,
        17 if dual_src => B::ONE_MINUS_SRC1_COLOR,
        18 if dual_src => B::SRC1_ALPHA,
        19 if dual_src => B::ONE_MINUS_SRC1_ALPHA,
        16 => B::SRC_COLOR,
        17 => B::ONE_MINUS_SRC_COLOR,
        18 => B::SRC_ALPHA,
        19 => B::ONE_MINUS_SRC_ALPHA,
        0x61 => B::CONSTANT_COLOR,
        0x62 => B::ONE_MINUS_CONSTANT_COLOR,
        0x63 => B::CONSTANT_ALPHA,
        0x64 => B::ONE_MINUS_CONSTANT_ALPHA,
        _ => B::ONE,
    }
}

fn blend_op(e: u32) -> vk::BlendOp {
    match e {
        2 => vk::BlendOp::SUBTRACT,
        3 => vk::BlendOp::REVERSE_SUBTRACT,
        4 => vk::BlendOp::MIN,
        5 => vk::BlendOp::MAX,
        _ => vk::BlendOp::ADD,
    }
}

fn is_int_format(f: vk::Format) -> bool {
    matches!(
        f,
        vk::Format::A2B10G10R10_UINT_PACK32
            | vk::Format::R16G16B16A16_SINT
            | vk::Format::R16G16B16A16_UINT
            | vk::Format::R8G8B8A8_UINT
            | vk::Format::R8G8B8A8_SINT
    )
}

impl Renderer {
    pub(super) unsafe fn host_buffer(&self, size: u64, usage: vk::BufferUsageFlags) -> Result<(vk::Buffer, vk::DeviceMemory, *mut u8)> {
        let info = vk::BufferCreateInfo::default().size(size).usage(usage).sharing_mode(vk::SharingMode::EXCLUSIVE);
        let buffer = self.device.create_buffer(&info, None)?;
        let req = self.device.get_buffer_memory_requirements(buffer);
        let props = self.instance.get_physical_device_memory_properties(self.physical_device);
        let mt = (0..props.memory_type_count)
            .find(|&i| {
                req.memory_type_bits & (1 << i) != 0
                    && props.memory_types[i as usize]
                        .property_flags
                        .contains(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
            })
            .ok_or_else(|| anyhow!("no host-visible memory"))?;
        let memory = self
            .device
            .allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt), None)?;
        self.device.bind_buffer_memory(buffer, memory, 0)?;
        let ptr = self.device.map_memory(memory, 0, size, vk::MemoryMapFlags::empty())? as *mut u8;
        Ok((buffer, memory, ptr))
    }

    pub(super) unsafe fn gpu_addr(&self, buffer: vk::Buffer) -> u64 {
        match &self.bda {
            Some(b) => b.get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer)),
            None => 0,
        }
    }

    pub(super) unsafe fn one_shot_begin(&self) -> Result<vk::CommandBuffer> {
        let cb = self.device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(self.command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1),
        )?[0];
        self.device
            .begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT))?;
        Ok(cb)
    }

    pub(super) unsafe fn one_shot_end(&self, cb: vk::CommandBuffer) -> Result<()> {
        self.device.end_command_buffer(cb)?;
        let cbs = [cb];
        self.device
            .queue_submit(self.queue, &[vk::SubmitInfo::default().command_buffers(&cbs)], vk::Fence::null())?;
        self.device.queue_wait_idle(self.queue)?;
        self.device.free_command_buffers(self.command_pool, &[cb]);
        Ok(())
    }

    pub(super) unsafe fn ensure_gpu(&mut self) -> Result<()> {
        if self.gpu.is_some() {
            return Ok(());
        }
        let Some((heap_base, heap_size)) = crate::gfx::heap() else { bail!("guest heap is not registered") };
        if self.bda.is_none() {
            bail!("buffer device address is not supported");
        }
        let usage = vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
            | vk::BufferUsageFlags::TRANSFER_SRC
            | vk::BufferUsageFlags::TRANSFER_DST;
        let (mirror, mirror_memory, mirror_ptr) = self.host_buffer(heap_size, usage).context("guest mirror")?;
        let mirror_addr = self.gpu_addr(mirror);
        let (zero, zero_memory, zero_ptr) = self.host_buffer(0x1_0000, usage).context("zero page")?;
        std::ptr::write_bytes(zero_ptr, 0, 0x1_0000);
        let zero_addr = self.gpu_addr(zero);
        let arena_size = 16 << 20;
        let (arena, arena_memory, arena_ptr) = self
            .host_buffer(arena_size, usage | vk::BufferUsageFlags::INDEX_BUFFER)
            .context("gpu arena")?;
        let arena_addr = self.gpu_addr(arena);
        let dummy_image = self.device.create_image(
            &vk::ImageCreateInfo::default()
                .image_type(vk::ImageType::TYPE_2D)
                .format(vk::Format::R8G8B8A8_UNORM)
                .extent(vk::Extent3D { width: 1, height: 1, depth: 1 })
                .mip_levels(1)
                .array_layers(1)
                .samples(vk::SampleCountFlags::TYPE_1)
                .tiling(vk::ImageTiling::OPTIMAL)
                .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .initial_layout(vk::ImageLayout::UNDEFINED),
            None,
        )?;
        let req = self.device.get_image_memory_requirements(dummy_image);
        let props = self.instance.get_physical_device_memory_properties(self.physical_device);
        let mt = (0..props.memory_type_count)
            .find(|&i| req.memory_type_bits & (1 << i) != 0 && props.memory_types[i as usize].property_flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL))
            .unwrap_or(0);
        let dummy_memory = self
            .device
            .allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt), None)?;
        self.device.bind_image_memory(dummy_image, dummy_memory, 0)?;
        {
            let cb = self.one_shot_begin()?;
            barrier(
                cb,
                &self.device,
                dummy_image,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::AccessFlags::NONE,
                vk::AccessFlags::TRANSFER_WRITE,
            );
            self.device.cmd_copy_buffer_to_image(
                cb,
                zero,
                dummy_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[vk::BufferImageCopy::default()
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .layer_count(1),
                    )
                    .image_extent(vk::Extent3D { width: 1, height: 1, depth: 1 })],
            );
            barrier(
                cb,
                &self.device,
                dummy_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::AccessFlags::TRANSFER_WRITE,
                vk::AccessFlags::SHADER_READ,
            );
            self.one_shot_end(cb)?;
        }
        let dummy_view = self.device.create_image_view(
            &vk::ImageViewCreateInfo::default()
                .image(dummy_image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(vk::Format::R8G8B8A8_UNORM)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                }),
            None,
        )?;
        let dummy_sampler = self.device.create_sampler(
            &vk::SamplerCreateInfo::default()
                .mag_filter(vk::Filter::LINEAR)
                .min_filter(vk::Filter::LINEAR)
                .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE),
            None,
        )?;
        let sizes = [
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1 << 17),
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1 << 18),
            vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLER).descriptor_count(1 << 17),
        ];
        let pool = self
            .device
            .create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(1 << 16).pool_sizes(&sizes), None)?;
        let (staging, staging_mem, staging_ptr) = self.host_buffer(4, vk::BufferUsageFlags::TRANSFER_SRC)?;
        for i in 0..4 {
            *staging_ptr.add(i) = 0xff;
        }
        let cb = self.one_shot_begin()?;
        barrier(
            cb,
            &self.device,
            dummy_image,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::NONE,
            vk::AccessFlags::TRANSFER_WRITE,
        );
        self.device.cmd_copy_buffer_to_image(
            cb,
            staging,
            dummy_image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[vk::BufferImageCopy::default()
                .image_subresource(vk::ImageSubresourceLayers::default().aspect_mask(vk::ImageAspectFlags::COLOR).layer_count(1))
                .image_extent(vk::Extent3D { width: 1, height: 1, depth: 1 })],
        );
        barrier(
            cb,
            &self.device,
            dummy_image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::SHADER_READ,
        );
        self.one_shot_end(cb)?;
        self.device.unmap_memory(staging_mem);
        self.device.destroy_buffer(staging, None);
        self.device.free_memory(staging_mem, None);
        log::info!("[vk] gpu memory ready: heap {heap_base:#x}+{heap_size:#x}, mirror {mirror_addr:#x}");
        self.gpu = Some(GpuState {
            mirror,
            mirror_memory,
            mirror_ptr,
            heap_base,
            heap_size,
            mirror_addr,
            zero,
            zero_memory,
            zero_ptr,
            zero_addr,
            arena,
            arena_memory,
            arena_ptr,
            arena_addr,
            arena_used: 0,
            arena_size,
            dummy_image,
            dummy_memory,
            dummy_view,
            dummy_sampler,
            modules: HashMap::new(),
            compute_pipes: HashMap::new(),
            pipes: HashMap::new(),
            pool,
            frame: 0,
            features: HashMap::new(),
            images: HashMap::new(),
            sampled: HashMap::new(),
            samplers: HashMap::new(),
            skipped: Default::default(),
            passes: HashMap::new(),
            target_passes: HashMap::new(),
            framebuffers: HashMap::new(),
        });
        Ok(())
    }

    fn mirror_copy(g: &GpuState, addr: u64, size: u64) {
        if addr == 0 || size == 0 || addr < g.heap_base {
            return;
        }
        let off = addr - g.heap_base;
        if off >= g.heap_size {
            return;
        }
        let n = size.min(g.heap_size - off);
        unsafe { std::ptr::copy_nonoverlapping(addr as *const u8, g.mirror_ptr.add(off as usize), n as usize) };
    }

    fn mirror_addr(g: &GpuState, addr: u64) -> u64 {
        if addr >= g.heap_base && addr < g.heap_base + g.heap_size {
            g.mirror_addr + (addr - g.heap_base)
        } else {
            g.zero_addr
        }
    }

    unsafe fn arena_write_at(&mut self, bytes: &[u8], align: u64) -> Result<(u64, u64)> {
        let (ptr, addr, size, mut used) = {
            let g = self.gpu.as_ref().unwrap();
            (g.arena_ptr, g.arena_addr, g.arena_size, g.arena_used)
        };
        let off = (used + align - 1) & !(align - 1);
        if off + bytes.len() as u64 > size {
            bail!("gpu arena full");
        }
        used = off + bytes.len() as u64;
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.add(off as usize), bytes.len());
        self.gpu.as_mut().unwrap().arena_used = used;
        Ok((off, addr + off))
    }

    unsafe fn arena_write(&mut self, bytes: &[u8], align: u64) -> Result<u64> {
        Ok(self.arena_write_at(bytes, align)?.1)
    }

    unsafe fn gpu_module(&mut self, blob: &[u8], uint_targets: u32, sint_targets: u32) -> Result<(u64, nxshader::glsl::Interface)> {
        let hash = (fnv1a(blob) | 1) ^ ((uint_targets as u64) << 56) ^ ((sint_targets as u64) << 57);
        if let Some((_, iface)) = self.gpu.as_ref().unwrap().modules.get(&hash) {
            return Ok((hash, iface.clone()));
        }
        #[cfg(mush8_shaderc)]
        if let Some((words, iface)) = spv_cache(hash) {
            let module = self.device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None)?;
            self.gpu.as_mut().unwrap().modules.insert(hash, (module, iface.clone()));
            return Ok((hash, iface));
        }
        #[cfg(not(mush8_shaderc))]
        bail!("on-device GLSL compilation is unavailable");
        #[cfg(mush8_shaderc)]
        {
            let t = nxshader::glsl::translate_with(blob, 0, uint_targets, sint_targets).context("NVN -> GLSL")?;
            if let Some(dir) = std::env::var_os("NX_DUMP_GLSL") {
                let dir = std::path::PathBuf::from(dir);
                let _ = std::fs::create_dir_all(&dir);
                let _ = std::fs::write(dir.join(format!("{hash:016x}.glsl")), &t.source);
                let tex: Vec<u32> = t.interface.textures.keys().copied().collect();
                let smp: Vec<u32> = t.interface.samplers.values().copied().collect();
                let _ = std::fs::write(dir.join(format!("{hash:016x}.iface")), format!("{:?}\n{tex:?}\n{smp:?}\n", t.stage));
            }
            let kind = match t.stage {
                nxshader::sph::Stage::Vertex => 0,
                nxshader::sph::Stage::Fragment => 1,
                other => bail!("unsupported stage {other:?}"),
            };
            let words = crate::shaderc::compile(kind, &t.source, "game").context("shaderc")?;
            let module = self
                .device
                .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None)?;
            self.gpu.as_mut().unwrap().modules.insert(hash, (module, t.interface.clone()));
            Ok((hash, t.interface))
        }
    }

    unsafe fn format_features(&mut self, format: vk::Format) -> vk::FormatFeatureFlags {
        if let Some(&f) = self.gpu.as_ref().unwrap().features.get(&format) {
            return f;
        }
        let props = self.instance.get_physical_device_format_properties(self.physical_device, format);
        let f = props.optimal_tiling_features;
        self.gpu.as_mut().unwrap().features.insert(format, f);
        f
    }

    unsafe fn gpu_pipe(&mut self, vsh: u64, fsh: u64, topology: u32, pass: vk::RenderPass, format: u32, depth_nvn: u32, blend_lo: u32, blend_hi: u32, write_mask: u32) -> Result<vk::Pipeline> {
        let key = (vsh, fsh, topology, format, depth_nvn, blend_lo, blend_hi, write_mask);
        if let Some(p) = self.gpu.as_ref().unwrap().pipes.get(&key) {
            return Ok(p.pipeline);
        }
        PIPE_MISSES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (vmodule, viface, fmodule, fiface) = {
            let modules = &self.gpu.as_ref().unwrap().modules;
            let (vm, vi) = &modules[&vsh];
            let (fm, fi) = &modules[&fsh];
            (*vm, vi.clone(), *fm, fi.clone())
        };
        let vtex = sorted_keys(&viface.textures);
        let ftex = sorted_keys(&fiface.textures);
        let vslots = sorted_slots(&viface.samplers);
        let fslots = sorted_slots(&fiface.samplers);
        let vslot_bind: Vec<(u32, u32)> = viface.samplers.iter().map(|(&b, &s)| (s, b)).collect();
        let fslot_bind: Vec<(u32, u32)> = fiface.samplers.iter().map(|(&b, &s)| (s, b)).collect();
        let mut set0 = vec![
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX),
            vk::DescriptorSetLayoutBinding::default()
                .binding(32)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        ];
        for &b in &vtex {
            set0.push(
                vk::DescriptorSetLayoutBinding::default()
                    .binding(1 + b)
                    .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::VERTEX),
            );
        }
        for &b in &ftex {
            set0.push(
                vk::DescriptorSetLayoutBinding::default()
                    .binding(33 + b)
                    .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT),
            );
        }
        let mut set1: Vec<vk::DescriptorSetLayoutBinding> = vslots
            .iter()
            .map(|&slot| {
                vk::DescriptorSetLayoutBinding::default()
                    .binding(slot)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::VERTEX)
            })
            .chain(fslots.iter().map(|&slot| {
                vk::DescriptorSetLayoutBinding::default()
                    .binding(32 + slot)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            }))
            .collect();
        if set1.is_empty() {
            set1.push(
                vk::DescriptorSetLayoutBinding::default()
                    .binding(0)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT),
            );
        }
        let sets = [
            self.device
                .create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&set0), None)?,
            self.device
                .create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&set1), None)?,
        ];
        let layout = self
            .device
            .create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&sets), None)?;
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vmodule)
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fmodule)
                .name(c"main"),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(match topology {
            0 => vk::PrimitiveTopology::POINT_LIST,
            1 => vk::PrimitiveTopology::LINE_LIST,
            3 => vk::PrimitiveTopology::LINE_STRIP,
            5 => vk::PrimitiveTopology::TRIANGLE_STRIP,
            _ => vk::PrimitiveTopology::TRIANGLE_LIST,
        });
        let viewport_state = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let mut mask = vk::ColorComponentFlags::empty();
        for (bit, flag) in [
            (1u32, vk::ColorComponentFlags::R),
            (2, vk::ColorComponentFlags::G),
            (4, vk::ColorComponentFlags::B),
            (8, vk::ColorComponentFlags::A),
        ] {
            if write_mask & bit != 0 {
                mask |= flag;
            }
        }
        // Depth-only shaders have no color output; the dummy one must not write.
        if fiface.color_targets == 0 {
            static SEEN: std::sync::Mutex<Option<std::collections::HashSet<u64>>> = std::sync::Mutex::new(None);
            if SEEN.lock().unwrap().get_or_insert_with(Default::default).insert(fsh) {
                log::info!("[vk] no color targets: fs {fsh:016x} writes_depth {} mask {}", fiface.writes_depth, write_mask);
            }
            mask = vk::ColorComponentFlags::empty();
        }
        let vkf = vk_format(format).unwrap_or(vk::Format::UNDEFINED);
        let color_props = self.format_features(vkf);
        if !color_props.contains(vk::FormatFeatureFlags::COLOR_ATTACHMENT) {
            bail!("color format {format} ({vkf:?}) cannot be a render target, features {color_props:?}");
        }
        let blend_on = write_mask & 0x10 != 0
            && !matches!(format, 63 | 64)
            && !is_int_format(vk_format(format).unwrap_or(vk::Format::UNDEFINED))
            && color_props.contains(vk::FormatFeatureFlags::COLOR_ATTACHMENT_BLEND);
        let mut attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(mask)
            .blend_enable(blend_on)
            .src_color_blend_factor(blend_factor(blend_hi & 0x7F, self.dual_src_blend))
            .dst_color_blend_factor(blend_factor(blend_hi >> 8 & 0x7F, self.dual_src_blend))
            .color_blend_op(blend_op(blend_lo >> 24 & 0xF))
            .src_alpha_blend_factor(blend_factor(blend_hi >> 16 & 0x7F, self.dual_src_blend))
            .dst_alpha_blend_factor(blend_factor(blend_hi >> 24 & 0x7F, self.dual_src_blend))
            .alpha_blend_op(blend_op(blend_lo >> 28 & 0xF));
        if !blend_on {
            attachment = attachment.src_color_blend_factor(vk::BlendFactor::ONE).dst_color_blend_factor(vk::BlendFactor::ZERO);
        }
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(std::slice::from_ref(&attachment));
        let dynamic_states = [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::BLEND_CONSTANTS,
        ];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
        let mut depth_state = vk::PipelineDepthStencilStateCreateInfo::default();
        if depth_nvn != 0 {
            depth_state = depth_state
                .depth_test_enable(true)
                .depth_write_enable(true)
                .depth_compare_op(vk::CompareOp::LESS)
                .depth_bounds_test_enable(false)
                .stencil_test_enable(false)
                .min_depth_bounds(0.0)
                .max_depth_bounds(1.0);
        }
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .dynamic_state(&dynamic)
            .depth_stencil_state(&depth_state)
            .layout(layout)
            .render_pass(pass)
            .subpass(0);
        let pipe_start = std::time::Instant::now();
        let pipeline = self
            .device
            .create_graphics_pipelines(self.pipe_cache, &[info], None)
            .map_err(|(_, e)| anyhow!(
                "create draw pipeline: {e} (color fmt {format} -> {vkf:?} feats {color_props:?} ct {:#x} wd {}, depth {depth_nvn}, blend {blend_lo:08x}/{blend_hi:08x} on {blend_on}, mask {write_mask:x}, vs {vsh:016x} fs {fsh:016x}, topo {topology})",
                fiface.color_targets,
                fiface.writes_depth
            ))?[0];
        {
            let ms = pipe_start.elapsed().as_millis();
            if ms > 20 {
                log::info!("[vk] pipeline compiled in {ms} ms");
            }
        }
        self.gpu
            .as_mut()
            .unwrap()
            .pipes
            .insert(key, GpuPipe { pipeline, layout, sets, vtex, ftex, vslots, fslots, vslot_bind, fslot_bind });
        self.save_pipe_cache_throttled();
        Ok(pipeline)
    }

    /// Debug helper: copy an image to host memory and write it raw, so the
    /// contents can be inspected offline (enabled with NX_DUMP_GPU).
    pub(super) unsafe fn dump_image(&self, tex: GpuTex, dir: &std::path::Path, tag: &str) {
        let Some(g) = self.gpu.as_ref() else { return };
        let Some(img) = g.images.get(&tex) else { return };
        let (w, h) = (img.extent.width, img.extent.height);
        let (name, bpp) = match img.format {
            vk::Format::R8G8B8A8_UNORM => ("rgba8", 4u64),
            vk::Format::R8G8B8A8_SRGB => ("rgba8s", 4),
            vk::Format::A2B10G10R10_UNORM_PACK32 => ("rgb10a2", 4),
            vk::Format::R16G16B16A16_SFLOAT => ("rgba16f", 8),
            _ => return,
        };
        if img.layout == vk::ImageLayout::UNDEFINED {
            return;
        }
        let size = bpp * w as u64 * h as u64;
        let path = dir.join(format!("gpu{tag}-{name}-{w}x{h}-{:x}.raw", tex.addr));
        let r = (|| -> Result<()> {
            let buffer = self.device.create_buffer(
                &vk::BufferCreateInfo::default().size(size).usage(vk::BufferUsageFlags::TRANSFER_DST),
                None,
            )?;
            let req = self.device.get_buffer_memory_requirements(buffer);
            let props = self.instance.get_physical_device_memory_properties(self.physical_device);
            let mt = (0..props.memory_type_count)
                .find(|&i| {
                    req.memory_type_bits & (1 << i) != 0
                        && props.memory_types[i as usize]
                            .property_flags
                            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
                })
                .unwrap_or(0);
            let memory = self.device.allocate_memory(
                &vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt),
                None,
            )?;
            self.device.bind_buffer_memory(buffer, memory, 0)?;
            let pool = self.device.create_command_pool(
                &vk::CommandPoolCreateInfo::default().queue_family_index(self.queue_family),
                None,
            )?;
            let cb = self.device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1),
            )?[0];
            self.device.begin_command_buffer(
                cb,
                &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
            )?;
            let region = vk::BufferImageCopy::default()
                .buffer_offset(0)
                .buffer_row_length(0)
                .buffer_image_height(0)
                .image_subresource(vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
                .image_extent(vk::Extent3D { width: w, height: h, depth: 1 });
            self.device.cmd_copy_image_to_buffer(cb, img.image, img.layout, buffer, &[region]);
            self.device.end_command_buffer(cb)?;
            let fence = self.device.create_fence(&vk::FenceCreateInfo::default(), None)?;
            self.device.queue_submit(self.queue, &[vk::SubmitInfo::default().command_buffers(&[cb])], fence)?;
            self.device.wait_for_fences(&[fence], true, u64::MAX)?;
            let ptr = self.device.map_memory(memory, 0, size, vk::MemoryMapFlags::empty())?;
            let data = std::slice::from_raw_parts(ptr as *const u8, size as usize);
            std::fs::write(&path, data)?;
            self.device.unmap_memory(memory);
            self.device.destroy_fence(fence, None);
            self.device.destroy_command_pool(pool, None);
            self.device.destroy_buffer(buffer, None);
            self.device.free_memory(memory, None);
            Ok(())
        })();
        match r {
            Ok(()) => log::info!("[vk] dumped {}", path.display()),
            Err(e) => log::warn!("[vk] dump failed: {e:#}"),
        }
    }

    /// Copy a rendered image to host memory as RGBA8 and hand it to the CPU
    /// present path (NX_SCENE_READBACK): a way to see the scene while the
    /// guest's own composite blit is not landing.
    pub(super) unsafe fn present_scene_readback(&self) {
        let Some(g) = self.gpu.as_ref() else { return };
        let Some((tex, img)) = g
            .images
            .iter()
            .filter(|(k, i)| !i.depth && k.format == 63 && k.width >= 1280 && i.last_frame > 0)
            .max_by_key(|(k, i)| (k.width == 1440, i.last_frame))
        else {
            return;
        };
        let (w, h) = (img.extent.width, img.extent.height);
        let bpp: u64 = match img.format {
            vk::Format::R16G16B16A16_SFLOAT => 8,
            vk::Format::R8G8B8A8_UNORM | vk::Format::R8G8B8A8_SRGB => 4,
            _ => return,
        };
        let size = bpp * w as u64 * h as u64;
        let buffer = match self.device.create_buffer(
            &vk::BufferCreateInfo::default().size(size).usage(vk::BufferUsageFlags::TRANSFER_DST),
            None,
        ) {
            Ok(b) => b,
            Err(_) => return,
        };
        let req = self.device.get_buffer_memory_requirements(buffer);
        let props = self.instance.get_physical_device_memory_properties(self.physical_device);
        let mt = (0..props.memory_type_count)
            .find(|&i| {
                req.memory_type_bits & (1 << i) != 0
                    && props.memory_types[i as usize]
                        .property_flags
                        .contains(vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)
            })
            .unwrap_or(0);
        let Ok(memory) = self.device.allocate_memory(
            &vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt),
            None,
        ) else {
            return;
        };
        let _ = self.device.bind_buffer_memory(buffer, memory, 0);
        let Ok(pool) = self.device.create_command_pool(
            &vk::CommandPoolCreateInfo::default().queue_family_index(self.queue_family),
            None,
        ) else { return };
        let Ok(cbs) = self.device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1),
        ) else { return };
        let cb = cbs[0];
        let _ = self.device.begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default());
        let region = vk::BufferImageCopy::default()
            .image_subresource(vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            })
            .image_extent(vk::Extent3D { width: w, height: h, depth: 1 });
        self.device.cmd_copy_image_to_buffer(cb, img.image, img.layout, buffer, &[region]);
        let _ = self.device.end_command_buffer(cb);
        let Ok(fence) = self.device.create_fence(&vk::FenceCreateInfo::default(), None) else { return };
        let _ = self.device.queue_submit(self.queue, &[vk::SubmitInfo::default().command_buffers(&[cb])], fence);
        let _ = self.device.wait_for_fences(&[fence], true, u64::MAX);
        if let Ok(ptr) = self.device.map_memory(memory, 0, size, vk::MemoryMapFlags::empty()) {
            let raw = std::slice::from_raw_parts(ptr as *const u8, size as usize);
            let mut rgba = vec![0u8; (w * h * 4) as usize];
            for i in 0..(w * h) as usize {
                if bpp == 8 {
                    let half = |o: usize| f16_to_f32(u16::from_le_bytes([raw[8 * i + o], raw[8 * i + o + 1]]));
                    let f = |x: f32| ((x * 3.0).max(0.0).min(1.0).powf(1.0 / 2.2) * 255.0) as u8;
                    rgba[4 * i] = f(half(0));
                    rgba[4 * i + 1] = f(half(2));
                    rgba[4 * i + 2] = f(half(4));
                    rgba[4 * i + 3] = 255;
                } else {
                    rgba[4 * i] = raw[4 * i];
                    rgba[4 * i + 1] = raw[4 * i + 1];
                    rgba[4 * i + 2] = raw[4 * i + 2];
                    rgba[4 * i + 3] = 255;
                }
            }
            self.device.unmap_memory(memory);
            crate::gfx_vk::submit_frame(w, h, rgba);
        }
        self.device.destroy_fence(fence, None);
        self.device.destroy_command_pool(pool, None);
        self.device.destroy_buffer(buffer, None);
        self.device.free_memory(memory, None);
        let _ = tex;
    }

    unsafe fn ensure_image(&mut self, tex: GpuTex) -> Result<()> {
        if self.gpu.as_ref().unwrap().images.contains_key(&tex) {
            return Ok(());
        }
        let Some(format) = vk_format(tex.format) else {
            bail!("no Vulkan format for NVN format {}", tex.format);
        };
        let depth = is_depth_format(tex.format);
        let aspect = if depth {
            match tex.format {
                53 | 54 => vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL,
                49 => vk::ImageAspectFlags::STENCIL,
                _ => vk::ImageAspectFlags::DEPTH,
            }
        } else {
            vk::ImageAspectFlags::COLOR
        };
        let usage = if depth {
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED
        } else {
            vk::ImageUsageFlags::COLOR_ATTACHMENT
                | vk::ImageUsageFlags::SAMPLED
                | vk::ImageUsageFlags::TRANSFER_SRC
                | vk::ImageUsageFlags::TRANSFER_DST
        };
        let image = self.device.create_image(
            &vk::ImageCreateInfo::default()
                .image_type(vk::ImageType::TYPE_2D)
                .format(format)
                .extent(vk::Extent3D { width: tex.width, height: tex.height, depth: 1 })
                .mip_levels(1)
                .array_layers(1)
                .samples(vk::SampleCountFlags::TYPE_1)
                .tiling(vk::ImageTiling::OPTIMAL)
                .usage(usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .initial_layout(vk::ImageLayout::UNDEFINED),
            None,
        )?;
        let req = self.device.get_image_memory_requirements(image);
        let props = self.instance.get_physical_device_memory_properties(self.physical_device);
        let mt = (0..props.memory_type_count)
            .find(|&i| req.memory_type_bits & (1 << i) != 0 && props.memory_types[i as usize].property_flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL))
            .unwrap_or(0);
        let memory = self
            .device
            .allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt), None)?;
        self.device.bind_image_memory(image, memory, 0)?;
        let view = self.device.create_image_view(
            &vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: aspect,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                }),
            None,
        )?;
        let (clear_pass, load_pass, framebuffer, load_framebuffer) = if depth {
            (vk::RenderPass::null(), vk::RenderPass::null(), vk::Framebuffer::null(), vk::Framebuffer::null())
        } else {
            let clear_pass = self.target_pass(format, vk::AttachmentLoadOp::CLEAR)?;
            let load_pass = self.target_pass(format, vk::AttachmentLoadOp::LOAD)?;
            let attachments = [view];
            let framebuffer = self.device.create_framebuffer(
                &vk::FramebufferCreateInfo::default()
                    .render_pass(clear_pass)
                    .attachments(&attachments)
                    .width(tex.width)
                    .height(tex.height)
                    .layers(1),
                None,
            )?;
            let load_framebuffer = framebuffer;
            (clear_pass, load_pass, framebuffer, load_framebuffer)
        };
        log::info!("[vk] gpu image {}x{} fmt {} -> {:?}{}", tex.width, tex.height, tex.format, format, if depth { " (depth)" } else { "" });
        self.gpu.as_mut().unwrap().images.insert(
            tex,
            GpuImage { image, memory, view, clear_pass, load_pass, framebuffer, load_framebuffer, extent: vk::Extent2D { width: tex.width, height: tex.height }, format, rendered: false, last_frame: 0, depth, layout: vk::ImageLayout::UNDEFINED },
        );
        Ok(())
    }

    unsafe fn combined_pass(&mut self, color: vk::Format, depth_nvn: u32, load: vk::AttachmentLoadOp) -> Result<vk::RenderPass> {
        if let Some(&(clear, load_pass)) = self.gpu.as_ref().unwrap().passes.get(&(color, depth_nvn)) {
            if load == vk::AttachmentLoadOp::CLEAR {
                return Ok(clear);
            }
            if load_pass != vk::RenderPass::null() {
                return Ok(load_pass);
            }
        }
        let depth = vk_format(depth_nvn).ok_or_else(|| anyhow!("no depth format"))?;
        let color_att = vk::AttachmentDescription::default()
            .format(color)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(load)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let depth_att = vk::AttachmentDescription::default()
            .format(depth)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(if load == vk::AttachmentLoadOp::CLEAR { vk::AttachmentLoadOp::CLEAR } else { vk::AttachmentLoadOp::LOAD })
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL);
        let color_ref = [vk::AttachmentReference::default().attachment(0).layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let depth_ref = [vk::AttachmentReference::default().attachment(1).layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)];
        let subpass = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_ref)
            .depth_stencil_attachment(&depth_ref[0])];
        let dep = [vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)];
        let pass = self.device.create_render_pass(
            &vk::RenderPassCreateInfo::default().attachments(&[color_att, depth_att]).subpasses(&subpass).dependencies(&dep),
            None,
        )?;
        let entry = self.gpu.as_mut().unwrap().passes.entry((color, depth_nvn)).or_insert((pass, vk::RenderPass::null()));
        if load == vk::AttachmentLoadOp::CLEAR {
            entry.0 = pass;
            Ok(pass)
        } else {
            entry.1 = pass;
            Ok(pass)
        }
    }

    unsafe fn combined_framebuffer(&mut self, target: GpuTex, depth: GpuTex, pass: vk::RenderPass) -> Result<vk::Framebuffer> {
        if let Some(&fb) = self.gpu.as_ref().unwrap().framebuffers.get(&(target, Some(depth))) {
            return Ok(fb);
        }
        let (color_view, extent) = {
            let g = self.gpu.as_ref().unwrap();
            (g.images[&target].view, g.images[&target].extent)
        };
        let depth_view = self.gpu.as_ref().unwrap().images[&depth].view;
        let attachments = [color_view, depth_view];
        let fb = self.device.create_framebuffer(
            &vk::FramebufferCreateInfo::default()
                .render_pass(pass)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1),
            None,
        )?;
        self.gpu.as_mut().unwrap().framebuffers.insert((target, Some(depth)), fb);
        Ok(fb)
    }

    /// Clear/load render passes are format-only, so share them across every
    /// target of that format: pipelines are keyed by pass, and per-image passes
    /// would multiply the pipeline cache.
    unsafe fn target_pass(&mut self, format: vk::Format, load: vk::AttachmentLoadOp) -> Result<vk::RenderPass> {
        let clear = load == vk::AttachmentLoadOp::CLEAR;
        if let Some(&p) = self.gpu.as_ref().unwrap().target_passes.get(&(format, clear)) {
            return Ok(p);
        }
        let pass = Self::make_target_pass(&self.device, format, load)?;
        self.gpu.as_mut().unwrap().target_passes.insert((format, clear), pass);
        Ok(pass)
    }

    unsafe fn make_target_pass(device: &ash::Device, format: vk::Format, load: vk::AttachmentLoadOp) -> Result<vk::RenderPass> {
        let attachment = vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(load)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let color_ref = [vk::AttachmentReference::default().attachment(0).layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let subpass = [vk::SubpassDescription::default().pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS).color_attachments(&color_ref)];
        let dep = [vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)];
        Ok(device.create_render_pass(
            &vk::RenderPassCreateInfo::default().attachments(&[attachment]).subpasses(&subpass).dependencies(&dep),
            None,
        )?)
    }

    unsafe fn make_gpu_sampler(&self, b: &[u8; 0x60]) -> vk::Sampler {
        let u32_at = |o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        let f32_at = |o: usize| f32::from_bits(u32_at(o));
        let min = u32_at(8);
        let min_filter = if matches!(min, 1 | 3 | 5) { vk::Filter::LINEAR } else { vk::Filter::NEAREST };
        let mag_filter = if u32_at(0xc) == 1 { vk::Filter::LINEAR } else { vk::Filter::NEAREST };
        let mipmap_mode = match min {
            4 | 5 => vk::SamplerMipmapMode::LINEAR,
            2 | 3 => vk::SamplerMipmapMode::NEAREST,
            _ => vk::SamplerMipmapMode::NEAREST,
        };
        let wrap = |w: u32| match w {
            1 => vk::SamplerAddressMode::REPEAT,
            2..=4 => vk::SamplerAddressMode::CLAMP_TO_EDGE,
            5 => vk::SamplerAddressMode::CLAMP_TO_BORDER,
            6 => vk::SamplerAddressMode::MIRRORED_REPEAT,
            _ => vk::SamplerAddressMode::CLAMP_TO_EDGE,
        };
        let (lo, hi) = (f32_at(0x1c), f32_at(0x20));
        let min_lod = if lo.is_finite() { lo.max(0.0) } else { 0.0 };
        let max_lod = if !matches!(min, 2..=5) {
            0.0
        } else if hi.is_finite() && lo.is_finite() && lo <= hi {
            hi
        } else {
            vk::LOD_CLAMP_NONE
        };
        let border = [f32_at(0x30), f32_at(0x34), f32_at(0x38), f32_at(0x3c)];
        let border_color = if border == [1.0; 4] {
            vk::BorderColor::FLOAT_OPAQUE_WHITE
        } else if border[3] == 1.0 {
            vk::BorderColor::FLOAT_OPAQUE_BLACK
        } else {
            vk::BorderColor::FLOAT_TRANSPARENT_BLACK
        };
        let aniso = f32_at(0x40);
        let anisotropy = aniso.is_finite() && aniso >= 1.0;
        let info = vk::SamplerCreateInfo::default()
            .mag_filter(mag_filter)
            .min_filter(min_filter)
            .mipmap_mode(mipmap_mode)
            .address_mode_u(wrap(u32_at(0x10)))
            .address_mode_v(wrap(u32_at(0x14)))
            .address_mode_w(wrap(u32_at(0x18)))
            .min_lod(min_lod)
            .max_lod(max_lod)
            .border_color(border_color)
            .anisotropy_enable(anisotropy)
            .max_anisotropy(if anisotropy { (aniso as f32).clamp(1.0, 16.0) } else { 1.0 });
        self.device.create_sampler(&info, None).unwrap_or_else(|_| self.gpu.as_ref().unwrap().dummy_sampler)
    }

    unsafe fn sampler_for(&mut self, d: &GpuDraw, stage: usize, b: u32) -> vk::Sampler {
        let h = d.textures[stage].get(b as usize).copied().unwrap_or(0);
        let sid = ((h >> 20) & 0xFFF) as u32;
        let Some(desc) = crate::nvn::draw::sampler_desc(d.sampler_pool, sid) else {
            return self.gpu.as_ref().unwrap().dummy_sampler;
        };
        if let Some(&s) = self.gpu.as_ref().unwrap().samplers.get(&desc) {
            return s;
        }
        let s = self.make_gpu_sampler(&desc);
        self.gpu.as_mut().unwrap().samplers.insert(desc, s);
        s
    }

    fn sample_view(&mut self, key: Option<GpuTex>) -> (vk::ImageView, vk::ImageLayout) {
        if let Some(k) = key {
            if let Some(v) = self.sample_cache.borrow().get(&k) {
                return *v;
            }
        }
        let r = self.sample_view_uncached(key);
        if let Some(k) = key {
            self.sample_cache.borrow_mut().insert(k, r);
        }
        r
    }

    fn sample_view_uncached(&self, key: Option<GpuTex>) -> (vk::ImageView, vk::ImageLayout) {
        let g = self.gpu.as_ref().unwrap();
        if let Some(k) = key {
            let trace = |msg: &str| {
                if std::env::var_os("NX_TRACE_SAMPLE").is_some() {
                    static SEEN: std::sync::Mutex<Option<std::collections::HashSet<(u32, u32, u32, u64)>>> =
                        std::sync::Mutex::new(None);
                    let mut guard = SEEN.lock().unwrap();
                    if guard.get_or_insert_with(Default::default).insert((k.format, k.width, k.height, k.addr)) {
                        log::info!("[smp] fmt {} {}x{} addr {:#x} -> {}", k.format, k.width, k.height, k.addr, msg);
                    }
                }
            };
            if let Some(img) = g.images.get(&k) {
                trace(&format!("image exact, last_frame {}", img.last_frame));
                if img.depth {
                    return (img.view, vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL);
                }
                return (img.view, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            }
            if let Some((_, _, view)) = g.sampled.get(&k) {
                trace("sampled exact");
                return (*view, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            }
            // Same guest texture seen through a different view (format): reuse its image.
            if let Some((k2, img)) = g
                .images
                .iter()
                .find(|(k2, i)| !i.depth && k2.addr == k.addr && k2.width == k.width && k2.height == k.height)
            {
                trace(&format!("image reuse fmt {} last_frame {}", k2.format, img.last_frame));
                return (img.view, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            }
            if let Some((k2, (_, _, view))) = g
                .sampled
                .iter()
                .find(|(k2, _)| k2.addr == k.addr && k2.width == k.width && k2.height == k.height)
            {
                trace(&format!("sampled reuse fmt {}", k2.format));
                return (*view, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            }
            trace("DUMMY");
            {
                static SEEN: std::sync::Mutex<Option<std::collections::HashSet<(u32, u32, u32, u32, u64)>>> =
                    std::sync::Mutex::new(None);
                let mut guard = SEEN.lock().unwrap();
                let set = guard.get_or_insert_with(Default::default);
                let id = (k.format, k.width, k.height, k.target, k.addr);
                if set.insert(id) {
                    log::info!(
                        "[tex] sample fallback fmt {} {}x{} target {} layers {} addr {:#x}",
                        k.format,
                        k.width,
                        k.height,
                        k.target,
                        k.layers,
                        k.addr
                    );
                }
            }
        }
        (g.dummy_view, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
    }

    unsafe fn ensure_sampled(&mut self, pool: u64, handle: u64) -> Result<()> {
        let Some(key) = crate::nvn::draw::tex_key(pool, handle) else {
            {
                static SEEN: std::sync::Mutex<Option<std::collections::HashSet<(u64, u64)>>> = std::sync::Mutex::new(None);
                let mut guard = SEEN.lock().unwrap();
                let set = guard.get_or_insert_with(Default::default);
                if set.insert((pool, handle)) {
                    log::info!("[tex] unresolved pool {pool:#x} handle {handle:#x} tid {:#x}", handle & 0xF_FFFF);
                }
            }
            return Ok(());
        };
        {
            let g = self.gpu.as_ref().unwrap();
            if g.images.contains_key(&key) || g.sampled.contains_key(&key) {
                return Ok(());
            }
        }
        if is_depth_format(key.format) {
            return Ok(());
        }
        let layers = key.layers.max(1);
        let mut skip = |me: &mut Self, why: &str| {
            if me.gpu.as_mut().unwrap().skipped.insert(key) {
                log::info!("[tex] skip {why}: fmt {} {}x{} target {} layers {}", key.format, key.width, key.height, key.target, key.layers);
            }
        };
        if matches!(key.target, 9) {
            skip(self, "cube array");
            return Ok(());
        }
        let cube = key.target == 8;
        let format = match sampled_vk_format(key.format) {
            Some(f) => f,
            None => {
                if crate::bc_soft::soft_rgba(key.format).is_none() {
                    skip(self, "format");
                    return Ok(());
                }
                if crate::bc_soft::srgb(key.format) {
                    vk::Format::R8G8B8A8_SRGB
                } else {
                    vk::Format::R8G8B8A8_UNORM
                }
            }
        };
        let array = layers > 1;
        let Some(b) = crate::nvn::draw::tex_builder(pool, handle) else { skip(self, "builder"); return Ok(()) };
        let pixels = match crate::nvn::draw::uploaded(&key) {
    Some((_, px)) => px,
    None => match crate::nvn::draw::tex_pixels(&b) {
        Some((px, _, _)) => px,
        None => {
            skip(self, "pixels");
            return Ok(());
        }
    },
};
        let size = pixels.len() as u64;
        if size == 0 {
            return Ok(());
        }
        if key.width <= 16 && key.height <= 16 {
            static SMALL: std::sync::Mutex<Option<std::collections::HashSet<GpuTex>>> = std::sync::Mutex::new(None);
            let mut guard = SMALL.lock().unwrap();
            if guard.get_or_insert_with(Default::default).insert(key) {
                let hex: Vec<String> = pixels.iter().take(24).map(|b| format!("{b:02x}")).collect();
                log::info!(
                    "[tex] small {}x{} fmt {} target {} pixels [{}]",
                    key.width,
                    key.height,
                    key.format,
                    key.target,
                    hex.join("")
                );
            }
        }
        let image_layers = if cube { 6 } else { layers };
        let copy_layers = if size % image_layers.max(1) as u64 == 0 { image_layers.max(1) } else { 1 };
        let layer_bytes = size / copy_layers as u64;
        let (staging, staging_mem, staging_ptr) = self.host_buffer(size, vk::BufferUsageFlags::TRANSFER_SRC)?;
        std::ptr::copy_nonoverlapping(pixels.as_ptr(), staging_ptr, size as usize);
        let image = self.device.create_image(
            &vk::ImageCreateInfo::default()
                .image_type(vk::ImageType::TYPE_2D)
                .format(format)
                .extent(vk::Extent3D { width: key.width, height: key.height, depth: 1 })
                .mip_levels(1)
                .array_layers(if cube { 6 } else { layers })
                .samples(vk::SampleCountFlags::TYPE_1)
                .tiling(vk::ImageTiling::OPTIMAL)
                .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .initial_layout(vk::ImageLayout::UNDEFINED),
            None,
        )?;
        let req = self.device.get_image_memory_requirements(image);
        let props = self.instance.get_physical_device_memory_properties(self.physical_device);
        let mt = (0..props.memory_type_count)
            .find(|&i| req.memory_type_bits & (1 << i) != 0 && props.memory_types[i as usize].property_flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL))
            .unwrap_or(0);
        let memory = self
            .device
            .allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(mt), None)?;
        self.device.bind_image_memory(image, memory, 0)?;
        let view = self.device.create_image_view(
            &vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(if cube {
                    vk::ImageViewType::CUBE
                } else if array {
                    vk::ImageViewType::TYPE_2D_ARRAY
                } else {
                    vk::ImageViewType::TYPE_2D
                })
                .format(format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: if cube { 6 } else { layers },
                }),
            None,
        )?;
        let cb = self.one_shot_begin()?;
        barrier(
            cb,
            &self.device,
            image,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::NONE,
            vk::AccessFlags::TRANSFER_WRITE,
        );
        let copies: Vec<vk::BufferImageCopy> = (0..copy_layers)
            .map(|layer| {
                vk::BufferImageCopy::default()
                    .buffer_offset(layer as u64 * layer_bytes)
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .base_array_layer(layer)
                            .layer_count(1),
                    )
                    .image_extent(vk::Extent3D { width: key.width, height: key.height, depth: 1 })
            })
            .collect();
        self.device.cmd_copy_buffer_to_image(cb, staging, image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &copies);
        barrier(
            cb,
            &self.device,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::SHADER_READ,
        );
        self.one_shot_end(cb)?;
        self.device.unmap_memory(staging_mem);
        self.device.destroy_buffer(staging, None);
        self.device.free_memory(staging_mem, None);
        self.gpu.as_mut().unwrap().sampled.insert(key, (image, memory, view));
        Ok(())
    }

    pub(super) unsafe fn gpu_present(&mut self, f: GpuFrame) -> Result<()> {
        crate::gfx_vk::stage(1);
        if std::env::var_os("NX_SCENE_READBACK").is_some() {
            self.present_scene_readback();
        }
        if f.draws.is_empty() {
            // No new draw this frame: re-present the last image for this texture
            // instead of falling back to the differently oriented CPU path.
            let view = {
                let g = match self.gpu.as_ref() {
                    Some(g) => g,
                    None => return Ok(()),
                };
                g.images
                    .get(&f.present)
                    .filter(|i| i.last_frame > 0)
                    .map(|i| i.view)
                    .or_else(|| {
                        g.images
                            .values()
                            .filter(|i| !i.depth && i.last_frame > 0 && i.extent.width == f.present.width && i.extent.height == f.present.height)
                            .max_by_key(|i| i.last_frame)
                            .map(|i| i.view)
                    })
            };
            let Some(view) = view else { return Ok(()) };
            self.device.wait_for_fences(&[self.in_flight], true, u64::MAX)?;
            let (index, _) = match self.swapchain_loader.acquire_next_image(self.swapchain, u64::MAX, self.image_available, vk::Fence::null()) {
                Ok(v) => v,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    self.destroy_swapchain();
                    self.create_swapchain("present-only out of date".into())?;
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            };
            self.device.reset_fences(&[self.in_flight])?;
            let cb = self.command_buffers[index as usize];
            self.device.reset_command_buffer(cb, vk::CommandBufferResetFlags::empty())?;
            self.device.begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
            let image_info = [vk::DescriptorImageInfo::default()
                .image_view(view)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
            let buffer_info = [vk::DescriptorBufferInfo::default().buffer(self.flags_buffer).offset(0).range(vk::WHOLE_SIZE)];
            let sampler_info = [vk::DescriptorImageInfo::default().sampler(self.sampler)];
            let writes = [
                vk::WriteDescriptorSet::default().dst_set(self.gpu_present_sets[0]).dst_binding(0).descriptor_type(vk::DescriptorType::STORAGE_BUFFER).buffer_info(&buffer_info),
                vk::WriteDescriptorSet::default().dst_set(self.gpu_present_sets[0]).dst_binding(1).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&image_info),
                vk::WriteDescriptorSet::default().dst_set(self.gpu_present_sets[1]).dst_binding(0).descriptor_type(vk::DescriptorType::SAMPLER).image_info(&sampler_info),
            ];
            self.device.update_descriptor_sets(&writes, &[]);
            let clear = [vk::ClearValue { color: vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } }];
            let begin = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[index as usize])
                .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: self.extent })
                .clear_values(&clear);
            self.device.cmd_begin_render_pass(cb, &begin, vk::SubpassContents::INLINE);
            self.device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
            self.device.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, &self.gpu_present_sets, &[]);
            let (dw, dh, dx, dy) = fit_frame(f.present.width, f.present.height, self.extent);
            self.device.cmd_set_viewport(cb, 0, &[vk::Viewport { x: dx as f32, y: dy as f32, width: dw as f32, height: dh as f32, min_depth: 0.0, max_depth: 1.0 }]);
            self.device.cmd_set_scissor(cb, 0, &[vk::Rect2D { offset: vk::Offset2D { x: dx, y: dy }, extent: vk::Extent2D { width: dw as u32, height: dh as u32 } }]);
            self.device.cmd_draw(cb, 3, 1, 0, 0);
            self.device.cmd_end_render_pass(cb);
            self.device.end_command_buffer(cb)?;
            let wait = [self.image_available];
            let stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let signal = [self.render_finished];
            let cbs = [cb];
            let submit = vk::SubmitInfo::default().wait_semaphores(&wait).wait_dst_stage_mask(&stages).command_buffers(&cbs).signal_semaphores(&signal);
            self.device.queue_submit(self.queue, &[submit], self.in_flight)?;
            let swapchains = [self.swapchain];
            let indices = [index];
            let present = vk::PresentInfoKHR::default().wait_semaphores(&signal).swapchains(&swapchains).image_indices(&indices);
            match self.swapchain_loader.queue_present(self.queue, &present) {
                Ok(_) => {}
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    self.destroy_swapchain();
                    self.create_swapchain("present-only out of date".into())?;
                }
                Err(e) => log::warn!("[vk] present-only: {e:?}"),
            }
            return Ok(());
        }
        self.ensure_gpu()?;
        {
            static HB: std::sync::Mutex<Option<(std::time::Instant, u32, u32)>> = std::sync::Mutex::new(None);
            let mut hb = HB.lock().unwrap();
            let now = std::time::Instant::now();
            let e = hb.get_or_insert((now, 0, 0));
            e.1 += 1;
            if f.draws.iter().any(|d| d.target == f.present) {
                e.2 += 1;
            }
            let secs = now.duration_since(e.0).as_secs_f32();
            if secs >= 2.0 {
                let mut by: std::collections::BTreeMap<(u32, u32, u32), usize> = Default::default();
                for d in &f.draws {
                    *by.entry((d.target.format, d.target.width, d.target.height)).or_default() += 1;
                }
                let full = by.iter().filter(|((_, w, h), _)| *w == 1920 && *h == 1080).map(|(_, n)| *n).sum::<usize>();
                let (nimg, mib) = {
                    let g = self.gpu.as_ref().unwrap();
                    let mib: u64 = g
                        .images
                        .keys()
                        .map(|k| (k.width as u64 * k.height as u64 * 4 * k.layers.max(1) as u64) >> 20)
                        .sum();
                    (g.images.len(), mib)
                };
                log::info!(
                    "[vk] {:.1} fps ({} draws, {full} at 1080p, {} hits, {nimg} imgs ~{mib} MiB, draw {:.0} ms (upd {:.0} bind {:.0} cmd {:.0}), pipes +{}, ds {:.0} ms)",
                    e.1 as f32 / secs,
                    f.draws.len(),
                    e.2,
                    DRAW_NS.swap(0, std::sync::atomic::Ordering::Relaxed) as f64 / 1e6,
                    UDP_NS.swap(0, std::sync::atomic::Ordering::Relaxed) as f64 / 1e6,
                    BDS_NS.swap(0, std::sync::atomic::Ordering::Relaxed) as f64 / 1e6,
                    DCMD_NS.swap(0, std::sync::atomic::Ordering::Relaxed) as f64 / 1e6,
                    PIPE_MISSES.swap(0, std::sync::atomic::Ordering::Relaxed),
                    DS_NS.swap(0, std::sync::atomic::Ordering::Relaxed) as f64 / 1e6
                );
                *e = (now, 0, 0);
            }
        }
        // Wait for the previous submission instead of a full device idle: the
        // descriptor pool and arena are only safe to reuse once it completed.
        {
            let t = std::time::Instant::now();
            self.device.wait_for_fences(&[self.in_flight], true, u64::MAX)?;
            let ms = t.elapsed().as_millis();
            if ms > 300 {
                log::info!("[vk] stall: in-flight fence {ms} ms");
            }
        }
        crate::gfx_vk::stage(2);
        self.gpu.as_mut().unwrap().arena_used = 0;
        self.device
            .reset_descriptor_pool(self.gpu.as_ref().unwrap().pool, vk::DescriptorPoolResetFlags::empty())?;
        self.sample_cache.borrow_mut().clear();
        let mut mirrored: std::collections::HashSet<(u64, u64)> = Default::default();
        for d in &f.draws {
            for &(addr, size) in d.vbufs.iter() {
                if addr != 0 && size != 0 && mirrored.insert((addr, size)) {
                    Self::mirror_copy(self.gpu.as_ref().unwrap(), addr, size);
                }
            }
            for stage in &d.ubos {
                for &(addr, size) in stage.iter() {
                    let size = size.max(0x1000);
                    if addr != 0 && mirrored.insert((addr, size)) {
                        Self::mirror_copy(self.gpu.as_ref().unwrap(), addr, size);
                    }
                }
            }
            for stage in &d.storage {
                for &(addr, size) in stage.iter() {
                    if addr != 0 && size != 0 && mirrored.insert((addr, size)) {
                        Self::mirror_copy(self.gpu.as_ref().unwrap(), addr, size);
                    }
                }
            }
        }
        for (addr, size, data) in &f.patches {
            let g = self.gpu.as_ref().unwrap();
            Self::mirror_copy(g, *addr, *size);
            if *addr >= g.heap_base {
                let off = addr - g.heap_base;
                if off + data.len() as u64 <= g.heap_size {
                    std::ptr::copy_nonoverlapping(data.as_ptr(), g.mirror_ptr.add(off as usize), data.len());
                }
            }
        }
        crate::gfx_vk::stage(3);
        {
            static DUMPED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            // Dump once the 1440x810 HDR scene has been rendering for a while:
            // that is the attract/race stage, and it is what presents green.
            // Dump a few consecutive tonemap frames (3-vertex blit into fmt61).
            static DUMP_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let wanted = std::env::var_os("NX_DUMP_GPU").is_some();
            let tonemap = f.draws.iter().any(|d| {
                d.target.width == 1920 && (d.target.format == 61 || d.target.format == 56) && d.count <= 6
            });
            {
                static DIFF: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                if std::env::var_os("NX_LOG_DRAWS").is_some() {
                    for d in f.draws.iter().filter(|d| d.target.width == 1920) {
                        if DIFF.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 400 {
                            let hashes: Vec<String> = d.stages.iter().map(|(b, _)| format!("{:016x}", fnv1a(b) | 1)).collect();
                            if d.channel_mask == 0xf && d.color_state == 0x8030000 {
                                let (va, _vb) = d.vbufs[0];
                                let stride = d.streams[0].0 as u64;
                                let verts: Vec<String> = (0..6)
                                    .map(|i| {
                                        let p = va + i * stride;
                                        let f: Vec<f32> = (0..3)
                                            .map(|k| {
                                                let q = p + 4 * k as u64;
                                                unsafe { f32::from_bits((q as *const u32).read_unaligned()) }
                                            })
                                            .collect();
                                        format!("{f:?}")
                                    })
                                    .collect();
                                let idx = match d.index {
                                    Some((0, a)) => (0..6).map(|i| unsafe { (a as *const u8).add(i).read() } as u32).collect::<Vec<_>>(),
                                    Some((1, a)) => (0..6).map(|i| unsafe { (a as *const u16).add(i).read() } as u32).collect::<Vec<_>>(),
                                    Some((_, a)) => (0..6).map(|i| unsafe { (a as *const u32).add(i).read() }).collect::<Vec<_>>(),
                                    None => Vec::new(),
                                };
                                log::info!("[vk] composite verts stride {stride}: {verts:?} idx {idx:?} vbuf {va:#x}");
                                log::info!(
                                    "[vk] composite pass: open {:?} want ({:?}, {:?}) layout {:?}",
                                    self.open_pass,
                                    d.target,
                                    d.depth,
                                    self.gpu.as_ref().unwrap().images.get(&d.target).map(|i| i.layout)
                                );
                            }
                            log::info!(
                                "[diff] draw {}x{} depth {} colors 1 count {} mode {} index {} blend {:#x} mask {:#x} color_state {:#x} stages {:?}",
                                d.target.width,
                                d.target.height,
                                d.depth.is_some(),
                                d.count,
                                d.mode,
                                d.index.is_some(),
                                d.blend,
                                d.channel_mask,
                                d.color_state,
                                hashes
                            );
                        }
                    }
                }
            }
            let due = wanted && tonemap && DUMP_N.load(std::sync::atomic::Ordering::Relaxed) < 6;
            if due {
                DUMPED.store(true, std::sync::atomic::Ordering::Relaxed);
                let tag = format!("f{}", DUMP_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
                // What draws into the presented buffer, and what does it sample?
                for d in f.draws.iter().filter(|d| d.target.width == 1920 && (d.target.format == 61 || d.target.format == 56)) {
                    let desc = |stage: usize| -> Vec<String> {
                        d.textures[stage]
                            .iter()
                            .take(4)
                            .filter(|t| **t != 0)
                            .map(|t| match crate::nvn::draw::tex_key(d.texture_pool, *t) {
                                Some(k) => format!("{:#x}=fmt{} {}x{}@{:x}", t, k.format, k.width, k.height, k.addr),
                                None => format!("{t:#x}=?")
                            })
                            .collect()
                    };
                    let tex0 = desc(0);
                    let tex1 = desc(1);
                    log::info!(
                        "[vk] present-draw: fmt{} {}x{} stages {} tex0 [{}] tex1 [{}] blend {:#x} mask {:#x} mode {} count {} vp {:?} sc {:?} vbuf {:?} attrib {:?}",
                        d.target.format,
                        d.target.width,
                        d.target.height,
                        d.stages.len(),
                        tex0.join(" "),
                        tex1.join(" "),
                        d.blend,
                        d.channel_mask,
                        d.mode,
                        d.count,
                        d.viewport,
                        d.scissor,
                        d.vbufs[0],
                        d.attribs[0]
                    );
                    let (uint_targets, sint_targets) = target_int_kind(f.present.format);
                    for (t, c) in f.clears.iter() {
                        log::info!("[vk] clear fmt{} {}x{}@{:x} -> {:?}", t.format, t.width, t.height, t.addr, c);
                    }
                    log::info!(
                        "[vk] present-draw state: depth {:?} flags {:#x} color_state {:#x} alpha {} ref {} index {} stages {:?}",
                        d.depth.map(|t| (t.format, t.width, t.height)),
                        d.flags,
                        d.color_state,
                        d.alpha_func,
                        d.alpha_ref,
                        d.index.is_some(),
                        d.stages.iter().map(|(b, _)| format!("{:016x}", (fnv1a(b) | 1) ^ ((uint_targets as u64) << 56) ^ ((sint_targets as u64) << 57))).collect::<Vec<_>>()
                    );
                    let (uaddr, ulen) = d.ubos[0][0];
                    let g = self.gpu.as_ref().unwrap();
                    let uoff = uaddr.wrapping_sub(g.heap_base);
                    if uaddr != 0 && ulen >= 32 && uoff + 32 <= g.heap_size {
                        let raw = std::slice::from_raw_parts(g.mirror_ptr.add(uoff as usize), 32);
                        let floats: Vec<f32> = (0..8)
                            .map(|i| f32::from_le_bytes([raw[4 * i], raw[4 * i + 1], raw[4 * i + 2], raw[4 * i + 3]]))
                            .collect();
                        log::info!("[vk] present-draw ubo0 {:?}", floats);
                        let moff = uoff + 528;
                        if moff + 32 <= g.heap_size {
                            let raw = std::slice::from_raw_parts(g.mirror_ptr.add(moff as usize), 32);
                            let m: Vec<f32> = (0..8)
                                .map(|i| f32::from_le_bytes([raw[4 * i], raw[4 * i + 1], raw[4 * i + 2], raw[4 * i + 3]]))
                                .collect();
                            log::info!("[vk] present-draw ubo0+528 {:?}", m);
                        }
                    }
                    let (addr, len) = d.vbufs[0];
                    let g = self.gpu.as_ref().unwrap();
                    let off = addr.wrapping_sub(g.heap_base);
                    if addr != 0 && len >= 32 && off + 32 <= g.heap_size {
                        let raw = std::slice::from_raw_parts(g.mirror_ptr.add(off as usize), 32);
                        let floats: Vec<f32> = (0..8)
                            .map(|i| f32::from_le_bytes([raw[4 * i], raw[4 * i + 1], raw[4 * i + 2], raw[4 * i + 3]]))
                            .collect();
                        log::info!("[vk] present-draw vertex0 {:?}", floats);
                    }
                }
                if let Some(dir) = crate::gfx_vk::dump_dir() {
                    let list: Vec<GpuTex> = self.gpu.as_ref().unwrap().images.keys().copied().collect();
                    for t in list {
                        let big = self
                            .gpu
                            .as_ref()
                            .unwrap()
                            .images
                            .get(&t)
                            .is_some_and(|i| !i.depth && ((i.format == vk::Format::A2B10G10R10_UNORM_PACK32 && i.extent.width == 1920) || (i.format == vk::Format::R16G16B16A16_SFLOAT && i.extent.width == 1440)));
                        if big {
                            self.dump_image(t, &dir, &tag);
                        }
                    }
                }
            }
        }
        let this_frame = {
            let g = self.gpu.as_mut().unwrap();
            g.frame += 1;
            g.frame
        };
        for img in self.gpu.as_mut().unwrap().images.values_mut() {
            img.rendered = false;
        }
        {
            let mut needed = std::collections::BTreeSet::new();
            for d in &f.draws {
                for stage in 0..2 {
                    for &h in d.textures[stage].iter() {
                        if h != 0 {
                            needed.insert((d.texture_pool, h));
                        }
                    }
                }
            }
            let mut uploaded = 0;
            let t_tex = std::time::Instant::now();
            let n_needed = needed.len();
            for (pool, handle) in needed {
                match self.ensure_sampled(pool, handle) {
                    Ok(()) => uploaded += 1,
                    Err(e) => log::warn!("[vk] texture skipped: {e:#}"),
                }
            }
            let tex_ms = t_tex.elapsed().as_millis();
            if tex_ms > 200 {
                log::info!("[vk] ensure_sampled {n_needed} textures in {tex_ms} ms");
            }
            static TEXLOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = TEXLOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 4 {
                log::info!("[vk] textures this frame: {} candidates", uploaded);
            }
        }
        crate::gfx_vk::stage(4);
        self.ensure_image(f.present)?;
        crate::gfx_vk::stage(5);
        {
            let t = std::time::Instant::now();
            self.device.wait_for_fences(&[self.in_flight], true, u64::MAX)?;
            let ms = t.elapsed().as_millis();
            if ms > 300 {
                log::info!("[vk] stall: fence before acquire {ms} ms");
            }
        }
        crate::gfx_vk::stage(6);
        let acquire_start = std::time::Instant::now();
        let (index, suboptimal) = match self
            .swapchain_loader
            .acquire_next_image(self.swapchain, u64::MAX, self.image_available, vk::Fence::null())
        {
            Ok(v) => v,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.destroy_swapchain();
                self.create_swapchain("gpu present out of date".into())?;
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        };
        {
            let ms = acquire_start.elapsed().as_millis();
            if ms > 300 {
                log::info!("[vk] stall: acquire {ms} ms -> image {index}");
            }
        }
        crate::gfx_vk::stage(7);
        self.device.reset_fences(&[self.in_flight])?;
        let cb = self.command_buffers[index as usize];
        self.device.reset_command_buffer(cb, vk::CommandBufferResetFlags::empty())?;
        self.device.begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
        let mut drawn = 0usize;
        crate::gfx_vk::stage(8);
        for d in &f.dispatches {
            if let Err(e) = self.gpu_compute(cb, d) {
                log::warn!("[vk] dispatch skipped: {e:#}");
            }
        }
        crate::gfx_vk::stage(9);
        for d in &f.draws {
            let t0 = std::time::Instant::now();
            let r = self.gpu_draw(cb, d, &f.clears, &f.depth_clears);
            DRAW_NS.fetch_add(t0.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
            match r {
                Ok(()) => drawn += 1,
                Err(e) => log::warn!("[vk] draw skipped: {e:#}"),
            }
        }
        self.close_pass(cb);
        crate::gfx_vk::stage(10);
        for c in &f.copies {
            if let Err(e) = self.gpu_copy(cb, c) {
                log::warn!("[vk] copy skipped: {e:#}");
            }
        }
        let picked = {
            let g = self.gpu.as_ref().unwrap();
            let same_mem = |k: &GpuTex, i: &GpuImage| {
                !i.depth && i.last_frame > 0 && k.addr == f.present.addr && k.width == f.present.width && k.height == f.present.height
            };
            // The guest presents one memory range under a format that can differ from
            // the one it rendered into, so prefer the freshest image of that range.
            let forced_pick = if std::env::var_os("NX_PRESENT_FRESH").is_some() {
                g.images
                    .iter()
                    .filter(|(_, i)| !i.depth && i.last_frame > 0 && i.extent.width == f.present.width && i.extent.height == f.present.height)
                    .max_by_key(|(_, i)| i.last_frame)
            } else {
                None
            };
            forced_pick
                .or_else(|| g.images.iter().filter(|(k, i)| same_mem(k, i)).max_by_key(|(_, i)| i.last_frame))
                .or_else(|| g.images.get(&f.present).filter(|i| i.last_frame > 0).map(|i| (&f.present, i)))
                .or_else(|| {
                    g.images
                        .iter()
                        .filter(|(_, i)| !i.depth && i.last_frame > 0 && i.extent.width == f.present.width && i.extent.height == f.present.height)
                        .max_by_key(|(_, i)| i.last_frame)
                })
        };
        let Some((src, img)) = picked else {
            self.device.end_command_buffer(cb)?;
            self.device.queue_wait_idle(self.queue)?;
            bail!("no rendered image for the presented texture");
        };
        let view = img.view;
        {
            static LOG: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);
            let mut l = LOG.lock().unwrap();
            let now = std::time::Instant::now();
            if l.is_none_or(|t| now.duration_since(t).as_secs_f32() >= 2.0) {
                *l = Some(now);
                log::info!(
                    "[vk] present source fmt{} {}x{}@{:x} last_frame {} (present fmt{} @{:x})",
                    src.format, src.width, src.height, src.addr, img.last_frame, f.present.format, f.present.addr
                );
            }
        }
        let image_info = [vk::DescriptorImageInfo::default()
            .image_view(view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let buffer_info = [vk::DescriptorBufferInfo::default().buffer(self.flags_buffer).offset(0).range(vk::WHOLE_SIZE)];
        let sampler_info = [vk::DescriptorImageInfo::default().sampler(self.sampler)];
        let writes = [
            vk::WriteDescriptorSet::default()
                .dst_set(self.gpu_present_sets[0])
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(&buffer_info),
            vk::WriteDescriptorSet::default()
                .dst_set(self.gpu_present_sets[0])
                .dst_binding(1)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&image_info),
            vk::WriteDescriptorSet::default()
                .dst_set(self.gpu_present_sets[1])
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::SAMPLER)
                .image_info(&sampler_info),
        ];
        self.device.update_descriptor_sets(&writes, &[]);
        let clear = [vk::ClearValue { color: vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } }];
        let present_begin = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[index as usize])
            .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: self.extent })
            .clear_values(&clear);
        self.device.cmd_begin_render_pass(cb, &present_begin, vk::SubpassContents::INLINE);
        self.device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
        self.device.cmd_bind_descriptor_sets(
            cb,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline_layout,
            0,
            &self.gpu_present_sets,
            &[],
        );
        let (dw, dh, dx, dy) = fit_frame(f.present.width, f.present.height, self.extent);
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
        crate::gfx_vk::stage(11);
        {
            let t = std::time::Instant::now();
            self.device.queue_submit(self.queue, &[submit], self.in_flight)?;
            let ms = t.elapsed().as_millis();
            if ms > 300 {
                log::info!("[vk] stall: submit {ms} ms");
            }
        }
        crate::gfx_vk::stage(12);
        let swapchains = [self.swapchain];
        let indices = [index];
        let present = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal)
            .swapchains(&swapchains)
            .image_indices(&indices);
        let t_present = std::time::Instant::now();
        let result = self.swapchain_loader.queue_present(self.queue, &present);
        {
            let ms = t_present.elapsed().as_millis();
            if ms > 300 {
                log::info!("[vk] stall: queue_present {ms} ms");
            }
        }
        let _ = suboptimal;
        let needs_recreate = matches!(result, Err(vk::Result::ERROR_OUT_OF_DATE_KHR));
        if needs_recreate {
            self.destroy_swapchain();
            self.create_swapchain("gpu present suboptimal".into())?;
        } else if let Err(e) = result {
            return Err(e.into());
        }
        static FRAMES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = FRAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 8 || drawn != f.draws.len() {
            log::info!("[vk] gpu frame: {drawn}/{} draws drawn", f.draws.len());
        }
        crate::gfx_vk::stage(13);
        Ok(())
    }

    unsafe fn gpu_compute(&mut self, cb: vk::CommandBuffer, d: &crate::gfx_vk::GpuDispatch) -> Result<()> {
        #[cfg(not(mush8_shaderc))]
        {
            let _ = (cb, d);
            bail!("compute needs shaderc")
        }
        #[cfg(mush8_shaderc)]
        {
            let hash = fnv1a(&d.blob) ^ (1u64 << 62);
            let cached = self.gpu.as_ref().unwrap().compute_pipes.contains_key(&hash);
            if !cached {
                let t = nxshader::glsl::translate_compute(&d.blob, 0, d.local_size).context("NVN -> GLSL compute")?;
            if let Some(dir) = std::env::var_os("NX_DUMP_GLSL") {
                let dir = std::path::PathBuf::from(dir);
                let _ = std::fs::create_dir_all(&dir);
                let _ = std::fs::write(dir.join(format!("{hash:016x}.glsl")), &t.source);
            }
                let words = crate::shaderc::compile(2, &t.source, "game-compute").context("shaderc compute")?;
                let module = self.device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None)?;
                let tex_keys = sorted_keys(&t.interface.textures);
                let slots = sorted_slots(&t.interface.samplers);
                let mut set0 = vec![vk::DescriptorSetLayoutBinding::default()
                    .binding(0)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE)];
                for &b in &tex_keys {
                    set0.push(
                        vk::DescriptorSetLayoutBinding::default()
                            .binding(1 + b)
                            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                            .descriptor_count(1)
                            .stage_flags(vk::ShaderStageFlags::COMPUTE),
                    );
                }
                let mut set1: Vec<vk::DescriptorSetLayoutBinding> = slots
                    .iter()
                    .map(|&slot| {
                        vk::DescriptorSetLayoutBinding::default()
                            .binding(slot)
                            .descriptor_type(vk::DescriptorType::SAMPLER)
                            .descriptor_count(1)
                            .stage_flags(vk::ShaderStageFlags::COMPUTE)
                    })
                    .collect();
                if set1.is_empty() {
                    set1.push(
                        vk::DescriptorSetLayoutBinding::default()
                            .binding(0)
                            .descriptor_type(vk::DescriptorType::SAMPLER)
                            .descriptor_count(1)
                            .stage_flags(vk::ShaderStageFlags::COMPUTE),
                    );
                }
                let sets = [
                    self.device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&set0), None)?,
                    self.device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&set1), None)?,
                ];
                let layout = self.device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&sets), None)?;
                let stage = vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::COMPUTE)
                    .module(module)
                    .name(c"main");
                let info = vk::ComputePipelineCreateInfo::default().stage(stage).layout(layout);
                let pipe = self
                    .device
                    .create_compute_pipelines(vk::PipelineCache::null(), &[info], None)
                    .map_err(|(_, e)| anyhow!("compute pipeline: {e}"))?[0];
                self.device.destroy_shader_module(module, None);
                self.gpu
                    .as_mut()
                    .unwrap()
                    .compute_pipes
                    .insert(hash, ComputePipe { pipe, layout, sets, tex_keys, slots });
            }
            let (pipe, layout, sets_layouts, tex_keys, slots) = {
                let c = &self.gpu.as_ref().unwrap().compute_pipes[&hash];
                (c.pipe, c.layout, c.sets, c.tex_keys.clone(), c.slots.clone())
            };
            let mut res = GpuRes::default();
            {
                let g = self.gpu.as_ref().unwrap();
                res.heap_base = g.heap_base;
                res.heap_gpu = g.mirror_addr;
                res.heap_size = g.heap_size;
                res.zero_gpu = g.zero_addr;
                res.cb = [g.zero_addr; 18];
                res.vb = [g.zero_addr; 16];
            }
            if d.storage.iter().any(|s| s.0 != 0) {
                let mut bank = vec![0u8; 0x400];
                for (i, &(addr, size)) in d.storage.iter().enumerate().take(16) {
                    if addr != 0 {
                        let a = Self::mirror_addr(self.gpu.as_ref().unwrap(), addr);
                        bank[0x44 + 4 * i..0x4c + 4 * i].copy_from_slice(&a.to_le_bytes());
                        bank[0x4c + 4 * i..0x50 + 4 * i].copy_from_slice(&(size as u32).to_le_bytes());
                    }
                }
                res.cb[0] = self.arena_write(&bank, 16)?;
            }
            res.cb[1] = Self::mirror_addr(self.gpu.as_ref().unwrap(), d.constants);
            let mut handles = [0u32; 0x48];
            for (i, h) in d.textures.iter().enumerate().take(0x40) {
                handles[8 + 2 * i] = *h as u32;
                handles[9 + 2 * i] = (*h >> 32) as u32;
            }
            let hbytes: &[u8] = std::slice::from_raw_parts(handles.as_ptr() as *const u8, handles.len() * 4);
            res.cb[2] = self.arena_write(hbytes, 16)?;
            for i in 0..15 {
                let (addr, _) = *d.ubos.get(i).unwrap_or(&(0, 0));
                if addr != 0 {
                    res.cb[3 + i] = Self::mirror_addr(self.gpu.as_ref().unwrap(), addr);
                }
            }
            let bytes: &[u8] = std::slice::from_raw_parts(&res as *const GpuRes as *const u8, std::mem::size_of::<GpuRes>());
            let (off, _) = self.arena_write_at(bytes, 16)?;
            let arena = self.gpu.as_ref().unwrap().arena;
            let buffer_info = vk::DescriptorBufferInfo::default().buffer(arena).offset(off).range(bytes.len() as u64);
            let mut image_infos: Vec<vk::DescriptorImageInfo> = Vec::with_capacity(tex_keys.len());
            for &b in &tex_keys {
                let key = crate::nvn::draw::tex_key(d.texture_pool, *d.textures.get(b as usize).unwrap_or(&0));
                let (view, ilayout) = self.sample_view(key);
                image_infos.push(vk::DescriptorImageInfo::default().image_view(view).image_layout(ilayout));
            }
            let sampler_infos: Vec<vk::DescriptorImageInfo> = if slots.is_empty() {
                vec![vk::DescriptorImageInfo::default().sampler(self.sampler)]
            } else {
                slots.iter().map(|_| vk::DescriptorImageInfo::default().sampler(self.sampler)).collect()
            };
            let pool = self.gpu.as_ref().unwrap().pool;
            let dt = std::time::Instant::now();
            let sets = self.device.allocate_descriptor_sets(
                &vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&sets_layouts),
            )?;
            DS_NS.fetch_add(dt.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
            let mut writes = vec![vk::WriteDescriptorSet::default()
                .dst_set(sets[0])
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(std::slice::from_ref(&buffer_info))];
            for (i, &b) in tex_keys.iter().enumerate() {
                writes.push(
                    vk::WriteDescriptorSet::default()
                        .dst_set(sets[0])
                        .dst_binding(1 + b)
                        .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                        .image_info(std::slice::from_ref(&image_infos[i])),
                );
            }
            {
                let bind_slots: Vec<u32> = if slots.is_empty() { vec![0] } else { slots.clone() };
                for (i, &slot) in bind_slots.iter().enumerate() {
                    writes.push(
                        vk::WriteDescriptorSet::default()
                            .dst_set(sets[1])
                            .dst_binding(slot)
                            .descriptor_type(vk::DescriptorType::SAMPLER)
                            .image_info(std::slice::from_ref(&sampler_infos[i])),
                    );
                }
            }
            self.device.update_descriptor_sets(&writes, &[]);
            self.device.cmd_pipeline_barrier(
                cb,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[],
            );
            self.device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::COMPUTE, pipe);
            self.device.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::COMPUTE, layout, 0, &sets, &[]);
            self.device.cmd_dispatch(cb, d.groups[0], d.groups[1], d.groups[2]);
            self.device.cmd_pipeline_barrier(
                cb,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[],
            );
            Ok(())
        }
    }

    unsafe fn gpu_draw(&mut self, cb: vk::CommandBuffer, d: &GpuDraw, clears: &[(GpuTex, [f32; 4])], depth_clears: &[(GpuTex, f32)]) -> Result<()> {
        let mut stages: Option<(u64, u64, u64, u64)> = None;
        let (uint_targets, sint_targets) = target_int_kind(d.target.format);
        for (blob, constants) in &d.stages {
            let Some(sph) = nxshader::sph::Sph::parse(&blob[0x30..]) else { continue };
            let (hash, _) = self.gpu_module(blob, uint_targets, sint_targets)?;
            match sph.stage {
                nxshader::sph::Stage::Fragment => {
                    stages = Some((stages.map_or(0, |s| s.0), stages.map_or(0, |s| s.1), hash, *constants))
                }
                nxshader::sph::Stage::Vertex => {
                    stages = Some((hash, *constants, stages.map_or(0, |s| s.2), stages.map_or(0, |s| s.3)))
                }
                _ => {}
            }
        }
        let Some((vsh, vs_const, fsh, fs_const)) = stages.filter(|s| s.0 != 0 && s.2 != 0) else {
            bail!("missing vertex or fragment stage");
        };
        let mut topology = match d.mode {
            0 => 0,
            1 => 1,
            3 => 3,
            5 => 5,
            _ => 4,
        };
        let mut converted: Option<Vec<u32>> = None;
        let mut use_indexed = d.index.is_some();
        let mut first = d.first;
        let mut vertex_offset = d.base_vertex;
        match d.mode {
            6 | 9 => {
                let mut v = Vec::with_capacity((d.count.saturating_sub(2) * 3) as usize);
                for i in 1..d.count.saturating_sub(1) {
                    v.extend_from_slice(&[0, i, i + 1]);
                }
                converted = Some(v);
                topology = 4;
                use_indexed = true;
                first = 0;
                vertex_offset = d.first as i32;
            }
            7 => {
                let mut v = Vec::with_capacity((d.count / 4 * 6) as usize);
                for q in 0..d.count / 4 {
                    let b = q * 4;
                    v.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3]);
                }
                converted = Some(v);
                topology = 4;
                use_indexed = true;
                first = 0;
                vertex_offset = d.first as i32;
            }
            8 => {
                let mut v = Vec::new();
                let mut i = 0;
                while i + 3 < d.count {
                    v.extend_from_slice(&[i, i + 1, i + 2, i + 2, i + 1, i + 3]);
                    i += 2;
                }
                converted = Some(v);
                topology = 4;
                use_indexed = true;
                first = 0;
                vertex_offset = d.first as i32;
            }
            2 => {
                let mut v = Vec::with_capacity(d.count as usize * 2);
                for i in 0..d.count {
                    v.extend_from_slice(&[i, (i + 1) % d.count]);
                }
                converted = Some(v);
                topology = 1;
                use_indexed = true;
                first = 0;
                vertex_offset = d.first as i32;
            }
            _ => {}
        }
        self.ensure_image(d.target)?;
        let mut depth_tex = d.depth.filter(|t| is_depth_format(t.format));
        // A full-screen blit (one triangle) sits on the far plane; NVN's draw
        // state disables depth testing for those, and our pipeline would
        // otherwise cull every fragment against the scene's depth buffer.
        if let Some(t) = depth_tex {
            self.ensure_image(t)?;
        }
        let mut depth_nvn = depth_tex.map_or(0, |t| t.format);
        let (color_format, extent, rendered) = {
            let g = self.gpu.as_ref().unwrap();
            let img = &g.images[&d.target];
            (img.format, img.extent, img.rendered)
        };
        if d.count <= 6 && extent.width >= 1280 {
            if let Some(limit) = std::env::var("NX_TRACE_COMPOSITE").ok().and_then(|v| v.parse::<u32>().ok()).or_else(|| {
                std::env::var_os("NX_TRACE_COMPOSITE").is_some().then_some(60)
            }) {
                static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < limit {
                    let mut idxs = Vec::new();
                    if let Some((ty, addr)) = d.index {
                        for i in 0..6.min(d.count) {
                            idxs.push(unsafe {
                                match ty {
                                    0 => *(addr as *const u8).add(i as usize) as u32,
                                    1 => *(addr as *const u16).add(i as usize) as u32,
                                    _ => *(addr as *const u32).add(i as usize),
                                }
                            });
                        }
                    }
                    let vbs: Vec<(u64, u64)> = d.vbufs.iter().copied().filter(|&(a, s)| a != 0 && s != 0).take(3).collect();
                    let mut smp = Vec::new();
                    for (st, ts) in d.textures.iter().enumerate() {
                        for (slot, &h) in ts.iter().enumerate() {
                            if h != 0 {
                                let k = crate::nvn::draw::tex_key(d.texture_pool, h);
                                smp.push((st, slot, k.map(|k| (k.addr, k.format, k.width, k.height))));
                            }
                        }
                    }
                    log::info!(
                        "[comp] tgt {:#x} fmt {} {}x{} cnt {} mode {} idx {:?} first {} bv {} vbs {:?} smp {:?}",
                        d.target.addr, d.target.format, extent.width, extent.height, d.count, d.mode, idxs, d.first, d.base_vertex, vbs, smp
                    );
                }
            }
        }
        let blend_lo = d.blend as u32;
        let blend_hi = (d.blend >> 32) as u32;
        let write_mask = (d.channel_mask & 0xF) | if d.color_state >> 8 & 1 != 0 { 0x10 } else { 0 };
        let mut choose = |me: &mut Self, depth: Option<GpuTex>, depth_nvn: u32| -> Result<(vk::RenderPass, vk::RenderPass, vk::Framebuffer)> {
            if let Some(dt) = depth {
                let clear_pass = me.combined_pass(color_format, depth_nvn, vk::AttachmentLoadOp::CLEAR)?;
                let load_pass = me.combined_pass(color_format, depth_nvn, vk::AttachmentLoadOp::LOAD)?;
                let fb = me.combined_framebuffer(d.target, dt, if rendered { load_pass } else { clear_pass })?;
                Ok((clear_pass, load_pass, fb))
            } else {
                let g = me.gpu.as_ref().unwrap();
                let img = &g.images[&d.target];
                Ok((img.clear_pass, img.load_pass, img.framebuffer))
            }
        };
        let (mut clear_pass, mut load_pass, mut framebuffer) = choose(self, depth_tex, depth_nvn)?;
        let mut pipe = self.gpu_pipe(vsh, fsh, topology, clear_pass, d.target.format, depth_nvn, blend_lo, blend_hi, write_mask);
        if pipe.is_err() && depth_tex.is_some() {
            static WARN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            if !WARN.swap(true, std::sync::atomic::Ordering::Relaxed) {
                log::warn!("[vk] pipeline with depth failed, falling back to color-only: {:?}", pipe.as_ref().err().map(|e| format!("{e:#}")));
            }
            depth_tex = None;
            depth_nvn = 0;
            let (c, l, f) = choose(self, None, 0)?;
            clear_pass = c;
            load_pass = l;
            framebuffer = f;
            pipe = self.gpu_pipe(vsh, fsh, topology, clear_pass, d.target.format, 0, blend_lo, blend_hi, write_mask);
        }
        let pipeline = pipe?;
        let (layout, sets_layouts, vtex, ftex, vslot_bind, fslot_bind) = {
            let p = &self.gpu.as_ref().unwrap().pipes[&(vsh, fsh, topology, d.target.format, depth_nvn, blend_lo, blend_hi, write_mask)];
            (p.layout, p.sets, p.vtex.clone(), p.ftex.clone(), p.vslot_bind.clone(), p.fslot_bind.clone())
        };
        let clear = clears.iter().find(|(k, _)| *k == d.target).map(|(_, c)| *c).unwrap_or([0.0; 4]);
        let depth_clear = depth_tex.and_then(|t| depth_clears.iter().find(|(k, _)| *k == t).map(|(_, v)| *v)).unwrap_or(1.0);
        let clear_values = if depth_tex.is_some() {
            vec![
                vk::ClearValue { color: vk::ClearColorValue { float32: clear } },
                vk::ClearValue { depth_stencil: vk::ClearDepthStencilValue { depth: depth_clear, stencil: 0 } },
            ]
        } else {
            vec![vk::ClearValue { color: vk::ClearColorValue { float32: clear } }]
        };
        let begin = vk::RenderPassBeginInfo::default()
            .render_pass(if rendered { load_pass } else { clear_pass })
            .framebuffer(framebuffer)
            .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent })
            .clear_values(&clear_values);
        let pass_key = (d.target, depth_tex);
        if self.open_pass != Some(pass_key) {
            self.close_pass(cb);
            self.device.cmd_begin_render_pass(cb, &begin, vk::SubpassContents::INLINE);
            self.open_pass = Some(pass_key);
            self.state_pipe = vk::Pipeline::null();
            self.state_vp = (f32::NAN, f32::NAN, f32::NAN, f32::NAN);
            self.state_sc = (i32::MIN, i32::MIN, i32::MIN, i32::MIN);
            self.state_blend = [f32::NAN; 4];
        }
        if std::env::var_os("NX_MARK_TARGET").is_some()
            && d.channel_mask == 0xf
            && d.color_state == 0x8030000
            && d.target.format == 61
            && d.target.width == 1920
        {
            static ONCE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            if !ONCE.swap(true, std::sync::atomic::Ordering::Relaxed) {
                let area = [vk::ClearRect {
                    rect: vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent },
                    base_array_layer: 0,
                    layer_count: 1,
                }];
                let att = [vk::ClearAttachment {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    color_attachment: 0,
                    clear_value: vk::ClearValue { color: vk::ClearColorValue { float32: [1.0, 0.0, 1.0, 1.0] } },
                }];
                self.device.cmd_clear_attachments(cb, &att, &area);
                log::info!("[vk] marked composite target magenta");
            }
        }
        let dt = std::time::Instant::now();
        let sets = self.device.allocate_descriptor_sets(
            &vk::DescriptorSetAllocateInfo::default()
                .descriptor_pool(self.gpu.as_ref().unwrap().pool)
                .set_layouts(&sets_layouts),
        )?;
        let mut res_writes: [(vk::Buffer, u64, u64); 2] = [(vk::Buffer::null(), 0, 0); 2];
        for stage in 0..2usize {
            let constants = if stage == 0 { vs_const } else { fs_const };
            let mut res = GpuRes::default();
            {
                let g = self.gpu.as_ref().unwrap();
                res.heap_base = g.heap_base;
                res.heap_gpu = g.mirror_addr;
                res.heap_size = g.heap_size;
                res.zero_gpu = g.zero_addr;
                res.cb = [g.zero_addr; 18];
                res.vb = [g.zero_addr; 16];
                res.flags = d.flags;
                res.alpha_func = d.alpha_func;
                res.alpha_ref = d.alpha_ref;
                res.base_vertex = d.base_vertex;
                res.base_instance = d.base_instance;
                for i in 0..16 {
                    let a = d.attribs[i];
                    res.attr_format[i] = (a >> 21) & 0x3FF;
                    res.attr_offset[i] = (a >> 7) & 0x3FFF;
                    res.attr_stream[i] = (a & 0x1F) % 16;
                    res.vb_stride[i] = d.streams[i].0;
                    res.vb_divisor[i] = d.streams[i].1;
                    if d.vbufs[i].0 != 0 {
                        res.vb[i] = Self::mirror_addr(g, d.vbufs[i].0);
                    }
                }
            }
            let storage = &d.storage[stage];
            if storage.iter().any(|s| s.0 != 0) {
                let mut bank = vec![0u8; 0x400];
                for (i, &(addr, size)) in storage.iter().enumerate() {
                    if addr != 0 {
                        let a = Self::mirror_addr(self.gpu.as_ref().unwrap(), addr);
                        bank[0x44 + 4 * i..0x4c + 4 * i].copy_from_slice(&a.to_le_bytes());
                        bank[0x4c + 4 * i..0x50 + 4 * i].copy_from_slice(&(size as u32).to_le_bytes());
                    }
                }
                res.cb[0] = self.arena_write(&bank, 16)?;
            }
            res.cb[1] = Self::mirror_addr(self.gpu.as_ref().unwrap(), constants);
            let mut handles = [0u32; 0x48];
            for (i, h) in d.textures[stage].iter().enumerate() {
                handles[8 + 2 * i] = *h as u32;
                handles[9 + 2 * i] = (*h >> 32) as u32;
            }
            let bytes: &[u8] = unsafe { std::slice::from_raw_parts(handles.as_ptr() as *const u8, handles.len() * 4) };
            res.cb[2] = self.arena_write(bytes, 16)?;
            for i in 0..15 {
                let (addr, _) = d.ubos[stage][i];
                if addr != 0 {
                    res.cb[3 + i] = Self::mirror_addr(self.gpu.as_ref().unwrap(), addr);
                }
            }
            let bytes: &[u8] = unsafe { std::slice::from_raw_parts(&res as *const GpuRes as *const u8, std::mem::size_of::<GpuRes>()) };
            let (off, _) = self.arena_write_at(bytes, 16)?;
            res_writes[stage] = (self.gpu.as_ref().unwrap().arena, off, bytes.len() as u64);
        }
        let buffer_infos = [
            vk::DescriptorBufferInfo::default().buffer(res_writes[0].0).offset(res_writes[0].1).range(res_writes[0].2),
            vk::DescriptorBufferInfo::default().buffer(res_writes[1].0).offset(res_writes[1].1).range(res_writes[1].2),
        ];
        let mut image_infos: Vec<vk::DescriptorImageInfo> = Vec::with_capacity(vtex.len() + ftex.len());
        let mut sampler_infos: Vec<vk::DescriptorImageInfo> = Vec::with_capacity(vslot_bind.len() + fslot_bind.len());
        let dummy = self.gpu.as_ref().unwrap().dummy_view;
        for &b in &vtex {
            let key = crate::nvn::draw::tex_key(d.texture_pool, d.textures[0][b as usize]);
            let (view, layout) = if std::env::var_os("NX_DUMMY_SAMPLE").is_some()
                && d.target.format == 61
                && d.target.width == 1920
            {
                (self.gpu.as_ref().unwrap().dummy_view, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            } else {
                self.sample_view(key)
            };
            let _ = dummy;
            image_infos.push(vk::DescriptorImageInfo::default().image_view(view).image_layout(layout));
        }
        for &b in &ftex {
            let mut key = crate::nvn::draw::tex_key(d.texture_pool, d.textures[1][b as usize]);
            // Diagnostic: feed the freshly rendered HDR scene into the final
            // blit, because the guest's own composite leaves its target blank.
            if std::env::var_os("NX_SHOW_SCENE").is_some() {
                if let Some(k) = key {
                    let composite_target = d.target.format == 61 && d.target.width == 1920;
                    if composite_target || (k.format == 61 && k.width == 1920) {
                        let scene = self
                            .gpu
                            .as_ref()
                            .unwrap()
                            .images
                            .iter()
                            .filter(|(k2, i)| !i.depth && k2.format == 63 && k2.width >= 1280 && i.last_frame > 0)
                            .max_by_key(|(k2, _)| k2.width as u64 * k2.height as u64)
                            .map(|(k2, _)| *k2);
                        if let Some(s) = scene {
                            key = Some(s);
                        }
                    }
                }
            }
            let (view, layout) = self.sample_view(key);
            image_infos.push(vk::DescriptorImageInfo::default().image_view(view).image_layout(layout));
        }
        for &(_, b) in &vslot_bind {
            let smp = self.sampler_for(d, 0, b);
            sampler_infos.push(vk::DescriptorImageInfo::default().sampler(smp));
        }
        for &(_, b) in &fslot_bind {
            let smp = self.sampler_for(d, 1, b);
            sampler_infos.push(vk::DescriptorImageInfo::default().sampler(smp));
        }
        let mut writes = vec![
            vk::WriteDescriptorSet::default()
                .dst_set(sets[0])
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(std::slice::from_ref(&buffer_infos[0])),
            vk::WriteDescriptorSet::default()
                .dst_set(sets[0])
                .dst_binding(32)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(std::slice::from_ref(&buffer_infos[1])),
        ];
        for (i, &b) in vtex.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_set(sets[0])
                    .dst_binding(1 + b)
                    .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                    .image_info(std::slice::from_ref(&image_infos[i])),
            );
        }
        for (i, &b) in ftex.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_set(sets[0])
                    .dst_binding(33 + b)
                    .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                    .image_info(std::slice::from_ref(&image_infos[vtex.len() + i])),
            );
        }
        for (i, &(slot, _)) in vslot_bind.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_set(sets[1])
                    .dst_binding(slot)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .image_info(std::slice::from_ref(&sampler_infos[i])),
            );
        }
        for (i, &(slot, _)) in fslot_bind.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_set(sets[1])
                    .dst_binding(32 + slot)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .image_info(std::slice::from_ref(&sampler_infos[vslot_bind.len() + i])),
            );
        }
        let tu = std::time::Instant::now();
        self.device.update_descriptor_sets(&writes, &[]);
        UDP_NS.fetch_add(tu.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
        if self.state_pipe != pipeline {
            self.device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pipeline);
            self.state_pipe = pipeline;
        }
        if self.state_blend != d.blend_color {
            self.device.cmd_set_blend_constants(cb, &d.blend_color);
            self.state_blend = d.blend_color;
        }
        let tb = std::time::Instant::now();
        self.device.cmd_bind_descriptor_sets(cb, vk::PipelineBindPoint::GRAPHICS, layout, 0, &sets, &[]);
        BDS_NS.fetch_add(tb.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
        let (vx, vy, vw, vh) = d.viewport.map_or((0.0, 0.0, extent.width as f32, extent.height as f32), |v| (v[0], v[1], v[2], v[3]));
        if self.state_vp != (vx, vy, vw, vh) {
            self.device.cmd_set_viewport(
                cb,
                0,
                &[vk::Viewport { x: vx, y: vy, width: vw, height: vh, min_depth: 0.0, max_depth: 1.0 }],
            );
            self.state_vp = (vx, vy, vw, vh);
        }
        let (sx, sy, sw, sh) = d
            .scissor
            .map_or((0, 0, extent.width as i32, extent.height as i32), |s| (s[0], s[1], s[2], s[3]));
        if self.state_sc != (sx, sy, sw, sh) {
            self.device.cmd_set_scissor(
                cb,
                0,
                &[vk::Rect2D {
                    offset: vk::Offset2D { x: sx.max(0), y: sy.max(0) },
                    extent: vk::Extent2D { width: sw.max(1) as u32, height: sh.max(1) as u32 },
                }],
            );
            self.state_sc = (sx, sy, sw, sh);
        }
        if use_indexed {
            let indices: Vec<u32> = match (&converted, d.index) {
                (Some(v), _) => v.clone(),
                (None, Some((ty, addr))) => match ty {
                    0 => (0..d.count as u64).map(|i| unsafe { *(addr as *const u8).add(i as usize) as u32 }).collect(),
                    1 => (0..d.count as u64).map(|i| unsafe { *(addr as *const u16).add(i as usize) as u32 }).collect(),
                    _ => (0..d.count as u64).map(|i| unsafe { *(addr as *const u32).add(i as usize) }).collect(),
                },
                (None, None) => Vec::new(),
            };
            let bytes: &[u8] = unsafe { std::slice::from_raw_parts(indices.as_ptr() as *const u8, indices.len() * 4) };
            let (off, _) = self.arena_write_at(bytes, 4)?;
            let arena = self.gpu.as_ref().unwrap().arena;
            self.device.cmd_bind_index_buffer(cb, arena, off, vk::IndexType::UINT32);
            let td = std::time::Instant::now();
            self.device
                .cmd_draw_indexed(cb, indices.len() as u32, d.instances, first, vertex_offset, d.base_instance);
            DCMD_NS.fetch_add(td.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
        } else {
            let td = std::time::Instant::now();
            self.device.cmd_draw(cb, d.count, d.instances, d.first, d.base_instance);
            DCMD_NS.fetch_add(td.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
        }
        Ok(())
    }

    unsafe fn close_pass(&mut self, cb: vk::CommandBuffer) {
        let Some((target, _)) = self.open_pass.take() else { return };
        self.device.cmd_end_render_pass(cb);
        let frame = self.gpu.as_ref().unwrap().frame;
        if let Some(img) = self.gpu.as_mut().unwrap().images.get_mut(&target) {
            img.rendered = true;
            img.last_frame = frame;
            img.layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
        }
    }

    unsafe fn gpu_copy(&mut self, cb: vk::CommandBuffer, c: &GpuCopy) -> Result<()> {
        self.ensure_image(c.src)?;
        self.ensure_image(c.dst)?;
        let (src, src_old, src_format, src_extent) = {
            let g = self.gpu.as_ref().unwrap();
            let i = &g.images[&c.src];
            (i.image, i.layout, i.format, i.extent)
        };
        let (dst, dst_old, dst_format) = {
            let g = self.gpu.as_ref().unwrap();
            let i = &g.images[&c.dst];
            (i.image, i.layout, i.format)
        };
        if src_format != dst_format {
            bail!("copy between formats {src_format:?} and {dst_format:?}");
        }
        let extent = vk::Extent3D {
            width: c.extent[0].min(src_extent.width.saturating_sub(c.src_off[0])).max(1),
            height: c.extent[1].min(src_extent.height.saturating_sub(c.src_off[1])).max(1),
            depth: c.extent[2].max(1),
        };
        let stage = vk::PipelineStageFlags::TRANSFER;
        let all = vk::PipelineStageFlags::TOP_OF_PIPE | vk::PipelineStageFlags::ALL_COMMANDS;
        barrier(
            cb,
            &self.device,
            src,
            src_old,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            all,
            stage,
            vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
            vk::AccessFlags::TRANSFER_READ,
        );
        barrier(
            cb,
            &self.device,
            dst,
            dst_old,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            all,
            stage,
            vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE,
            vk::AccessFlags::TRANSFER_WRITE,
        );
        self.device.cmd_copy_image(
            cb,
            src,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            dst,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[vk::ImageCopy::default()
                .src_subresource(vk::ImageSubresourceLayers::default().aspect_mask(vk::ImageAspectFlags::COLOR).layer_count(1))
                .dst_subresource(vk::ImageSubresourceLayers::default().aspect_mask(vk::ImageAspectFlags::COLOR).layer_count(1))
                .src_offset(vk::Offset3D { x: c.src_off[0] as i32, y: c.src_off[1] as i32, z: c.src_off[2] as i32 })
                .dst_offset(vk::Offset3D { x: c.dst_off[0] as i32, y: c.dst_off[1] as i32, z: c.dst_off[2] as i32 })
                .extent(extent)],
        );
        let end_stage = vk::PipelineStageFlags::FRAGMENT_SHADER | vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        barrier(
            cb,
            &self.device,
            src,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            stage,
            end_stage,
            vk::AccessFlags::TRANSFER_READ,
            vk::AccessFlags::SHADER_READ,
        );
        barrier(
            cb,
            &self.device,
            dst,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            stage,
            end_stage,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::SHADER_READ,
        );
        let frame = self.gpu.as_ref().unwrap().frame;
        for key in [c.src, c.dst] {
            if let Some(img) = self.gpu.as_mut().unwrap().images.get_mut(&key) {
                img.layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
                if key == c.dst {
                    img.last_frame = frame;
                    img.rendered = true;
                }
            }
        }
        Ok(())
    }
}

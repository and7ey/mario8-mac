// Mush8 Android - draw_android.rs
// CPU clear replay plus a GPU replay of NVN draws through gfx_vk.
use super::cmd::{self, Cmd, TexBuilder};
use crate::gfx_vk::{self, GpuCopy, GpuDraw, GpuFrame, GpuTex};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{LazyLock, Mutex};

static UPPER_LEFT: AtomicBool = AtomicBool::new(false);
static ZERO_TO_ONE: AtomicBool = AtomicBool::new(false);
static PRESENTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static DISPATCHES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static COPY_TO_BUF: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static COPY_FROM_BUF: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn set_origin_upper_left(v: bool) {
    UPPER_LEFT.store(v, Relaxed);
}

pub fn set_depth_zero_to_one(v: bool) {
    ZERO_TO_ONE.store(v, Relaxed);
}

fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

fn rd32(addr: u64) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

fn texture_addr(b: &[u8; 0x68]) -> u64 {
    let pool = u64::from_le_bytes(b[8..16].try_into().unwrap());
    let offset = u64::from_le_bytes(b[16..24].try_into().unwrap());
    let s = super::objects::STATE.lock().unwrap();
    let addr = s.pool_offset_cpu(pool, offset);
    if addr.is_none() {
        static SEEN: std::sync::Mutex<Option<std::collections::HashSet<u64>>> = std::sync::Mutex::new(None);
        let mut g = SEEN.lock().unwrap();
        if g.get_or_insert_with(Default::default).insert(pool) {
            let p = s.pools.get(&pool);
            log::info!(
                "[tex] pool {pool:#x} unresolved: known {} cpu {:?} size {:?} vmaps {} registers {}",
                p.is_some(),
                p.map(|p| p.cpu),
                p.map(|p| p.size),
                s.virtual_maps.contains_key(&pool),
                s.registered_textures.keys().filter(|(k, _)| *k == pool).count()
            );
        }
    }
    addr.unwrap_or((pool | 1 << 63) + offset)
}

fn color_pattern(format: u32, rgba: [f32; 4]) -> Option<Vec<u8>> {
    let c = |i: usize| (rgba[i].clamp(0.0, 1.0) * 255.0).round() as u8;
    Some(match format {
        37 | 55 => vec![c(0), c(1), c(2), c(3)],
        60 => {
            let r = (rgba[0].clamp(0.0, 1.0) * 31.0).round() as u16;
            let g = (rgba[1].clamp(0.0, 1.0) * 63.0).round() as u16;
            let b = (rgba[2].clamp(0.0, 1.0) * 31.0).round() as u16;
            let a = if rgba[3] > 0.0 { 1u16 } else { 0 };
            ((r << 11) | (g << 5) | b | (a << 15)).to_le_bytes().to_vec()
        }
        _ => return None,
    })
}

fn clear_color(b: &TexBuilder, rgba: [f32; 4]) {
    let format = u32_at(b, 0x30);
    let Some(pattern) = color_pattern(format, rgba) else { return };
    let addr = texture_addr(b);
    if addr == 0 || addr >> 63 != 0 {
        return;
    }
    let size = super::texture_builder_storage(b.as_ptr() as u64).0;
    if size == 0 {
        return;
    }
    let slice = unsafe { std::slice::from_raw_parts_mut(addr as *mut u8, size as usize) };
    if pattern.len() == 4 {
        let p: [u8; 4] = pattern.try_into().unwrap();
        for chunk in slice.chunks_exact_mut(4) {
            chunk.copy_from_slice(&p);
        }
    } else if pattern.len() == 2 {
        let p: [u8; 2] = pattern.try_into().unwrap();
        for chunk in slice.chunks_exact_mut(2) {
            chunk.copy_from_slice(&p);
        }
    }
}

fn tex_of(b: &TexBuilder) -> GpuTex {
    let target = u32_at(b, 0x1c);
    let depth = u32_at(b, 0x28).max(1);
    let layers = match target {
        4 | 6 => depth,
        8 => 6,
        9 => depth,
        _ => 1,
    };
    GpuTex {
        addr: texture_addr(b),
        format: u32_at(b, 0x30),
        target,
        width: u32_at(b, 0x20).max(1),
        height: u32_at(b, 0x24).max(1),
        depth,
        levels: u32_at(b, 0x2c).max(1),
        layers,
    }
}

pub(crate) fn tex_key(pool: u64, handle: u64) -> Option<GpuTex> {
    if handle == 0 {
        return None;
    }
    let tid = (handle & 0xF_FFFF) as u32;
    let b = super::objects::STATE.lock().unwrap().registered_textures.get(&(pool, tid)).copied()?;
    Some(tex_of(&b))
}

#[derive(Clone)]
struct GState {
    program: Vec<(u64, u64)>,
    attribs: [u32; 16],
    streams: [(u32, u32); 16],
    vbufs: [(u64, u64); 16],
    ubos: [[(u64, u64); 15]; 2],
    storage: [[(u64, u64); 16]; 2],
    textures: [[u64; 32]; 2],
    viewport: Option<[f32; 4]>,
    scissor: Option<[i32; 4]>,
    color_state: u32,
    channel_mask: u32,
    blend: [u64; 8],
    blend_color: [f32; 4],
    depth: Option<Box<TexBuilder>>,
    texture_pool: u64,
    sampler_pool: u64,
    render_enable: bool,
    rasterizer_discard: bool,
}

impl Default for GState {
    fn default() -> Self {
        Self {
            program: Vec::new(),
            attribs: [0; 16],
            streams: [(0, 0); 16],
            vbufs: [(0, 0); 16],
            ubos: [[(0, 0); 15]; 2],
            storage: [[(0, 0); 16]; 2],
            textures: [[0; 32]; 2],
            viewport: None,
            scissor: None,
            color_state: 0,
            channel_mask: 0xFFFF_FFFF,
            blend: std::array::from_fn(|i| i as u64),
            blend_color: [0.0; 4],
            depth: None,
            texture_pool: 0,
            sampler_pool: 0,
            render_enable: true,
            rasterizer_discard: false,
        }
    }
}

#[derive(Default)]
struct Pending {
    draws: Vec<(GpuTex, GpuDraw)>,
    copies: Vec<GpuCopy>,
    clears: std::collections::HashMap<GpuTex, [f32; 4]>,
    depth_clears: Vec<(GpuTex, f32)>,
    patches: Vec<(u64, u64, Vec<u8>)>,
    dispatches: Vec<crate::gfx_vk::GpuDispatch>,
}

static PENDING: LazyLock<Mutex<Pending>> = LazyLock::new(Default::default);
static SEEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LOGGED: AtomicBool = AtomicBool::new(false);

fn stage_index(stage: u32) -> Option<usize> {
    match stage {
        0 => Some(0),
        1 => Some(1),
        _ => None,
    }
}

fn stage_blobs(program: &[(u64, u64)]) -> Vec<(Vec<u8>, u64)> {
    let mut out = Vec::new();
    for &(code, ctl) in program {
        let len = rd32(ctl + 0x700) as usize;
        if !(8..=0x10_0000).contains(&len) {
            continue;
        }
        let blob = unsafe { std::slice::from_raw_parts(code as *const u8, len) }.to_vec();
        let Some(sph) = nxshader::sph::Sph::parse(blob.get(0x30..).unwrap_or(&[])) else { continue };
        if !matches!(sph.stage, nxshader::sph::Stage::Vertex | nxshader::sph::Stage::Fragment) {
            continue;
        }
        out.push((blob, code + len as u64));
    }
    out
}

fn compute_stage(program: &[(u64, u64)]) -> Option<(Vec<u8>, u64, [u32; 3])> {
    for &(code, ctl) in program {
        let len = rd32(ctl + 0x700) as usize;
        if !(8..=0x10_0000).contains(&len) {
            continue;
        }
        let blob = unsafe { std::slice::from_raw_parts(code as *const u8, len) }.to_vec();
        let Some(sph) = nxshader::sph::Sph::parse(blob.get(0x30..).unwrap_or(&[])) else { continue };
        if sph.stage != nxshader::sph::Stage::Compute {
            continue;
        }
        let local_size = [0x748u64, 0x74c, 0x750].map(|o| rd32(ctl + o));
        if local_size.iter().any(|&n| n == 0 || n > 1024) {
            continue;
        }
        return Some((blob, code + len as u64, local_size));
    }
    None
}

fn make_draw(st: &GState, d: &cmd::Draw, target: GpuTex) -> Option<GpuDraw> {
    let depth = st.depth.as_ref().map(|b| tex_of(b));
    if d.count == 0 || d.instances == 0 || st.rasterizer_discard || !st.render_enable {
        return None;
    }
    let stages = stage_blobs(&st.program);
    if stages.is_empty() {
        return None;
    }
    let alpha = st.color_state >> 24 & 0xF;
    let mut flags = (!UPPER_LEFT.load(Relaxed)) as u32 | ((!ZERO_TO_ONE.load(Relaxed)) as u32) << 1;
    let (alpha_func, alpha_ref) = if alpha != 0 && alpha != 8 {
        flags |= 4;
        (alpha - 1, 0.0)
    } else {
        (7, 0.0)
    };
    Some(GpuDraw {
        target,
        depth,
        texture_pool: st.texture_pool,
        sampler_pool: st.sampler_pool,
        stages,
        attribs: st.attribs,
        streams: st.streams,
        vbufs: st.vbufs,
        ubos: st.ubos,
        storage: st.storage,
        textures: st.textures,
        viewport: st.viewport,
        scissor: st.scissor,
        flags,
        color_state: st.color_state,
        channel_mask: st.channel_mask,
        blend: st.blend[0],
        blend_color: st.blend_color,
        alpha_func,
        alpha_ref,
        mode: d.mode,
        first: d.first,
        count: d.count,
        index: d.index,
        instances: d.instances,
        base_instance: d.base_instance,
        base_vertex: d.base_vertex,
    })
}

fn replay(cmds: &[Cmd], colors: &mut Vec<Option<Box<TexBuilder>>>, depth: &mut Option<Box<TexBuilder>>, st: &mut GState, out: &mut Pending, level: usize) {
    if level > 8 {
        return;
    }
    for c in cmds {
        match c {
            Cmd::SetRenderTargets { colors: c, depth: d } => {
                *colors = c.clone();
                *depth = d.clone();
                st.depth = d.clone();
            }
            Cmd::ClearColor { index, rgba } => {
                if let Some(Some(b)) = colors.get(*index as usize) {
                    clear_color(b, *rgba);
                    if *index == 0 {
                        out.clears.insert(tex_of(b), *rgba);
                    }
                }
            }
            Cmd::ClearDepthStencil { depth: v, .. } => {
                if let Some(b) = st.depth.as_ref() {
                    out.depth_clears.push((tex_of(b), *v));
                }
            }
            Cmd::ClearTexture { dst, region, color, .. } => {
                let (w, h) = (u32_at(&dst[..], 0x20), u32_at(&dst[..], 0x24));
                let full = region[0] <= 0 && region[1] <= 0 && region[3] as u32 >= w && region[4] as u32 >= h;
                if full {
                    clear_color(dst, *color);
                }
            }
            Cmd::Call(handles) => {
                for h in handles {
                    if let Some(sub) = cmd::recording(*h) {
                        replay(&sub, colors, depth, st, out, level + 1);
                    }
                }
            }
            Cmd::Program(v) => st.program = v.clone(),
            Cmd::VertexAttribs(v) => {
                for (i, a) in v.iter().take(16).enumerate() {
                    st.attribs[i] = *a;
                }
            }
            Cmd::VertexStreams(v) => {
                for (i, s) in v.iter().take(16).enumerate() {
                    st.streams[i] = *s;
                }
            }
            Cmd::VertexBuffer { index, addr, size } => {
                if let Some(s) = st.vbufs.get_mut(*index as usize) {
                    *s = (*addr, *size);
                }
            }
            Cmd::Uniform { stage, index, addr, size } => {
                if let Some(u) = stage_index(*stage).and_then(|s| st.ubos[s].get_mut(*index as usize)) {
                    *u = (*addr, *size);
                }
            }
            Cmd::Storage { stage, index, addr, size } => {
                if let Some(u) = stage_index(*stage).and_then(|s| st.storage[s].get_mut(*index as usize)) {
                    *u = (*addr, *size);
                }
            }
            Cmd::Texture { stage, index, handle } => {
                if let Some(t) = stage_index(*stage).and_then(|s| st.textures[s].get_mut(*index as usize)) {
                    *t = *handle;
                }
            }
            Cmd::UpdateUniform { addr, size, offset, data } => {
                let size = (*size).clamp(*offset + data.len() as u64, 0x10000);
                let mut patched = unsafe { std::slice::from_raw_parts(*addr as *const u8, size as usize) }.to_vec();
                let off = *offset as usize;
                if off + data.len() <= patched.len() {
                    patched[off..off + data.len()].copy_from_slice(data);
                }
                out.patches.push((*addr, size, patched));
            }
            Cmd::TexturePool(p) => st.texture_pool = *p,
            Cmd::SamplerPool(p) => st.sampler_pool = *p,
            Cmd::Viewport(v) => st.viewport = Some(*v),
            Cmd::Scissor(v) => st.scissor = Some(*v),
            Cmd::ColorState(v) => st.color_state = *v,
            Cmd::ChannelMask(v) => st.channel_mask = *v,
            Cmd::Blend(v) => st.blend[(*v & 7) as usize] = *v,
            Cmd::BlendColor(v) => st.blend_color = *v,
            Cmd::RenderEnable(v) => st.render_enable = *v,
            Cmd::RasterizerDiscard(v) => st.rasterizer_discard = *v,
            Cmd::Draw(d) => {
                SEEN.fetch_add(1, Relaxed);
                if let Some(Some(b)) = colors.first() {
                    let target = tex_of(b);
                    if let Some(draw) = make_draw(st, d, target) {
                        out.draws.push((target, draw));
                    }
                }
            }
            Cmd::CopyTextureToTexture { src, src_region, dst, dst_region, flags: _ } => {
                let (s, d) = (tex_of(src), tex_of(dst));
                let region = |r: [i32; 6]| ([r[0].max(0) as u32, r[1].max(0) as u32, r[2].max(0) as u32], [r[3].max(1) as u32, r[4].max(1) as u32, r[5].max(1) as u32]);
                let (so, se) = region(*src_region);
                let (do_, _) = region(*dst_region);
                out.copies.push(GpuCopy { src: s, dst: d, src_off: so, dst_off: do_, extent: se });
            }
            Cmd::CopyTextureToBuffer { .. } => {
                COPY_TO_BUF.fetch_add(1, Relaxed);
            }
            Cmd::CopyBufferToTexture { .. } => {
                COPY_FROM_BUF.fetch_add(1, Relaxed);
            }
            Cmd::Dispatch(groups) => {
                DISPATCHES.fetch_add(1, Relaxed);
                if std::env::var_os("NX_VK_COMPUTE").is_some() {
                    if let Some((blob, constants, local_size)) = compute_stage(&st.program) {
                        out.dispatches.push(crate::gfx_vk::GpuDispatch {
                            blob,
                            constants,
                            local_size,
                            groups: *groups,
                            ubos: st.ubos[0].to_vec(),
                            storage: st.storage[0].to_vec(),
                            textures: st.textures[0].to_vec(),
                            texture_pool: st.texture_pool,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn submit(handles: &[u64]) {
    let mut colors = Vec::new();
    let mut depth = None;
    let mut st = GState::default();
    let mut out = Pending::default();
    for h in handles {
        if let Some(cmds) = cmd::recording(*h) {
            replay(&cmds, &mut colors, &mut depth, &mut st, &mut out, 0);
        }
    }
    let seen = SEEN.load(Relaxed);
    if seen > 0 && !LOGGED.swap(true, Relaxed) {
        let mut by: std::collections::BTreeMap<GpuTex, (usize, Vec<u32>)> = Default::default();
        for (t, d) in &out.draws {
            let e = by.entry(*t).or_insert((0, Vec::new()));
            e.0 += 1;
            if e.1.len() < 8 {
                e.1.push(d.count);
            }
        }
        log::info!("[gpu] draw commands seen: {seen}, collected {} targets {by:?}", out.draws.len());
    }
    let mut pending = PENDING.lock().unwrap();
    pending.draws.extend(out.draws);
    pending.patches.extend(out.patches);
    pending.depth_clears.extend(out.depth_clears);
    for (k, v) in out.clears {
        pending.clears.insert(k, v);
    }
}

struct PStat {
    since: Option<std::time::Instant>,
    n: u32,
    kept: u32,
    dropped: u32,
    copies: u32,
    hits: u32,
}

static PSTATS: LazyLock<Mutex<PStat>> =
    LazyLock::new(|| Mutex::new(PStat { since: None, n: 0, kept: 0, dropped: 0, copies: 0, hits: 0 }));

fn present_stats(target: &GpuTex, draws: &[GpuDraw], dropped: usize, copies: usize, hits: usize, drop_desc: &str) {
    let mut s = PSTATS.lock().unwrap();
    let since = *s.since.get_or_insert_with(std::time::Instant::now);
    s.n += 1;
    s.kept += draws.len() as u32;
    s.dropped += dropped as u32;
    s.copies += copies as u32;
    s.hits += hits as u32;
    let secs = since.elapsed().as_secs_f32();
    if secs < 2.0 {
        return;
    }
    let mut by: std::collections::BTreeMap<(u32, u32, u32, u64), u32> = Default::default();
    for d in draws {
        *by.entry((d.target.format, d.target.width, d.target.height, d.target.addr)).or_default() += 1;
    }
    let distinct = by.len();
    let mut ranked: Vec<_> = by.into_iter().collect();
    ranked.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    let top: Vec<String> = ranked.iter().take(3).map(|((f, w, h, a), n)| format!("fmt{f} {w}x{h}@{a:x} x{n}")).collect();
    log::info!(
        "[frame] {secs:.0}s: {} presents; present fmt{} {}x{}@{:x}; draws kept {} dropped {} hits {}; copies {}; computes {}; tex->buf {} buf->tex {}; dropped [{}]; {distinct} targets [{}]",
        s.n, target.format, target.width, target.height, target.addr, s.kept, s.dropped, s.hits, s.copies, DISPATCHES.swap(0, Relaxed), COPY_TO_BUF.swap(0, Relaxed), COPY_FROM_BUF.swap(0, Relaxed), drop_desc, top.join(", ")
    );
    *s = PStat { since: Some(std::time::Instant::now()), n: 0, kept: 0, dropped: 0, copies: 0, hits: 0 };
}

pub fn present(texture: u64) {
    let Some(t) = super::objects::texture(texture) else {
        static MISS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        if MISS.fetch_add(1, Relaxed) < 3 {
            log::warn!("[frame] present of unknown texture {texture:#x}");
        }
        return;
    };
    let b = t.builder;
    let target = tex_of(&b);
    let (draws, copies, clears, depth_clears, patches, dispatches, dropped, drop_desc) = {
        let mut pending = PENDING.lock().unwrap();
        let patches = std::mem::take(&mut pending.patches);
        let depth_clears = std::mem::take(&mut pending.depth_clears);
        let copies = std::mem::take(&mut pending.copies);
        let dispatches = std::mem::take(&mut pending.dispatches);
        let clears: Vec<(GpuTex, [f32; 4])> = pending.clears.drain().collect();
        let mut by_size: std::collections::BTreeMap<(u32, u32, u32), u32> = Default::default();
        let draws: Vec<GpuDraw> = pending
            .draws
            .drain(..)
            .map(|(t, d)| {
                *by_size.entry((t.format, t.width, t.height)).or_default() += 1;
                d
            })
            .collect();
        let dropped = 0usize;
        let mut dv: Vec<_> = by_size.iter().collect();
        dv.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let drop_desc: String = dv.iter().take(3).map(|((f, w, h), n)| format!("fmt{f} {w}x{h} x{n}")).collect::<Vec<_>>().join(", ");
        (draws, copies, clears, depth_clears, patches, dispatches, dropped, drop_desc)
    };
    let n = PRESENTS.fetch_add(1, Relaxed);
    if n < 8 {
        log::info!("[frame] present #{n}: {}x{} fmt={} draws={} clears={}", target.width, target.height, target.format, draws.len(), clears.len());
    }
    let hits = draws.iter().filter(|d| d.target == target).count();
    present_stats(&target, &draws, dropped, copies.len(), hits, &drop_desc);
    if !draws.is_empty() || !copies.is_empty() || gfx_vk::gpu_active() {
        gfx_vk::submit_gpu_frame(GpuFrame { present: target, dispatches, draws, copies, patches, clears, depth_clears });
        return;
    }
    if let Some((w, h, mut rgba)) = read_rgba(&b) {
        if !UPPER_LEFT.load(Relaxed) {
            let row = w as usize * 4;
            let mut flipped = vec![0u8; rgba.len()];
            for y in 0..h as usize {
                flipped[y * row..(y + 1) * row].copy_from_slice(&rgba[(h as usize - 1 - y) * row..(h as usize - y) * row]);
            }
            rgba = flipped;
        }
        crate::gfx_vk::submit_frame(w, h, rgba);
    }
}

fn read_rgba(b: &[u8; 0x68]) -> Option<(u32, u32, Vec<u8>)> {
    let format = u32_at(b, 0x30);
    let bpp = match format {
        37 | 55 => 4u32,
        60 => 2u32,
        _ => return None,
    };
    if u32_at(b, 0x1c) != 1 {
        return None;
    }
    let width = u32_at(b, 0x20).max(1);
    let height = u32_at(b, 0x24).max(1);
    if width > 4096 || height > 4096 {
        return None;
    }
    let addr = texture_addr(b);
    if addr == 0 || addr >> 63 != 0 {
        return None;
    }
    let row = (width * bpp) as usize;
    let mut raw = vec![0u8; row * height as usize];
    match super::texture_layout(b.as_ptr() as u64) {
        Some((shape, layout)) => {
            if shape.width != width || shape.height != height || layout.level_offsets.is_empty() {
                return None;
            }
            let offset = layout.level_offsets[0] as usize;
            let len = layout.total_size as usize;
            let src = unsafe { std::slice::from_raw_parts((addr as *const u8).add(offset), len.saturating_sub(offset)) };
            let (gy, gz) = layout.level_gobs[0];
            super::tex::untile_into(src, row as u32, height, 0, (gy, gz), 0..height, &mut raw);
        }
        None => {
            let stride = u32_at(b, 0x5c) as usize;
            if stride < row {
                return None;
            }
            let src = unsafe { std::slice::from_raw_parts(addr as *const u8, stride * height as usize) };
            for y in 0..height as usize {
                raw[y * row..(y + 1) * row].copy_from_slice(&src[y * stride..y * stride + row]);
            }
        }
    }
    if bpp == 4 {
        return Some((width, height, raw));
    }
    let mut rgba = vec![0u8; width as usize * height as usize * 4];
    for (i, px) in raw.chunks_exact(2).enumerate() {
        let v = u16::from_le_bytes([px[0], px[1]]);
        let r = ((v >> 11) & 0x1f) as u8;
        let g = ((v >> 5) & 0x3f) as u8;
        let b = (v & 0x1f) as u8;
        rgba[i * 4] = (r << 3) | (r >> 2);
        rgba[i * 4 + 1] = (g << 2) | (g >> 4);
        rgba[i * 4 + 2] = (b << 3) | (b >> 2);
        rgba[i * 4 + 3] = 255;
    }
    Some((width, height, rgba))
}

pub fn forget_memory(_addr: u64, _size: u64) {}

static SYNCS: LazyLock<Mutex<std::collections::HashMap<u64, u64>>> = LazyLock::new(|| Mutex::new(Default::default()));

pub fn fence_sync(sync: u64) {
    let seq = crate::gfx_vk::submitted();
    if std::env::var_os("NX_SYNC_TRACE").is_some() {
        log::info!("[sync] fence {sync:#x} seq {seq} presented {}", crate::gfx_vk::presented_now());
    }
    SYNCS.lock().unwrap().insert(sync, seq);
}

pub fn finish() {}

pub fn sync_wait(sync: u64) -> u64 {
    let Some(seq) = SYNCS.lock().unwrap().remove(&sync) else { return 0 };
    let t0 = std::time::Instant::now();
    crate::gfx_vk::wait_presented(seq, std::time::Duration::from_secs(10));
    let ms = t0.elapsed().as_millis();
    if ms > 200 || std::env::var_os("NX_SYNC_TRACE").is_some() {
        log::info!(
            "[sync] wait {sync:#x} seq {seq} took {ms} ms: submitted {} presented {}",
            crate::gfx_vk::submitted(),
            crate::gfx_vk::presented_now()
        );
    }
    1
}

static UPLOADS: LazyLock<Mutex<std::collections::HashMap<GpuTex, (u64, Vec<u8>)>>> = LazyLock::new(|| Mutex::new(Default::default()));
static UPLOAD_GEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

pub(crate) fn uploaded(key: &GpuTex) -> Option<(u64, Vec<u8>)> {
    UPLOADS.lock().unwrap().get(key).cloned()
}

pub fn write_texels(mut b: TexBuilder, view: u64, region: [i32; 6], data: *const u8, strides: Option<(usize, usize)>) {
    super::objects::apply_view(&mut b, view, true);
    if data.is_null() {
        return;
    }
    let key = tex_of(&b);
    let (bw, bh, bpb) = super::format_info(key.format);
    let [x, y, _, w, h, d] = region.map(|v| v.max(0) as usize);
    if w == 0 || h == 0 {
        return;
    }
    let (fw, fh) = (key.width as usize, key.height as usize);
    let full_row = fw.div_ceil(bw as usize) * bpb as usize;
    let row = w.div_ceil(bw as usize) * bpb as usize;
    let (src_row, _) = strides.filter(|s| s.0 >= row).unwrap_or((row, 0));
    let mut guard = UPLOADS.lock().unwrap();
    let g = UPLOAD_GEN.fetch_add(1, Relaxed);
    let entry = guard.entry(key).or_insert_with(|| (0, vec![0u8; full_row * fh * d.max(1)]));
    entry.0 = g;
    if entry.1.len() < full_row * fh * d.max(1) {
        entry.1.resize(full_row * fh * d.max(1), 0);
    }
    for layer in 0..d.max(1) {
        for yy in 0..h {
            let dst = layer * full_row * fh + (y + yy) * full_row + x * bpb as usize;
            if dst + row > entry.1.len() {
                continue;
            }
            let src = unsafe { std::slice::from_raw_parts(data.add(layer * src_row * h + yy * src_row), row) };
            entry.1[dst..dst + row].copy_from_slice(src);
        }
    }
}

pub fn read_texels(_b: TexBuilder, _view: u64, _region: [i32; 6], _data: *mut u8, _strides: Option<(usize, usize)>) {}

pub fn precompile(_romfs: nxfs::storage::DynStorage, _modules: &[std::path::PathBuf]) -> anyhow::Result<()> {
    Ok(())
}

pub fn prewarm() {}

pub(crate) fn tex_builder(pool: u64, handle: u64) -> Option<Box<TexBuilder>> {
    if handle == 0 {
        return None;
    }
    let tid = (handle & 0xF_FFFF) as u32;
    super::objects::STATE.lock().unwrap().registered_textures.get(&(pool, tid)).copied().map(Box::new)
}

pub(crate) fn sampler_desc(pool: u64, sid: u32) -> Option<[u8; 0x60]> {
    super::objects::STATE.lock().unwrap().registered_samplers.get(&(pool, sid)).copied()
}

pub(crate) fn tex_pixels(b: &TexBuilder) -> Option<(Vec<u8>, u32, u32)> {
    let format = u32_at(b, 0x30);
    let target = u32_at(b, 0x1c);
    let width = u32_at(b, 0x20).max(1);
    let height = if target == 3 { 1 } else { u32_at(b, 0x24).max(1) };
    if width > 8192 || height > 8192 {
        return None;
    }
    let depth = u32_at(b, 0x28).max(1);
    let addr = texture_addr(b);
    if addr == 0 || addr >> 63 != 0 {
        return None;
    }
    let layout = super::texture_layout(b.as_ptr() as u64);
    let default_layers = match target {
        4 | 6 | 9 => depth,
        8 => 6,
        _ => 1,
    };
    let layers = layout.as_ref().map_or(default_layers, |(s, _)| s.layers.max(1));
    let bpp = match format {
        1 | 2 | 3 | 4 => 1u32,
        5 | 6 | 7 | 8 | 9 | 13 | 14 | 15 | 16 => 2,
        10 | 11 | 12 | 17 | 18 | 19 | 37 | 38 | 39 | 40 | 55 | 56 | 60 | 61 | 62 | 63 | 64 => 4,
        41 | 42 | 43 | 44 | 45 | 22 | 23 | 24 => 8,
        46 | 47 | 48 => 16,
        _ => 0,
    };
    if let Some((bw, bh, _srgb)) = astc_kind(format) {
        let cols = width.div_ceil(bw);
        let rows = height.div_ceil(bh);
        let row = cols * 16;
        let mut raw = vec![0u8; (row * rows * layers) as usize];
        let layer_size = (row * rows) as usize;
        for layer in 0..layers {
            fill_layer(&layout, addr, layer, row, rows, &mut raw[layer as usize * layer_size..])?;
        }
        return Some((raw, row, height));
    }
    if let Some(kind) = crate::bc_soft::soft_rgba(format) {
        let block = crate::bc_soft::block_bytes(kind) as u32;
        let cols = width.div_ceil(4);
        let rows = height.div_ceil(4);
        let row = cols * block;
        let mut decoded = Vec::with_capacity((width * height * 4 * layers) as usize);
        for layer in 0..layers {
            let mut raw = vec![0u8; (row * rows) as usize];
            fill_layer(&layout, addr, layer, row, rows, &mut raw)?;
            let d = crate::bc_soft::decode(kind, &raw, row as usize, width as usize, height as usize);
            decoded.extend_from_slice(&d);
        }
        return Some((decoded, width * 4, height));
    }
    if bpp == 0 {
        return None;
    }
    let row = width * bpp;
    let mut raw = vec![0u8; (row * height * layers) as usize];
    let layer_size = (row * height) as usize;
    for layer in 0..layers {
        fill_layer(&layout, addr, layer, row, height, &mut raw[layer as usize * layer_size..])?;
    }
    Some((raw, row, height))
}

fn astc_kind(nvn: u32) -> Option<(u32, u32, bool)> {
    const DIMS: [(u32, u32); 14] = [(4, 4), (5, 4), (5, 5), (6, 5), (6, 6), (8, 5), (8, 6), (8, 8), (10, 5), (10, 6), (10, 8), (10, 10), (12, 10), (12, 12)];
    if (121..=148).contains(&nvn) {
        let i = ((nvn - 121) % 14) as usize;
        Some((DIMS[i].0, DIMS[i].1, nvn >= 135))
    } else {
        None
    }
}

fn fill_layer(layout: &Option<(super::tex::Shape, super::tex::Layout)>, addr: u64, layer: u32, row: u32, rows: u32, dst: &mut [u8]) -> Option<()> {
    match layout {
        Some((_shape, lay)) => {
            if lay.level_offsets.is_empty() {
                return None;
            }
            let offset = lay.level_offsets[0];
            let base = layer as u64 * lay.layer_size + offset;
            if base >= lay.total_size {
                return None;
            }
            let len = (lay.total_size - base) as usize;
            let src = unsafe { std::slice::from_raw_parts((addr as *const u8).add(base as usize), len) };
            let (gy, gz) = lay.level_gobs[0];
            super::tex::untile_into(src, row, rows, 0, (gy, gz), 0..rows, dst);
        }
        None => {
            let stride = row as usize;
            let base = layer as usize * stride * rows as usize;
            let src = unsafe { std::slice::from_raw_parts(addr as *const u8, base + stride * rows as usize) };
            for y in 0..rows as usize {
                dst[y * stride..(y + 1) * stride].copy_from_slice(&src[base + y * stride..base + y * stride + stride]);
            }
        }
    }
    Some(())
}

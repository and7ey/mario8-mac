// Mush8 - webm.rs
// Constantin Clerc (c22dev) - 0.1
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct Frame {
    pub us: i64,
    pub key: bool,
    pub range: std::ops::Range<usize>,
}

pub struct Movie {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub frames: Vec<Frame>,
    pub duration_us: i64,
}

pub fn is_webm(data: &[u8]) -> bool {
    data.starts_with(&[0x1a, 0x45, 0xdf, 0xa3])
}

fn vint(d: &[u8], p: usize, mask: bool) -> Option<(u64, usize)> {
    let b = *d.get(p)?;
    let len = b.leading_zeros() as usize + 1;
    if len > 8 {
        return None;
    }
    let mut v = if mask { (b as u64) & ((1u64 << (8 - len)) - 1) } else { b as u64 };
    for i in 1..len {
        v = v << 8 | *d.get(p + i)? as u64;
    }
    Some((v, len))
}

fn uint(d: &[u8]) -> u64 {
    d.iter().fold(0, |v, &b| v << 8 | b as u64)
}

const MASTERS: [u64; 6] = [0x18538067, 0x1F43B675, 0x1654AE6B, 0xAE, 0xE0, 0xA0];

impl Movie {
    pub fn parse(data: Vec<u8>) -> Option<Arc<Self>> {
        let mut m = Movie { data: Vec::new(), width: 0, height: 0, frames: Vec::new(), duration_us: 0 };
        let mut scale_ns = 1_000_000u64;
        let mut cluster = 0u64;
        let mut stack = vec![(0usize, data.len())];
        while let Some((mut p, end)) = stack.pop() {
            while p < end {
                let (id, l) = vint(&data, p, false)?;
                let (mut size, l2) = vint(&data, p + l, true)?;
                p += l + l2;
                if size == (1u64 << (7 * l2)) - 1 {
                    size = (end - p) as u64;
                }
                let body = p..(p + size as usize).min(end);
                p = body.end;
                match id {
                    _ if MASTERS.contains(&id) => {
                        stack.push((p, end));
                        stack.push((body.start, body.end));
                        break;
                    }
                    0x2AD7B1 => scale_ns = uint(&data[body]),
                    0xB0 => m.width = uint(&data[body]) as u32,
                    0xBA => m.height = uint(&data[body]) as u32,
                    0xE7 => cluster = uint(&data[body]),
                    0xA3 | 0xA1 => {
                        let (_, tl) = vint(&data, body.start, true)?;
                        let h = body.start + tl;
                        let rel = i16::from_be_bytes([*data.get(h)?, *data.get(h + 1)?]) as i64;
                        let flags = *data.get(h + 2)?;
                        let us = (cluster as i64 + rel) * scale_ns as i64 / 1000;
                        let key = id == 0xA3 && flags & 0x80 != 0 || id == 0xA1 && m.frames.is_empty();
                        m.frames.push(Frame { us, key, range: h + 3..body.end });
                    }
                    _ => {}
                }
            }
        }
        m.frames.sort_by_key(|f| f.us);
        let step = m.frames.windows(2).last().map_or(0, |w| w[1].us - w[0].us);
        m.duration_us = m.frames.last().map_or(0, |f| f.us + step);
        m.data = data;
        (m.width > 0 && m.height > 0 && !m.frames.is_empty()).then(|| Arc::new(m))
    }

    pub fn keyframe_nv12(&self, i: usize, out: &mut [u8], stride: usize, rows: usize) -> bool {
        let f = &self.data[self.frames[i].range.clone()];
        let mut riff = Vec::with_capacity(f.len() + 21);
        riff.extend_from_slice(b"RIFF");
        riff.extend_from_slice(&((f.len() + f.len() % 2 + 12) as u32).to_le_bytes());
        riff.extend_from_slice(b"WEBPVP8 ");
        riff.extend_from_slice(&(f.len() as u32).to_le_bytes());
        riff.extend_from_slice(f);
        if f.len() % 2 == 1 {
            riff.push(0);
        }
        let Ok(mut dec) = image_webp::WebPDecoder::new(std::io::Cursor::new(riff)) else { return false };
        let (w, h) = dec.dimensions();
        let bpp = if dec.has_alpha() { 4 } else { 3 };
        let mut rgb = vec![0u8; w as usize * h as usize * bpp];
        if dec.read_image(&mut rgb).is_err() {
            return false;
        }
        let (w, h) = ((w as usize).min(stride), (h as usize).min(rows));
        let px = |x: usize, y: usize| {
            let o = (y * w + x) * bpp;
            (rgb[o] as i32, rgb[o + 1] as i32, rgb[o + 2] as i32)
        };
        for y in 0..h {
            for x in 0..w {
                let (r, g, b) = px(x, y);
                out[y * stride + x] = ((66 * r + 129 * g + 25 * b + 128) >> 8).wrapping_add(16) as u8;
            }
        }
        let uv = stride * rows;
        for y in (0..h - h % 2).step_by(2) {
            for x in (0..w - w % 2).step_by(2) {
                let (r, g, b) = px(x, y);
                out[uv + y / 2 * stride + x] = (((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128) as u8;
                out[uv + y / 2 * stride + x + 1] = (((112 * r - 94 * g - 18 * b + 128) >> 8) + 128) as u8;
            }
        }
        true
    }
}

const CACHE: &str = "work/movie_cache";

fn converted_name(name: &str) -> String {
    format!("{}.mp4", Path::new(name).file_name().map_or(name.into(), |n| n.to_string_lossy()))
}

pub fn converted(name: &str) -> Option<(String, Vec<u8>)> {
    let file = converted_name(name);
    let bundled = std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join("movies").join(&file)));
    [Some(Path::new(CACHE).join(&file)), bundled].into_iter().flatten().find_map(|p| std::fs::read(p).ok()).map(|d| (file, d))
}

fn ffmpeg() -> Option<PathBuf> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path).chain(["/opt/homebrew/bin".into(), "/usr/local/bin".into()]).map(|d| d.join("ffmpeg")).find(|p| p.is_file())
}

pub fn convert(name: &str, data: &[u8]) -> bool {
    let Some(ffmpeg) = ffmpeg() else { return false };
    let _ = std::fs::create_dir_all(CACHE);
    let out = Path::new(CACHE).join(converted_name(name));
    let (src, tmp) = (out.with_extension("src.webm"), out.with_extension("tmp.mp4"));
    if std::fs::write(&src, data).is_err() {
        return false;
    }
    let ok = std::process::Command::new(ffmpeg)
        .args(["-y", "-loglevel", "error", "-i"])
        .arg(&src)
        .args(["-c:v", "h264_videotoolbox", "-q:v", "55", "-pix_fmt", "yuv420p", "-c:a", "aac", "-movflags", "+faststart"])
        .arg(&tmp)
        .status()
        .is_ok_and(|s| s.success())
        && std::fs::rename(&tmp, &out).is_ok();
    let _ = std::fs::remove_file(&src);
    let _ = std::fs::remove_file(&tmp);
    ok
}

#[cfg(target_os = "macos")]
pub fn convert_later(name: &str, data: &[u8]) {
    static STARTED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
    let mut started = STARTED.lock().unwrap();
    if started.iter().any(|n| n == name) || ffmpeg().is_none() {
        return;
    }
    started.push(name.into());
    let (name, data) = (name.to_string(), data.to_vec());
    std::thread::spawn(move || {
        if convert(&name, &data) {
            eprintln!("[movie] {name}: converted to MP4 for next time");
        }
    });
}

pub fn convert_all(romfs: nxfs::storage::DynStorage) -> anyhow::Result<()> {
    let r = nxfs::romfs::RomFs::open(romfs)?;
    for f in r.files().iter().filter(|f| f.path.to_ascii_lowercase().ends_with(".webm")) {
        if converted(&f.path).is_some() {
            continue;
        }
        let data = r.storage().read_vec(f.offset, f.size as usize)?;
        anyhow::ensure!(convert(&f.path, &data), "{}: conversion failed (is ffmpeg installed?)", f.path);
        let mp4 = converted(&f.path).map_or(0, |(_, d)| d.len());
        eprintln!("[movie] {} converted: {:.1} MB WebM → {:.1} MB MP4", f.path, f.size as f64 / 1e6, mp4 as f64 / 1e6);
    }
    Ok(())
}

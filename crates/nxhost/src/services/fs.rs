// Mush8 - fs.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Request, Response, Service, Stub, write_buffer};
use nxfs::storage::DynStorage;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

mod rc {
    pub const PATH_NOT_FOUND: u32 = 0x202;
    pub const PATH_ALREADY_EXISTS: u32 = 0x402;
    pub const OUT_OF_RANGE: u32 = 0x3E02;
    pub const INVALID_PATH: u32 = 0x1A202;
    pub const TARGET_NOT_FOUND: u32 = 0x7D402;
}

pub struct Config {
    pub romfs: DynStorage,
    pub save_dir: PathBuf,
    pub sd_dir: PathBuf,
    pub system_dir: PathBuf,
    pub firmware_dir: PathBuf,
    pub keys: nxfs::keys::Keys,
    pub dlc: Vec<(u64, DynStorage)>,
}

fn is_data(n: &nxfs::nca::Nca) -> bool {
    matches!(n.content_type, nxfs::nca::ContentType::Data | nxfs::nca::ContentType::PublicData)
}

fn firmware_index(cfg: &Config) -> &'static std::collections::HashMap<u64, PathBuf> {
    static INDEX: OnceLock<std::collections::HashMap<u64, PathBuf>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        let (mut unreadable, mut missing_key) = (0, None);
        for e in std::fs::read_dir(&cfg.firmware_dir).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().is_none_or(|x| x != "nca") {
                continue;
            }
            match nxfs::open_nca(&p, &cfg.keys) {
                Ok(n) if is_data(&n) => {
                    m.insert(n.program_id, p);
                }
                Ok(_) => {}
                Err(err) => {
                    unreadable += 1;
                    missing_key.get_or_insert(err.to_string());
                }
            }
        }
        if unreadable > 0 {
            eprintln!("[fs] firmware: {unreadable} NCAs unreadable ({})", missing_key.unwrap_or_default());
        }
        m
    })
}

pub fn system_data(cfg: &Config, id: u64) -> Option<DynStorage> {
    static CACHE: Mutex<Option<std::collections::HashMap<u64, Option<DynStorage>>>> = Mutex::new(None);
    let mut c = CACHE.lock().unwrap();
    c.get_or_insert_with(Default::default)
        .entry(id)
        .or_insert_with(|| {
            let path = cfg.system_dir.join(format!("{id:016x}.nsp"));
            let romfs = |nca: &nxfs::nca::Nca| -> anyhow::Result<DynStorage> {
                let sec = nca.sections.iter().find(|s| s.fs_type == nxfs::nca::FsType::RomFs).ok_or_else(|| anyhow::anyhow!("no RomFS"))?;
                nca.fs_storage(sec)
            };
            let from_nsp = || -> anyhow::Result<DynStorage> {
                let nsp = nxfs::Nsp::open(&path, &cfg.keys)?;
                let nca = nsp.ncas.iter().find(|n| is_data(n)).ok_or_else(|| anyhow::anyhow!("no Data NCA"))?;
                romfs(nca)
            };
            if path.exists() {
                match from_nsp() {
                    Ok(s) => return Some(s),
                    Err(e) => eprintln!("[fs] {}: {e}", path.display()),
                }
            }
            if let Some(nca) = firmware_index(cfg).get(&id).and_then(|p| nxfs::open_nca(p, &cfg.keys).ok()) {
                match romfs(&nca) {
                    Ok(s) => return Some(s),
                    Err(e) => eprintln!("[fs] firmware title {id:016x}: {e}"),
                }
            }
            eprintln!(
                "[fs] system data {id:016x} unavailable: not in {} (needs keys for its key generation?) nor {}",
                cfg.firmware_dir.display(),
                path.display()
            );
            None
        })
        .clone()
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct FspSrv;

impl Service for FspSrv {
    fn name(&self) -> String {
        "fsp-srv".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        let cfg = CONFIG.get().expect("fs not configured");
        match req.cmd {
            1 | 1003 | 1006 | 22 => {}
            18 => rsp.objects.push(HostFs::open("sdmc", &cfg.sd_dir)),
            51 | 52 => {
                let attr_type = req.data.get(0x08 + 0x20).copied().unwrap_or(0);
                let index = req.data.get(0x08 + 0x22).copied().unwrap_or(0);
                let dir = cfg.save_dir.join(format!("type{attr_type}-index{index}"));
                rsp.objects.push(HostFs::open("save", &dir));
            }
            61 | 60 => rsp.objects.push(Arc::new(EmptyInfoReader)),
            200 | 203 => rsp.objects.push(Arc::new(Storage { name: "romfs", s: cfg.romfs.clone() })),
            202 => {
                let id = req.u64(8);
                if let Some((_, s)) = cfg.dlc.iter().find(|(t, _)| *t == id) {
                    rsp.objects.push(Arc::new(Storage { name: "add-on content", s: s.clone() }));
                    return 0;
                }
                match system_data(cfg, id) {
                    Some(s) => rsp.objects.push(Arc::new(Storage { name: "system data", s })),
                    None => {
                        eprintln!("[fs] system data {id:016x} not found");
                        return rc::TARGET_NOT_FOUND;
                    }
                }
            }
            1005 => rsp.push_u32(0),
            1011 => rsp.push_u32(0),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

struct EmptyInfoReader;
impl Service for EmptyInfoReader {
    fn name(&self) -> String {
        "fs::ISaveDataInfoReader".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => rsp.push_u64(0),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

struct Storage {
    name: &'static str,
    s: DynStorage,
}

impl Service for Storage {
    fn name(&self) -> String {
        format!("fs::IStorage({})", self.name)
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => {
                let (off, size) = (req.u64(0), req.u64(8));
                if std::env::var_os("NX_PROFILE").is_some() {
                    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
                    static N: AtomicU64 = AtomicU64::new(0);
                    static BYTES: AtomicU64 = AtomicU64::new(0);
                    static LO: AtomicU64 = AtomicU64::new(u64::MAX);
                    static HI: AtomicU64 = AtomicU64::new(0);
                    static T0: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);
                    N.fetch_add(1, Relaxed);
                    BYTES.fetch_add(size, Relaxed);
                    LO.fetch_min(off, Relaxed);
                    HI.fetch_max(off + size, Relaxed);
                    let mut t = T0.lock().unwrap();
                    let now = std::time::Instant::now();
                    let start = *t.get_or_insert(now);
                    let secs = now.duration_since(start).as_secs_f32();
                    if secs >= 5.0 {
                        eprintln!(
                            "[fs] {} {:.0}s: {} reads, {} MiB, off {:#x}..{:#x}",
                            self.name,
                            secs,
                            N.swap(0, Relaxed),
                            BYTES.swap(0, Relaxed) >> 20,
                            LO.swap(u64::MAX, Relaxed),
                            HI.swap(0, Relaxed)
                        );
                        *t = Some(now);
                    }
                }
                let Some(b) = req.b.first() else { return rc::OUT_OF_RANGE };
                let size = size.min(b.size);
                if off + size > self.s.size() {
                    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                    if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 20 {
                        eprintln!("[fs] {} read out of range {off:#x}+{size:#x} (size {:#x})", self.name, self.s.size());
                    }
                    return rc::OUT_OF_RANGE;
                }
                let dst = unsafe { std::slice::from_raw_parts_mut(b.addr as *mut u8, size as usize) };
                if let Err(e) = crate::kernel::blocking(|| self.s.read_at(off, dst)) {
                    eprintln!("[fs] {} read {off:#x}+{size:#x}: {e}", self.name);
                    return rc::OUT_OF_RANGE;
                }
            }
            4 => rsp.push_u64(self.s.size()),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

fn req_path(req: &Request) -> Option<PathBuf> {
    let b = req.x.first()?;
    let raw = unsafe { std::slice::from_raw_parts(b.addr as *const u8, b.size as usize) };
    let s = String::from_utf8_lossy(&raw[..raw.iter().position(|&c| c == 0).unwrap_or(raw.len())]).into_owned();
    let p = Path::new(s.trim_start_matches('/'));
    p.components().all(|c| matches!(c, Component::Normal(_))).then(|| p.to_path_buf())
}

struct HostFs {
    name: &'static str,
    root: PathBuf,
}

impl HostFs {
    fn open(name: &'static str, root: &Path) -> Arc<dyn Service> {
        let _ = std::fs::create_dir_all(root);
        Arc::new(HostFs { name, root: root.to_path_buf() })
    }
}

fn io_rc(e: &std::io::Error) -> u32 {
    match e.kind() {
        std::io::ErrorKind::NotFound => rc::PATH_NOT_FOUND,
        std::io::ErrorKind::AlreadyExists => rc::PATH_ALREADY_EXISTS,
        _ => rc::INVALID_PATH,
    }
}

impl Service for HostFs {
    fn name(&self) -> String {
        format!("fs::IFileSystem({})", self.name)
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        if matches!(req.cmd, 10 | 11 | 12) {
            match req.cmd {
                10 => {}
                _ => rsp.push_u64(4 << 30),
            }
            return 0;
        }
        let Some(rel) = req_path(req) else { return rc::INVALID_PATH };
        let p = self.root.join(&rel);
        let r: std::io::Result<()> = (|| {
            match req.cmd {
                0 => {
                    if p.exists() {
                        return Err(std::io::ErrorKind::AlreadyExists.into());
                    }
                    File::create(&p)?.set_len(req.u64(8))?
                }
                1 => std::fs::remove_file(&p)?,
                2 => std::fs::create_dir(&p)?,
                3 => std::fs::remove_dir(&p)?,
                4 => std::fs::remove_dir_all(&p)?,
                5 | 6 => {
                    let to = req.x.get(1).and_then(|_| {
                        let r2 = Request { x: req.x[1..].to_vec(), ..Default::default() };
                        req_path(&r2)
                    });
                    let Some(to) = to else { return Err(std::io::ErrorKind::InvalidInput.into()) };
                    std::fs::rename(&p, self.root.join(to))?
                }
                7 => rsp.push_u32(if std::fs::metadata(&p)?.is_dir() { 0 } else { 1 }),
                8 => {
                    let mode = req.u32(0);
                    let f = OpenOptions::new().read(true).write(mode & 2 != 0).append(false).open(&p)?;
                    if f.metadata()?.is_dir() {
                        return Err(std::io::ErrorKind::NotFound.into());
                    }
                    rsp.objects.push(Arc::new(HostFile { f: Mutex::new(f), append: mode & 4 != 0 }));
                }
                9 => {
                    let mode = req.u32(0);
                    let mut entries = Vec::new();
                    for e in std::fs::read_dir(&p)? {
                        let e = e?;
                        let md = e.metadata()?;
                        let is_dir = md.is_dir();
                        if (is_dir && mode & 1 != 0) || (!is_dir && mode & 2 != 0) {
                            entries.push((e.file_name().to_string_lossy().into_owned(), is_dir, md.len()));
                        }
                    }
                    rsp.objects.push(Arc::new(HostDir { entries: Mutex::new(entries.into()) }));
                }
                13 => {
                    for e in std::fs::read_dir(&p)? {
                        let e = e?.path();
                        if e.is_dir() { std::fs::remove_dir_all(e)? } else { std::fs::remove_file(e)? }
                    }
                }
                14 => rsp.data.extend_from_slice(&[0u8; 0x20]),
                _ => {
                    Stub(self.name()).call(req, rsp);
                }
            }
            Ok(())
        })();
        match r {
            Ok(()) => 0,
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("[fs] {} cmd {} {}: {e}", self.name, req.cmd, rel.display());
                } else {
                    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                    if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 30 {
                        eprintln!("[fs] {} cmd {} missing {}", self.name, req.cmd, rel.display());
                    }
                }
                io_rc(&e)
            }
        }
    }
}

struct HostFile {
    f: Mutex<File>,
    append: bool,
}

impl Service for HostFile {
    fn name(&self) -> String {
        "fs::IFile".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        let mut f = self.f.lock().unwrap();
        let r: std::io::Result<()> = (|| {
            match req.cmd {
                0 => {
                    let (off, size) = (req.u64(8), req.u64(16));
                    let Some(b) = req.b.first() else { return Ok(()) };
                    let len = size.min(b.size) as usize;
                    let dst = unsafe { std::slice::from_raw_parts_mut(b.addr as *mut u8, if b.addr == 0 { 0 } else { len }) };
                    let n = crate::kernel::blocking(|| -> std::io::Result<usize> {
                        let mut n = 0;
                        while n < dst.len() {
                            let k = f.read_at(&mut dst[n..], off + n as u64)?;
                            if k == 0 {
                                break;
                            }
                            n += k;
                        }
                        Ok(n)
                    })?;
                    rsp.push_u64(n as u64);
                }
                1 => {
                    let (off, size) = (req.u64(8), req.u64(16));
                    let Some(a) = req.a.first() else { return Ok(()) };
                    let src = unsafe { std::slice::from_raw_parts(a.addr as *const u8, size.min(a.size) as usize) };
                    if self.append {
                        f.seek(SeekFrom::End(0))?;
                        f.write_all(src)?;
                    } else {
                        f.write_all_at(src, off)?;
                    }
                }
                2 => f.flush()?,
                3 => f.set_len(req.u64(0))?,
                4 => rsp.push_u64(f.metadata()?.len()),
                _ => {
                    Stub(self.name()).call(req, rsp);
                }
            }
            Ok(())
        })();
        match r {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("[fs] file cmd {}: {e}", req.cmd);
                io_rc(&e)
            }
        }
    }
}

struct HostDir {
    entries: Mutex<std::collections::VecDeque<(String, bool, u64)>>,
}

impl Service for HostDir {
    fn name(&self) -> String {
        "fs::IDirectory".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        let mut entries = self.entries.lock().unwrap();
        match req.cmd {
            0 => {
                let Some(b) = req.b.first() else { return 0 };
                let max = (b.size / 0x310) as usize;
                let mut out = Vec::new();
                let mut n = 0;
                while n < max {
                    let Some((name, is_dir, size)) = entries.pop_front() else { break };
                    let mut e = vec![0u8; 0x310];
                    let nb = name.as_bytes();
                    e[..nb.len().min(0x300)].copy_from_slice(&nb[..nb.len().min(0x300)]);
                    e[0x304] = if is_dir { 0 } else { 1 };
                    e[0x308..0x310].copy_from_slice(&size.to_le_bytes());
                    out.extend(e);
                    n += 1;
                }
                write_buffer(b, &out);
                rsp.push_u64(n as u64);
            }
            1 => rsp.push_u64(entries.len() as u64),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

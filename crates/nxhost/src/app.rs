// Mush8 - app.rs
// Game startup shared by the macOS/iOS binaries and the Android JNI host.
use crate::{diag, guest, hle, kernel, loader, mem, movie, nvn, remote, services};
use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::sync::OnceLock;

static INITS: OnceLock<Vec<u64>> = OnceLock::new();

extern "C" fn init_done() {}

extern "C" fn init_modules() {
    for &f in INITS.get().unwrap() {
        let f: extern "C" fn() = unsafe { std::mem::transmute(f as usize) };
        f();
    }
}

struct Npdm {
    priority: u8,
    core: u8,
    stack_size: u32,
}

fn read_npdm(path: &PathBuf) -> Result<Npdm> {
    let b = std::fs::read(path).with_context(|| path.display().to_string())?;
    anyhow::ensure!(b.len() >= 0x20 && &b[..4] == b"META", "not an NPDM");
    Ok(Npdm { priority: b[0xE], core: b[0xF], stack_size: u32::from_le_bytes(b[0x1C..0x20].try_into()?) })
}

fn program(n: &nxfs::Nsp) -> Result<&nxfs::nca::Nca> {
    n.find(nxfs::nca::ContentType::Program).context("no Program NCA")
}

fn load_dlc(args: &Args, keys: &nxfs::keys::Keys) -> Vec<(u64, nxfs::storage::DynStorage)> {
    let default = PathBuf::from("dlc.nsp");
    let paths = if args.dlc.is_empty() && default.exists() { vec![default] } else { args.dlc.clone() };
    let mut out = Vec::new();
    for p in paths {
        let found = (|| -> Result<Vec<(u64, nxfs::storage::DynStorage)>> {
            let nsp = nxfs::Nsp::open(&p, keys)?;
            let mut v = Vec::new();
            for nca in nsp.ncas.iter().filter(|n| n.content_type == nxfs::nca::ContentType::PublicData) {
                let sec = nca.sections.iter().find(|s| s.fs_type == nxfs::nca::FsType::RomFs).context("no RomFS")?;
                v.push((nca.program_id, nca.fs_storage(sec)?));
            }
            Ok(v)
        })();
        match found {
            Ok(v) => {
                for (id, _) in &v {
                    eprintln!("[host] add-on content {id:016x} ({})", p.display());
                }
                out.extend(v);
            }
            Err(e) => eprintln!("[host] {}: {e:#}", p.display()),
        }
    }
    out
}

#[derive(Parser, Clone)]
#[command(about = "Run rewritten Switch modules natively")]
pub struct Args {
    #[arg(long, default_value = "work/v-upd/patched")]
    pub patched: PathBuf,
    #[arg(long, default_value = "work/v-upd/exefs/main.npdm")]
    pub npdm: PathBuf,
    #[arg(long, default_value = "prod.keys")]
    pub keys: PathBuf,
    #[arg(long, default_value = "base.nsp")]
    pub base: PathBuf,
    #[arg(long, default_value = "upd4-0-1.nsp")]
    pub update: PathBuf,
    #[arg(long, default_value = "work/save")]
    pub save: PathBuf,
    #[arg(long, default_value = "work/sdmc")]
    pub sdmc: PathBuf,
    #[arg(long, default_value = "system")]
    pub system: PathBuf,
    #[arg(long, default_value = "Firmware")]
    pub firmware: PathBuf,
    #[arg(long)]
    pub dlc: Vec<PathBuf>,
    #[arg(long)]
    pub precompile: bool,
    #[arg(long)]
    pub movies: bool,
}

#[derive(Clone, Default)]
pub struct GameConfig {
    pub patched: PathBuf,
    pub npdm: PathBuf,
    pub keys: PathBuf,
    pub base: PathBuf,
    pub update: PathBuf,
    pub save: PathBuf,
    pub sdmc: PathBuf,
    pub system: PathBuf,
    pub firmware: PathBuf,
    pub dlc: Vec<PathBuf>,
}

impl Args {
    pub fn from_config(c: GameConfig) -> Self {
        Self {
            patched: c.patched,
            npdm: c.npdm,
            keys: c.keys,
            base: c.base,
            update: c.update,
            save: c.save,
            sdmc: c.sdmc,
            system: c.system,
            firmware: c.firmware,
            dlc: c.dlc,
            precompile: false,
            movies: false,
        }
    }
}

pub fn run_game(config: GameConfig) -> Result<()> {
    start(Args::from_config(config))
}

pub fn start(args: Args) -> Result<()> {
    #[cfg(target_os = "android")]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }
    let dir = args.patched.clone();
    let npdm = read_npdm(&args.npdm)?;

    let keys = nxfs::keys::Keys::load(&args.keys)?;
    let base = nxfs::Nsp::open(&args.base, &keys)?;
    let update = nxfs::Nsp::open(&args.update, &keys)?;
    let romfs = program(&update)?.patched_romfs(program(&base)?)?;
    eprintln!("[host] RomFS: {:.2} GB", nxfs::storage::Storage::size(&*romfs) as f64 / 1e9);
    if args.movies {
        movie::webm::convert_all(romfs)?;
        std::process::exit(0);
    }
    if args.precompile {
        let modules: Vec<PathBuf> = ["main", "subsdk0", "sdk"].iter().map(|m| dir.join(format!("{m}.bin"))).collect();
        nvn::precompile(romfs, &modules)?;
        std::process::exit(0);
    }
    let _ = services::fs::CONFIG.set(services::fs::Config {
        romfs,
        save_dir: args.save.clone(),
        sd_dir: args.sdmc.clone(),
        system_dir: args.system.clone(),
        firmware_dir: args.firmware.clone(),
        dlc: load_dlc(&args, &keys),
        keys,
    });

    if remote::enabled() {
        remote::host();
    }
    diag::install_handlers();
    guest::init_tsd();
    let _ = kernel::KERNEL.aspace.set(mem::AddressSpace::reserve()?);
    let a = kernel::aspace();
    eprintln!("[host] guest address space at {:#x}..{:#x}", a.base, a.base + a.layout.total());

    let mods = loader::load(&dir, &["main", "subsdk0", "sdk"], &hle::table())?;
    let _ = diag::MODULES.set(
        mods.iter()
            .map(|m| {
                let mut symbols: Vec<(u64, String)> = m.dynamic.symbols.iter()
                    .filter(|s| s.defined() && !s.name.is_empty())
                    .map(|s| (s.value, nxfs::nso::demangle(&s.name)))
                    .collect();
                symbols.sort();
                diag::ModInfo { name: m.name.clone(), base: m.base, size: m.size, symbols }
            })
            .collect(),
    );
    diag::start_watch(&mods.iter().map(|m| (m.name.clone(), m.base)).collect::<Vec<_>>());
    let _ = INITS.set(mods.iter().filter_map(|m| m.dynamic.init.map(|i| m.base + i)).collect());
    let start = loader::symbol(&mods, "_ZN2nn4init5StartEmmPFvvES2_").context("nn::init::Start not found")?;
    let sdk = mods.iter().find(|m| m.name == "sdk").context("no sdk module")?;
    nvn::init(sdk.base, &sdk.module_id)?;
    if std::env::var_os("NX_PREWARM").is_some() {
        nvn::prewarm();
    }
    movie::init(&mods);
    crate::diag::start_site_watch();
    if let Some(v) = loader::symbol(&mods, "_ZNK2nn3mii6detail11CharInfoRaw6VerifyEv") {
        let _ = services::mii::VERIFY.set(v);
    }

    let stack_size = (npdm.stack_size as u64).next_multiple_of(mem::PAGE);
    a.set(a.stack(), stack_size, mem::Block { state: mem::state::STACK, perm: mem::perm::RW, attr: 0 })?;
    let h = kernel::create_thread(start, 0, a.stack() + stack_size, npdm.priority as i32, npdm.core as i32)?;
    let t = kernel::thread(h).unwrap();
    eprintln!("[host] main thread {h:#x}: prio {} core {} stack {stack_size:#x}; entering nn::init::Start", npdm.priority, npdm.core);
    let main_base = mods[0].base;
    let j = kernel::start_thread(t, [h as u64, main_base, init_done as *const () as usize as u64, init_modules as *const () as usize as u64])?;
    std::thread::spawn(move || {
        j.join().ok();
        eprintln!("[host] main thread returned");
        std::process::exit(0);
    });
    Ok(())
}

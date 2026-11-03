// Mush8 - main.rs
// Constantin Clerc (c22dev) - 0.1
use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use nxfs::nca::{ContentType, FsType, Nca};
use nxfs::pfs0::Pfs0;
use nxfs::romfs::RomFs;
use nxfs::storage::Storage;
use nxfs::{Nsp, keys::Keys, nso};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(about = "Switch dump tooling: NSP/NCA decryption, update patching, ExeFS/RomFS/NSO extraction")]
struct Cli {
    #[arg(long, global = true, default_value = "prod.keys")]
    keys: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Info { nsp: Vec<PathBuf> },
    Extract {
        #[arg(long)]
        base: PathBuf,
        #[arg(long)]
        update: Option<PathBuf>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        romfs: bool,
    },
    NcaRomfs {
        nca: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Magic { files: Vec<PathBuf> },
    Sarc { files: Vec<PathBuf> },
    Nso {
        file: PathBuf,
        #[arg(long)]
        imports: bool,
        #[arg(long)]
        mangled: bool,
        #[arg(long)]
        relocs: bool,
        #[arg(long)]
        image: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Info { nsp } => {
            let keys = Keys::load(&cli.keys)?;
            for p in nsp {
                if p.extension().is_some_and(|e| e == "nca") {
                    match nxfs::open_nca(&p, &keys) {
                        Ok(n) => println!("{:016x} {:?} {} keygen={}", n.program_id, n.content_type, n.name, n.key_generation),
                        Err(e) => println!("{}: {e}", p.display()),
                    }
                    continue;
                }
                println!("{}", p.display());
                for n in Nsp::open(&p, &keys)?.ncas {
                    println!("  {} {:?} program={:016x} keygen={}", n.name, n.content_type, n.program_id, n.key_generation);
                    for s in &n.sections {
                        println!("    [{}] {:?} {:?} offset={:#x} size={:#x}", s.index, s.fs_type, s.encryption, s.offset, s.size);
                    }
                }
            }
        }
        Cmd::Extract { base, update, out, romfs } => {
            let keys = Keys::load(&cli.keys)?;
            let base = Nsp::open(&base, &keys)?;
            let update = update.map(|u| Nsp::open(&u, &keys)).transpose()?;
            let latest = update.as_ref().unwrap_or(&base);

            let control = latest.find(ContentType::Control).context("no Control NCA")?;
            let cfs = RomFs::open(control.fs_storage(romfs_section(control)?)?)?;
            for f in cfs.files() {
                write_file(cfs.storage().as_ref(), f.offset, f.size, &out.join("control").join(&f.path[1..]))?;
            }
            if let Ok(nacp) = std::fs::read(out.join("control/control.nacp")) {
                let v = &nacp[0x3060..0x3070];
                println!("display version: {}", String::from_utf8_lossy(&v[..v.iter().position(|&c| c == 0).unwrap_or(16)]));
            }

            let prog = program(latest)?;
            let exefs = prog.sections.iter().find(|s| s.fs_type == FsType::PartitionFs).context("no ExeFS")?;
            let pfs = Pfs0::open(prog.fs_storage(exefs)?)?;
            for e in &pfs.entries {
                write_file(pfs.storage_of(e).as_ref(), 0, e.size, &out.join("exefs").join(&e.name))?;
                println!("exefs/{} ({} bytes)", e.name, e.size);
            }

            if romfs {
                let storage = match &update {
                    Some(u) => program(u)?.patched_romfs(program(&base)?)?,
                    None => prog.fs_storage(romfs_section(prog)?)?,
                };
                let fs = RomFs::open(storage)?;
                let files = fs.files();
                let total: u64 = files.iter().map(|f| f.size).sum();
                println!("romfs: {} files, {:.2} GB", files.len(), total as f64 / 1e9);
                for (i, f) in files.iter().enumerate() {
                    write_file(fs.storage().as_ref(), f.offset, f.size, &out.join("romfs").join(&f.path[1..]))?;
                    if i % 500 == 0 {
                        eprintln!("  [{i}/{}] {}", files.len(), f.path);
                    }
                }
            }
        }
        Cmd::NcaRomfs { nca, out } => {
            let keys = Keys::load(&cli.keys)?;
            let n = nxfs::open_nca(&nca, &keys)?;
            let fs = RomFs::open(n.fs_storage(romfs_section(&n)?)?)?;
            for f in fs.files() {
                println!("{:10} {}", f.size, f.path);
                if let Some(o) = &out {
                    write_file(fs.storage().as_ref(), f.offset, f.size, &o.join(&f.path[1..]))?;
                }
            }
        }
        Cmd::Magic { files } => {
            for p in files {
                let raw = nxfs::sarc::yaz0(&std::fs::read(&p)?)?;
                let magic = String::from_utf8_lossy(&raw[..raw.len().min(4)]).replace(|c: char| !c.is_ascii_graphic(), ".");
                let count = |m: &[u8]| raw.windows(4).filter(|w| *w == m).count();
                println!("{}\t{magic}\t{}\t{}\t{}", p.display(), raw.len(), count(b"BNSH"), count(b"FSHA"));
            }
        }
        Cmd::Sarc { files } => {
            for p in files {
                let raw = nxfs::sarc::yaz0(&std::fs::read(&p)?)?;
                match nxfs::sarc::sarc(&raw) {
                    Ok(entries) => {
                        for e in entries {
                            let magic = String::from_utf8_lossy(&e.data[..e.data.len().min(4)]).replace(|c: char| !c.is_ascii_graphic(), ".");
                            println!("{}\t{}\t{}\t{}", p.display(), e.name, e.data.len(), magic);
                        }
                    }
                    Err(err) => eprintln!("{}: {err}", p.display()),
                }
            }
        }
        Cmd::Nso { file, imports, image, relocs, mangled } => {
            let n = nso::Nso::parse(&std::fs::read(&file)?)?;
            if let Some(p) = image {
                std::fs::write(p, &n.image)?;
            }
            println!("module id {}", hex::encode(&n.module_id[..20]));
            for (name, s) in [("text", n.text), ("ro", n.ro), ("data", n.data)] {
                println!("{name:5} {:#010x} size {:#x}", s.mem_offset, s.size);
            }
            println!("bss   size {:#x}", n.bss_size);
            let d = nxfs::module::parse(&n.image)?;
            let mut kinds = std::collections::BTreeMap::new();
            let mut self_slots = 0;
            for r in &d.relocs {
                *kinds.entry(r.kind).or_insert(0) += 1;
                if r.kind == nxfs::module::R_AARCH64_JUMP_SLOT && d.symbols.get(r.sym as usize).is_some_and(|s| s.defined()) {
                    self_slots += 1;
                }
            }
            println!("MOD0 {:#x} bss {:#x}..{:#x} init {:?} init_array {:x?} fini_array {:x?}", d.mod0, d.bss.0, d.bss.1, d.init, d.init_array, d.fini_array);
            println!("relocs by type {kinds:?}; jump slots bound to own definitions: {self_slots}");
            if relocs {
                for r in &d.relocs {
                    let name = d.symbols.get(r.sym as usize).map_or("", |s| s.name.as_str());
                    println!("R {:#010x} {:4} {:#x} {}", r.offset, r.kind, r.addend, nso::demangle(name));
                }
            }
            for s in n.symbols().iter().filter(|s| !imports || !s.defined) {
                let name = if mangled { s.name.clone() } else { nso::demangle(&s.name) };
                println!("{} {:#010x} {}", if s.defined { "D" } else { "U" }, s.value, name);
            }
        }
    }
    Ok(())
}

fn program(n: &Nsp) -> Result<&Nca> {
    n.find(ContentType::Program).context("no Program NCA")
}

fn romfs_section(n: &Nca) -> Result<&nxfs::nca::Section> {
    n.sections.iter().find(|s| s.fs_type == FsType::RomFs).context("no RomFS section")
}

fn write_file(src: &dyn Storage, offset: u64, size: u64, dest: &Path) -> Result<()> {
    if let Some(dir) = dest.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut f = std::io::BufWriter::new(std::fs::File::create(dest)?);
    let mut buf = vec![0; 8 << 20];
    let mut done = 0;
    while done < size {
        let n = (size - done).min(buf.len() as u64) as usize;
        src.read_at(offset + done, &mut buf[..n]).with_context(|| format!("reading {}", dest.display()))?;
        f.write_all(&buf[..n])?;
        done += n as u64;
    }
    if done != size {
        bail!("short write");
    }
    Ok(())
}

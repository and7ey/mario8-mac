// Mush8 Android - diag_android.rs
use std::sync::OnceLock;

pub struct ModInfo {
    pub name: String,
    pub base: u64,
    pub size: u64,
    pub symbols: Vec<(u64, String)>,
}

pub static MODULES: OnceLock<Vec<ModInfo>> = OnceLock::new();

pub fn symbolize(addr: u64) -> String {
    if let Some(ms) = MODULES.get() {
        for m in ms {
            if (m.base..m.base + m.size).contains(&addr) {
                let off = addr - m.base;
                let i = m.symbols.partition_point(|(o, _)| *o <= off);
                let sym = if i > 0 {
                    format!(" {}+{:#x}", m.symbols[i - 1].1, off - m.symbols[i - 1].0)
                } else {
                    String::new()
                };
                return format!("{}+{off:#x}{sym}", m.name);
            }
        }
    }
    format!("{addr:#x}")
}

pub fn backtrace(_fp: u64) {}

pub fn backtrace_lines(_fp: u64) -> Vec<String> {
    Vec::new()
}

pub fn report_and_exit(msg: &str) -> ! {
    eprintln!("[fatal] {msg}");
    std::process::exit(1)
}

pub fn install_altstack() {}

pub fn install_handlers() {}

pub fn start_watch(_mods: &[(String, u64)]) {}

pub fn start_site_watch() {
    if std::env::var_os("NX_PROFILE").is_none() {
        return;
    }
    std::thread::Builder::new()
        .name("site-watch".into())
        .spawn(|| {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(5));
                {
                    static WARN: std::sync::Once = std::sync::Once::new();
                    let broken = crate::guest::gate_canary_ok();
                    if !broken.is_empty() {
                        WARN.call_once(|| eprintln!("[gate] stack canary smashed on threads {broken:x?}"));
                    }
                }
                let sites = crate::guest::live_sites();
                let mut line = String::new();
                for (h, lr, what, bt) in sites.iter().take(48) {
                    let (kind, code) = (what >> 32, what & 0xffff_ffff);
                    let name = if kind == 0 { crate::stubs::name_of(code as usize) } else { format!("svc{code:#04x}") };
                    let chain: Vec<String> = bt.iter().filter(|a| **a != 0).map(|a| symbolize(*a)).collect();
                    line += &format!(" [{h:#x} {} {name} bt:{}]", symbolize(*lr), chain.join(" < "));
                }
                eprintln!("[site] t={} {}", crate::svc::system_tick(), line);
            }
        })
        .ok();
}

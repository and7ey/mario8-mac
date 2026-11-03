// Mush8 - main.rs
// Constantin Clerc (c22dev) - 0.1
#[cfg(any(target_os = "macos", target_os = "ios"))]
use clap::Parser;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use nxhost::app::Args;

#[cfg(target_os = "macos")]
fn main() -> anyhow::Result<()> {
    nxhost::app::start(Args::parse())?;
    nxhost::gfx::run_app("Mush8", 1280.0, 720.0)
}

#[cfg(target_os = "ios")]
fn main() {
    use std::path::PathBuf;
    let home = PathBuf::from(std::env::var_os("HOME").expect("HOME"));
    let docs = home.join("Documents");
    let _ = std::env::set_current_dir(&docs);
    let _ = std::fs::create_dir_all("work");
    let _ = std::fs::rename("work/nxhost.log", "work/nxhost.prev.log");
    if let Ok(log) = std::fs::File::create("work/nxhost.log") {
        use std::os::fd::AsRawFd;
        unsafe { libc::dup2(log.as_raw_fd(), 2) };
    }
    let bundle = std::env::current_exe().ok().and_then(|e| e.parent().map(PathBuf::from)).unwrap_or_default();
    let mut env: Vec<(String, String)> = vec![
        ("NX_SHADER_CACHE".into(), bundle.join("shaders").display().to_string()),
        ("NX_PIPELINE_LIST".into(), bundle.join("shaders/pipelines.tsv").display().to_string()),
    ];
    for line in std::fs::read_to_string("env.txt").unwrap_or_default().lines() {
        if let Some((k, v)) = line.trim().split_once('=').filter(|(k, _)| !k.is_empty() && !k.starts_with('#')) {
            env.push((k.trim().into(), v.trim().into()));
        }
    }
    for (k, v) in env {
        unsafe { std::env::set_var(k, v) };
    }
    eprintln!("[host] iOS: files in {}, bundle {}", docs.display(), bundle.display());
    let args = Args::parse_from(["nxhost", "--patched", &bundle.join("modules").display().to_string(), "--npdm", &bundle.join("modules/main.npdm").display().to_string()]);
    nxhost::gfx::run_app(move || {
        apply_launcher_settings();
        let missing: Vec<&str> = ["prod.keys", "base.nsp", "upd4-0-1.nsp"].into_iter().filter(|f| !std::path::Path::new(f).exists()).collect();
        anyhow::ensure!(
            missing.is_empty(),
            "Missing {}.\n\nCopy prod.keys, base.nsp, upd4-0-1.nsp (and optionally dlc.nsp and the Firmware folder) into this app's folder: Finder → your iPhone or iPad → Files → Mush8, or the Files app → On My iPhone → Mush8.",
            missing.join(", ")
        );
        nxhost::app::start(args)
    })
}

#[cfg(target_os = "ios")]
fn apply_launcher_settings() {
    let config = std::fs::read_to_string("config.json").unwrap_or_default();
    let field = |key: &str| {
        let rest = config.split(&format!("\"{key}\"")).nth(1)?;
        let rest = rest.trim_start().strip_prefix(':')?.trim_start();
        if let Some(rest) = rest.strip_prefix('"') {
            let mut out = String::new();
            let mut chars = rest.chars();
            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(n) = chars.next() {
                        out.push(n);
                    }
                } else if c == '"' {
                    break;
                } else {
                    out.push(c);
                }
            }
            Some(out)
        } else {
            Some(rest.split([',', '\n', '}']).next()?.trim().to_string())
        }
    };
    let network = field("network").unwrap_or_else(|| if field("lan").as_deref() == Some("true") { "lan".into() } else { "off".into() });
    unsafe { std::env::set_var("NX_NET", &network) };
    if network == "lan" {
        unsafe { std::env::set_var("NX_LAN", "1") };
    }
    if let Some(server) = field("lanplay_server").filter(|s| !s.is_empty()) {
        unsafe { std::env::set_var("NX_LANPLAY_SERVER", server) };
    }
    if field("prewarm_pipelines").as_deref() != Some("false") {
        unsafe { std::env::set_var("NX_PREWARM", "1") };
    }
    if field("background_compile").as_deref() == Some("false") {
        unsafe { std::env::set_var("NX_SYNC_SHADERS", "1") };
    }
    if field("rumble").as_deref() == Some("false") {
        unsafe { std::env::set_var("NX_RUMBLE", "0") };
    }
    if field("motion_controls").as_deref() == Some("true") {
        unsafe { std::env::set_var("NX_MOTION", "1") };
    }
    if field("handheld").as_deref() != Some("false") {
        unsafe { std::env::set_var("NX_HANDHELD", "1") };
    }
    if field("phone_pads").as_deref() == Some("true") {
        unsafe { std::env::set_var("NX_PHONE_PADS", "1") };
    }
}

#[cfg(target_os = "android")]
fn main() {}

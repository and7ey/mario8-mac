// Mush8 Android - android.rs
// JNI entry points for libnxhost.so.
use jni::JNIEnv;
use jni::JavaVM;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{JNI_VERSION_1_6, jint};
use log::{error, info};
use std::ffi::c_void;
use std::sync::Once;

#[unsafe(no_mangle)]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM, _reserved: *mut c_void) -> jint {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag("Mush8"),
    );
    apply_conf();
    redirect_stdio();
    info!("libnxhost loaded");
    JNI_VERSION_1_6
}

/// `setprop wrap.<pkg>` is refused on retail devices (ro.debuggable=0), so the
/// NX_* switches are read from a KEY=VALUE file in the game root instead.
fn apply_conf() {
    for path in ["/sdcard/Mush8/mush8.conf", "/storage/emulated/0/Android/data/dev.and7ey.mush8/files/Mush8/mush8.conf"] {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((k, v)) = line.split_once('=') else { continue };
            let (k, v) = (k.trim(), v.trim());
            if k.is_empty() || v.is_empty() {
                continue;
            }
            unsafe { std::env::set_var(k, v) };
            info!("[conf] {k}={v}");
        }
        break;
    }
}

fn redirect_stdio() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        let mut fds = [0i32; 2];
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return;
        }
        libc::dup2(fds[1], 1);
        libc::dup2(fds[1], 2);
        libc::close(fds[1]);
        std::thread::Builder::new().name("stdio".into()).spawn(move || {
            let mut buffer = [0u8; 4096];
            let mut line = Vec::new();
            loop {
                let n = libc::read(fds[0], buffer.as_mut_ptr() as *mut _, buffer.len());
                if n <= 0 {
                    break;
                }
                line.extend_from_slice(&buffer[..n as usize]);
                while let Some(pos) = line.iter().position(|&b| b == b'\n') {
                    let text = String::from_utf8_lossy(&line[..pos]).into_owned();
                    line.drain(..=pos);
                    info!("{text}");
                }
            }
        }).ok();
    });
}

static GAME_STARTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static GAME_ROOT: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

pub fn cache_dir() -> Option<std::path::PathBuf> {
    GAME_ROOT.get().cloned()
}

fn start_game(root: String) {
    if GAME_STARTED.swap(true, std::sync::atomic::Ordering::Relaxed) {
        info!("game already started");
        return;
    }
    let base = std::path::PathBuf::from(&root);
    let _ = GAME_ROOT.set(base.clone());
    let config = crate::app::GameConfig {
        patched: base.join("patched"),
        npdm: base.join("exefs/main.npdm"),
        keys: base.join("prod.keys"),
        base: base.join("base.nsp"),
        update: base.join("update.nsp"),
        save: base.join("save"),
        sdmc: base.join("sdmc"),
        system: base.join("system"),
        firmware: base.join("Firmware"),
        dlc: Vec::new(),
    };
    info!("starting nxhost from {root}");
    let _ = std::thread::Builder::new().name("nxhost".into()).spawn(move || match crate::app::run_game(config) {
        Ok(()) => info!("nxhost returned"),
        Err(e) => error!("nxhost failed: {e:#}"),
    });
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_and7ey_mush8_Native_runGame(mut env: JNIEnv, _class: JClass, root: JString) {
    let root: String = match env.get_string(&root) {
        Ok(s) => s.into(),
        Err(e) => {
            error!("runGame: bad path: {e}");
            return;
        }
    };
    start_game(root);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_and7ey_mush8_Native_attachSurface(env: JNIEnv, _class: JClass, surface: JObject) {
    let window = unsafe { ndk_sys::ANativeWindow_fromSurface(env.get_raw(), surface.as_raw()) };
    if window.is_null() {
        error!("attachSurface: null ANativeWindow");
        return;
    }
    crate::gfx_vk::attach(window as *mut c_void, 1280, 720);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_and7ey_mush8_Native_surfaceChanged(_env: JNIEnv, _class: JClass, width: jint, height: jint) {
    if width > 0 && height > 0 {
        crate::gfx_vk::resize(width as u32, height as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_and7ey_mush8_Native_surfaceDestroyed(_env: JNIEnv, _class: JClass) {
    crate::gfx_vk::detach();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_and7ey_mush8_Native_setButtons(_env: JNIEnv, _class: JClass, mask: jni::sys::jlong) {
    crate::input::TOUCH_BUTTONS.store(mask as u64, std::sync::atomic::Ordering::Relaxed);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_and7ey_mush8_Native_setStick(_env: JNIEnv, _class: JClass, x: jni::sys::jfloat, y: jni::sys::jfloat) {
    crate::input::set_touch_stick(x, y);
}

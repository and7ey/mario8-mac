// Mush8 Android - input_android.rs
// Scripted pad input is available now; Android InputDevice/touch mapping comes later.
use crate::services::hid::PadState;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub const A: u64 = 1 << 0;
pub const B: u64 = 1 << 1;
pub const X: u64 = 1 << 2;
pub const Y: u64 = 1 << 3;
pub const STICK_L: u64 = 1 << 4;
pub const STICK_R: u64 = 1 << 5;
pub const L: u64 = 1 << 6;
pub const R: u64 = 1 << 7;
pub const ZL: u64 = 1 << 8;
pub const ZR: u64 = 1 << 9;
pub const PLUS: u64 = 1 << 10;
pub const MINUS: u64 = 1 << 11;
pub const LEFT: u64 = 1 << 12;
pub const UP: u64 = 1 << 13;
pub const RIGHT: u64 = 1 << 14;
pub const DOWN: u64 = 1 << 15;

pub const MAX_PLAYERS: usize = 8;

#[derive(Clone, Copy)]
pub struct Motion {
    pub accel: [f32; 3],
    pub gyro: [f32; 3],
}

pub static MOTION: Mutex<[Option<Motion>; MAX_PLAYERS]> = Mutex::new([None; MAX_PLAYERS]);
pub static TOUCH_BUTTONS: AtomicU64 = AtomicU64::new(0);
pub static TOUCH_STICK: AtomicU64 = AtomicU64::new(0);
pub static SCREEN_ACTIVE: AtomicBool = AtomicBool::new(false);
pub static SLOT_NAMES: Mutex<[Option<String>; MAX_PLAYERS]> = Mutex::new([const { None }; MAX_PLAYERS]);

pub const TOUCH_START: u32 = 1;
pub const TOUCH_END: u32 = 2;

pub fn set_touch_stick(x: f32, y: f32) {
    TOUCH_STICK.store((x.to_bits() as u64) << 32 | y.to_bits() as u64, Ordering::Relaxed);
}

pub fn screen_touch(_key: usize, _pos: Option<(f32, f32)>) {}

pub fn screen_sample() -> Vec<(u32, f32, f32, u32)> {
    Vec::new()
}

fn scripted(player: usize) -> u64 {
    static SCRIPT: std::sync::LazyLock<Vec<(f32, u64, usize)>> = std::sync::LazyLock::new(|| {
        let names = [
            ("A", A), ("B", B), ("X", X), ("Y", Y), ("L", L), ("R", R), ("ZL", ZL), ("ZR", ZR), ("PLUS", PLUS),
            ("MINUS", MINUS), ("LSTICK", STICK_L), ("RSTICK", STICK_R), ("LEFT", LEFT), ("UP", UP), ("RIGHT", RIGHT), ("DOWN", DOWN),
        ];
        std::env::var("NX_AUTOINPUT").unwrap_or_default().split(',').filter_map(|e| {
            let (t, b) = e.trim().split_once(':')?;
            let (b, player) = b.split_once('@').map_or(Some((b, 1)), |(b, p)| Some((b, p.parse().ok()?)))?;
            Some((t.parse().ok()?, names.iter().find(|(n, _)| n.eq_ignore_ascii_case(b))?.1, player))
        }).collect()
    });
    static START: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(std::time::Instant::now);
    let t = START.elapsed().as_secs_f32();
    SCRIPT.iter().filter(|(at, _, p)| *p == player + 1 && t >= *at && t < at + 0.15).fold(0, |m, (_, b, _)| m | b)
}

pub struct Pads {
    started: bool,
}

impl Pads {
    pub fn new() -> Self {
        Self { started: false }
    }

    pub fn poll(&mut self) -> [Option<PadState>; MAX_PLAYERS] {
        if !self.started {
            self.started = true;
            SLOT_NAMES.lock().unwrap()[0] = Some("Android input".into());
        }
        let mut out: [Option<PadState>; MAX_PLAYERS] = Default::default();
        let buttons = scripted(0) | TOUCH_BUTTONS.load(Ordering::Relaxed);
        static LAST: AtomicU64 = AtomicU64::new(0);
        let last = LAST.swap(buttons, Ordering::Relaxed);
        if buttons != last {
            log::info!("[pad] buttons {last:#x} -> {buttons:#x}");
        }
        let s = TOUCH_STICK.load(Ordering::Relaxed);
        let (x, y) = (f32::from_bits((s >> 32) as u32), f32::from_bits(s as u32));
        let left = ((x.clamp(-1.0, 1.0) * 32767.0) as i32, (y.clamp(-1.0, 1.0) * 32767.0) as i32);
        out[0] = Some(PadState { buttons, left, right: (0, 0) });
        out
    }
}

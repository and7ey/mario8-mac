// Mush8 Android - remote_android.rs
use std::net::SocketAddr;

#[derive(Clone, Copy)]
pub struct Pad {
    pub buttons: u64,
    pub sticks: [f32; 4],
    pub motion: Option<crate::input::Motion>,
}

pub fn enabled() -> bool {
    false
}

pub fn host() {}

pub fn connected() -> Vec<(SocketAddr, Pad)> {
    Vec::new()
}

pub fn set_player(_addr: SocketAddr, _player: u8) {}

pub fn run_controller(_status: fn(&str)) {}

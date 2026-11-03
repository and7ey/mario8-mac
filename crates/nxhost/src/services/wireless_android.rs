// Mush8 Android - wireless_android.rs
use crate::services::vnet::Transport;

pub fn start(_address: [u8; 4]) -> Option<Box<dyn Transport>> {
    None
}

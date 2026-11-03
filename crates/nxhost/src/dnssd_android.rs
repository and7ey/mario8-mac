// Mush8 Android - dnssd_android.rs
pub const SHARE: u32 = 0x4000;
pub const ADD: u32 = 0x2;
pub const POLICY_DENIED: i32 = -65570;

pub fn denied(_code: i32) -> &'static str {
    "local discovery is unavailable on Android in this build"
}

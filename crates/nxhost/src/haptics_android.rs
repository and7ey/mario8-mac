// Mush8 Android - haptics_android.rs
#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub amp_low: f32,
    pub freq_low: f32,
    pub amp_high: f32,
    pub freq_high: f32,
}

impl Value {
    pub const IDLE: Value = Value { amp_low: 0.0, freq_low: 160.0, amp_high: 0.0, freq_high: 320.0 };
}

pub fn send(_player: usize, _side: usize, _value: Value) {}

pub fn last(_player: usize, _side: usize) -> Value {
    Value::IDLE
}

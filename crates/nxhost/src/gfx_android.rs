// Mush8 Android - gfx_android.rs
// Headless graphics placeholders until the Vulkan NVN backend is wired in.
pub mod overlay {
    pub fn controller_screen(_min: usize, _max: usize) -> Option<usize> {
        Some(1)
    }
}

static HEAP: std::sync::Mutex<Option<(u64, u64)>> = std::sync::Mutex::new(None);

pub fn wrap_heap(base: u64, size: u64) -> bool {
    log::info!("[gpu] guest heap {base:#x}+{size:#x}");
    *HEAP.lock().unwrap() = Some((base, size));
    true
}

pub fn heap() -> Option<(u64, u64)> {
    *HEAP.lock().unwrap()
}

pub fn set_crop(_x: i32, _y: i32, _w: i32, _h: i32) {}

pub fn set_present_interval(_n: u32) {}

pub fn open_url(_url: &'static str) {}

pub fn wait_foreground() {}

pub fn set_touch_controls(_visible: bool) {}

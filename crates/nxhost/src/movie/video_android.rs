// Mush8 Android - movie/video_android.rs
//
// Android has no NVDEC behind nvdrv, so the guest's own decoder would stall
// waiting for decoded frames (the game stops taking input while a movie plays,
// e.g. MovieMenu.mp4 behind the main menu). Replace the platform decoder with a
// black-frame source: every sample the guest feeds in produces one NV12 frame
// of the right size, so playback runs to its end and the game continues.
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub enum Source {
    File(PathBuf),
    Webm(Arc<super::webm::Movie>),
}

pub fn seek_all(_extractor: u64, _us: i64) {}

pub struct FrameSource {
    stride: usize,
    rows: usize,
}

impl FrameSource {
    pub fn open(_source: Source, _extractor: u64, stride: usize, rows: usize) -> Option<Self> {
        Some(Self { stride, rows })
    }

    pub fn next_frame(&mut self, out: &mut Vec<u8>) -> bool {
        // Limited-range black: Y=16, chroma 128. All-zero NV12 (chroma 0) is
        // saturated green, which is what a "blank" movie looked like on screen.
        let y = self.stride * self.rows;
        out.clear();
        out.resize(y * 3 / 2, 128);
        out[..y].fill(16);
        true
    }
}

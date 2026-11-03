// Mush8 Android - output_android.rs
// Clock-driven silent output: keeps the game's audio renderer advancing.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

pub type Source = Box<dyn FnMut(&mut [i16]) + Send>;

pub struct Output {
    stop: Arc<AtomicBool>,
    join: Option<JoinHandle<()>>,
}

impl Drop for Output {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

pub fn start(rate: u32, frames: usize, mut source: Source) -> Output {
    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    let period = Duration::from_secs_f64(frames as f64 / rate.max(1) as f64);
    let join = std::thread::Builder::new().name("audio-out".into()).spawn(move || {
        let mut buffer = vec![0i16; frames * 2];
        let mut next = Instant::now();
        while !flag.load(Ordering::Relaxed) {
            source(&mut buffer);
            next += period;
            std::thread::sleep(next.saturating_duration_since(Instant::now()));
        }
    }).ok();
    Output { stop, join }
}

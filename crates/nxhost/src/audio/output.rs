// Mush8 - output.rs
// Constantin Clerc (c22dev) - 0.1
use objc2_audio_toolbox::{AudioQueueAllocateBuffer, AudioQueueBufferRef, AudioQueueDispose, AudioQueueEnqueueBuffer, AudioQueueNewOutput, AudioQueueRef, AudioQueueStart};
use objc2_core_audio_types::{AudioStreamBasicDescription, kAudioFormatFlagIsPacked, kAudioFormatFlagIsSignedInteger, kAudioFormatLinearPCM};
use std::ffi::c_void;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::time::{Duration, Instant};

pub type Source = Box<dyn FnMut(&mut [i16]) + Send>;

const QUEUED: usize = 3;

struct Pull {
    source: Source,
    dump: Option<Wav>,
}

impl Pull {
    fn fill(&mut self, buf: &mut [i16]) {
        (self.source)(buf);
        if let Some(w) = &mut self.dump {
            w.write(buf);
        }
    }
}

pub struct Output {
    queue: AudioQueueRef,
    pull: *mut Pull,
    stop: Arc<AtomicBool>,
}

unsafe impl Send for Output {}
unsafe impl Sync for Output {}

unsafe extern "C-unwind" fn callback(user: *mut c_void, queue: AudioQueueRef, buf: AudioQueueBufferRef) {
    let pull = unsafe { &mut *(user as *mut Pull) };
    let b = unsafe { &mut *buf };
    let samples = unsafe { std::slice::from_raw_parts_mut(b.mAudioData.as_ptr() as *mut i16, b.mAudioDataBytesCapacity as usize / 2) };
    pull.fill(samples);
    b.mAudioDataByteSize = b.mAudioDataBytesCapacity;
    unsafe { AudioQueueEnqueueBuffer(queue, buf, 0, std::ptr::null()) };
}

pub fn start(rate: u32, frames: usize, source: Source) -> Output {
    let dump = std::env::var_os("NX_DUMP_AUDIO").and_then(|p| match Wav::create(&p, rate) {
        Ok(w) => Some(w),
        Err(e) => {
            eprintln!("[audio] can't create {}: {e}", p.to_string_lossy());
            None
        }
    });
    let pull = Box::into_raw(Box::new(Pull { source, dump }));
    let stop = Arc::new(AtomicBool::new(false));
    let opened = if std::env::var_os("NX_SILENT_AUDIO").is_some() { Err(0) } else { unsafe { open_queue(rate, frames, pull) } };
    match opened {
        Ok(queue) => Output { queue, pull, stop },
        Err(status) => {
            eprintln!("[audio] no output device (AudioQueue error {status}); rendering without output");
            let (s, p) = (stop.clone(), pull as usize);
            std::thread::Builder::new()
                .name("audio-out".into())
                .spawn(move || {
                    let pull = unsafe { &mut *(p as *mut Pull) };
                    let period = Duration::from_secs_f64(frames as f64 / rate as f64);
                    let mut buf = vec![0i16; frames * 2];
                    let mut next = Instant::now();
                    while !s.load(Relaxed) {
                        pull.fill(&mut buf);
                        next += period;
                        std::thread::sleep(next.saturating_duration_since(Instant::now()));
                    }
                    drop(unsafe { Box::from_raw(p as *mut Pull) });
                })
                .unwrap();
            Output { queue: std::ptr::null_mut(), pull: std::ptr::null_mut(), stop }
        }
    }
}

unsafe fn open_queue(rate: u32, frames: usize, pull: *mut Pull) -> Result<AudioQueueRef, i32> {
    let format = AudioStreamBasicDescription {
        mSampleRate: rate as f64,
        mFormatID: kAudioFormatLinearPCM,
        mFormatFlags: kAudioFormatFlagIsSignedInteger | kAudioFormatFlagIsPacked,
        mBytesPerPacket: 4,
        mFramesPerPacket: 1,
        mBytesPerFrame: 4,
        mChannelsPerFrame: 2,
        mBitsPerChannel: 16,
        mReserved: 0,
    };
    let check = |s: i32| if s == 0 { Ok(()) } else { Err(s) };
    let mut queue: AudioQueueRef = std::ptr::null_mut();
    unsafe {
        check(AudioQueueNewOutput(NonNull::from(&format), Some(callback), pull as *mut c_void, None, None, 0, NonNull::from(&mut queue)))?;
        let started = (|| {
            for _ in 0..QUEUED {
                let mut buf: AudioQueueBufferRef = std::ptr::null_mut();
                check(AudioQueueAllocateBuffer(queue, (frames * 4) as u32, NonNull::from(&mut buf)))?;
                callback(pull as *mut c_void, queue, buf);
            }
            check(AudioQueueStart(queue, std::ptr::null()))
        })();
        if started.is_err() {
            AudioQueueDispose(queue, true);
        }
        started.map(|_| queue)
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        self.stop.store(true, Relaxed);
        if !self.queue.is_null() {
            unsafe {
                AudioQueueDispose(self.queue, true);
                drop(Box::from_raw(self.pull));
            }
        }
    }
}

struct Wav {
    file: File,
    bytes: u32,
}

impl Wav {
    fn create(path: &std::ffi::OsStr, rate: u32) -> std::io::Result<Self> {
        let mut file = File::create(path)?;
        let mut h = Vec::with_capacity(44);
        h.extend_from_slice(b"RIFF\0\0\0\0WAVEfmt ");
        for v in [16u32, 1 | 2 << 16, rate, rate * 4, 4 | 16 << 16] {
            h.extend_from_slice(&v.to_le_bytes());
        }
        h.extend_from_slice(b"data\0\0\0\0");
        file.write_all(&h)?;
        Ok(Wav { file, bytes: 0 })
    }

    fn write(&mut self, s: &[i16]) {
        let data: Vec<u8> = s.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.bytes += data.len() as u32;
        let _ = (|| -> std::io::Result<()> {
            self.file.write_all(&data)?;
            self.file.seek(SeekFrom::Start(4))?;
            self.file.write_all(&(36 + self.bytes).to_le_bytes())?;
            self.file.seek(SeekFrom::Start(40))?;
            self.file.write_all(&self.bytes.to_le_bytes())?;
            self.file.seek(SeekFrom::End(0)).map(|_| ())
        })();
    }
}

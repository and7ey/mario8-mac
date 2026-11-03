// Mush8 - effects.rs
// Constantin Clerc (c22dev) - 0.1
use super::{u32_at, u64_at};
use std::sync::atomic::{AtomicU32, Ordering};

fn q14(p: &[u8], o: usize) -> f32 {
    u32_at(p, o) as i32 as f32 / 16384.0
}

pub struct Aux {
    send: u64,
    ret: u64,
    count_max: u32,
}

impl Aux {
    pub fn new(p: &[u8]) -> Self {
        Aux { send: u64_at(p, 0x40), ret: u64_at(p, 0x50), count_max: u32_at(p, 0x38) }
    }

    pub fn ring_size(p: &[u8]) -> u64 {
        0x80 + 4 * u32_at(p, 0x38) as u64
    }

    pub fn rings(p: &[u8]) -> [u64; 2] {
        [u64_at(p, 0x40), u64_at(p, 0x50)]
    }

    fn dsp_info(ring: u64, field: u64) -> &'static AtomicU32 {
        unsafe { &*((ring + 0x40 + field) as *const AtomicU32) }
    }

    fn samples(&self, ring: u64) -> &'static mut [i32] {
        unsafe { std::slice::from_raw_parts_mut((ring + 0x80) as *mut i32, self.count_max as usize) }
    }

    pub fn process(&self, input: &[f32], output: &mut [f32], offset: u32, advance: u32) {
        let max = self.count_max;
        if max == 0 {
            return;
        }
        let write = Self::dsp_info(self.send, 4);
        let mut at = offset + write.load(Ordering::Relaxed);
        if at <= max {
            let ring = self.samples(self.send);
            for &s in input {
                at %= max;
                ring[at as usize] = s as i32;
                at += 1;
            }
            if advance != 0 {
                write.store((write.load(Ordering::Relaxed) + advance) % max, Ordering::Release);
            }
        }
        let read = Self::dsp_info(self.ret, 0);
        let mut at = offset + read.load(Ordering::Acquire);
        if at > max {
            output.fill(0.0);
            return;
        }
        let ring = self.samples(self.ret);
        for o in output.iter_mut() {
            at %= max;
            *o = ring[at as usize] as f32;
            at += 1;
        }
        if advance != 0 {
            read.store((read.load(Ordering::Relaxed) + advance) % max, Ordering::Release);
        }
    }

    pub fn reset(&self) {
        for ring in [self.send, self.ret] {
            for field in [0, 4, 0xC] {
                Self::dsp_info(ring, field).store(0, Ordering::Release);
            }
        }
    }
}

struct DelayLine {
    buf: Vec<f32>,
    pos: usize,
    last: usize,
}

impl DelayLine {
    fn new(max: usize, len: usize) -> Self {
        let len = len.min(max);
        DelayLine { buf: vec![0.0; max + 1], pos: 0, last: len.saturating_sub(1) }
    }

    fn read(&self) -> f32 {
        self.buf[self.pos]
    }

    fn push(&mut self, v: f32) {
        self.buf[self.pos] = v;
        self.pos += 1;
        if self.pos >= self.last {
            self.pos = 0;
        }
    }
}

pub struct Delay {
    key: (usize, u32, u32, u32),
    lines: Vec<DelayLine>,
    lowpass: Vec<f32>,
}

const CHANNELS: usize = 0xE;
const TIME_MAX: usize = 0x10;
const TIME: usize = 0x14;
const RATE: usize = 0x18;

impl Delay {
    pub fn channels(p: &[u8]) -> Option<usize> {
        let n = u16::from_le_bytes([p[CHANNELS], p[CHANNELS + 1]]) as usize;
        matches!(n, 1 | 2 | 4 | 6).then_some(n)
    }

    pub fn fits(&self, p: &[u8]) -> bool {
        self.key == Self::key(p)
    }

    fn key(p: &[u8]) -> (usize, u32, u32, u32) {
        (Self::channels(p).unwrap_or(0), u32_at(p, TIME_MAX), u32_at(p, TIME), u32_at(p, RATE))
    }

    pub fn new(p: &[u8]) -> Self {
        let key @ (n, max, time, rate) = Self::key(p);
        let khz = ((rate >> 14) / 1000) as f32;
        let samples = |ms: u32| (khz * ms as f32).round() as usize;
        Delay { key, lines: (0..n).map(|_| DelayLine::new(samples(max), samples(time))).collect(), lowpass: vec![0.0; n] }
    }

    pub fn process(&mut self, p: &[u8], io: &mut [Vec<f32>]) {
        let n = self.lines.len();
        let (in_gain, out_gain, dry) = (q14(p, 0x1C), q14(p, 0x24), q14(p, 0x28));
        let raw_feedback = q14(p, 0x20);
        let feedback = raw_feedback * 0.98;
        let spread = q14(p, 0x2C);
        let base = (1.0 - spread) * feedback;
        let cross = spread * feedback * if n >= 4 { 0.5 } else { 1.0 };
        let lp_feedback = 0.95 * q14(p, 0x30);
        let (b, c) = (base, cross);
        let matrix: Vec<[f32; 6]> = match n {
            1 => vec![[raw_feedback, 0.0, 0.0, 0.0, 0.0, 0.0]],
            2 => vec![[b, c, 0.0, 0.0, 0.0, 0.0], [c, b, 0.0, 0.0, 0.0, 0.0]],
            4 => vec![[b, c, c, 0.0, 0.0, 0.0], [c, b, 0.0, c, 0.0, 0.0], [c, 0.0, b, c, 0.0, 0.0], [0.0, c, c, b, 0.0, 0.0]],
            _ => vec![
                [b, 0.0, c, 0.0, c, 0.0],
                [0.0, b, c, 0.0, 0.0, c],
                [c, c, b, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, raw_feedback, 0.0, 0.0],
                [c, 0.0, 0.0, 0.0, b, c],
                [0.0, c, 0.0, 0.0, c, b],
            ],
        };
        let len = io.first().map_or(0, Vec::len);
        let mut taps = [0.0f32; 6];
        for i in 0..len {
            for (t, l) in taps.iter_mut().zip(&self.lines) {
                *t = l.read();
            }
            for k in 0..n {
                let input = io[k][i] * 64.0;
                let fed: f32 = matrix[k].iter().zip(&taps).map(|(m, t)| m * t).sum::<f32>() + input * in_gain;
                self.lowpass[k] = lp_feedback * self.lowpass[k] + (1.0 - lp_feedback) * fed;
                self.lines[k].push(self.lowpass[k]);
                io[k][i] = (input * dry + taps[k] * out_gain) / 64.0;
            }
        }
    }
}

// Mush8 - dsp.rs
// Constantin Clerc (c22dev) - 0.1
use super::{SampleFormat, VoiceState, WaveBuffer};

pub const TAPS: usize = 4;
pub const SCRATCH: usize = 0x3F00;

const PLAYED_COUNT_RESET_ON_LOOP: u16 = 1;
const SKIP_PITCH_AND_SRC: u16 = 2;

pub struct Source<'a> {
    pub format: SampleFormat,
    pub sample_rate: u32,
    pub pitch: f32,
    pub channel: usize,
    pub channels: usize,
    pub behaviour: u16,
    pub coefs: [i16; 16],
    pub wave_buffers: &'a [WaveBuffer; 4],
}

unsafe fn guest<'a, T>(addr: u64, len: usize) -> &'a [T] {
    if addr == 0 || len == 0 { &[] } else { unsafe { std::slice::from_raw_parts(addr as *const T, len) } }
}

pub fn adpcm_size(samples: u32) -> u64 {
    let (frames, rem) = (samples as u64 / 14, samples as u64 % 14);
    frames * 8 + if rem != 0 { rem / 2 + 1 + rem % 2 } else { 0 }
}

fn adpcm_nibbles(samples: usize) -> usize {
    let rem = samples % 14;
    rem + 16 * (samples / 14) + if rem != 0 { 2 } else { 0 }
}

fn decode_adpcm(out: &mut [i16], input: &[u8], start: usize, end: usize, offset: usize, count: usize, coefs: &[i16; 16], ctx: &mut [i16; 3]) -> usize {
    let n = count.min(end.saturating_sub(start + offset));
    let mut nibble = adpcm_nibbles(start + offset);
    let mut byte = nibble / 2;
    let [mut ps, mut h0, mut h1] = *ctx;
    for o in out.iter_mut().take(n) {
        if nibble % 16 == 0 {
            let Some(&b) = input.get(byte) else { return 0 };
            ps = b as i16;
            byte += 1;
            nibble += 2;
        }
        let Some(&b) = input.get(byte) else { return 0 };
        let v = if nibble & 1 == 0 { (b as i8) >> 4 } else { ((b << 4) as i8) >> 4 } as i32;
        if nibble & 1 == 1 {
            byte += 1;
        }
        nibble += 1;
        let c = ((ps >> 4) & 7) as usize * 2;
        let pred = coefs[c] as i32 * h0 as i32 + coefs[c + 1] as i32 * h1 as i32;
        let s = (((v << (ps & 0xF)) << 11) + pred + 0x400) >> 11;
        h1 = h0;
        h0 = s.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        *o = h0;
    }
    *ctx = [ps, h0, h1];
    n
}

fn decode_pcm<T: Copy>(out: &mut [i16], wb: &WaveBuffer, (start, end): (usize, usize), src: &Source, offset: usize, count: usize, conv: impl Fn(T) -> i16) -> usize {
    let frame = size_of::<T>() * src.channels;
    let first = (start + offset) * frame;
    let avail = (wb.size as usize).saturating_sub(first) / frame;
    let n = count.min(end.saturating_sub(start + offset)).min(avail);
    let data: &[T] = unsafe { guest(wb.addr + first as u64, n * src.channels) };
    for (o, f) in out.iter_mut().zip(data.chunks_exact(src.channels)) {
        *o = conv(f[src.channel]);
    }
    n
}

pub fn process_wave_buffers(out: &mut [f32], src: &Source, st: &mut VoiceState, target_rate: u32, scratch: &mut [i16]) {
    let n = out.len();
    let ratio = src.sample_rate as f32 / target_rate as f32 * src.pitch;
    let skip_src = src.behaviour & SKIP_PITCH_AND_SRC != 0;
    let needed = (st.fraction + ratio * n as f32).trunc();
    if !(0.0..=(SCRATCH - TAPS) as f32).contains(&needed) {
        return;
    }
    let per_iter = (((SCRATCH as f32 - st.fraction) / ratio).trunc() as usize).min(n).max(1);
    let mut i = 0;
    while i < n {
        let base = if skip_src { 0 } else { TAPS };
        scratch[..base].copy_from_slice(&st.history[..base]);
        let count = (n - i).min(per_iter);
        let to_decode = ((st.fraction + ratio * count as f32).trunc() as usize).min(SCRATCH - TAPS);
        let mut y = 0;
        while y < to_decode {
            let idx = st.wave_buffer_index as usize;
            if !st.valid[idx] {
                break;
            }
            let wb = &src.wave_buffers[idx];
            if st.offset == 0 && src.format == SampleFormat::Adpcm && wb.context != 0 {
                let ctx: &[i16] = unsafe { guest(wb.context, 3) };
                st.adpcm = [ctx[0], ctx[1], ctx[2]];
            }
            let range @ (start, end) = wb.range(st.loop_count);
            let want = to_decode - y;
            let dst = &mut scratch[base + y..];
            let offset = st.offset as usize;
            let decoded = match src.format {
                SampleFormat::PcmInt16 => decode_pcm::<i16>(dst, wb, range, src, offset, want, |s| s),
                SampleFormat::PcmFloat => decode_pcm::<f32>(dst, wb, range, src, offset, want, |s| (s * 32767.0).clamp(-32768.0, 32767.0) as i16),
                SampleFormat::Adpcm => {
                    let data = unsafe { guest(wb.addr, wb.size as usize) };
                    decode_adpcm(dst, data, start, end, offset, want, &src.coefs, &mut st.adpcm)
                }
            };
            y += decoded;
            st.offset += decoded as u32;
            st.played_samples += decoded as u64;
            if st.offset as usize >= end.saturating_sub(start) || decoded == 0 {
                st.offset = 0;
                if wb.looping {
                    st.loop_count += 1;
                    if wb.loop_count >= 0 && (decoded == 0 || st.loop_count > wb.loop_count) {
                        st.end_wave_buffer(wb);
                    }
                    if decoded == 0 {
                        break;
                    }
                    if src.behaviour & PLAYED_COUNT_RESET_ON_LOOP != 0 {
                        st.played_samples = 0;
                    }
                } else {
                    st.end_wave_buffer(wb);
                }
            }
        }
        if skip_src {
            for (o, &s) in out[i..i + count].iter_mut().zip(&scratch[..y]) {
                *o = s as f32;
            }
        } else {
            scratch[base + y..base + to_decode].fill(0);
            resample(&mut out[i..i + count], &scratch[..base + to_decode], ratio, &mut st.fraction);
            st.history.copy_from_slice(&scratch[to_decode..to_decode + TAPS]);
        }
        i += count;
    }
}

fn resample(out: &mut [f32], input: &[i16], ratio: f32, fraction: &mut f32) {
    let mut j = 0;
    for o in out {
        let p = |k: usize| input.get(j + k).copied().unwrap_or(0) as f32;
        let (p0, p1, p2, p3, t) = (p(0), p(1), p(2), p(3), *fraction);
        *o = (p1 + 0.5 * t * (p2 - p0 + t * (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3 + t * (3.0 * (p1 - p2) + p3 - p0)))).round();
        *fraction += ratio;
        j += *fraction as usize;
        *fraction = fraction.fract();
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Biquad {
    pub enable: bool,
    pub num: [f32; 3],
    pub den: [f32; 2],
}

impl Biquad {
    pub fn read(d: &[u8]) -> Self {
        let q = |o: usize| i16::from_le_bytes([d[o], d[o + 1]]) as f32 / 16384.0;
        Self { enable: d[0] != 0, num: [q(2), q(4), q(6)], den: [q(8), q(10)] }
    }

    pub fn process(&self, state: &mut [f32; 2], buf: &mut [f32]) {
        for s in buf {
            let x = *s;
            let y = x * self.num[0] + state[0];
            state[0] = x * self.num[1] + y * self.den[0] + state[1];
            state[1] = x * self.num[2] + y * self.den[1];
            *s = y;
        }
    }
}

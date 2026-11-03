// Mush8 - fx.rs
// Constantin Clerc (c22dev) - 0.1
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Default, Clone, Copy)]
pub struct Fx(u64);

impl Hasher for Fx {
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(8);
        for c in &mut chunks {
            self.write_u64(u64::from_le_bytes(c.try_into().unwrap()));
        }
        for &b in chunks.remainder() {
            self.write_u64(b as u64);
        }
    }
    fn write_u8(&mut self, n: u8) {
        self.write_u64(n as u64);
    }
    fn write_u32(&mut self, n: u32) {
        self.write_u64(n as u64);
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(0x51_7cc1_b727_220a_95);
    }
    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }
    fn finish(&self) -> u64 {
        self.0 ^ self.0 >> 29
    }
}

pub type FastMap<K, V> = HashMap<K, V, std::hash::BuildHasherDefault<Fx>>;
pub type FastSet<K> = std::collections::HashSet<K, std::hash::BuildHasherDefault<Fx>>;

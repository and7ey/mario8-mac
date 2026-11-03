// Mush8 - corpus.rs
// Constantin Clerc (c22dev) - 0.1
use std::collections::BTreeMap;

pub fn key(blob: &[u8]) -> Option<u64> {
    Some(fnv1a(&blob[0x30..code_len(blob)?]))
}

pub fn scan(data: &[u8], found: &mut BTreeMap<u64, Vec<u8>>) {
    scan_at(data, found, 0);
}

const NVN_CODE_MAGIC: [u8; 4] = 0x1234_5678u32.to_le_bytes();

fn code_len(blob: &[u8]) -> Option<usize> {
    let code = blob.get(0x80..)?;
    for (i, c) in code.chunks_exact(8).enumerate().take(0x8000) {
        if i % 4 == 0 {
            continue;
        }
        let w = u64::from_le_bytes(c.try_into().unwrap());
        let rel = ((((w >> 20) as i32) << 8) >> 8) + 8;
        if crate::table::lookup(w).0 == crate::table::InstName::Bra && (-32..=0).contains(&rel) {
            return Some(0x80 + (i / 4 + 1) * 32);
        }
    }
    None
}

fn valid_sph(blob: &[u8]) -> bool {
    blob.len() >= 0x80 && {
        let w0 = u32::from_le_bytes(blob[0x30..0x34].try_into().unwrap());
        (1..=5).contains(&(w0 >> 10 & 0xF)) || (1..=5).contains(&(w0 & 0x1F))
    }
}

fn scan_at(data: &[u8], found: &mut BTreeMap<u64, Vec<u8>>, depth: usize) {
    let data = match nxfs::sarc::yaz0(data) {
        Ok(d) => d,
        Err(_) => return,
    };
    if depth < 4 {
        if let Ok(entries) = nxfs::sarc::sarc(&data) {
            for e in entries {
                scan_at(e.data, found, depth + 1);
            }
            return;
        }
    }
    let mut i = 0;
    while let Some(p) = data[i..].windows(4).position(|w| w == NVN_CODE_MAGIC) {
        let at = i + p;
        i = at + 4;
        if at % 0x10 != 0 || data.get(at + 4..at + 0x30).is_none_or(|z| z.iter().any(|&b| b != 0)) {
            continue;
        }
        let blob = &data[at..];
        if !valid_sph(blob) {
            continue;
        }
        if let Some(len) = code_len(blob) {
            let code = blob[..len].to_vec();
            let h = fnv1a(&code[0x30..]);
            found.entry(h).or_insert(code);
        }
    }
}

pub fn fnv1a(b: &[u8]) -> u64 {
    b.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &x| (h ^ x as u64).wrapping_mul(0x100_0000_01b3))
}

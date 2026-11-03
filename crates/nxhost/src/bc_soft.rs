// Mush8 - bc_soft.rs
// Software BC1/BC2/BC3/BC7 decoding to RGBA8 (used where hardware BC is unavailable).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bc1,
    Bc2,
    Bc3,
    Bc4(bool),
    Bc5(bool),
    Bc7,
}

pub fn soft_rgba(nvn: u32) -> Option<Kind> {
    Some(match nvn {
        65 | 66 | 69 | 70 => Kind::Bc1,
        67 | 71 => Kind::Bc2,
        68 | 72 => Kind::Bc3,
        73 | 74 => Kind::Bc4(nvn == 74),
        75 | 76 => Kind::Bc5(nvn == 76),
        77 | 78 => Kind::Bc7,
        _ => return None,
    })
}

pub fn srgb(nvn: u32) -> bool {
    matches!(nvn, 69..=72 | 78)
}

pub fn block_bytes(k: Kind) -> usize {
    match k {
        Kind::Bc1 | Kind::Bc4(_) => 8,
        _ => 16,
    }
}

pub fn decode(k: Kind, src: &[u8], row_bytes: usize, w: usize, h: usize) -> Vec<u8> {
    let mut out = vec![0u8; w * h * 4];
    if out.is_empty() {
        return out;
    }
    let mut tmp = [0u8; 4 * 4 * 4];
    let mut tmp1 = [0u8; 16];
    let mut tmp2 = [0u8; 32];
    let block = block_bytes(k);
    for by in 0..h.div_ceil(4) {
        for bx in 0..w.div_ceil(4) {
            let o = by * row_bytes + bx * block;
            let Some(b) = src.get(o..o + block) else { return out };
            match k {
                Kind::Bc1 => bcdec_rs::bc1(b, &mut tmp, 16),
                Kind::Bc2 => bcdec_rs::bc2(b, &mut tmp, 16),
                Kind::Bc3 => bcdec_rs::bc3(b, &mut tmp, 16),
                Kind::Bc4(signed) => {
                    bcdec_rs::bc4(b, &mut tmp1, 4, signed);
                    for i in 0..16 {
                        tmp[i * 4] = tmp1[i];
                        tmp[i * 4 + 1] = 0;
                        tmp[i * 4 + 2] = 0;
                        tmp[i * 4 + 3] = 255;
                    }
                }
                Kind::Bc5(signed) => {
                    bcdec_rs::bc5(b, &mut tmp2, 8, signed);
                    for i in 0..16 {
                        tmp[i * 4] = tmp2[i * 2];
                        tmp[i * 4 + 1] = tmp2[i * 2 + 1];
                        tmp[i * 4 + 2] = 0;
                        tmp[i * 4 + 3] = 255;
                    }
                }
                Kind::Bc7 => bcdec_rs::bc7(b, &mut tmp, 16),
            }
            let n = (w - bx * 4).min(4);
            for r in 0..(h - by * 4).min(4) {
                let d = ((by * 4 + r) * w + bx * 4) * 4;
                out[d..d + n * 4].copy_from_slice(&tmp[r * 16..r * 16 + n * 4]);
            }
        }
    }
    out
}

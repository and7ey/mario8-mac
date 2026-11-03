// Mush8 - tex.rs
// Constantin Clerc (c22dev) - 0.1
pub const GOB_STRIDE: u32 = 64;
pub const GOB_HEIGHT: u32 = 8;
pub const GOB_SIZE: u64 = 512;

fn div_up(a: u32, b: u32) -> u32 {
    a.div_ceil(b)
}

fn align_up(a: u64, b: u64) -> u64 {
    a.div_ceil(b) * b
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub levels: u32,
    pub layers: u32,
    pub block_w: u32,
    pub block_h: u32,
    pub bytes_per_block: u32,
}

pub fn default_block_log2(extent_in_gobs: u32, max_log2: u32) -> u32 {
    let mut l = max_log2;
    while l > 0 && extent_in_gobs <= 1 << (l - 1) {
        l -= 1;
    }
    l
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub level_offsets: Vec<u64>,
    pub level_gobs: Vec<(u32, u32)>,
    pub layer_size: u64,
    pub total_size: u64,
}

pub fn block_linear(s: &Shape, gob_y_log2: u32, gob_z_log2: u32) -> Layout {
    let is_3d = s.depth > 1;
    let (mut gy, mut gz) = (1u32 << gob_y_log2, 1u32 << gob_z_log2);
    let mut layer_size = 0u64;
    let mut level_offsets = Vec::new();
    let mut level_gobs = Vec::new();
    for level in 0..s.levels.max(1) {
        let w = div_up((s.width >> level).max(1), s.block_w);
        let h = div_up((s.height >> level).max(1), s.block_h);
        let d = (s.depth >> level).max(1);
        while h <= (gy >> 1) * GOB_HEIGHT && gy != 1 {
            gy >>= 1;
        }
        while d <= gz >> 1 && gz != 1 {
            gz >>= 1;
        }
        let width_in_gobs = div_up(w * s.bytes_per_block, GOB_STRIDE) as u64;
        let blocks_y = div_up(div_up(h, GOB_HEIGHT), gy) as u64;
        let blocks_z = div_up(d, gz) as u64;
        let rob = width_in_gobs * gy as u64 * gz as u64 * GOB_SIZE;
        level_offsets.push(layer_size);
        level_gobs.push((gy, gz));
        layer_size += if is_3d { blocks_z * blocks_y * rob } else { blocks_y * rob };
    }
    if s.layers > 1 {
        let h = div_up(s.height, s.block_h);
        let (mut gy, mut gz) = (1u32 << gob_y_log2, 1u32 << gob_z_log2);
        while h <= (gy >> 1) * GOB_HEIGHT && gy != 1 {
            gy >>= 1;
        }
        while s.depth <= gz >> 1 && gz != 1 {
            gz >>= 1;
        }
        layer_size = align_up(layer_size, gy as u64 * gz as u64 * GOB_SIZE);
    }
    Layout { level_offsets, level_gobs, total_size: layer_size * s.layers.max(1) as u64, layer_size }
}

#[inline]
fn gob_offset(x: u32, y: u32) -> u32 {
    (x % 64 / 32) * 256 + (y % 8 / 2) * 64 + (x % 32 / 16) * 32 + (y % 2) * 16 + x % 16
}

pub fn untile_into(src: &[u8], row_bytes: u32, rows: u32, z: u32, gobs: (u32, u32), ys: std::ops::Range<u32>, dst: &mut [u8]) {
    let (gy, gz) = gobs;
    let width_in_gobs = row_bytes.div_ceil(GOB_STRIDE);
    let block_size = gy * gz * GOB_SIZE as u32;
    let block_row = width_in_gobs * block_size;
    let blocks_y = rows.div_ceil(GOB_HEIGHT).div_ceil(gy);
    let slice_blocks = blocks_y * block_row;
    let zbase = (z / gz) * slice_blocks + (z % gz) * gy * GOB_SIZE as u32;
    let full = row_bytes & !15;
    let mut d = 0usize;
    for y in ys {
        let ybase = zbase + (y / (GOB_HEIGHT * gy)) * block_row + (y / GOB_HEIGHT % gy) * GOB_SIZE as u32;
        let mut x = 0;
        while x < full {
            let o = (ybase + (x / GOB_STRIDE) * block_size + gob_offset(x, y)) as usize;
            let out = &mut dst[d..d + 16];
            match src.get(o..o + 16) {
                Some(b) => out.copy_from_slice(<&[u8; 16]>::try_from(b).unwrap()),
                None => out.fill(0),
            }
            d += 16;
            x += 16;
        }
        if x < row_bytes {
            let n = (row_bytes - x) as usize;
            let o = (ybase + (x / GOB_STRIDE) * block_size + gob_offset(x, y)) as usize;
            match src.get(o..o + n) {
                Some(b) => dst[d..d + n].copy_from_slice(b),
                None => dst[d..d + n].fill(0),
            }
            d += n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn untile(src: &[u8], row_bytes: u32, rows: u32, depth: u32, gobs: (u32, u32), out: &mut Vec<u8>) {
        let start = out.len();
        out.resize(start + (row_bytes * rows * depth) as usize, 0);
        for z in 0..depth {
            let image = (row_bytes * rows) as usize;
            let dst = &mut out[start + z as usize * image..][..image];
            untile_into(src, row_bytes, rows, z, gobs, 0..rows, dst);
        }
    }

    fn untile_reference(src: &[u8], row_bytes: u32, rows: u32, depth: u32, gobs: (u32, u32), out: &mut Vec<u8>) {
        let (gy, gz) = gobs;
        let width_in_gobs = row_bytes.div_ceil(GOB_STRIDE);
        let block_size = gy * gz * GOB_SIZE as u32;
        let block_row = width_in_gobs * block_size;
        let blocks_y = rows.div_ceil(GOB_HEIGHT).div_ceil(gy);
        let slice_blocks = blocks_y * block_row;
        out.reserve((row_bytes * rows * depth) as usize);
        for z in 0..depth {
            let zbase = (z / gz) * slice_blocks + (z % gz) * gy * GOB_SIZE as u32;
            for y in 0..rows {
                let ybase = zbase + (y / (GOB_HEIGHT * gy)) * block_row + (y / GOB_HEIGHT % gy) * GOB_SIZE as u32;
                let mut x = 0;
                while x < row_bytes {
                    let n = (16 - x % 16).min(row_bytes - x);
                    let o = (ybase + (x / GOB_STRIDE) * block_size + gob_offset(x, y)) as usize;
                    match src.get(o..o + n as usize) {
                        Some(b) => out.extend_from_slice(b),
                        None => out.extend(std::iter::repeat_n(0, n as usize)),
                    }
                    x += n;
                }
            }
        }
    }

    #[test]
    fn untile_matches_reference() {
        let src: Vec<u8> = (0..300_000u32).map(|i| (i.wrapping_mul(2654435761) >> 13) as u8).collect();
        for &(row_bytes, rows, depth, gobs) in &[(256, 64, 1, (4, 1)), (100, 37, 1, (2, 1)), (520, 90, 1, (16, 1)), (64, 16, 4, (2, 2)), (8, 3, 1, (1, 1))] {
            let (mut a, mut b) = (vec![1, 2, 3], vec![1, 2, 3]);
            untile(&src, row_bytes, rows, depth, gobs, &mut a);
            untile_reference(&src, row_bytes, rows, depth, gobs, &mut b);
            assert_eq!(a, b, "{row_bytes}x{rows}x{depth} {gobs:?}");
            let short = &src[..1000];
            let (mut a, mut b) = (Vec::new(), Vec::new());
            untile(short, row_bytes, rows, depth, gobs, &mut a);
            untile_reference(short, row_bytes, rows, depth, gobs, &mut b);
            assert_eq!(a, b, "short {row_bytes}x{rows}");
        }
    }

    fn rgba8(w: u32, h: u32, levels: u32, layers: u32) -> Shape {
        Shape { width: w, height: h, depth: 1, levels, layers, block_w: 1, block_h: 1, bytes_per_block: 4 }
    }

    #[test]
    fn render_target_1280x720() {
        let bh = default_block_log2(720u32.div_ceil(8), 4);
        assert_eq!(bh, 4);
        let l = block_linear(&rgba8(1280, 720, 1, 1), bh, 0);
        assert_eq!(l.total_size, 1280 * 768 * 4);
    }

    #[test]
    fn small_texture_shrinks_block_height() {
        let bh = default_block_log2(64 / 8, 4);
        assert_eq!(bh, 3);
        assert_eq!(block_linear(&rgba8(64, 64, 1, 1), bh, 0).total_size, 64 * 64 * 4);
    }

    #[test]
    fn untile_roundtrip() {
        let (w, h, gy) = (128 * 4, 16, 2);
        let mut tiled = vec![0u8; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                let o = (y / (8 * gy)) * (w / 64 * gy * 512) + (x / 64) * gy * 512 + (y / 8 % gy) * 512 + gob_offset(x, y);
                tiled[o as usize] = (x * 7 + y * 13) as u8;
            }
        }
        let mut out = Vec::new();
        untile(&tiled, w, h, 1, (gy, 1), &mut out);
        for y in 0..h {
            for x in 0..w {
                assert_eq!(out[(y * w + x) as usize], (x * 7 + y * 13) as u8);
            }
        }
    }

    #[test]
    fn mip_chain_and_bc1() {
        let s = Shape { width: 256, height: 256, depth: 1, levels: 9, layers: 1, block_w: 4, block_h: 4, bytes_per_block: 8 };
        let l = block_linear(&s, 3, 0);
        assert_eq!(l.level_offsets[0], 0);
        assert_eq!(l.level_offsets[1], 512 * 64);
        assert!(l.total_size >= 512 * 64 + 256 * 32);
    }
}

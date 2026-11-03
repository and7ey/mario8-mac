// Mush8 - sph.rs
// Constantin Clerc (c22dev) - 0.1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Vertex,
    TessControl,
    TessEval,
    Geometry,
    Fragment,
    Compute,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imap {
    Unused,
    Constant,
    Perspective,
    ScreenLinear,
}

#[derive(Clone, Debug)]
pub struct Sph {
    pub stage: Stage,
    pub kills_pixels: bool,
    pub local_memory: u32,
    pub imap: [Imap; 32],
    pub omap_targets: u32,
    pub omap_depth: bool,
    pub omap_sample_mask: bool,
}

impl Sph {
    pub fn compute(local_memory: u32) -> Sph {
        Sph {
            stage: Stage::Compute,
            kills_pixels: false,
            local_memory,
            imap: [Imap::Unused; 32],
            omap_targets: 0,
            omap_depth: false,
            omap_sample_mask: false,
        }
    }

    // Compute programs are handed over as raw code, but some builds pass the
    // whole SPH blob; accept both and take the shared-memory size from the SPH.
    pub fn split_compute(blob: &[u8], local_memory: u32) -> (Sph, &[u8]) {
        let header = blob.len() > 0x80
            && u32::from_le_bytes(blob[..4].try_into().unwrap()) == 0x1234_5678
            && blob[4..0x30].iter().all(|&b| b == 0);
        if header {
            if let Some(mut s) = Sph::parse(&blob[0x30..]) {
                s.stage = Stage::Compute;
                if s.local_memory == 0 {
                    s.local_memory = local_memory;
                }
                return (s, &blob[0x80..]);
            }
        }
        (Sph::compute(local_memory), blob)
    }

    pub fn parse(h: &[u8]) -> Option<Sph> {
        if h.len() < 0x50 {
            return None;
        }
        let w = |i: usize| u32::from_le_bytes(h[i * 4..i * 4 + 4].try_into().unwrap());
        let stage = match (w(0) >> 10) & 0xF {
            0 | 1 => Stage::Vertex,
            2 => Stage::TessControl,
            3 => Stage::TessEval,
            4 => Stage::Geometry,
            5 => Stage::Fragment,
            6 => Stage::Compute,
            _ => return None,
        };
        let mut imap = [Imap::Unused; 32];
        for (i, m) in imap.iter_mut().enumerate() {
            let byte = (w(6 + (i >> 2)) >> ((i & 3) * 8)) & 0xFF;
            *m = (0..4)
                .map(|c| match (byte >> (c * 2)) & 3 {
                    1 => Imap::Constant,
                    2 => Imap::Perspective,
                    3 => Imap::ScreenLinear,
                    _ => Imap::Unused,
                })
                .find(|t| *t != Imap::Unused)
                .unwrap_or(Imap::Unused);
        }
        Some(Sph {
            stage,
            kills_pixels: w(0) >> 15 & 1 != 0,
            local_memory: (w(1) & 0xFF_FFFF) + (w(2) & 0xFF_FFFF),
            imap,
            omap_targets: w(18),
            omap_depth: w(19) >> 1 & 1 != 0,
            omap_sample_mask: w(19) & 1 != 0,
        })
    }
}

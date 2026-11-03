// Mush8 - nso.rs
// Constantin Clerc (c22dev) - 0.1
use anyhow::{Result, ensure};

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub mem_offset: u32,
    pub size: u32,
}

pub struct Nso {
    pub module_id: [u8; 32],
    pub text: Segment,
    pub ro: Segment,
    pub data: Segment,
    pub bss_size: u32,
    pub image: Vec<u8>,
    dynstr: (u32, u32),
    dynsym: (u32, u32),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub info: u8,
    pub defined: bool,
}

fn rd(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

impl Nso {
    pub fn parse(file: &[u8]) -> Result<Self> {
        ensure!(file.len() >= 0x100 && &file[..4] == b"NSO0", "not an NSO");
        let flags = rd(file, 0xC);
        let mut segs = [Segment { mem_offset: 0, size: 0 }; 3];
        let mut end = 0usize;
        let mut parts = Vec::new();
        for i in 0..3 {
            let h = 0x10 + i * 0x10;
            let (foff, moff, size) = (rd(file, h) as usize, rd(file, h + 4), rd(file, h + 8));
            let csize = rd(file, 0x60 + i * 4) as usize;
            let raw = &file[foff..foff + if flags & (1 << i) != 0 { csize } else { size as usize }];
            let bytes = if flags & (1 << i) != 0 { lz4_flex::block::decompress(raw, size as usize)? } else { raw.to_vec() };
            ensure!(bytes.len() == size as usize, "segment {i} size mismatch");
            segs[i] = Segment { mem_offset: moff, size };
            end = end.max((moff + size) as usize);
            parts.push(bytes);
        }
        let mut image = vec![0; end];
        for (s, b) in segs.iter().zip(&parts) {
            image[s.mem_offset as usize..][..b.len()].copy_from_slice(b);
        }
        Ok(Self {
            module_id: file[0x40..0x60].try_into()?,
            text: segs[0],
            ro: segs[1],
            data: segs[2],
            bss_size: rd(file, 0x3C),
            image,
            dynstr: (rd(file, 0x90), rd(file, 0x94)),
            dynsym: (rd(file, 0x98), rd(file, 0x9C)),
        })
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        let ro = self.ro.mem_offset as usize;
        let strs = &self.image[ro + self.dynstr.0 as usize..][..self.dynstr.1 as usize];
        let syms = &self.image[ro + self.dynsym.0 as usize..][..self.dynsym.1 as usize];
        syms.chunks_exact(24)
            .skip(1)
            .filter_map(|s| {
                let n = rd(s, 0) as usize;
                let end = strs[n..].iter().position(|&c| c == 0)?;
                let name = String::from_utf8_lossy(&strs[n..n + end]).into_owned();
                if name.is_empty() {
                    return None;
                }
                Some(Symbol {
                    name,
                    info: s[4],
                    defined: u16::from_le_bytes([s[6], s[7]]) != 0,
                    value: u64::from_le_bytes(s[8..16].try_into().unwrap()),
                    size: u64::from_le_bytes(s[16..24].try_into().unwrap()),
                })
            })
            .collect()
    }

    pub fn text_bytes(&self) -> &[u8] {
        &self.image[self.text.mem_offset as usize..][..self.text.size as usize]
    }
}

pub fn demangle(name: &str) -> String {
    cpp_demangle::Symbol::new(name)
        .ok()
        .and_then(|s| s.demangle().ok())
        .unwrap_or_else(|| name.to_string())
}

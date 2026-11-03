// Mush8 - module.rs
// Constantin Clerc (c22dev) - 0.1
use anyhow::{Result, bail, ensure};

pub const R_AARCH64_ABS64: u32 = 257;
pub const R_AARCH64_GLOB_DAT: u32 = 1025;
pub const R_AARCH64_JUMP_SLOT: u32 = 1026;
pub const R_AARCH64_RELATIVE: u32 = 1027;

#[derive(Debug, Clone, Copy)]
pub struct Rela {
    pub offset: u64,
    pub kind: u32,
    pub sym: u32,
    pub addend: i64,
}

#[derive(Debug, Clone)]
pub struct Sym {
    pub name: String,
    pub value: u64,
    pub info: u8,
    pub shndx: u16,
}

impl Sym {
    pub fn defined(&self) -> bool {
        self.shndx != 0
    }
    pub fn weak(&self) -> bool {
        self.info >> 4 == 2
    }
}

#[derive(Debug, Default)]
pub struct Dynamic {
    pub mod0: u64,
    pub bss: (u64, u64),
    pub relocs: Vec<Rela>,
    pub symbols: Vec<Sym>,
    pub init: Option<u64>,
    pub init_array: Option<(u64, u64)>,
    pub fini_array: Option<(u64, u64)>,
}

fn u32_at(b: &[u8], o: u64) -> Result<u32> {
    let o = o as usize;
    ensure!(o + 4 <= b.len(), "read past image at {o:#x}");
    Ok(u32::from_le_bytes(b[o..o + 4].try_into()?))
}
fn u64_at(b: &[u8], o: u64) -> Result<u64> {
    let o = o as usize;
    ensure!(o + 8 <= b.len(), "read past image at {o:#x}");
    Ok(u64::from_le_bytes(b[o..o + 8].try_into()?))
}

pub fn parse(image: &[u8]) -> Result<Dynamic> {
    let mod0 = u32_at(image, 4)? as u64;
    if u32_at(image, mod0)? != u32::from_le_bytes(*b"MOD0") {
        bail!("no MOD0 header");
    }
    let rel = |o: u64| -> Result<u64> { Ok((mod0 as i64 + u32_at(image, mod0 + o)? as i32 as i64) as u64) };
    let dynamic = rel(4)?;
    let mut d = Dynamic { mod0, bss: (rel(8)?, rel(12)?), ..Default::default() };

    let (mut rela, mut relasz, mut jmprel, mut pltrelsz) = (0, 0, 0, 0);
    let (mut symtab, mut strtab, mut syment, mut hash) = (0, 0, 24, 0);
    let (mut init_array, mut init_arraysz, mut fini_array, mut fini_arraysz) = (None, 0, None, 0);
    let mut o = dynamic;
    loop {
        let (tag, val) = (u64_at(image, o)? as i64, u64_at(image, o + 8)?);
        o += 16;
        match tag {
            0 => break,
            2 => pltrelsz = val,
            4 => hash = val,
            5 => strtab = val,
            6 => symtab = val,
            7 => rela = val,
            8 => relasz = val,
            11 => syment = val,
            12 => d.init = Some(val),
            23 => jmprel = val,
            25 => init_array = Some(val),
            26 => fini_array = Some(val),
            27 => init_arraysz = val,
            28 => fini_arraysz = val,
            36 | 35 => bail!("RELR relocations are not supported"),
            _ => {}
        }
    }
    d.init_array = init_array.map(|a| (a, init_arraysz));
    d.fini_array = fini_array.map(|a| (a, fini_arraysz));

    for (start, size) in [(rela, relasz), (jmprel, pltrelsz)] {
        let mut r = start;
        while r < start + size {
            let info = u64_at(image, r + 8)?;
            d.relocs.push(Rela {
                offset: u64_at(image, r)?,
                kind: info as u32,
                sym: (info >> 32) as u32,
                addend: u64_at(image, r + 16)? as i64,
            });
            r += 24;
        }
    }

    ensure!(hash != 0, "no DT_HASH");
    let nsyms = u32_at(image, hash + 4)? as u64;
    for i in 0..nsyms {
        let s = symtab + i * syment;
        let name_off = strtab + u32_at(image, s)? as u64;
        let end = image[name_off as usize..].iter().position(|&c| c == 0).unwrap_or(0);
        d.symbols.push(Sym {
            name: String::from_utf8_lossy(&image[name_off as usize..name_off as usize + end]).into_owned(),
            info: image[s as usize + 4],
            shndx: u16::from_le_bytes([image[s as usize + 6], image[s as usize + 7]]),
            value: u64_at(image, s + 8)?,
        });
    }
    Ok(d)
}

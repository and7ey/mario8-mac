// Mush8 - nca.rs
// Constantin Clerc (c22dev) - 0.1
use crate::crypto::{self, Key128};
use crate::keys::Keys;
use crate::storage::{DynStorage, Storage, Sub};
use aes::Aes128;
use anyhow::{Context, Result, bail, ensure};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Program,
    Meta,
    Control,
    Manual,
    Data,
    PublicData,
    Unknown(u8),
}

impl From<u8> for ContentType {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Program,
            1 => Self::Meta,
            2 => Self::Control,
            3 => Self::Manual,
            4 => Self::Data,
            5 => Self::PublicData,
            x => Self::Unknown(x),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    RomFs,
    PartitionFs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encryption {
    None,
    Xts,
    Ctr,
    CtrEx,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub index: usize,
    pub offset: u64,
    pub size: u64,
    pub fs_type: FsType,
    pub encryption: Encryption,
    pub header: Vec<u8>,
}

impl Section {
    fn ctr_hi(&self) -> u64 {
        u64::from_le_bytes(self.header[0x140..0x148].try_into().unwrap())
    }

    pub fn data_region(&self) -> (u64, u64) {
        let h = &self.header[8..];
        let rd = |o: usize| u64::from_le_bytes(h[o..o + 8].try_into().unwrap());
        match self.fs_type {
            FsType::PartitionFs => (rd(0x38), rd(0x40)),
            FsType::RomFs => (rd(0x10 + 5 * 0x18), rd(0x10 + 5 * 0x18 + 8)),
        }
    }

    fn patch_info(&self) -> (BktrHeader, BktrHeader) {
        let p = &self.header[0x100..0x140];
        (BktrHeader::parse(&p[..0x20]), BktrHeader::parse(&p[0x20..]))
    }
}

#[derive(Debug, Clone, Copy)]
struct BktrHeader {
    offset: u64,
    size: u64,
}

impl BktrHeader {
    fn parse(b: &[u8]) -> Self {
        Self {
            offset: u64::from_le_bytes(b[0..8].try_into().unwrap()),
            size: u64::from_le_bytes(b[8..16].try_into().unwrap()),
        }
    }
}

pub struct Nca {
    pub name: String,
    storage: DynStorage,
    pub content_type: ContentType,
    pub program_id: u64,
    pub key_generation: u8,
    pub rights_id: [u8; 16],
    pub sections: Vec<Section>,
    key: Option<Key128>,
}

pub type TitleKeys = HashMap<[u8; 16], Key128>;

pub fn ticket_title_key(keys: &Keys, tik: &[u8]) -> Result<([u8; 16], Key128)> {
    ensure!(tik.len() >= 0x2C0, "ticket too short");
    ensure!(&tik[0..4] == [4, 0, 1, 0], "unsupported ticket signature type");
    ensure!(tik[0x281] == 0, "personalized tickets are not supported");
    let rights: [u8; 16] = tik[0x2A0..0x2B0].try_into()?;
    let enc: [u8; 16] = tik[0x180..0x190].try_into()?;
    let generation = rights[15];
    let kek = keys.indexed::<16>("titlekek", generation.saturating_sub(1))?;
    Ok((rights, crypto::ecb_decrypt(&kek, &enc)))
}

impl Nca {
    pub fn open(name: &str, storage: DynStorage, keys: &Keys, title_keys: &TitleKeys) -> Result<Self> {
        let mut hdr = storage.read_vec(0, 0xC00)?;
        crypto::xts_decrypt(&mut hdr, &keys.get("header_key")?, 0x200, 0);
        ensure!(&hdr[0x200..0x204] == b"NCA3", "{name}: unsupported NCA version {:?}", &hdr[0x200..0x204]);

        let content_type = ContentType::from(hdr[0x205]);
        let kaek_index = hdr[0x207];
        let program_id = u64::from_le_bytes(hdr[0x210..0x218].try_into()?);
        let key_generation = hdr[0x206].max(hdr[0x220]);
        let mkey = key_generation.saturating_sub(1);
        let rights_id: [u8; 16] = hdr[0x230..0x240].try_into()?;

        let key = if rights_id != [0; 16] {
            Some(*title_keys.get(&rights_id).with_context(|| format!("{name}: no ticket for rights id {}", hex::encode(rights_id)))?)
        } else {
            let kind = ["application", "ocean", "system"].get(kaek_index as usize).context("bad key area index")?;
            let kaek = keys.indexed::<16>(&format!("key_area_key_{kind}"), mkey)?;
            let enc: [u8; 16] = hdr[0x320..0x330].try_into()?;
            Some(crypto::ecb_decrypt(&kaek, &enc))
        };

        let mut sections = Vec::new();
        for i in 0..4 {
            let e = &hdr[0x240 + i * 0x10..];
            let start = u32::from_le_bytes(e[0..4].try_into()?) as u64 * 0x200;
            let end = u32::from_le_bytes(e[4..8].try_into()?) as u64 * 0x200;
            if end == 0 {
                continue;
            }
            let fh = hdr[0x400 + i * 0x200..0x600 + i * 0x200].to_vec();
            let fs_type = match fh[2] {
                0 => FsType::RomFs,
                1 => FsType::PartitionFs,
                x => bail!("{name}: unknown fs type {x}"),
            };
            let encryption = match fh[4] {
                1 => Encryption::None,
                2 => Encryption::Xts,
                3 => Encryption::Ctr,
                4 => Encryption::CtrEx,
                x => bail!("{name}: unsupported encryption type {x}"),
            };
            sections.push(Section { index: i, offset: start, size: end - start, fs_type, encryption, header: fh });
        }

        Ok(Self { name: name.into(), storage, content_type, program_id, key_generation, rights_id, sections, key })
    }

    pub fn section(&self, index: usize) -> Option<&Section> {
        self.sections.iter().find(|s| s.index == index)
    }

    pub fn section_storage(&self, s: &Section) -> Result<DynStorage> {
        let raw: DynStorage = Arc::new(Sub::new(self.storage.clone(), s.offset, s.size));
        Ok(match s.encryption {
            Encryption::None => raw,
            Encryption::Ctr | Encryption::CtrEx => Arc::new(CtrStorage {
                inner: raw,
                cipher: crypto::aes(self.key.as_ref().context("no key")?),
                ctr_hi: s.ctr_hi(),
                abs_base: s.offset,
            }),
            Encryption::Xts => bail!("XTS sections are not supported"),
        })
    }

    pub fn fs_storage(&self, s: &Section) -> Result<DynStorage> {
        ensure!(s.encryption != Encryption::CtrEx, "{}: section {} is a patch; use patched_romfs", self.name, s.index);
        let (off, size) = s.data_region();
        Ok(Arc::new(Sub::new(self.section_storage(s)?, off, size)))
    }

    pub fn patched_romfs(&self, base: &Nca) -> Result<DynStorage> {
        let s = self.sections.iter().find(|s| s.encryption == Encryption::CtrEx).context("update has no BKTR section")?;
        let bs = base.sections.iter().find(|s| s.fs_type == FsType::RomFs).context("base has no RomFS")?;
        let tables = self.section_storage(s)?;
        let (ind, sub) = s.patch_info();
        let relocs = parse_relocations(&tables.read_vec(ind.offset, ind.size as usize)?)?;
        let subsections = parse_subsections(&tables.read_vec(sub.offset, sub.size as usize)?, sub.offset)?;

        let patch = Arc::new(CtrExStorage {
            inner: Arc::new(Sub::new(self.storage.clone(), s.offset, s.size)),
            cipher: crypto::aes(self.key.as_ref().context("no key")?),
            ctr_hi: s.ctr_hi(),
            abs_base: s.offset,
            subsections,
        });
        let virt: DynStorage = Arc::new(IndirectStorage { base: base.section_storage(bs)?, patch, relocs });
        let (off, size) = s.data_region();
        Ok(Arc::new(Sub::new(virt, off, size)))
    }
}

struct CtrStorage {
    inner: DynStorage,
    cipher: Aes128,
    ctr_hi: u64,
    abs_base: u64,
}

fn ctr_read(inner: &DynStorage, cipher: &Aes128, ctr_hi: u64, abs_base: u64, mut off: u64, mut buf: &mut [u8]) -> Result<()> {
    let partial = |off: u64, out: &mut [u8]| -> Result<()> {
        let start = off & !15;
        let lead = (off - start) as usize;
        let mut block = [0u8; 16];
        let avail = (inner.size() - start).min(16) as usize;
        inner.read_at(start, &mut block[..avail])?;
        crypto::ctr_apply(cipher, iv(ctr_hi, abs_base + start), &mut block);
        out.copy_from_slice(&block[lead..lead + out.len()]);
        Ok(())
    };
    if off % 16 != 0 {
        let n = ((16 - off % 16) as usize).min(buf.len());
        let (head, rest) = std::mem::take(&mut buf).split_at_mut(n);
        partial(off, head)?;
        buf = rest;
        off += n as u64;
    }
    let whole = buf.len() & !15;
    if whole > 0 {
        let (mid, rest) = std::mem::take(&mut buf).split_at_mut(whole);
        inner.read_at(off, mid)?;
        crypto::ctr_apply(cipher, iv(ctr_hi, abs_base + off), mid);
        buf = rest;
        off += whole as u64;
    }
    if !buf.is_empty() {
        partial(off, buf)?;
    }
    Ok(())
}

fn iv(ctr_hi: u64, abs_off: u64) -> u128 {
    ((ctr_hi as u128) << 64) | (abs_off >> 4) as u128
}

impl Storage for CtrStorage {
    fn size(&self) -> u64 {
        self.inner.size()
    }
    fn read_at(&self, off: u64, buf: &mut [u8]) -> Result<()> {
        ctr_read(&self.inner, &self.cipher, self.ctr_hi, self.abs_base, off, buf)
    }
}

#[derive(Debug, Clone, Copy)]
struct Subsection {
    offset: u64,
    ctr: Option<u32>,
}

struct CtrExStorage {
    inner: DynStorage,
    cipher: Aes128,
    ctr_hi: u64,
    abs_base: u64,
    subsections: Vec<Subsection>,
}

impl Storage for CtrExStorage {
    fn size(&self) -> u64 {
        self.inner.size()
    }
    fn read_at(&self, mut off: u64, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            let i = self.subsections.partition_point(|s| s.offset <= off).saturating_sub(1);
            let s = self.subsections[i];
            let end = self.subsections.get(i + 1).map_or(self.inner.size(), |n| n.offset);
            let n = ((end - off) as usize).min(buf.len());
            let hi = s.ctr.map_or(self.ctr_hi, |c| (self.ctr_hi & !0xFFFF_FFFF) | c as u64);
            let (head, rest) = buf.split_at_mut(n);
            ctr_read(&self.inner, &self.cipher, hi, self.abs_base, off, head)?;
            buf = rest;
            off += n as u64;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Relocation {
    virt: u64,
    phys: u64,
    is_patch: bool,
}

struct IndirectStorage {
    base: DynStorage,
    patch: DynStorage,
    relocs: (Vec<Relocation>, u64),
}

impl Storage for IndirectStorage {
    fn size(&self) -> u64 {
        self.relocs.1
    }
    fn read_at(&self, mut off: u64, mut buf: &mut [u8]) -> Result<()> {
        let (relocs, total) = (&self.relocs.0, self.relocs.1);
        while !buf.is_empty() {
            let i = relocs.partition_point(|r| r.virt <= off).checked_sub(1).context("read before first relocation")?;
            let r = relocs[i];
            let end = relocs.get(i + 1).map_or(total, |n| n.virt);
            ensure!(off < end, "read past end of patched storage");
            let n = ((end - off) as usize).min(buf.len());
            let src = if r.is_patch { &self.patch } else { &self.base };
            let (head, rest) = buf.split_at_mut(n);
            src.read_at(r.phys + (off - r.virt), head)?;
            buf = rest;
            off += n as u64;
        }
        Ok(())
    }
}

const BUCKET: usize = 0x4000;

fn bucket_tree(data: &[u8]) -> Result<(usize, u64)> {
    ensure!(data.len() >= BUCKET, "BKTR table too short");
    let n = u32::from_le_bytes(data[4..8].try_into()?) as usize;
    let total = u64::from_le_bytes(data[8..16].try_into()?);
    ensure!(data.len() >= BUCKET * (n + 1), "BKTR table truncated ({n} buckets)");
    Ok((n, total))
}

fn parse_relocations(data: &[u8]) -> Result<(Vec<Relocation>, u64)> {
    let (n, total) = bucket_tree(data)?;
    let mut out = Vec::new();
    for b in 0..n {
        let bk = &data[BUCKET * (b + 1)..BUCKET * (b + 2)];
        let cnt = u32::from_le_bytes(bk[4..8].try_into()?) as usize;
        for e in 0..cnt {
            let e = &bk[0x10 + e * 0x14..];
            out.push(Relocation {
                virt: u64::from_le_bytes(e[0..8].try_into()?),
                phys: u64::from_le_bytes(e[8..16].try_into()?),
                is_patch: u32::from_le_bytes(e[16..20].try_into()?) != 0,
            });
        }
    }
    Ok((out, total))
}

fn parse_subsections(data: &[u8], table_offset: u64) -> Result<Vec<Subsection>> {
    let (n, _) = bucket_tree(data)?;
    let mut out = Vec::new();
    for b in 0..n {
        let bk = &data[BUCKET * (b + 1)..BUCKET * (b + 2)];
        let cnt = u32::from_le_bytes(bk[4..8].try_into()?) as usize;
        for e in 0..cnt {
            let e = &bk[0x10 + e * 0x10..];
            out.push(Subsection {
                offset: u64::from_le_bytes(e[0..8].try_into()?),
                ctr: Some(u32::from_le_bytes(e[12..16].try_into()?)),
            });
        }
    }
    if out.last().is_none_or(|s| s.offset < table_offset) {
        out.push(Subsection { offset: table_offset, ctr: None });
    }
    Ok(out)
}

#[cfg(test)]
mod ctr_tests {
    use super::*;
    use crate::storage::Mem;

    #[test]
    fn unaligned_reads_match_full_decrypt() {
        let key = [7u8; 16];
        let cipher = crypto::aes(&key);
        let plain: Vec<u8> = (0..1000u32).map(|i| (i * 31 % 251) as u8).collect();
        let mut enc = plain.clone();
        crypto::ctr_apply(&cipher, iv(0x1234, 0x40), &mut enc);
        let inner: DynStorage = std::sync::Arc::new(Mem(enc));
        for (off, len) in [(0, 1000), (3, 10), (5, 100), (16, 32), (17, 1), (990, 10), (15, 2), (1, 999), (32, 0)] {
            let mut buf = vec![0u8; len];
            ctr_read(&inner, &cipher, 0x1234, 0x40, off as u64, &mut buf).unwrap();
            assert_eq!(buf, plain[off..off + len], "off {off} len {len}");
        }
    }
}

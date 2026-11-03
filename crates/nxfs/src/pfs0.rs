// Mush8 - pfs0.rs
// Constantin Clerc (c22dev) - 0.1
use crate::storage::{DynStorage, Sub};
use anyhow::{Result, bail};
use std::sync::Arc;

pub struct Entry {
    pub name: String,
    pub offset: u64,
    pub size: u64,
}

pub struct Pfs0 {
    storage: DynStorage,
    pub entries: Vec<Entry>,
}

impl Pfs0 {
    pub fn open(storage: DynStorage) -> Result<Self> {
        let hdr = storage.read_vec(0, 16)?;
        if &hdr[..4] != b"PFS0" {
            bail!("not a PFS0");
        }
        let n = u32::from_le_bytes(hdr[4..8].try_into()?) as usize;
        let st = u32::from_le_bytes(hdr[8..12].try_into()?) as usize;
        let table = storage.read_vec(16, n * 24 + st)?;
        let strs = &table[n * 24..];
        let data = (16 + n * 24 + st) as u64;
        let mut entries = Vec::with_capacity(n);
        for i in 0..n {
            let e = &table[i * 24..];
            let off = u64::from_le_bytes(e[0..8].try_into()?);
            let size = u64::from_le_bytes(e[8..16].try_into()?);
            let so = u32::from_le_bytes(e[16..20].try_into()?) as usize;
            let end = strs[so..].iter().position(|&c| c == 0).unwrap_or(strs.len() - so);
            let name = String::from_utf8_lossy(&strs[so..so + end]).into_owned();
            entries.push(Entry { name, offset: data + off, size });
        }
        Ok(Self { storage, entries })
    }

    pub fn file(&self, name: &str) -> Option<DynStorage> {
        self.entries.iter().find(|e| e.name == name).map(|e| self.storage_of(e))
    }

    pub fn storage_of(&self, e: &Entry) -> DynStorage {
        Arc::new(Sub::new(self.storage.clone(), e.offset, e.size))
    }
}

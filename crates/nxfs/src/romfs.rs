// Mush8 - romfs.rs
// Constantin Clerc (c22dev) - 0.1
use crate::storage::DynStorage;
use anyhow::{Result, ensure};

const NONE: u32 = u32::MAX;

pub struct File {
    pub path: String,
    pub offset: u64,
    pub size: u64,
}

pub struct RomFs {
    storage: DynStorage,
    data_offset: u64,
    dirs: Vec<u8>,
    files: Vec<u8>,
}

fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64_at(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}

impl RomFs {
    pub fn open(storage: DynStorage) -> Result<Self> {
        let h = storage.read_vec(0, 0x50)?;
        ensure!(u64_at(&h, 0) == 0x50, "bad RomFS header size");
        let dirs = storage.read_vec(u64_at(&h, 0x18), u64_at(&h, 0x20) as usize)?;
        let files = storage.read_vec(u64_at(&h, 0x38), u64_at(&h, 0x40) as usize)?;
        Ok(Self { storage, data_offset: u64_at(&h, 0x48), dirs, files })
    }

    pub fn storage(&self) -> &DynStorage {
        &self.storage
    }

    pub fn files(&self) -> Vec<File> {
        let mut out = Vec::new();
        self.walk(0, String::new(), &mut out);
        out
    }

    fn walk(&self, dir: u32, path: String, out: &mut Vec<File>) {
        let d = &self.dirs[dir as usize..];
        let mut f = u32_at(d, 0xC);
        while f != NONE {
            let e = &self.files[f as usize..];
            let len = u32_at(e, 0x1C) as usize;
            let name = String::from_utf8_lossy(&e[0x20..0x20 + len]);
            out.push(File { path: format!("{path}/{name}"), offset: self.data_offset + u64_at(e, 8), size: u64_at(e, 0x10) });
            f = u32_at(e, 4);
        }
        let mut c = u32_at(d, 8);
        while c != NONE {
            let e = &self.dirs[c as usize..];
            let len = u32_at(e, 0x14) as usize;
            let name = String::from_utf8_lossy(&e[0x18..0x18 + len]);
            self.walk(c, format!("{path}/{name}"), out);
            c = u32_at(e, 4);
        }
    }
}

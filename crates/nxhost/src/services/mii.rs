// Mush8 - mii.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Request, Response, Service, Stub, write_buffer};
use std::sync::{Arc, OnceLock};

const CHAR_INFO_SIZE: usize = 0x58;
const DEFAULT_COUNT: usize = 6;
const SOURCE_DATABASE: u32 = 1;
const SOURCE_DEFAULT: u32 = 2;
// CharInfoElement::Source tag (nn::mii::Source).
const TAG_DATABASE: u32 = 0;
const TAG_DEFAULT: u32 = 1;

pub static VERIFY: OnceLock<u64> = OnceLock::new();

mod f {
    pub const NAME: usize = 0x10;
    pub const FAVORITE_COLOR: usize = 0x27;
    pub const GENDER: usize = 0x28;
    pub const HEIGHT: usize = 0x29;
    pub const BUILD: usize = 0x2A;
    pub const FACELINE: usize = 0x2D;
    pub const HAIR: usize = 0x31;
    pub const EYE: usize = 0x34;
    pub const EYEBROW: usize = 0x3B;
    pub const NOSE: usize = 0x42;
    pub const MOUTH: usize = 0x45;
    pub const BEARD: usize = 0x4A;
    pub const GLASSES: usize = 0x4F;
    pub const MOLE: usize = 0x53;
}

fn default_mii(index: usize) -> [u8; CHAR_INFO_SIZE] {
    let mut c = [0u8; CHAR_INFO_SIZE];
    let mut id = *b"mk8dx-default-0\0";
    id[14] = b'0' + index as u8;
    id[6] = (id[6] & 0x0F) | 0x40;
    id[8] = (id[8] & 0x3F) | 0x80;
    c[..16].copy_from_slice(&id);
    for (i, ch) in "Mii".encode_utf16().enumerate() {
        c[f::NAME + 2 * i..f::NAME + 2 * i + 2].copy_from_slice(&ch.to_le_bytes());
    }
    let gender = (index % 2) as u8;
    c[f::FAVORITE_COLOR] = (index * 2 % 12) as u8;
    c[f::GENDER] = gender;
    c[f::HEIGHT] = 64;
    c[f::BUILD] = 64;
    c[f::FACELINE..f::FACELINE + 4].copy_from_slice(&[0, 0, 0, 0]);
    let hair = if gender == 0 { 33 } else { 12 };
    c[f::HAIR..f::HAIR + 3].copy_from_slice(&[hair, 1, 0]);
    c[f::EYE..f::EYE + 7].copy_from_slice(&[if gender == 0 { 2 } else { 4 }, 8, 4, 3, 4, 2, 12]);
    c[f::EYEBROW..f::EYEBROW + 7].copy_from_slice(&[if gender == 0 { 6 } else { 0 }, 1, 4, 3, 6, 2, 10]);
    c[f::NOSE..f::NOSE + 3].copy_from_slice(&[1, 4, 9]);
    c[f::MOUTH..f::MOUTH + 5].copy_from_slice(&[if gender == 0 { 23 } else { 1 }, 19, 4, 3, 13]);
    c[f::BEARD..f::BEARD + 5].copy_from_slice(&[1, 0, 0, 4, 10]);
    c[f::GLASSES..f::GLASSES + 4].copy_from_slice(&[0, 8, 4, 10]);
    c[f::MOLE..f::MOLE + 4].copy_from_slice(&[0, 4, 2, 20]);
    c
}

fn verify(c: &[u8; CHAR_INFO_SIZE]) -> u32 {
    let Some(&v) = VERIFY.get() else { return 0 };
    let f: extern "C" fn(*const u8) -> u32 = unsafe { std::mem::transmute(v as usize) };
    f(c.as_ptr())
}

fn defaults() -> &'static Vec<[u8; CHAR_INFO_SIZE]> {
    static D: OnceLock<Vec<[u8; CHAR_INFO_SIZE]>> = OnceLock::new();
    D.get_or_init(|| {
        let v: Vec<_> = (0..DEFAULT_COUNT).map(default_mii).collect();
        for (i, c) in v.iter().enumerate() {
            let r = verify(c);
            if r != 0 {
                eprintln!("[mii] default Mii {i} rejected by CharInfoRaw::Verify: code {r:#x}");
            }
        }
        v
    })
}

pub struct MiiService;

impl Service for MiiService {
    fn name(&self) -> String {
        "mii".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => rsp.objects.push(Arc::new(Database)),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

struct Database;

impl Database {
    /// Entries for a source flag, each tagged with the source it came from.
    ///
    /// The Database source is normally filled by the MiiEdit applet. There is no
    /// applet runtime here, so the synthetic Miis serve both sources; an empty
    /// Database list leaves the guest with an all-zero CharInfo (CreateId zero),
    /// which `CharInfoRaw::IsValid` rejects.
    fn listed(flag: u32) -> Vec<(u32, [u8; CHAR_INFO_SIZE])> {
        let mut v = Vec::new();
        if flag & SOURCE_DATABASE != 0 {
            v.extend(defaults().iter().map(|c| (TAG_DATABASE, *c)));
        }
        if flag & SOURCE_DEFAULT != 0 {
            v.extend(defaults().iter().map(|c| (TAG_DEFAULT, *c)));
        }
        v
    }
}

fn traced() -> bool {
    std::env::var_os("NX_MII_TRACE").is_some()
}

impl Service for Database {
    fn name(&self) -> String {
        "mii::IDatabaseService".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 | 1 => rsp.data.push(0),
            2 => {
                let n = Self::listed(req.u32(0)).len() as u32;
                if traced() {
                    eprintln!("[mii] count src {:#x} -> {n}", req.u32(0));
                }
                rsp.push_u32(n);
            }
            3 | 4 => {
                let flag = req.u32(0);
                let list = Self::listed(flag);
                let stride = if req.cmd == 3 { CHAR_INFO_SIZE + 4 } else { CHAR_INFO_SIZE };
                let mut out = Vec::with_capacity(list.len() * stride);
                for (src, c) in &list {
                    out.extend_from_slice(c);
                    if req.cmd == 3 {
                        out.extend_from_slice(&src.to_le_bytes());
                    }
                }
                let written = req.out_buffer().map_or(0, |b| write_buffer(&b, &out)) / stride;
                if traced() {
                    eprintln!("[mii] get cmd {} src {flag:#x} -> {written}/{}", req.cmd, list.len());
                }
                // Out<int> is the total list size, not the number copied.
                rsp.push_u32(list.len() as u32);
            }
            6 => {
                let gender = req.u32(4);
                let c = defaults()[if gender == 1 { 1 } else { 0 }];
                rsp.data.extend_from_slice(&c);
            }
            7 => {
                let i = (req.u32(0) as usize).min(DEFAULT_COUNT - 1);
                rsp.data.extend_from_slice(&defaults()[i]);
            }
            5 => rsp.data.extend_from_slice(&req.data[..CHAR_INFO_SIZE.min(req.data.len())]),
            21 => {
                // FindIndex by CreateId over every served source.
                let idx = req.data.get(..16).and_then(|key| {
                    defaults().iter().position(|c| c[..16] == *key).map(|i| i as u32)
                });
                if traced() {
                    eprintln!("[mii] getindex -> {:?}", idx);
                }
                rsp.push_u32(idx.unwrap_or(u32::MAX));
            }
            22 => {}
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

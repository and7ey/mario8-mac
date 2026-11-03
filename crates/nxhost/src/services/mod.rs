// Mush8 - mod.rs
// Constantin Clerc (c22dev) - 0.1
mod account;
mod aoc;
pub mod set;
mod audio;
pub mod am;
mod binder;
mod bsd;
mod friend;
pub mod fs;
pub mod hid;
#[cfg(not(target_os = "android"))]
mod lan;
#[cfg(target_os = "android")]
#[path = "lan_android.rs"]
mod lan;
mod lanplay;
mod lm;
pub mod mii;
mod nfp;
pub mod nifm;
mod pl;
mod time;
mod vi;
#[cfg(not(target_os = "android"))]
mod wireless;
#[cfg(target_os = "android")]
#[path = "wireless_android.rs"]
mod wireless;
pub mod vnet;

use crate::ipc::{Act, Request, Response, Service, Stub, Table};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Logs the first few calls of a service that has no implementation yet, so the
/// arguments (device name, allocation request) identify what the guest wants.
struct Probe(String);

static PROBED: Mutex<Option<HashMap<(String, u32), u32>>> = Mutex::new(None);

impl Service for Probe {
    fn name(&self) -> String {
        self.0.clone()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        let mut guard = PROBED.lock().unwrap();
        let map = guard.get_or_insert_with(HashMap::new);
        let seen = map.entry((self.0.clone(), req.cmd)).or_insert(0);
        if *seen < 6 {
            *seen += 1;
            let hex = |addr: u64, n: usize| -> String {
                if addr == 0 || n == 0 {
                    return String::new();
                }
                let s = unsafe { std::slice::from_raw_parts(addr as *const u8, n) };
                s.iter().map(|b| format!("{b:02x}")).collect()
            };
            let data = hex(req.data.as_ptr() as u64, req.data.len().min(0x40));
            let mut bufs = Vec::new();
            for (tag, list) in [("x", &req.x), ("a", &req.a), ("b", &req.b), ("c", &req.c)] {
                for b in list {
                    if b.addr != 0 && b.size != 0 {
                        let n = (b.size as usize).min(0x40);
                        bufs.push(format!("{tag}{:#x}[{}]", b.size, hex(b.addr, n)));
                    }
                }
            }
            eprintln!("[ipc] {} cmd {} data [{}] {}", self.0, req.cmd, data, bufs.join(" "));
        }
        rsp.data.resize(0x40, 0);
        0
    }
}

pub fn open(name: &str) -> Arc<dyn Service> {
    match name {
        "lm" => Arc::new(lm::Lm),
        "fsp-srv" => Arc::new(fs::FspSrv),
        "hid" => Arc::new(hid::Hid),
        "pl:u" | "pl:s" => Arc::new(pl::Pl),
        "audren:u" => Arc::new(audio::AudRen),
        "mii:e" | "mii:u" => Arc::new(mii::MiiService),
        "acc:u0" | "acc:u1" => Arc::new(account::Account),
        "time:u" | "time:a" | "time:s" => Arc::new(time::TimeService),
        "vi:u" | "vi:s" | "vi:m" => vi::open(name),
        "appletOE" => am::open(),
        "nifm:u" | "nifm:a" | "nifm:s" => nifm::open(),
        "bsd:u" => bsd::open("bsd:u"),
        "csrng" => Arc::new(Csrng),
        "bsd:s" => bsd::open("bsd:s"),
        "friend:u" | "friend:v" | "friend:m" | "friend:s" | "friend:a" => friend::open(),
        "aoc:u" => Arc::new(aoc::Aoc),
        "nfp:user" => nfp::open(),
        "set" => Arc::new(set::Settings),
        "pctl" | "pctl:s" | "pctl:a" | "pctl:r" => Arc::new(Table {
            name: "pctl",
            cmds: &[(0, Act::Obj(pctl_service)), (1, Act::Obj(pctl_service))],
        }),
        "apm" | "apm:p" => Arc::new(Table {
            name: "apm",
            cmds: &[(0, Act::Obj(|| Arc::new(Table { name: "apm::ISession", cmds: &[(0, Act::Ok), (1, Act::U32(0x0001_0001))] }))), (1, Act::U32With(|| vi::docked() as u32))],
        }),
        // Not implemented; the probe records what the guest asks for.
        "nvdrv" | "mm:u" => Arc::new(Probe(name.to_string())),
        _ => Arc::new(Stub(name.to_string())),
    }
}

fn pctl_service() -> Arc<dyn Service> {
    Arc::new(Table {
        name: "pctl::IParentalControlService",
        cmds: &[
            (1, Act::Ok),
            (1001, Act::Ok),
            (1013, Act::Ok),
            (1018, Act::U32(1)),
            (1031, Act::U32(0)),
            (1032, Act::U32(0)),
            (1403, Act::U32(0)),
            (1432, Act::Event(false)),
            (1456, Act::Event(false)),
            (1473, Act::Event(false)),
        ],
    })
}

struct Csrng;

impl Service for Csrng {
    fn name(&self) -> String {
        "csrng".into()
    }
    fn call(&self, req: &crate::ipc::Request, rsp: &mut crate::ipc::Response) -> u32 {
        match req.cmd {
            0 => {
                if let Some(b) = req.out_buffer() {
                    let out = unsafe { std::slice::from_raw_parts_mut(b.addr as *mut u8, b.size as usize) };
                    for chunk in out.chunks_mut(256) {
                        unsafe { libc::getentropy(chunk.as_mut_ptr() as *mut _, chunk.len()) };
                    }
                }
                0
            }
            _ => Stub(self.name()).call(req, rsp),
        }
    }
}

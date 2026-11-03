// Mush8 - nfp.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Act, Request, Response, Service, Stub, Table, new_event, write_buffer};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

const STATE_INITIALIZED: u32 = 1;
const DEVICE_INITIALIZED: u32 = 0;
const DEVICE_SEARCHING: u32 = 1;
const PLAYER_1: u64 = 0;

pub fn open() -> Arc<dyn Service> {
    Arc::new(Table { name: "nfp:user", cmds: &[(0, Act::Obj(|| Arc::new(User::new())))] })
}

struct User {
    state: AtomicU32,
    device: AtomicU32,
    activate: u32,
    deactivate: u32,
    availability: u32,
}

impl User {
    fn new() -> Self {
        User { state: AtomicU32::new(0), device: AtomicU32::new(DEVICE_INITIALIZED), activate: new_event(false), deactivate: new_event(false), availability: new_event(false) }
    }
}

impl Service for User {
    fn name(&self) -> String {
        "nfp::IUser".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => {
                eprintln!("[nfp] initialize");
                self.state.store(STATE_INITIALIZED, Relaxed);
            }
            1 => self.state.store(0, Relaxed),
            2 => {
                if let Some(b) = req.out_buffer() {
                    write_buffer(&b, &PLAYER_1.to_le_bytes());
                }
                rsp.push_u32(1);
            }
            3 => {
                eprintln!("[nfp] searching for an amiibo (none can be scanned here)");
                self.device.store(DEVICE_SEARCHING, Relaxed);
            }
            4 => self.device.store(DEVICE_INITIALIZED, Relaxed),
            17 => rsp.copy_handles.push(self.activate),
            18 => rsp.copy_handles.push(self.deactivate),
            19 => rsp.push_u32(self.state.load(Relaxed)),
            20 => rsp.push_u32(self.device.load(Relaxed)),
            21 => rsp.push_u32(req.u64(0) as u32),
            23 => rsp.copy_handles.push(self.availability),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

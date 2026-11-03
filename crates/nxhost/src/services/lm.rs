// Mush8 - lm.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Request, Response, Service, Stub};
use std::sync::Arc;

pub struct Lm;

impl Service for Lm {
    fn name(&self) -> String {
        "lm".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => {
                rsp.objects.push(Arc::new(Logger));
                0
            }
            _ => Stub(self.name()).call(req, rsp),
        }
    }
}

struct Logger;

impl Service for Logger {
    fn name(&self) -> String {
        "lm::ILogger".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => {
                for b in req.a.iter().chain(&req.x).filter(|b| b.size > 0) {
                    let p = unsafe { std::slice::from_raw_parts(b.addr as *const u8, b.size as usize) };
                    print_packet(p);
                }
                0
            }
            1 => 0,
            _ => Stub(self.name()).call(req, rsp),
        }
    }
}

fn print_packet(p: &[u8]) {
    if p.len() < 0x18 {
        return;
    }
    let severity = p[0x11];
    let mut i = 0x18;
    let mut text = String::new();
    let mut file = String::new();
    while i < p.len() {
        let key = p[i];
        i += 1;
        let (mut size, mut shift) = (0usize, 0);
        while i < p.len() {
            let b = p[i];
            i += 1;
            size |= ((b & 0x7F) as usize) << shift;
            shift += 7;
            if b & 0x80 == 0 {
                break;
            }
        }
        let v = &p[i..(i + size).min(p.len())];
        match key {
            2 => text += &String::from_utf8_lossy(v),
            4 => file = String::from_utf8_lossy(v).into_owned(),
            _ => {}
        }
        i += size;
    }
    if !text.is_empty() {
        let sev = ["trace", "info", "warn", "error", "fatal"].get(severity as usize).unwrap_or(&"?");
        let loc = if file.is_empty() { String::new() } else { format!(" ({file})") };
        eprintln!("[lm:{sev}] {}{loc}", text.trim_end());
    }
}

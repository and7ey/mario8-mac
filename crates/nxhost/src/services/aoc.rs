// Mush8 - aoc.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Act, Request, Response, Service, Stub, Table, new_event, write_buffer};
use std::sync::Arc;

const INVALID_BUFFER_SIZE: u32 = (200 << 9) | 166;
const NO_PURCHASED_PRODUCT_INFO: u32 = (400 << 9) | 164;

fn base_id() -> u64 {
    crate::svc::PROGRAM_ID + 0x1000
}

fn indices() -> Vec<u32> {
    let mut v: Vec<u32> = super::fs::CONFIG
        .get()
        .map(|c| c.dlc.iter().filter_map(|(id, _)| id.checked_sub(base_id()).map(|i| i as u32)).collect())
        .unwrap_or_default();
    v.sort();
    v
}

pub struct Aoc;

impl Aoc {
    fn list(req: &Request, rsp: &mut Response, start: usize, count: usize) -> u32 {
        let Some(b) = req.out_buffer() else { return INVALID_BUFFER_SIZE };
        if count > b.size as usize / 4 {
            return INVALID_BUFFER_SIZE;
        }
        let list: Vec<u32> = indices().into_iter().skip(start).take(count).collect();
        let bytes: Vec<u8> = list.iter().flat_map(|i| i.to_le_bytes()).collect();
        write_buffer(&b, &bytes);
        rsp.push_u32(list.len() as u32);
        0
    }
}

impl Service for Aoc {
    fn name(&self) -> String {
        "aoc:u".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 | 2 => rsp.push_u32(indices().len() as u32),
            3 => return Self::list(req, rsp, req.u32(0) as usize, req.u32(4) as usize),
            1 => return Self::list(req, rsp, req.u32(8) as usize, req.u32(12) as usize),
            4 | 5 => rsp.push_u64(base_id()),
            6 | 7 | 50 => {}
            8 | 10 => rsp.copy_handles.push(crate::ipc::new_event_named(false, "aoc")),
            9 => rsp.push_u64(0),
            100 | 101 => rsp.objects.push(Arc::new(Table {
                name: "ec::IPurchaseEventManager",
                cmds: &[
                    (0, Act::Ok),
                    (1, Act::Ok),
                    (2, Act::Event(false)),
                    (3, Act::Err(NO_PURCHASED_PRODUCT_INFO)),
                    (4, Act::Err(NO_PURCHASED_PRODUCT_INFO)),
                ],
            })),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

// Mush8 - account.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Request, Response, Service, Stub, write_buffer};
use crate::services::am::USER_ID;
use std::sync::Arc;

const NICKNAME: &str = "Player";

pub struct Account;

fn profile_base(out: &mut Vec<u8>) {
    out.extend_from_slice(&USER_ID);
    out.extend_from_slice(&0u64.to_le_bytes());
    let mut n = [0u8; 0x20];
    n[..NICKNAME.len()].copy_from_slice(NICKNAME.as_bytes());
    out.extend_from_slice(&n);
}

impl Service for Account {
    fn name(&self) -> String {
        "acc:u0".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => rsp.push_u32(1),
            1 => rsp.data.push((req.data.get(..16) == Some(&USER_ID)) as u8),
            2 | 3 | 131 | 141 => {
                if let Some(b) = req.out_buffer() {
                    let mut list = vec![0u8; b.size as usize];
                    list[..16].copy_from_slice(&USER_ID);
                    write_buffer(&b, &list);
                }
            }
            4 => rsp.data.extend_from_slice(&USER_ID),
            5 => rsp.objects.push(Arc::new(Profile)),
            50 => rsp.data.push(0),
            51 => rsp.data.extend_from_slice(&USER_ID),
            100 | 140 | 110 | 111 | 130 => {}
            101 => rsp.objects.push(Arc::new(ManagerForApplication)),
            150 => rsp.data.push(0),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

struct Profile;

impl Service for Profile {
    fn name(&self) -> String {
        "account::IProfile".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => {
                if let Some(b) = req.out_buffer() {
                    write_buffer(&b, &[0u8; 0x80]);
                }
                profile_base(&mut rsp.data);
            }
            1 => profile_base(&mut rsp.data),
            10 => rsp.push_u32(0),
            11 => rsp.push_u32(0),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

const NOT_LINKED: u32 = 124 | 341 << 9;

struct ManagerForApplication;

impl Service for ManagerForApplication {
    fn name(&self) -> String {
        "account::IManagerForApplication".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 | 2 | 3 | 130 | 170 => NOT_LINKED,
            1 => {
                rsp.push_u64(u64::from_le_bytes(USER_ID[..8].try_into().unwrap()));
                0
            }
            160 => 0,
            _ => Stub(self.name()).call(req, rsp),
        }
    }
}

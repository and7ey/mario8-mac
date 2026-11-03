// Mush8 - time.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Request, Response, Service, Stub, new_event, write_buffer};
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const SOURCE_ID: [u8; 16] = *b"nxhost-steady-01";

fn now_posix() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

fn steady_seconds() -> i64 {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_secs() as i64
}

fn time_point(out: &mut Vec<u8>) {
    out.extend_from_slice(&steady_seconds().to_le_bytes());
    out.extend_from_slice(&SOURCE_ID);
}

fn clock_context(out: &mut Vec<u8>) {
    out.extend_from_slice(&(now_posix() - steady_seconds()).to_le_bytes());
    time_point(out);
}

pub struct TimeService;

impl Service for TimeService {
    fn name(&self) -> String {
        "time:u".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 | 1 | 4 | 5 => rsp.objects.push(Arc::new(SystemClock)),
            2 => rsp.objects.push(Arc::new(SteadyClock)),
            3 => rsp.objects.push(Arc::new(TimeZone)),
            30 | 31 => rsp.copy_handles.push(crate::ipc::new_event_named(false, "time")),
            100 => rsp.data.push(1),
            200 => rsp.data.push(1),
            300 => rsp.push_u64((now_posix() - steady_seconds()) as u64),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

struct SystemClock;

impl Service for SystemClock {
    fn name(&self) -> String {
        "time::ISystemClock".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => rsp.push_u64(now_posix() as u64),
            1 | 3 => {}
            2 => clock_context(&mut rsp.data),
            4 => rsp.copy_handles.push(crate::ipc::new_event_named(false, "time-clock")),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

struct SteadyClock;

impl Service for SteadyClock {
    fn name(&self) -> String {
        "time::ISteadyClock".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => time_point(&mut rsp.data),
            2 | 200 => rsp.push_u64(0),
            100 => rsp.push_u64(now_posix() as u64),
            101 => rsp.data.push(0),
            102 => rsp.push_u32(0),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

struct TimeZone;

fn calendar(t: i64, out: &mut Vec<u8>) {
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&t, &mut tm) };
    out.extend_from_slice(&((tm.tm_year + 1900) as i16).to_le_bytes());
    out.extend([(tm.tm_mon + 1) as u8, tm.tm_mday as u8, tm.tm_hour as u8, tm.tm_min as u8, tm.tm_sec as u8, 0]);
    out.extend_from_slice(&(tm.tm_wday as u32).to_le_bytes());
    out.extend_from_slice(&(tm.tm_yday as u32).to_le_bytes());
    let mut name = [0u8; 8];
    let zone = if tm.tm_zone.is_null() { "UTC".into() } else { unsafe { std::ffi::CStr::from_ptr(tm.tm_zone) }.to_string_lossy().into_owned() };
    name[..zone.len().min(7)].copy_from_slice(&zone.as_bytes()[..zone.len().min(7)]);
    out.extend_from_slice(&name);
    out.extend_from_slice(&(tm.tm_isdst.max(0) as u32).to_le_bytes());
    out.extend_from_slice(&(tm.tm_gmtoff as i32).to_le_bytes());
}

impl Service for TimeZone {
    fn name(&self) -> String {
        "time::ITimeZoneService".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => {
                let mut n = [0u8; 0x24];
                n[..3].copy_from_slice(b"UTC");
                rsp.data.extend_from_slice(&n);
            }
            2 => rsp.push_u32(1),
            4 => {
                if let Some(b) = req.b.first() {
                    write_buffer(b, &vec![0u8; b.size as usize]);
                }
            }
            100 | 101 => calendar(req.u64(0) as i64, &mut rsp.data),
            201 | 202 => {
                let d = &req.data;
                let mut tm: libc::tm = unsafe { std::mem::zeroed() };
                tm.tm_year = i16::from_le_bytes([d[0], d[1]]) as i32 - 1900;
                tm.tm_mon = d[2] as i32 - 1;
                tm.tm_mday = d[3] as i32;
                tm.tm_hour = d[4] as i32;
                tm.tm_min = d[5] as i32;
                tm.tm_sec = d[6] as i32;
                tm.tm_isdst = -1;
                let t = unsafe { libc::mktime(&mut tm) };
                rsp.push_u32(1);
                if let Some(b) = req.out_buffer() {
                    write_buffer(&b, &(t as i64).to_le_bytes());
                }
            }
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}

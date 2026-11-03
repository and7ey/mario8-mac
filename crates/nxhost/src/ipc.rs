// Mush8 - ipc.rs
// Constantin Clerc (c22dev) - 0.1
use crate::kernel::{KERNEL, Obj};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

const SFCI: u32 = u32::from_le_bytes(*b"SFCI");
const SFCO: u32 = u32::from_le_bytes(*b"SFCO");

pub mod rc {
    pub const UNKNOWN_COMMAND: u32 = 0x1A60A;
    pub const SESSION_CLOSED: u32 = 0xF601;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Buffer {
    pub addr: u64,
    pub size: u64,
}

#[derive(Default, Debug)]
pub struct Request {
    pub cmd: u32,
    pub data: Vec<u8>,
    pub pid: Option<u64>,
    pub copy_handles: Vec<u32>,
    pub move_handles: Vec<u32>,
    pub x: Vec<Buffer>,
    pub a: Vec<Buffer>,
    pub b: Vec<Buffer>,
    pub w: Vec<Buffer>,
    pub c: Vec<Buffer>,
    pub objects: Vec<u32>,
    pub in_objects: InObjects,
}

#[derive(Default, Clone)]
pub struct InObjects(pub Vec<Arc<dyn Service>>);

impl std::fmt::Debug for InObjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter().map(|o| o.name())).finish()
    }
}

impl std::ops::Deref for InObjects {
    type Target = Vec<Arc<dyn Service>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Request {
    pub fn u32(&self, off: usize) -> u32 {
        self.data.get(off..off + 4).map_or(0, |b| u32::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn u64(&self, off: usize) -> u64 {
        self.data.get(off..off + 8).map_or(0, |b| u64::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn out_buffer(&self) -> Option<Buffer> {
        self.b.iter().chain(&self.c).find(|b| b.addr != 0 && b.size != 0).copied()
    }

    pub fn in_buffer(&self) -> Option<Buffer> {
        self.a.iter().chain(&self.x).find(|b| b.addr != 0 && b.size != 0).copied()
    }
}

#[derive(Default)]
pub struct Response {
    pub data: Vec<u8>,
    pub copy_handles: Vec<u32>,
    pub move_handles: Vec<u32>,
    pub objects: Vec<Arc<dyn Service>>,
}

impl Response {
    pub fn push_u32(&mut self, v: u32) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    pub fn push_u64(&mut self, v: u64) {
        while self.data.len() % 8 != 0 {
            self.data.push(0);
        }
        self.data.extend_from_slice(&v.to_le_bytes());
    }
}

pub trait Service: Send + Sync {
    fn name(&self) -> String;
    fn call(&self, req: &Request, rsp: &mut Response) -> u32;
    fn storage_bytes(&self) -> Option<Vec<u8>> {
        None
    }
}

pub enum Act {
    Obj(fn() -> Arc<dyn Service>),
    Event(bool),
    Data(&'static [u8]),
    U32(u32),
    U64(u64),
    U32With(fn() -> u32),
    Ok,
    Err(u32),
}

pub struct Table {
    pub name: &'static str,
    pub cmds: &'static [(u32, Act)],
}

impl Service for Table {
    fn name(&self) -> String {
        self.name.into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        let Some((_, act)) = self.cmds.iter().find(|(c, _)| *c == req.cmd) else {
            return Stub(self.name.into()).call(req, rsp);
        };
        match act {
            Act::Obj(f) => rsp.objects.push(f()),
            Act::Event(signaled) => rsp.copy_handles.push(new_event(*signaled)),
            Act::Data(d) => rsp.data.extend_from_slice(d),
            Act::U32(v) => rsp.push_u32(*v),
            Act::U64(v) => rsp.push_u64(*v),
            Act::U32With(f) => rsp.push_u32(f()),
            Act::Ok => {}
            Act::Err(r) => return *r,
        }
        0
    }
}

pub fn new_event(signaled: bool) -> u32 {
    new_event_named(signaled, "event")
}

pub fn new_event_named(signaled: bool, name: &'static str) -> u32 {
    let (_w, r) = crate::kernel::create_event_named(name);
    if signaled {
        if let Some(Obj::ReadableEvent(e)) = KERNEL.lock().get(r) {
            e.signaled.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
    r
}

pub fn write_buffer(b: &Buffer, data: &[u8]) -> usize {
    let n = data.len().min(b.size as usize);
    if n > 0 && b.addr != 0 {
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), b.addr as *mut u8, n) };
    }
    n
}

pub struct Stub(pub String);

static LOGGED: Mutex<Option<HashSet<(String, u32)>>> = Mutex::new(None);

impl Service for Stub {
    fn name(&self) -> String {
        self.0.clone()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        let mut l = LOGGED.lock().unwrap();
        if l.get_or_insert_with(HashSet::new).insert((self.0.clone(), req.cmd)) {
            eprintln!("[ipc] STUB {} cmd {} (in {} bytes, bufs x{} a{} b{} c{})", self.0, req.cmd, req.data.len(), req.x.len(), req.a.len(), req.b.len(), req.c.len());
        }
        rsp.data.resize(0x40, 0);
        0
    }
}

pub struct ServiceManager;

impl Service for ServiceManager {
    fn name(&self) -> String {
        "sm:".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => 0,
            1 => {
                let raw = req.u64(0).to_le_bytes();
                let name = String::from_utf8_lossy(&raw[..raw.iter().position(|&c| c == 0).unwrap_or(8)]).into_owned();
                eprintln!("[sm] GetServiceHandle {name}");
                rsp.objects.push(crate::services::open(&name));
                0
            }
            _ => Stub("sm:".into()).call(req, rsp),
        }
    }
}

struct Domain {
    objects: HashMap<u32, Arc<dyn Service>>,
    next: u32,
}

pub struct Session {
    root: Arc<dyn Service>,
    domain: Arc<Mutex<Option<Domain>>>,
}

impl Session {
    pub fn new(root: Arc<dyn Service>) -> Arc<Self> {
        Arc::new(Self { root, domain: Arc::new(Mutex::new(None)) })
    }

    pub fn name(&self) -> String {
        self.root.name()
    }
}

pub fn new_session_handle(svc: Arc<dyn Service>) -> u32 {
    KERNEL.lock().add(Obj::Session(Session::new(svc)))
}

fn trace() -> bool {
    static T: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *T.get_or_init(|| std::env::var_os("NX_TRACE_IPC").is_some())
}

fn rd(buf: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(buf[i * 4..i * 4 + 4].try_into().unwrap())
}

pub fn handle(session: &Arc<Session>, handle: u32, buf: &mut [u8]) -> bool {
    let w0 = rd(buf, 0);
    let w1 = rd(buf, 1);
    let kind = w0 & 0xFFFF;
    let (nx, na, nb, nw) = ((w0 >> 16) & 0xF, (w0 >> 20) & 0xF, (w0 >> 24) & 0xF, (w0 >> 28) & 0xF);
    let raw_words = (w1 & 0x3FF) as usize;
    let c_flags = (w1 >> 10) & 0xF;

    let mut req = Request::default();
    let mut i = 2;
    if w1 >> 31 != 0 {
        let hd = rd(buf, 2);
        i = 3;
        if hd & 1 != 0 {
            req.pid = Some(rd(buf, 3) as u64 | (rd(buf, 4) as u64) << 32);
            i += 2;
        }
        for _ in 0..(hd >> 1) & 0xF {
            req.copy_handles.push(rd(buf, i));
            i += 1;
        }
        for _ in 0..(hd >> 5) & 0xF {
            req.move_handles.push(rd(buf, i));
            i += 1;
        }
    }
    for _ in 0..nx {
        let (a, b) = (rd(buf, i), rd(buf, i + 1));
        let addr = b as u64 | (((a >> 12) & 0xF) as u64) << 32 | (((a >> 6) & 0x7) as u64) << 36;
        req.x.push(Buffer { addr, size: (a >> 16) as u64 });
        i += 2;
    }
    for (count, which) in [(na, 0), (nb, 1), (nw, 2)] {
        for _ in 0..count {
            let (s, a, f) = (rd(buf, i), rd(buf, i + 1), rd(buf, i + 2));
            if trace() {
                eprintln!("[ipc] buffer descriptor {s:08x} {a:08x} {f:08x}");
            }
            let addr = a as u64 | (((f >> 28) & 0xF) as u64) << 32 | (((f >> 2) & 0x7) as u64) << 36;
            let size = s as u64 | (((f >> 24) & 0xF) as u64) << 32;
            let b = Buffer { addr, size };
            match which {
                0 => req.a.push(b),
                1 => req.b.push(b),
                _ => req.w.push(b),
            }
            i += 3;
        }
    }
    let raw_start = (i * 4).next_multiple_of(16);
    let raw_end = i * 4 + raw_words * 4;
    if c_flags >= 2 {
        let mut ci = raw_end / 4;
        for _ in 0..(if c_flags == 2 { 1 } else { c_flags - 2 }) {
            let (lo, hi) = (rd(buf, ci), rd(buf, ci + 1));
            req.c.push(Buffer { addr: lo as u64 | ((hi & 0xFFFF) as u64) << 32, size: (hi >> 16) as u64 });
            ci += 2;
        }
    }

    let mut rsp = Response::default();
    let mut domain_obj: Option<u32> = None;
    let result;
    match kind {
        2 => {
            if trace() {
                eprintln!("[ipc] {} ({handle:#x}): close", session.name());
            }
            return false;
        }
        4 | 6 => {
            let mut r = raw_start;
            let mut target = session.root.clone();
            let is_domain = session.domain.lock().unwrap().is_some();
            if is_domain {
                let dcmd = buf[r];
                let nobj = buf[r + 1] as usize;
                let payload = u16::from_le_bytes([buf[r + 2], buf[r + 3]]) as usize;
                let obj = rd(buf, r / 4 + 1);
                let dom = session.domain.lock().unwrap();
                let d = dom.as_ref().unwrap();
                for k in 0..nobj {
                    let id = rd(buf, (r + 0x10 + payload) / 4 + k);
                    req.objects.push(id);
                    if let Some(o) = d.objects.get(&id) {
                        req.in_objects.0.push(o.clone());
                    }
                }
                if dcmd == 2 {
                    drop(dom);
                    session.domain.lock().unwrap().as_mut().unwrap().objects.remove(&obj);
                    write_reply(buf, 0, &rsp, Some(0), &[]);
                    return true;
                }
                match d.objects.get(&obj) {
                    Some(s) => target = s.clone(),
                    None => {
                        drop(dom);
                        write_reply(buf, rc::SESSION_CLOSED, &rsp, Some(0), &[]);
                        return true;
                    }
                }
                domain_obj = Some(obj);
                r += 0x10;
            }
            if rd(buf, r / 4) != SFCI {
                let words: Vec<String> = (0..16).map(|k| format!("{:08x}", rd(buf, k))).collect();
                eprintln!("[ipc] {}: bad CMIF magic {:#x} at {r:#x}; message {}", session.name(), rd(buf, r / 4), words.join(" "));
            }
            if !is_domain {
                let k = KERNEL.lock();
                for &h in req.move_handles.iter().chain(&req.copy_handles) {
                    if let Some(Obj::Session(s)) = k.get(h) {
                        req.in_objects.0.push(s.root.clone());
                    }
                }
            }
            req.cmd = rd(buf, r / 4 + 2);
            let data_start = r + 0x10;
            req.data = buf[data_start.min(raw_end)..raw_end.max(data_start)].to_vec();
            result = target.call(&req, &mut rsp);
            if trace() {
                eprintln!(
                    "[ipc] {} ({handle:#x}{}) cmd {} in {:02x?} -> {result:#x} out {:02x?} objects {}",
                    target.name(),
                    domain_obj.map(|o| format!(" obj {o}")).unwrap_or_default(),
                    req.cmd,
                    &req.data[..req.data.len().min(32)],
                    &rsp.data[..rsp.data.len().min(32)],
                    rsp.objects.len()
                );
            }
        }
        5 | 7 => {
            req.cmd = rd(buf, raw_start / 4 + 2);
            result = control(session, &req, &mut rsp);
            if trace() {
                eprintln!("[ipc] {} ({handle:#x}) control {} -> {result:#x}", session.name(), req.cmd);
            }
        }
        _ => {
            eprintln!("[ipc] {}: unsupported message type {kind}", session.name());
            result = rc::UNKNOWN_COMMAND;
        }
    }

    let mut out_objects = Vec::new();
    let objects = std::mem::take(&mut rsp.objects);
    if domain_obj.is_some() {
        let mut dom = session.domain.lock().unwrap();
        let d = dom.as_mut().unwrap();
        for o in objects {
            let id = d.next;
            d.next += 1;
            d.objects.insert(id, o);
            out_objects.push(id);
        }
    } else {
        for o in objects {
            rsp.move_handles.push(new_session_handle(o));
        }
    }
    write_reply(buf, result, &rsp, domain_obj.map(|_| out_objects.len() as u32), &out_objects);
    true
}

fn control(session: &Arc<Session>, req: &Request, rsp: &mut Response) -> u32 {
    match req.cmd {
        0 => {
            let mut d = session.domain.lock().unwrap();
            let mut objects = HashMap::new();
            objects.insert(1, session.root.clone());
            *d = Some(Domain { objects, next: 2 });
            rsp.push_u32(1);
            0
        }
        2 | 4 => {
            let clone = Arc::new(Session { root: session.root.clone(), domain: session.domain.clone() });
            rsp.move_handles.push(KERNEL.lock().add(Obj::Session(clone)));
            0
        }
        3 => {
            rsp.push_u32(0x8000);
            0
        }
        c => {
            eprintln!("[ipc] {}: control command {c}", session.name());
            rc::UNKNOWN_COMMAND
        }
    }
}

fn write_reply(buf: &mut [u8], result: u32, rsp: &Response, domain_out: Option<u32>, out_objects: &[u32]) {
    let mut words: Vec<u32> = Vec::new();
    let has_hd = !rsp.copy_handles.is_empty() || !rsp.move_handles.is_empty();
    words.push(0);
    words.push(0);
    if has_hd {
        words.push(((rsp.copy_handles.len() as u32) << 1) | ((rsp.move_handles.len() as u32) << 5));
        words.extend(&rsp.copy_handles);
        words.extend(&rsp.move_handles);
    }
    let header_words = words.len();
    while words.len() % 4 != 0 {
        words.push(0);
    }
    let pad = words.len() - header_words;
    if let Some(n) = domain_out {
        words.extend([n, 0, 0, 0]);
    }
    words.extend([SFCO, 0, result, 0]);
    let mut data = rsp.data.clone();
    while data.len() % 4 != 0 {
        data.push(0);
    }
    words.extend(data.chunks(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())));
    words.extend(out_objects);
    let raw = (words.len() - header_words - pad) + 4;
    words[1] = raw as u32 | if has_hd { 1 << 31 } else { 0 };
    for (i, w) in words.iter().enumerate() {
        if i * 4 + 4 <= buf.len() {
            buf[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
        }
    }
}

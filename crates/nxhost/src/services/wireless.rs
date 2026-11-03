// Mush8 - wireless.rs
// Constantin Clerc (c22dev) - 0.1
use crate::services::vnet::{self, Transport};
use block2::{DynBlock, RcBlock, StackBlock};
use objc2::runtime::Bool;
use dispatch2::{DispatchData, DispatchQueue, DispatchRetained};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char, c_void};
use std::net::Ipv4Addr;
use std::ptr::null;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

const SERVICE: &CStr = c"_mush8-mp._udp";
const BACKLOG: usize = 512;
const RETRY: Duration = Duration::from_secs(1);
const RESTART: Duration = Duration::from_secs(2);
const STUCK: Duration = Duration::from_secs(5);
const IDLE: Duration = Duration::from_secs(30);

const CONNECTION_WAITING: u32 = 1;
const CONNECTION_READY: u32 = 3;
const CONNECTION_FAILED: u32 = 4;
const CONNECTION_CANCELLED: u32 = 5;
const LISTENER_WAITING: u32 = 1;
const LISTENER_FAILED: u32 = 3;
const BROWSER_FAILED: u32 = 2;
const BROWSER_WAITING: u32 = 4;
const RESULT_ADDED: u64 = 0x02;
const RESULT_REMOVED: u64 = 0x04;
const RESULT_INTERFACE_ADDED: u64 = 0x08;

type Obj = *mut c_void;

#[link(name = "Network", kind = "framework")]
unsafe extern "C" {
    static _nw_parameters_configure_protocol_disable: Obj;
    static _nw_parameters_configure_protocol_default_configuration: Obj;
    static _nw_connection_send_idempotent_content: Obj;
    static _nw_content_context_default_message: Obj;
    fn nw_retain(obj: Obj) -> Obj;
    fn nw_release(obj: Obj);
    fn nw_parameters_create_secure_udp(dtls: Obj, udp: Obj) -> Obj;
    fn nw_parameters_set_include_peer_to_peer(parameters: Obj, include: bool);
    fn nw_advertise_descriptor_create_bonjour_service(name: *const c_char, kind: *const c_char, domain: *const c_char) -> Obj;
    fn nw_listener_create(parameters: Obj) -> Obj;
    fn nw_listener_set_queue(listener: Obj, queue: Obj);
    fn nw_listener_set_advertise_descriptor(listener: Obj, descriptor: Obj);
    fn nw_listener_set_state_changed_handler(listener: Obj, handler: &DynBlock<dyn Fn(u32, Obj)>);
    fn nw_listener_set_new_connection_handler(listener: Obj, handler: &DynBlock<dyn Fn(Obj)>);
    fn nw_listener_start(listener: Obj);
    fn nw_listener_cancel(listener: Obj);
    fn nw_browse_descriptor_create_bonjour_service(kind: *const c_char, domain: *const c_char) -> Obj;
    fn nw_browser_create(descriptor: Obj, parameters: Obj) -> Obj;
    fn nw_browser_set_queue(browser: Obj, queue: Obj);
    fn nw_browser_set_browse_results_changed_handler(browser: Obj, handler: &DynBlock<dyn Fn(Obj, Obj, Bool)>);
    fn nw_browser_set_state_changed_handler(browser: Obj, handler: &DynBlock<dyn Fn(u32, Obj)>);
    fn nw_browser_start(browser: Obj);
    fn nw_browser_cancel(browser: Obj);
    fn nw_browse_result_get_changes(old: Obj, new: Obj) -> u64;
    fn nw_browse_result_copy_endpoint(result: Obj) -> Obj;
    fn nw_endpoint_get_bonjour_service_name(endpoint: Obj) -> *const c_char;
    fn nw_endpoint_copy_address_string(endpoint: Obj) -> *mut c_char;
    fn nw_connection_create(endpoint: Obj, parameters: Obj) -> Obj;
    fn nw_connection_copy_endpoint(connection: Obj) -> Obj;
    fn nw_connection_set_queue(connection: Obj, queue: Obj);
    fn nw_connection_set_state_changed_handler(connection: Obj, handler: &DynBlock<dyn Fn(u32, Obj)>);
    fn nw_connection_set_viability_changed_handler(connection: Obj, handler: &DynBlock<dyn Fn(Bool)>);
    fn nw_connection_set_path_changed_handler(connection: Obj, handler: &DynBlock<dyn Fn(Obj)>);
    fn nw_connection_start(connection: Obj);
    fn nw_connection_cancel(connection: Obj);
    fn nw_connection_receive_message(connection: Obj, completion: &DynBlock<dyn Fn(Obj, Obj, Bool, Obj)>);
    fn nw_connection_send(connection: Obj, content: Obj, context: Obj, is_complete: bool, completion: Obj);
    fn nw_connection_copy_current_path(connection: Obj) -> Obj;
    fn nw_path_enumerate_interfaces(path: Obj, handler: &DynBlock<dyn Fn(Obj) -> Bool + '_>);
    fn nw_interface_get_name(interface: Obj) -> *const c_char;
    fn nw_error_get_error_code(error: Obj) -> i32;
}

#[derive(Clone, Copy, PartialEq)]
struct Nw(Obj);

unsafe impl Send for Nw {}

struct Wireless;

static PENDING: AtomicUsize = AtomicUsize::new(0);

impl Transport for Wireless {
    fn send(&self, packet: &[u8]) {
        if PENDING.fetch_add(1, Relaxed) >= BACKLOG {
            PENDING.fetch_sub(1, Relaxed);
            return;
        }
        let packet = packet.to_vec();
        queue().exec_async(move || {
            PENDING.fetch_sub(1, Relaxed);
            with(|s| s.send(&packet));
        });
    }
}

pub fn start(address: [u8; 4]) -> Option<Box<dyn Transport>> {
    let me = format!("{} {}", host(), Ipv4Addr::from(address));
    eprintln!("[wireless] looking for nearby players as {me}");
    *STATE.lock().unwrap() = Some(State {
        me: CString::new(me).ok()?,
        links: HashMap::new(),
        owners: HashMap::new(),
        browsed: HashMap::new(),
        retry: Vec::new(),
        listener: None,
        browser: None,
        restart: None,
        next: 0,
    });
    queue().exec_async(|| _ = with(State::open));
    std::thread::Builder::new()
        .name("wireless".into())
        .spawn(|| loop {
            std::thread::sleep(Duration::from_secs(1));
            queue().exec_async(|| _ = with(State::tick));
        })
        .ok()?;
    Some(Box::new(Wireless))
}

fn queue() -> &'static DispatchRetained<DispatchQueue> {
    static QUEUE: OnceLock<DispatchRetained<DispatchQueue>> = OnceLock::new();
    QUEUE.get_or_init(|| DispatchQueue::new("mush8.wireless", None))
}

fn queue_ptr() -> Obj {
    DispatchRetained::as_ptr(queue()).as_ptr().cast()
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut State) -> R) -> Option<R> {
    STATE.lock().unwrap().as_mut().map(f)
}

fn host() -> String {
    let mut buf = [0u8; 256];
    unsafe { libc::gethostname(buf.as_mut_ptr().cast(), buf.len()) };
    let s = CStr::from_bytes_until_nul(&buf).map(|c| c.to_string_lossy().into_owned()).unwrap_or_default();
    s.trim_end_matches(".local").chars().take(40).collect()
}

fn params() -> Obj {
    unsafe {
        let p = nw_parameters_create_secure_udp(_nw_parameters_configure_protocol_disable, _nw_parameters_configure_protocol_default_configuration);
        nw_parameters_set_include_peer_to_peer(p, true);
        p
    }
}

fn error_code(error: Obj) -> i32 {
    if error.is_null() { 0 } else { unsafe { nw_error_get_error_code(error) } }
}

fn interface(connection: Obj) -> String {
    let name = RefCell::new(String::from("?"));
    unsafe {
        let path = nw_connection_copy_current_path(connection);
        if path.is_null() {
            return name.into_inner();
        }
        let block = StackBlock::new(|i: Obj| {
            *name.borrow_mut() = CStr::from_ptr(nw_interface_get_name(i)).to_string_lossy().into_owned();
            Bool::NO
        });
        nw_path_enumerate_interfaces(path, &block);
        nw_release(path);
    }
    name.into_inner()
}

struct Link {
    conn: Nw,
    label: String,
    name: Option<String>,
    up: bool,
    viable: bool,
    iface: String,
    owner: Option<[u8; 4]>,
    heard: Instant,
    since: Instant,
}

impl Link {
    fn usable(&self) -> bool {
        self.up && self.viable
    }
}

struct State {
    me: CString,
    links: HashMap<u64, Link>,
    owners: HashMap<[u8; 4], u64>,
    browsed: HashMap<String, Nw>,
    retry: Vec<(Instant, String)>,
    listener: Option<Nw>,
    browser: Option<Nw>,
    restart: Option<Instant>,
    next: u64,
}

impl State {
    fn open(&mut self) {
        self.restart = None;
        unsafe {
            if self.listener.is_none() {
                let p = params();
                let l = nw_listener_create(p);
                nw_release(p);
                if l.is_null() {
                    eprintln!("[wireless] cannot advertise this device");
                    self.restart = Some(Instant::now() + RESTART);
                } else {
                    let ad = nw_advertise_descriptor_create_bonjour_service(self.me.as_ptr(), SERVICE.as_ptr(), null());
                    nw_listener_set_advertise_descriptor(l, ad);
                    nw_release(ad);
                    nw_listener_set_queue(l, queue_ptr());
                    nw_listener_set_state_changed_handler(
                        l,
                        &RcBlock::new(|state: u32, error: Obj| match state {
                            LISTENER_FAILED => _ = with(|s| s.reopen(false, error)),
                            LISTENER_WAITING => eprintln!("[wireless] cannot advertise this device yet (error {})", error_code(error)),
                            _ => {}
                        }),
                    );
                    nw_listener_set_new_connection_handler(l, &RcBlock::new(|c: Obj| _ = with(|s| s.accept(c))));
                    nw_listener_start(l);
                    self.listener = Some(Nw(l));
                }
            }
            if self.browser.is_none() {
                let d = nw_browse_descriptor_create_bonjour_service(SERVICE.as_ptr(), null());
                let p = params();
                let b = nw_browser_create(d, p);
                nw_release(d);
                nw_release(p);
                nw_browser_set_queue(b, queue_ptr());
                nw_browser_set_browse_results_changed_handler(b, &RcBlock::new(|old: Obj, new: Obj, _: Bool| _ = with(|s| s.browsed(old, new))));
                nw_browser_set_state_changed_handler(
                    b,
                    &RcBlock::new(|state: u32, error: Obj| match state {
                        BROWSER_FAILED => _ = with(|s| s.reopen(true, error)),
                        BROWSER_WAITING => eprintln!("[wireless] cannot look for nearby players yet (error {})", error_code(error)),
                        _ => {}
                    }),
                );
                nw_browser_start(b);
                self.browser = Some(Nw(b));
            }
        }
    }

    fn reopen(&mut self, browser: bool, error: Obj) {
        let what = if browser { "look for nearby players" } else { "advertise this device" };
        eprintln!("[wireless] cannot {what} (error {}), retrying", error_code(error));
        unsafe {
            if browser {
                if let Some(b) = self.browser.take() {
                    nw_browser_cancel(b.0);
                    nw_release(b.0);
                }
            } else if let Some(l) = self.listener.take() {
                nw_listener_cancel(l.0);
                nw_release(l.0);
            }
        }
        self.restart = Some(Instant::now() + RESTART);
    }

    fn browsed(&mut self, old: Obj, new: Obj) {
        let changes = unsafe { nw_browse_result_get_changes(old, new) };
        let (result, seen) = if changes & (RESULT_ADDED | RESULT_INTERFACE_ADDED) != 0 && !new.is_null() {
            (new, true)
        } else if changes & RESULT_REMOVED != 0 && !old.is_null() {
            (old, false)
        } else {
            return;
        };
        let endpoint = unsafe { nw_browse_result_copy_endpoint(result) };
        if endpoint.is_null() {
            return;
        }
        let name = unsafe { CStr::from_ptr(nw_endpoint_get_bonjour_service_name(endpoint)) };
        if name == self.me.as_c_str() {
            unsafe { nw_release(endpoint) };
            return;
        }
        let name = name.to_string_lossy().into_owned();
        if let Some(e) = self.browsed.remove(&name) {
            unsafe { nw_release(e.0) };
        }
        if seen {
            self.browsed.insert(name.clone(), Nw(endpoint));
            let outbound = self.links.iter().find(|(_, l)| l.name.as_ref() == Some(&name)).map(|(&id, l)| (id, l.usable()));
            match outbound {
                Some((_, true)) => {}
                Some((id, false)) => {
                    self.remove(id);
                    self.connect(&name);
                }
                None => self.connect(&name),
            }
        } else {
            unsafe { nw_release(endpoint) };
        }
    }

    fn connect(&mut self, name: &str) {
        self.retry.retain(|(_, n)| n != name);
        let Some(&endpoint) = self.browsed.get(name) else { return };
        let p = params();
        let c = unsafe { nw_connection_create(endpoint.0, p) };
        unsafe { nw_release(p) };
        if !c.is_null() {
            self.add(c, name.to_string(), Some(name.to_string()));
        }
    }

    fn accept(&mut self, c: Obj) {
        let label = unsafe {
            let e = nw_connection_copy_endpoint(c);
            let a = if e.is_null() { std::ptr::null_mut() } else { nw_endpoint_copy_address_string(e) };
            let label = if a.is_null() { "?".to_string() } else { CStr::from_ptr(a).to_string_lossy().into_owned() };
            if !a.is_null() {
                libc::free(a.cast());
            }
            if !e.is_null() {
                nw_release(e);
            }
            label
        };
        self.add(unsafe { nw_retain(c) }, format!("{label} (incoming)"), None);
    }

    fn add(&mut self, c: Obj, label: String, name: Option<String>) {
        let id = self.next;
        self.next += 1;
        let conn = Nw(c);
        unsafe {
            nw_connection_set_queue(c, queue_ptr());
            nw_connection_set_state_changed_handler(
                c,
                &RcBlock::new(move |state: u32, _: Obj| {
                    if state == CONNECTION_CANCELLED {
                        nw_release(conn.0);
                    } else {
                        with(|s| s.state(id, state));
                    }
                }),
            );
            nw_connection_set_viability_changed_handler(c, &RcBlock::new(move |v: Bool| _ = with(|s| s.viability(id, v.as_bool()))));
            nw_connection_set_path_changed_handler(c, &RcBlock::new(move |_: Obj| _ = with(|s| s.path(id))));
            nw_connection_start(c);
        }
        let now = Instant::now();
        self.links.insert(id, Link { conn, label, name, up: false, viable: true, iface: String::new(), owner: None, heard: now, since: now });
        receive(id, conn);
    }

    fn remove(&mut self, id: u64) {
        let Some(l) = self.links.remove(&id) else { return };
        if l.usable() {
            eprintln!("[wireless] {} lost", l.label);
        }
        self.owners.retain(|_, o| *o != id);
        unsafe { nw_connection_cancel(l.conn.0) };
        if let Some(n) = l.name
            && self.browsed.contains_key(&n)
        {
            self.retry.push((Instant::now() + RETRY, n));
        }
    }

    fn state(&mut self, id: u64, state: u32) {
        match state {
            CONNECTION_FAILED => self.remove(id),
            CONNECTION_READY | CONNECTION_WAITING => {
                let Some(l) = self.links.get_mut(&id) else { return };
                let up = state == CONNECTION_READY;
                if up == l.up {
                    return;
                }
                l.up = up;
                l.viable = true;
                l.since = Instant::now();
                if up {
                    l.iface = interface(l.conn.0);
                    eprintln!("[wireless] reaching {} via {}", l.label, l.iface);
                } else {
                    eprintln!("[wireless] {} lost", l.label);
                }
            }
            _ => {}
        }
    }

    fn viability(&mut self, id: u64, viable: bool) {
        let Some(l) = self.links.get_mut(&id) else { return };
        if !l.up || l.viable == viable {
            return;
        }
        l.viable = viable;
        l.since = Instant::now();
        if viable {
            l.iface = interface(l.conn.0);
            eprintln!("[wireless] reaching {} via {}", l.label, l.iface);
        } else {
            eprintln!("[wireless] {} lost", l.label);
        }
    }

    fn path(&mut self, id: u64) {
        let Some(l) = self.links.get_mut(&id) else { return };
        if !l.usable() {
            return;
        }
        let iface = interface(l.conn.0);
        if iface != l.iface {
            eprintln!("[wireless] reaching {} via {iface}", l.label);
            l.iface = iface;
        }
    }

    fn received(&mut self, id: u64, packet: &[u8]) -> bool {
        let Some(l) = self.links.get_mut(&id) else { return false };
        l.heard = Instant::now();
        if packet.len() >= 20 {
            let src: [u8; 4] = packet[12..16].try_into().unwrap();
            if src[..2] == vnet::GATEWAY[..2] {
                l.owner = Some(src);
                self.owners.insert(src, id);
            }
        }
        true
    }

    fn send(&self, packet: &[u8]) {
        if packet.len() < 20 {
            return;
        }
        let dst: [u8; 4] = packet[16..20].try_into().unwrap();
        let owner = |a: &[u8; 4]| self.owners.get(a).and_then(|id| self.links.get(id)).filter(|l| l.usable());
        let targets: Vec<Nw> = match owner(&dst) {
            Some(l) => vec![l.conn],
            None => self
                .links
                .iter()
                .filter(|(id, l)| l.usable() && l.owner.is_none_or(|o| self.owners.get(&o) == Some(id) || owner(&o).is_none()))
                .map(|(_, l)| l.conn)
                .collect(),
        };
        if targets.is_empty() {
            return;
        }
        let data = DispatchData::from_bytes(packet);
        let content: Obj = DispatchRetained::as_ptr(&data).as_ptr().cast();
        for c in targets {
            unsafe { nw_connection_send(c.0, content, _nw_content_context_default_message, true, _nw_connection_send_idempotent_content) };
        }
    }

    fn tick(&mut self) {
        let now = Instant::now();
        if self.restart.is_some_and(|t| t <= now) {
            self.open();
        }
        let due: Vec<String> = self.retry.iter().filter(|(t, _)| *t <= now).map(|(_, n)| n.clone()).collect();
        self.retry.retain(|(t, _)| *t > now);
        let stale: Vec<u64> = self
            .links
            .iter()
            .filter(|(_, l)| {
                let browsed = l.name.as_ref().is_some_and(|n| self.browsed.contains_key(n));
                (browsed && !l.usable() && l.since.elapsed() > STUCK) || (!browsed && l.heard.elapsed() > IDLE)
            })
            .map(|(&id, _)| id)
            .collect();
        for id in stale {
            self.remove(id);
        }
        for n in due {
            if !self.links.values().any(|l| l.name.as_ref() == Some(&n)) {
                self.connect(&n);
            }
        }
    }
}

fn receive(id: u64, conn: Nw) {
    let block = RcBlock::new(move |content: Obj, _: Obj, _: Bool, error: Obj| {
        let packet = (!content.is_null()).then(|| unsafe { &*(content as *const DispatchData) }.to_vec());
        let alive = with(|s| match &packet {
            Some(p) => s.received(id, p),
            None => s.links.contains_key(&id),
        })
        .unwrap_or(false);
        if let Some(p) = &packet {
            vnet::receive(p);
        }
        if !alive {
            return;
        }
        if error.is_null() {
            receive(id, conn);
        } else {
            with(|s| s.remove(id));
        }
    });
    unsafe { nw_connection_receive_message(conn.0, &block) };
}

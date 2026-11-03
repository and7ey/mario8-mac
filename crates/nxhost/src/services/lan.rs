// Mush8 - lan.rs
// Constantin Clerc (c22dev) - 0.1
use crate::dnssd::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char, c_void};
use std::sync::{Mutex, OnceLock};

const TYPE: &str = "_mush8._tcp";
const PORT: u16 = 21420;

#[derive(Clone, Copy, PartialEq)]
struct Peer {
    ip: [u8; 4],
    hears_broadcast: bool,
}

fn peers() -> &'static Mutex<HashMap<String, Peer>> {
    static PEERS: OnceLock<Mutex<HashMap<String, Peer>>> = OnceLock::new();
    PEERS.get_or_init(Default::default)
}

static OURS: OnceLock<String> = OnceLock::new();
static OWN: OnceLock<([u8; 4], [u8; 4])> = OnceLock::new();

fn on_our_lan(ip: [u8; 4]) -> bool {
    OWN.get().is_some_and(|&(own, mask)| ip != own && (0..4).all(|i| ip[i] & mask[i] == own[i] & mask[i]))
}

pub fn unicast_targets(broadcast_sent: bool) -> Vec<[u8; 4]> {
    let mut all: Vec<[u8; 4]> = peers().lock().unwrap().values().filter(|p| !broadcast_sent || !p.hears_broadcast).map(|p| p.ip).collect();
    all.sort_unstable();
    all.dedup();
    all
}

pub fn start(own: [u8; 4], mask: [u8; 4], hears_broadcast: bool) {
    let _ = OWN.set((own, mask));
    let _ = std::thread::Builder::new().name("lan".into()).spawn(move || {
        loop {
            advertise_and_browse(own, hears_broadcast);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn advertise_and_browse(own: [u8; 4], hears_broadcast: bool) {
    let name = format!("mush8-{:x}-{}", std::process::id(), own.map(|b| format!("{b:02x}")).concat());
    let _ = OURS.set(name.clone());
    let (name, typ) = (CString::new(name).unwrap(), CString::new(TYPE).unwrap());
    let txt: &[u8] = if hears_broadcast { b"\x04bc=1" } else { b"\x04bc=0" };
    unsafe {
        let mut shared: DNSServiceRef = std::ptr::null_mut();
        if DNSServiceCreateConnection(&mut shared) != 0 {
            eprintln!("[lan] bonjour unavailable");
            return;
        }
        let (mut reg, mut browse) = (shared, shared);
        let mut r = DNSServiceRegister(&mut reg, SHARE, 0, name.as_ptr(), typ.as_ptr(), std::ptr::null(), std::ptr::null(), PORT.to_be(), txt.len() as u16, txt.as_ptr(), None, std::ptr::null_mut());
        if r == 0 {
            r = DNSServiceBrowse(&mut browse, SHARE, 0, typ.as_ptr(), std::ptr::null(), Some(on_browse), std::ptr::null_mut());
        }
        if r != 0 {
            eprintln!("[lan] bonjour did not start ({r}){}", denied(r));
            DNSServiceRefDeallocate(shared);
            std::thread::sleep(std::time::Duration::from_secs(4));
            return;
        }
        eprintln!("[lan] looking for other Mush8 ({TYPE})");
        let mut r = 0;
        while wait(shared, -1) {
            r = DNSServiceProcessResult(shared);
            if r != 0 {
                break;
            }
        }
        DNSServiceRefDeallocate(shared);
        eprintln!("[lan] bonjour connection lost ({r}), reconnecting");
    }
}

unsafe extern "C" fn on_browse(_: DNSServiceRef, flags: u32, iface: u32, err: i32, name: *const c_char, regtype: *const c_char, domain: *const c_char, _: *mut c_void) {
    if err != 0 {
        eprintln!("[lan] bonjour browse error {err}{}", denied(err));
        return;
    }
    if name.is_null() || regtype.is_null() || domain.is_null() {
        return;
    }
    let (name, regtype, domain) = unsafe { (CStr::from_ptr(name).to_owned(), CStr::from_ptr(regtype).to_owned(), CStr::from_ptr(domain).to_owned()) };
    let key = name.to_string_lossy().into_owned();
    if OURS.get() == Some(&key) {
        return;
    }
    if flags & ADD == 0 {
        if peers().lock().unwrap().remove(&key).is_some() {
            eprintln!("[lan] {key} left");
        }
        return;
    }
    std::thread::spawn(move || {
        if let Some(peer) = resolve(&name, &regtype, &domain, iface) {
            let mut all = peers().lock().unwrap();
            if all.get(&key) != Some(&peer) {
                let ip = peer.ip;
                eprintln!("[lan] found {key} at {}.{}.{}.{}{}", ip[0], ip[1], ip[2], ip[3], if peer.hears_broadcast { "" } else { " (gets copies of broadcasts)" });
                all.insert(key, peer);
            }
        }
    });
}

fn resolve(name: &CStr, regtype: &CStr, domain: &CStr, iface: u32) -> Option<Peer> {
    let r = crate::dnssd::resolve(name, regtype, domain, iface)?;
    on_our_lan(r.ip).then(|| Peer { ip: r.ip, hears_broadcast: r.txt.windows(4).any(|w| w == b"bc=1") })
}

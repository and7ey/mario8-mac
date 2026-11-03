// Mush8 - dnssd.rs
// Constantin Clerc (c22dev) - 0.1
use std::ffi::{CStr, CString, c_char, c_void};

pub const SHARE: u32 = 0x4000;
pub const ADD: u32 = 0x2;
pub const POLICY_DENIED: i32 = -65570;
const IPV4: u32 = 1;

#[repr(C)]
pub struct OpaqueDNS(u8);
pub type DNSServiceRef = *mut OpaqueDNS;

pub type BrowseReply = unsafe extern "C" fn(DNSServiceRef, u32, u32, i32, *const c_char, *const c_char, *const c_char, *mut c_void);

unsafe extern "C" {
    pub fn DNSServiceCreateConnection(sd: *mut DNSServiceRef) -> i32;
    pub fn DNSServiceRegister(sd: *mut DNSServiceRef, flags: u32, iface: u32, name: *const c_char, regtype: *const c_char, domain: *const c_char, host: *const c_char, port: u16, txt_len: u16, txt: *const u8, callback: Option<unsafe extern "C" fn()>, ctx: *mut c_void) -> i32;
    pub fn DNSServiceBrowse(sd: *mut DNSServiceRef, flags: u32, iface: u32, regtype: *const c_char, domain: *const c_char, callback: Option<BrowseReply>, ctx: *mut c_void) -> i32;
    fn DNSServiceResolve(sd: *mut DNSServiceRef, flags: u32, iface: u32, name: *const c_char, regtype: *const c_char, domain: *const c_char, callback: Option<unsafe extern "C" fn(DNSServiceRef, u32, u32, i32, *const c_char, *const c_char, u16, u16, *const u8, *mut c_void)>, ctx: *mut c_void) -> i32;
    fn DNSServiceGetAddrInfo(sd: *mut DNSServiceRef, flags: u32, iface: u32, protocol: u32, hostname: *const c_char, callback: Option<unsafe extern "C" fn(DNSServiceRef, u32, u32, i32, *const c_char, *const libc::sockaddr, u32, *mut c_void)>, ctx: *mut c_void) -> i32;
    pub fn DNSServiceProcessResult(sd: DNSServiceRef) -> i32;
    fn DNSServiceRefSockFD(sd: DNSServiceRef) -> i32;
    pub fn DNSServiceRefDeallocate(sd: DNSServiceRef);
}

pub fn denied(code: i32) -> &'static str {
    if code == POLICY_DENIED { ": local network access is off for Mush8 (Settings → Privacy & Security → Local Network)" } else { "" }
}

pub fn wait(sd: DNSServiceRef, timeout_ms: i32) -> bool {
    let mut p = libc::pollfd { fd: unsafe { DNSServiceRefSockFD(sd) }, events: libc::POLLIN, revents: 0 };
    unsafe { libc::poll(&mut p, 1, timeout_ms) > 0 }
}

pub struct Resolved {
    pub ip: [u8; 4],
    #[cfg_attr(not(target_os = "ios"), allow(dead_code))]
    pub port: u16,
    pub txt: Vec<u8>,
}

#[derive(Default)]
struct Found {
    host: Option<CString>,
    port: u16,
    txt: Vec<u8>,
    ip: Option<[u8; 4]>,
}

pub fn resolve(name: &CStr, regtype: &CStr, domain: &CStr, iface: u32) -> Option<Resolved> {
    let mut found = Found::default();
    let ctx = &mut found as *mut Found as *mut c_void;
    unsafe {
        let mut sd: DNSServiceRef = std::ptr::null_mut();
        if DNSServiceResolve(&mut sd, 0, iface, name.as_ptr(), regtype.as_ptr(), domain.as_ptr(), Some(on_resolve), ctx) != 0 {
            return None;
        }
        if wait(sd, 5000) {
            DNSServiceProcessResult(sd);
        }
        DNSServiceRefDeallocate(sd);
        let host = found.host.take()?;
        if DNSServiceGetAddrInfo(&mut sd, 0, iface, IPV4, host.as_ptr(), Some(on_addr), ctx) != 0 {
            return None;
        }
        if wait(sd, 5000) {
            DNSServiceProcessResult(sd);
        }
        DNSServiceRefDeallocate(sd);
    }
    Some(Resolved { ip: found.ip?, port: found.port, txt: found.txt })
}

unsafe extern "C" fn on_resolve(_: DNSServiceRef, _: u32, _: u32, err: i32, _: *const c_char, host: *const c_char, port: u16, txt_len: u16, txt: *const u8, ctx: *mut c_void) {
    let found = unsafe { &mut *(ctx as *mut Found) };
    if err == 0 && !host.is_null() {
        found.host = Some(unsafe { CStr::from_ptr(host) }.to_owned());
        found.port = u16::from_be(port);
        if !txt.is_null() {
            found.txt = unsafe { std::slice::from_raw_parts(txt, txt_len as usize) }.to_vec();
        }
    }
}

unsafe extern "C" fn on_addr(_: DNSServiceRef, flags: u32, _: u32, err: i32, _: *const c_char, addr: *const libc::sockaddr, _: u32, ctx: *mut c_void) {
    let found = unsafe { &mut *(ctx as *mut Found) };
    if err == 0 && flags & ADD != 0 && !addr.is_null() && unsafe { (*addr).sa_family } as i32 == libc::AF_INET {
        found.ip = Some(unsafe { (*(addr as *const libc::sockaddr_in)).sin_addr.s_addr.to_ne_bytes() });
    }
}

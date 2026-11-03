// Mush8 - guest.rs
// Constantin Clerc (c22dev) - 0.1
use nxabi::ctx;
use std::arch::global_asm;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

#[unsafe(no_mangle)]
pub static NX_CTX_SLOT: AtomicU64 = AtomicU64::new(0);

static KEY: AtomicU64 = AtomicU64::new(u64::MAX);

#[repr(C)]
pub struct GuestThread {
    pub x18: u64,
    pub tpidr: u64,
    pub tpidrro: u64,
    pub spill: [u64; 3],
    pub gate_stack_top: u64,
    pub host_sp: u64,
    pub handle: u32,
    pub site_lr: AtomicU64,
    pub site_what: AtomicU64,
    pub site_fp: AtomicU64,
    pub bt: [AtomicU64; 4],
    gate_stack: Vec<u8>,
}

pub struct Live(pub std::sync::Mutex<Vec<*mut GuestThread>>);
unsafe impl Send for Live {}
unsafe impl Sync for Live {}

pub static LIVE: LazyLock<Live> = LazyLock::new(|| Live(std::sync::Mutex::new(Vec::new())));

impl Drop for GuestThread {
    fn drop(&mut self) {
        let me = self as *mut GuestThread;
        LIVE.0.lock().unwrap().retain(|p| *p != me);
    }
}

pub fn record_site(lr: u64, what: u64) {
    if let Some(t) = try_current() {
        t.site_lr.store(lr, Ordering::Relaxed);
        t.site_what.store(what, Ordering::Relaxed);
    }
}

pub fn record_fp(fp: u64) {
    if let Some(t) = try_current() {
        t.site_fp.store(fp, Ordering::Relaxed);
        let chain = walk_fp(fp);
        for (i, slot) in t.bt.iter().enumerate() {
            slot.store(chain.get(i).copied().unwrap_or(0), Ordering::Relaxed);
        }
    }
}

pub fn walk_fp(mut fp: u64) -> Vec<u64> {
    let mut out = Vec::new();
    for _ in 0..8 {
        if fp == 0 || fp % 8 != 0 {
            break;
        }
        let (next, lr) = unsafe { (*(fp as *const u64), *((fp + 8) as *const u64)) };
        if lr == 0 {
            break;
        }
        out.push(lr);
        if next <= fp {
            break;
        }
        fp = next;
    }
    out
}

pub fn gate_canary_ok() -> Vec<u32> {
    let g = LIVE.0.lock().unwrap();
    g.iter()
        .filter_map(|p| unsafe {
            let t = &**p;
            let v = u64::from_le_bytes(t.gate_stack[..8].try_into().unwrap());
            (v != CANARY).then_some(t.handle)
        })
        .collect()
}

pub fn live_sites() -> Vec<(u32, u64, u64, [u64; 4])> {
    let g = LIVE.0.lock().unwrap();
    g.iter()
        .map(|p| unsafe {
            let t = &**p;
            let bt = [t.bt[0].load(Ordering::Relaxed), t.bt[1].load(Ordering::Relaxed), t.bt[2].load(Ordering::Relaxed), t.bt[3].load(Ordering::Relaxed)];
            (t.handle, t.site_lr.load(Ordering::Relaxed), t.site_what.load(Ordering::Relaxed), bt)
        })
        .collect()
}

const _: () = {
    assert!(std::mem::offset_of!(GuestThread, x18) == ctx::X18 as usize);
    assert!(std::mem::offset_of!(GuestThread, tpidr) == ctx::TPIDR as usize);
    assert!(std::mem::offset_of!(GuestThread, tpidrro) == ctx::TPIDRRO as usize);
    assert!(std::mem::offset_of!(GuestThread, spill) == ctx::SPILL0 as usize);
    assert!(std::mem::offset_of!(GuestThread, gate_stack_top) == ctx::GATE_STACK as usize);
    assert!(std::mem::offset_of!(GuestThread, host_sp) == 0x38);
};

const GATE_STACK_SIZE: usize = 64 * 1024 * 1024;
const CANARY: u64 = 0xCAFE_BABE_DEAD_BEEF;

pub fn init_tsd() {
    let mut key: libc::pthread_key_t = 0;
    assert_eq!(unsafe { libc::pthread_key_create(&mut key, None) }, 0);
    KEY.store(key as u64, Ordering::Relaxed);
    #[cfg(not(target_os = "android"))]
    NX_CTX_SLOT.store(key as u64 * 8, Ordering::Relaxed);
    #[cfg(target_os = "android")]
    {
        // bionic keeps pthread_getspecific's values in a per-thread array whose
        // offset is not key * 8. Store a sentinel and locate it once.
        let sentinel = 0x4d55_5348_3853_4c54u64 as *mut libc::c_void;
        assert_eq!(unsafe { libc::pthread_setspecific(key, sentinel) }, 0);
        let tls: usize;
        unsafe { std::arch::asm!("mrs {}, tpidr_el0", out(reg) tls) };
        let mut offset = None;
        for off in (0..0x1000).step_by(8) {
            if unsafe { std::ptr::read_volatile((tls + off) as *const u64) } == sentinel as u64 {
                offset = Some(off as u64);
                break;
            }
        }
        assert!(offset.is_some(), "Android TLS slot not found");
        NX_CTX_SLOT.store(offset.unwrap(), Ordering::Relaxed);
        assert_eq!(unsafe { libc::pthread_setspecific(key, std::ptr::null()) }, 0);
    }
}

impl GuestThread {
    pub fn new(handle: u32, tlr: u64) -> Box<Self> {
        let mut gate_stack = vec![0u8; GATE_STACK_SIZE];
        gate_stack[..8].copy_from_slice(&CANARY.to_le_bytes());
        let top = (gate_stack.as_ptr() as u64 + GATE_STACK_SIZE as u64) & !15;
        let mut b = Box::new(Self {
            x18: 0,
            tpidr: 0,
            tpidrro: tlr,
            spill: [0; 3],
            gate_stack_top: top,
            host_sp: 0,
            handle,
            site_lr: AtomicU64::new(0),
            site_what: AtomicU64::new(0),
            site_fp: AtomicU64::new(0),
            bt: [const { AtomicU64::new(0) }; 4],
            gate_stack,
        });
        LIVE.0.lock().unwrap().push(&mut *b as *mut GuestThread);
        b
    }

    pub fn install(self: &mut Box<Self>) {
        #[cfg(not(target_os = "android"))]
        {
            let base: u64;
            unsafe { std::arch::asm!("mrs {}, tpidrro_el0", out(reg) base) };
            assert_eq!(base & 7, 0, "Darwin TSD base has low bits set; rewritten code assumes it does not");
        }
        let p: *mut GuestThread = &mut **self;
        assert_eq!(unsafe { libc::pthread_setspecific(KEY.load(Ordering::Relaxed) as _, p as *const _) }, 0);
    }
}

pub fn try_current() -> Option<&'static mut GuestThread> {
    let p = unsafe { libc::pthread_getspecific(KEY.load(Ordering::Relaxed) as _) } as *mut GuestThread;
    (!p.is_null()).then(|| unsafe { &mut *p })
}

pub fn current() -> &'static mut GuestThread {
    let p = unsafe { libc::pthread_getspecific(KEY.load(Ordering::Relaxed) as _) } as *mut GuestThread;
    assert!(!p.is_null(), "not a guest thread");
    unsafe { &mut *p }
}

#[repr(C)]
pub struct Frame {
    pub x: [u64; 31],
    pub sp: u64,
    pub nzcv: u64,
    pub fpcr: u64,
    pub fpsr: u64,
    _pad: u64,
    pub q: [u128; 32],
}
const _: () = assert!(std::mem::size_of::<Frame>() == 0x320);

#[repr(C)]
pub struct Entry {
    pub pc: u64,
    pub sp: u64,
    pub x: [u64; 4],
}

#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn nx_enter_guest(entry: *const Entry, ctx: *mut GuestThread);
    pub fn nx_exit_to_host(ctx: *mut GuestThread) -> !;
    pub fn nx_svc_gate();
}

global_asm!(
    ".globl _nx_enter_guest",
    ".p2align 2",
    "_nx_enter_guest:",
    "stp x19, x20, [sp, #-0xa0]!",
    "stp x21, x22, [sp, #0x10]",
    "stp x23, x24, [sp, #0x20]",
    "stp x25, x26, [sp, #0x30]",
    "stp x27, x28, [sp, #0x40]",
    "stp x29, x30, [sp, #0x50]",
    "stp d8, d9, [sp, #0x60]",
    "stp d10, d11, [sp, #0x70]",
    "stp d12, d13, [sp, #0x80]",
    "stp d14, d15, [sp, #0x90]",
    "mov x9, sp",
    "str x9, [x1, #0x38]",
    "mov x19, x1",
    "ldp x9, x10, [x0]",
    "mov sp, x10",
    "ldp x2, x3, [x0, #0x20]",
    "ldp x0, x1, [x0, #0x10]",
    "adr x30, 1f",
    "br x9",
    "1:",
    "mov x0, x19",
    ".globl _nx_exit_to_host",
    "_nx_exit_to_host:",
    "ldr x9, [x0, #0x38]",
    "mov sp, x9",
    "ldp x21, x22, [sp, #0x10]",
    "ldp x23, x24, [sp, #0x20]",
    "ldp x25, x26, [sp, #0x30]",
    "ldp x27, x28, [sp, #0x40]",
    "ldp x29, x30, [sp, #0x50]",
    "ldp d8, d9, [sp, #0x60]",
    "ldp d10, d11, [sp, #0x70]",
    "ldp d12, d13, [sp, #0x80]",
    "ldp d14, d15, [sp, #0x90]",
    "ldp x19, x20, [sp], #0xa0",
    "ret",
    ".globl _nx_svc_gate",
    ".p2align 2",
    "_nx_svc_gate:",
    "sub sp, sp, #0x340",
    "stp x0, x1, [sp, #0x00]",
    "stp x2, x3, [sp, #0x10]",
    "stp x4, x5, [sp, #0x20]",
    "stp x6, x7, [sp, #0x30]",
    "stp x8, x9, [sp, #0x40]",
    "stp x10, x11, [sp, #0x50]",
    "stp x12, x13, [sp, #0x60]",
    "stp x14, x15, [sp, #0x70]",
    "stp x16, x17, [sp, #0x80]",
    "stp x18, x19, [sp, #0x90]",
    "stp x20, x21, [sp, #0xa0]",
    "stp x22, x23, [sp, #0xb0]",
    "stp x24, x25, [sp, #0xc0]",
    "stp x26, x27, [sp, #0xd0]",
    "stp x28, x29, [sp, #0xe0]",
    "str x30, [sp, #0xf0]",
    "add x2, sp, #0x340",
    "str x2, [sp, #0xf8]",
    "mrs x2, nzcv",
    "str x2, [sp, #0x100]",
    "mrs x2, fpcr",
    "str x2, [sp, #0x108]",
    "mrs x2, fpsr",
    "str x2, [sp, #0x110]",
    "add x2, sp, #0x120",
    "stp q0, q1, [x2, #0x00]",
    "stp q2, q3, [x2, #0x20]",
    "stp q4, q5, [x2, #0x40]",
    "stp q6, q7, [x2, #0x60]",
    "stp q8, q9, [x2, #0x80]",
    "stp q10, q11, [x2, #0xa0]",
    "stp q12, q13, [x2, #0xc0]",
    "stp q14, q15, [x2, #0xe0]",
    "stp q16, q17, [x2, #0x100]",
    "stp q18, q19, [x2, #0x120]",
    "stp q20, q21, [x2, #0x140]",
    "stp q22, q23, [x2, #0x160]",
    "stp q24, q25, [x2, #0x180]",
    "stp q26, q27, [x2, #0x1a0]",
    "stp q28, q29, [x2, #0x1c0]",
    "stp q30, q31, [x2, #0x1e0]",
    "mov x0, sp",
    "bl {dispatch}",
    "mov x1, sp",
    "add x2, x1, #0x120",
    "ldp q0, q1, [x2, #0x00]",
    "ldp q2, q3, [x2, #0x20]",
    "ldp q4, q5, [x2, #0x40]",
    "ldp q6, q7, [x2, #0x60]",
    "ldp q8, q9, [x2, #0x80]",
    "ldp q10, q11, [x2, #0xa0]",
    "ldp q12, q13, [x2, #0xc0]",
    "ldp q14, q15, [x2, #0xe0]",
    "ldp q16, q17, [x2, #0x100]",
    "ldp q18, q19, [x2, #0x120]",
    "ldp q20, q21, [x2, #0x140]",
    "ldp q22, q23, [x2, #0x160]",
    "ldp q24, q25, [x2, #0x180]",
    "ldp q26, q27, [x2, #0x1a0]",
    "ldp q28, q29, [x2, #0x1c0]",
    "ldp q30, q31, [x2, #0x1e0]",
    "ldr x2, [x1, #0x100]",
    "msr nzcv, x2",
    "ldr x2, [x1, #0x108]",
    "msr fpcr, x2",
    "ldr x2, [x1, #0x110]",
    "msr fpsr, x2",
    "ldr x2, [x1, #0xf8]",
    "mov sp, x2",
    "ldp x2, x3, [x1, #0x10]",
    "ldp x4, x5, [x1, #0x20]",
    "ldp x6, x7, [x1, #0x30]",
    "ldp x8, x9, [x1, #0x40]",
    "ldp x10, x11, [x1, #0x50]",
    "ldp x12, x13, [x1, #0x60]",
    "ldp x14, x15, [x1, #0x70]",
    "ldp x16, x17, [x1, #0x80]",
    "ldp x20, x21, [x1, #0xa0]",
    "ldp x22, x23, [x1, #0xb0]",
    "ldp x24, x25, [x1, #0xc0]",
    "ldp x26, x27, [x1, #0xd0]",
    "ldp x28, x29, [x1, #0xe0]",
    "ldr x19, [x1, #0x98]",
    "ldr x30, [x1, #0xf0]",
    "ldp x0, x1, [x1]",
    "ret",
    dispatch = sym crate::svc::dispatch,
);

#[cfg(target_os = "android")]
global_asm!(
    ".globl nx_enter_guest",
    ".set nx_enter_guest, _nx_enter_guest",
    ".globl nx_exit_to_host",
    ".set nx_exit_to_host, _nx_exit_to_host",
    ".globl nx_svc_gate",
    ".set nx_svc_gate, _nx_svc_gate",
);

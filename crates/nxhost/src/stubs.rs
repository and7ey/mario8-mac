// Mush8 - stubs.rs
// Constantin Clerc (c22dev) - 0.1
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
pub struct Args(pub [u64; 8], pub [u64; 8]);

impl Args {
    pub fn f32(&self, i: usize) -> f32 {
        f32::from_bits(self.1[i] as u32)
    }
}

pub type Handler = fn(tag: u64, args: &Args) -> u64;

const STUB_SIZE: u64 = 8;
const MAX_STUBS: usize = 8192;

static HANDLERS: [OnceLock<(Handler, u64)>; MAX_STUBS] = [const { OnceLock::new() }; MAX_STUBS];
static NEXT: AtomicUsize = AtomicUsize::new(0);

pub fn site_on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("NX_PROFILE").is_some())
}

fn trace_hle() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("NX_TRACE_HLE").is_some())
}

thread_local! {
    static LAST_CALLER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

pub fn last_caller() -> u64 {
    LAST_CALLER.with(|c| c.get())
}

#[unsafe(no_mangle)]
extern "C" fn nx_stub_dispatch(idx: u64, args: &Args, caller: u64) -> u64 {
    LAST_CALLER.with(|c| c.set(caller));
    if site_on() {
        crate::guest::record_site(caller, idx);
    }
    PROFILE.record(caller, idx);
    let (h, tag) = *HANDLERS[idx as usize].get().expect("stub called before it was made");
    let r = h(tag, args);
    if trace_hle() {
        let name = name_of(idx as usize);
        if !name.starts_with("nvn") {
            eprintln!(
                "[hle] {name}({:#x}, {:#x}, {:#x}, {:#x}) = {r:#x} <- {}",
                args.0[0],
                args.0[1],
                args.0[2],
                args.0[3],
                crate::diag::symbolize(caller)
            );
        }
    }
    r
}

struct Profile {
    slot: std::sync::Mutex<Option<(std::time::Instant, std::collections::BTreeMap<(u64, u64), u32>)>>,
}

static PROFILE: Profile = Profile { slot: std::sync::Mutex::new(None) };

impl Profile {
    fn record(&self, caller: u64, idx: u64) {
        static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        if !*ON.get_or_init(|| std::env::var_os("NX_PROFILE").is_some()) {
            return;
        }
        let mut g = self.slot.lock().unwrap();
        let now = std::time::Instant::now();
        let e = g.get_or_insert_with(|| (now, Default::default()));
        *e.1.entry((caller, idx)).or_default() += 1;
        if now.duration_since(e.0).as_secs_f32() >= 2.0 {
            let mut v: Vec<((u64, u64), u32)> = e.1.iter().map(|(k, n)| (*k, *n)).collect();
            v.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            let top: Vec<String> = v
                .iter()
                .take(8)
                .map(|((a, i), n)| format!("{} -> {} x{n}", crate::diag::symbolize(*a), name_of(*i as usize)))
                .collect();
            eprintln!("[prof] {}", top.join(" | "));
            let rest: Vec<String> = v
                .iter()
                .filter(|((_, i), _)| !name_of(*i as usize).starts_with("nvn"))
                .take(8)
                .map(|((a, i), n)| format!("{} -> {} x{n}", crate::diag::symbolize(*a), name_of(*i as usize)))
                .collect();
            if !rest.is_empty() {
                eprintln!("[prof] hle: {}", rest.join(" | "));
            }
            *e = (now, Default::default());
        }
    }
}

unsafe extern "C" {
    fn nx_stubs();
}

#[cfg(not(target_os = "android"))]
macro_rules! read_tls_base {
    () => {
        "mrs x13, tpidrro_el0\n"
    };
}
#[cfg(target_os = "android")]
macro_rules! read_tls_base {
    () => {
        "mrs x13, tpidr_el0\n"
    };
}

#[cfg(not(target_os = "android"))]
macro_rules! ctx_slot_addr {
    () => {
        "adrp x14, {slot}@PAGE\nadd x14, x14, {slot}@PAGEOFF\n"
    };
}
#[cfg(target_os = "android")]
macro_rules! ctx_slot_addr {
    () => {
        "adrp x14, :got:{slot}\nldr x14, [x14, :got_lo12:{slot}]\n"
    };
}

std::arch::global_asm!(
    concat!(
        ".globl _nx_stubs\n",
        ".p2align 4\n",
        "_nx_stubs:\n",
        ".set nx_stub_i, 0\n",
        ".rept 8192\n",
        "movz x16, #nx_stub_i\n",
        "b _nx_stub_entry\n",
        ".set nx_stub_i, nx_stub_i + 1\n",
        ".endr\n",
        ".p2align 2\n",
        "_nx_stub_entry:\n",
        "mov x9, x0\n",
        "mov x15, sp\n",
        // guest threads are handed stacks as small as 0x8000, which host
        // libraries (Metal pipeline creation, audio, font) overflow; run every
        // import on the thread's own 4 MB gate stack instead
        ctx_slot_addr!(),
        "ldr x14, [x14]\n",
        read_tls_base!(),
        "ldr x13, [x13, x14]\n",
        "cbz x13, 2f\n",
        "ldr x14, [x13, #0x30]\n",
        "cbz x14, 2f\n",
        "and x14, x14, #~15\n",
        "sub x14, x14, #16\n",
        "str x15, [x14]\n",
        "mov sp, x14\n",
        "b 3f\n",
        "2:\n",
        "and x14, x15, #~15\n",
        "sub x14, x14, #16\n",
        "str x15, [x14]\n",
        "mov sp, x14\n",
        "3:\n",
        "stp x29, x30, [sp, #-0x90]!\n",
        "mov x29, sp\n",
        "stp x9, x1, [sp, #0x10]\n",
        "stp x2, x3, [sp, #0x20]\n",
        "stp x4, x5, [sp, #0x30]\n",
        "stp x6, x7, [sp, #0x40]\n",
        "stp d0, d1, [sp, #0x50]\n",
        "stp d2, d3, [sp, #0x60]\n",
        "stp d4, d5, [sp, #0x70]\n",
        "stp d6, d7, [sp, #0x80]\n",
        "mov x0, x16\n",
        "add x1, sp, #0x10\n",
        "mov x2, x30\n",
        "bl {f}\n",
        "ldp x29, x30, [sp], #0x90\n",
        "ldr x17, [sp]\n",
        "mov sp, x17\n",
        "ret\n",
    ),
    slot = sym crate::guest::NX_CTX_SLOT,
    f = sym nx_stub_dispatch,
);

#[cfg(target_os = "android")]
std::arch::global_asm!(
    ".globl nx_stubs",
    ".set nx_stubs, _nx_stubs",
);

pub fn make(h: Handler, tag: u64) -> u64 {
    let i = NEXT.fetch_add(1, Ordering::Relaxed);
    assert!(i < MAX_STUBS, "out of stubs");
    let _ = HANDLERS[i].set((h, tag));
    NAMES.lock().unwrap().resize(i + 1, String::new());
    nx_stubs as *const () as usize as u64 + i as u64 * STUB_SIZE
}

pub fn make_named(h: Handler, tag: u64, name: &str) -> u64 {
    let addr = make(h, tag);
    let i = ((addr - nx_stubs as *const () as usize as u64) / STUB_SIZE) as usize;
    if let Some(slot) = NAMES.lock().unwrap().get_mut(i) {
        *slot = name.to_string();
    }
    addr
}

static NAMES: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

pub fn name_of(i: usize) -> String {
    NAMES.lock().unwrap().get(i).cloned().unwrap_or_default()
}

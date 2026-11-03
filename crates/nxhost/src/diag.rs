// Mush8 - diag.rs
// Constantin Clerc (c22dev) - 0.1
use std::sync::OnceLock;

pub struct ModInfo {
    pub name: String,
    pub base: u64,
    pub size: u64,
    pub symbols: Vec<(u64, String)>,
}

pub static MODULES: OnceLock<Vec<ModInfo>> = OnceLock::new();

#[cfg(not(target_os = "android"))]
unsafe extern "C" {
    fn _dyld_get_image_vmaddr_slide(index: u32) -> isize;
}

pub fn symbolize(addr: u64) -> String {
    if let Some(ms) = MODULES.get() {
        for m in ms {
            if (m.base..m.base + m.size).contains(&addr) {
                let off = addr - m.base;
                let i = m.symbols.partition_point(|(o, _)| *o <= off);
                let sym = if i > 0 { format!(" {}+{:#x}", m.symbols[i - 1].1, off - m.symbols[i - 1].0) } else { String::new() };
                return format!("{}+{off:#x}{sym}", m.name);
            }
        }
    }
    format!("{addr:#x}")
}

pub fn backtrace(fp: u64) {
    for l in backtrace_lines(fp) {
        eprintln!("        at {l}");
    }
}

pub fn backtrace_lines(mut fp: u64) -> Vec<String> {
    let mut out = Vec::new();
    for _ in 0..32 {
        if fp == 0 || fp % 8 != 0 {
            break;
        }
        let (next, lr) = unsafe { (*(fp as *const u64), *((fp + 8) as *const u64)) };
        if lr == 0 {
            break;
        }
        out.push(symbolize(lr));
        if next <= fp {
            break;
        }
        fp = next;
    }
    out
}

pub fn start_site_watch() {
    if std::env::var_os("NX_PROFILE").is_none() {
        return;
    }
    std::thread::Builder::new()
        .name("site-watch".into())
        .spawn(|| {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(5));
                {
                    static WARN: std::sync::Once = std::sync::Once::new();
                    let broken = crate::guest::gate_canary_ok();
                    if !broken.is_empty() {
                        WARN.call_once(|| eprintln!("[gate] stack canary smashed on threads {broken:x?}"));
                    }
                }
                let sites = crate::guest::live_sites();
                let mut line = String::new();
                for (h, lr, what, bt) in sites.iter().take(48) {
                    let (kind, code) = (what >> 32, what & 0xffff_ffff);
                    let name = if kind == 0 { crate::stubs::name_of(code as usize) } else { format!("svc{code:#04x}") };
                    let chain: Vec<String> = bt.iter().filter(|a| **a != 0).map(|a| symbolize(*a)).collect();
                    line += &format!(" [{h:#x} {} {name} bt:{}]", symbolize(*lr), chain.join(" < "));
                }
                eprintln!("[site] t={} {}", crate::svc::system_tick(), line);
            }
        })
        .ok();
}

pub fn report_and_exit(msg: &str) -> ! {
    eprintln!("[fatal] {msg}");
    crate::nvn::summary();
    std::process::exit(1);
}

extern "C" fn on_signal(sig: i32, info: *mut libc::siginfo_t, uc: *mut libc::c_void) {
    unsafe {
        let uc = uc as *mut libc::ucontext_t;
        let mc = &*(*uc).uc_mcontext;
        let ss = &mc.__ss;
        if sig == libc::SIGTRAP && WATCHING.load(std::sync::atomic::Ordering::Relaxed) && (*(ss.__pc as *const u32)) & 0x1F == 31 {
            disarm_current();
            return;
        }
        #[cfg(not(target_os = "android"))]
        let slide = unsafe { _dyld_get_image_vmaddr_slide(0) } as u64;
        #[cfg(target_os = "android")]
        let slide = 0u64;
        eprintln!(
            "[fatal] signal {sig} at pc={} lr={} fault_addr={:#x} host_slide={slide:#x}",
            symbolize(ss.__pc),
            symbolize(ss.__lr),
            (*info).si_addr as u64
        );
        for i in (0..29).step_by(4) {
            let mut line = String::new();
            for j in i..(i + 4).min(29) {
                line += &format!("x{j:<2}={:#018x} ", ss.__x[j]);
            }
            eprintln!("        {line}");
        }
        eprintln!("        fp={:#x} sp={:#x}", ss.__fp, ss.__sp);
        crate::kernel::describe_current_thread();
        backtrace(ss.__fp);
        crate::nvn::summary();
        libc::_exit(1);
    }
}

pub fn install_altstack() {
    let size = 256 * 1024;
    let mem = Box::leak(vec![0u8; size].into_boxed_slice());
    let st = libc::stack_t { ss_sp: mem.as_mut_ptr() as *mut _, ss_size: size, ss_flags: 0 };
    unsafe { libc::sigaltstack(&st, std::ptr::null_mut()) };
}

pub fn install_handlers() {
    install_altstack();
    for sig in [libc::SIGSEGV, libc::SIGBUS, libc::SIGILL, libc::SIGTRAP, libc::SIGFPE] {
        unsafe {
            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_sigaction = on_signal as *const () as usize;
            sa.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK;
            libc::sigaction(sig, &sa, std::ptr::null_mut());
        }
    }
}

pub fn start_watch(mods: &[(String, u64)]) {
    let Ok(spec) = std::env::var("NX_WATCH") else { return };
    let parse = || -> Option<(u64, Vec<Option<u64>>)> {
        let mut it = spec.split(',');
        let (name, off) = it.next()?.split_once('+')?;
        let base = mods.iter().find(|(n, _)| n == name)?.1 + u64::from_str_radix(off.trim_start_matches("0x"), 16).ok()?;
        let steps = it.map(|s| if s == "*" { Some(None) } else { u64::from_str_radix(s.trim_start_matches('+').trim_start_matches("0x"), 16).ok().map(Some) }).collect::<Option<Vec<_>>>()?;
        Some((base, steps))
    };
    let Some((loc, steps)) = parse() else {
        eprintln!("[watch] NX_WATCH={spec}: expected module+offset,*,+off,...");
        return;
    };
    std::thread::spawn(move || {
        let target = loop {
            let mut a = loc;
            let mut ok = true;
            for s in &steps {
                match s {
                    None => {
                        a = unsafe { *(a as *const u64) };
                        if a == 0 {
                            ok = false;
                            break;
                        }
                    }
                    Some(off) => a += off,
                }
            }
            if ok {
                break a;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        };
        let delay = std::env::var("NX_WATCH_DELAY_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
        std::thread::sleep(std::time::Duration::from_millis(delay));
        eprintln!("[watch] armed on {target:#x} (value now {:#x})", unsafe { *(target as *const u32) });
        WATCHING.store(true, std::sync::atomic::Ordering::Relaxed);
        loop {
            arm_all(target);
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    });
}

static WATCHING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn disarm_current() {
    unsafe extern "C" {
        fn mach_thread_self() -> u32;
        fn thread_set_state(thread: u32, flavor: i32, state: *const u32, count: u32) -> i32;
        fn mach_port_deallocate(task: u32, name: u32) -> i32;
        static mach_task_self_: u32;
    }
    let st = ArmDebugState64 { bvr: [0; 16], bcr: [0; 16], wvr: [0; 16], wcr: [0; 16], mdscr_el1: 0 };
    unsafe {
        let t = mach_thread_self();
        thread_set_state(t, 15, &st as *const _ as *const u32, (std::mem::size_of::<ArmDebugState64>() / 4) as u32);
        mach_port_deallocate(mach_task_self_, t);
    }
}

#[repr(C)]
struct ArmDebugState64 {
    bvr: [u64; 16],
    bcr: [u64; 16],
    wvr: [u64; 16],
    wcr: [u64; 16],
    mdscr_el1: u64,
}

fn arm_all(addr: u64) {
    unsafe extern "C" {
        fn task_threads(task: u32, list: *mut *mut u32, count: *mut u32) -> i32;
        fn thread_set_state(thread: u32, flavor: i32, state: *const u32, count: u32) -> i32;
        fn mach_port_deallocate(task: u32, name: u32) -> i32;
        fn vm_deallocate(task: u32, addr: usize, size: usize) -> i32;
        static mach_task_self_: u32;
    }
    const ARM_DEBUG_STATE64: i32 = 15;
    let mut st = ArmDebugState64 { bvr: [0; 16], bcr: [0; 16], wvr: [0; 16], wcr: [0; 16], mdscr_el1: 0 };
    st.wvr[0] = addr & !7;
    let bas = 0xFu64 << (addr & 7);
    st.wcr[0] = 1 | (2 << 1) | (2 << 3) | (bas << 5);
    unsafe {
        let task = mach_task_self_;
        let (mut list, mut n) = (std::ptr::null_mut(), 0u32);
        if task_threads(task, &mut list, &mut n) != 0 {
            return;
        }
        for i in 0..n as usize {
            let t = *list.add(i);
            thread_set_state(t, ARM_DEBUG_STATE64, &st as *const _ as *const u32, (std::mem::size_of::<ArmDebugState64>() / 4) as u32);
            mach_port_deallocate(task, t);
        }
        vm_deallocate(task, list as usize, n as usize * 4);
    }
}

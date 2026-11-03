// Mush8 - shm.rs
// Constantin Clerc (c22dev) - 0.1
use anyhow::{Result, ensure};
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Shm {
    pub fd: i32,
    pub host: u64,
}

pub fn create(size: u64) -> Result<Shm> {
    static N: AtomicU32 = AtomicU32::new(0);
    #[cfg(target_os = "android")]
    let fd = {
        let name = std::ffi::CString::new(format!("nxhost.{}.{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)))?;
        let fd = unsafe { libc::syscall(libc::SYS_memfd_create, name.as_ptr(), 0u32) } as i32;
        ensure!(fd >= 0, "memfd_create: {}", std::io::Error::last_os_error());
        fd
    };
    #[cfg(not(target_os = "android"))]
    let fd = {
        let name = format!("/nxhost.{}.{}\0", std::process::id(), N.fetch_add(1, Ordering::Relaxed));
        let mut fd = unsafe { libc::shm_open(name.as_ptr() as *const _, libc::O_RDWR | libc::O_CREAT | libc::O_EXCL, 0o600) };
        if fd >= 0 {
            unsafe { libc::shm_unlink(name.as_ptr() as *const _) };
        } else {
            let path = std::env::temp_dir().join(format!("nxhost.{}.{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
            let c = std::ffi::CString::new(path.as_os_str().as_encoded_bytes())?;
            fd = unsafe { libc::open(c.as_ptr(), libc::O_RDWR | libc::O_CREAT | libc::O_EXCL, 0o600) };
            ensure!(fd >= 0, "shared memory file {}: {}", path.display(), std::io::Error::last_os_error());
            unsafe { libc::unlink(c.as_ptr()) };
        }
        fd
    };
    ensure!(unsafe { libc::ftruncate(fd, size as i64) } == 0, "ftruncate: {}", std::io::Error::last_os_error());
    let p = unsafe { libc::mmap(std::ptr::null_mut(), size as usize, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };
    ensure!(p != libc::MAP_FAILED, "mmap shm: {}", std::io::Error::last_os_error());
    Ok(Shm { fd, host: p as u64 })
}

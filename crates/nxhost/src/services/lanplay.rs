// Mush8 - lanplay.rs
// Constantin Clerc (c22dev) - 0.1
use crate::services::vnet::{self, Transport};
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::os::fd::AsRawFd;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

const DEFAULT_PORT: u16 = 11451;
const KEEPALIVE: u8 = 0;
const IPV4: u8 = 1;
const PING: u8 = 2;
const IPV4_FRAG: u8 = 3;
const AUTH_ME: u8 = 4;
const INFO: u8 = 0x10;
const INTERVAL: Duration = Duration::from_secs(10);
const SILENCE: Duration = Duration::from_secs(30);

struct LanPlay {
    sock: Arc<OnceLock<UdpSocket>>,
}

impl Transport for LanPlay {
    fn send(&self, packet: &[u8]) {
        if let Some(s) = self.sock.get() {
            let mut frame = Vec::with_capacity(packet.len() + 1);
            frame.push(IPV4);
            frame.extend_from_slice(packet);
            unsafe { libc::send(s.as_raw_fd(), frame.as_ptr().cast(), frame.len(), libc::MSG_DONTWAIT) };
        }
    }
}

fn resolve(server: &str) -> Option<SocketAddr> {
    let with_port = if server.parse::<SocketAddr>().is_ok() || server.rsplit_once(':').is_some_and(|(h, p)| !h.contains(':') && p.parse::<u16>().is_ok()) {
        server.to_string()
    } else {
        let host = server.trim_start_matches('[').trim_end_matches(']');
        if host.contains(':') { format!("[{host}]:{DEFAULT_PORT}") } else { format!("{host}:{DEFAULT_PORT}") }
    };
    let all: Vec<SocketAddr> = with_port.to_socket_addrs().ok()?.collect();
    all.iter().find(|a| a.is_ipv4()).or(all.first()).copied()
}

pub fn start(server: &str, _address: [u8; 4]) -> Option<Box<dyn Transport>> {
    let sock = Arc::new(OnceLock::new());
    let (shared, server) = (sock.clone(), server.to_string());
    std::thread::Builder::new().name("lanplay".into()).spawn(move || run(&server, &shared)).ok()?;
    Some(Box::new(LanPlay { sock }))
}

fn connect(server: &str) -> std::io::Result<(UdpSocket, SocketAddr)> {
    let addr = resolve(server).ok_or_else(|| std::io::Error::other("cannot resolve the name"))?;
    let s = UdpSocket::bind(if addr.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" })?;
    s.connect(addr)?;
    s.set_read_timeout(Some(Duration::from_secs(1)))?;
    Ok((s, addr))
}

fn run(server: &str, shared: &OnceLock<UdpSocket>) {
    let mut failed = false;
    let addr = loop {
        match connect(server) {
            Ok((s, addr)) => {
                let _ = shared.set(s);
                break addr;
            }
            Err(e) => {
                if !failed {
                    eprintln!("[lanplay] cannot use server {server}: {e}, retrying");
                    failed = true;
                }
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    };
    let sock = shared.get().unwrap();
    let send = |frame: &[u8]| sock.send(frame);
    let mut frags = Frags::default();
    let mut buf = vec![0u8; 65536];
    let (mut last_sent, mut last_heard): (Option<Instant>, Option<Instant>) = (None, None);
    let mut ping_at = Instant::now();
    let (mut connected, mut trouble, mut auth_logged, mut silent_logged) = (false, false, false, false);
    let started = Instant::now();
    loop {
        if last_sent.is_none_or(|t| t.elapsed() >= INTERVAL) {
            ping_at = Instant::now();
            let mut nonce = [0u8; 4];
            unsafe { libc::arc4random_buf(nonce.as_mut_ptr().cast(), 4) };
            let r = send(&[KEEPALIVE]).and_then(|_| send(&[PING, nonce[0], nonce[1], nonce[2], nonce[3]]));
            if let (Err(e), false) = (r, trouble) {
                eprintln!("[lanplay] cannot reach {server} ({addr}): {e}");
                trouble = true;
            }
            last_sent = Some(Instant::now());
        }
        if !connected && !silent_logged && last_heard.is_none() && started.elapsed() > INTERVAL {
            eprintln!("[lanplay] no answer from {server} ({addr}) yet");
            silent_logged = true;
        }
        if connected && last_heard.is_some_and(|t| t.elapsed() > SILENCE) {
            eprintln!("[lanplay] lost {server} ({addr}), no answer for {} s", SILENCE.as_secs());
            connected = false;
        }
        let n = match sock.recv(&mut buf) {
            Ok(n) if n > 0 => n,
            Ok(_) => continue,
            Err(e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut | std::io::ErrorKind::Interrupted) => continue,
            Err(e) => {
                if !trouble {
                    eprintln!("[lanplay] {server} ({addr}): {e}");
                    trouble = true;
                }
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }
        };
        last_heard = Some(Instant::now());
        if !connected {
            eprintln!("[lanplay] connected to {server} ({addr}) as {}, ping {} ms", std::net::Ipv4Addr::from(vnet::address()), ping_at.elapsed().as_millis());
            connected = true;
            trouble = false;
        }
        let body = &buf[1..n];
        match buf[0] & 0x7F {
            IPV4 => vnet::receive(body),
            IPV4_FRAG => {
                if let Some(p) = frags.add(body) {
                    vnet::receive(&p);
                }
            }
            AUTH_ME if !auth_logged => {
                eprintln!("[lanplay] {server} asks for a login, which Mush8 does not support; use a server without one");
                auth_logged = true;
            }
            INFO => eprintln!("[lanplay] {server} says: {}", String::from_utf8_lossy(body)),
            _ => {}
        }
    }
}

#[derive(Default)]
struct Frags(HashMap<([u8; 4], u16), (Vec<Option<Vec<u8>>>, u16, Instant)>);

impl Frags {
    fn add(&mut self, f: &[u8]) -> Option<Vec<u8>> {
        if f.len() < 16 {
            return None;
        }
        let be = |o: usize| u16::from_be_bytes([f[o], f[o + 1]]);
        let (src, id, part, total, len, pmtu) = (f[0..4].try_into().unwrap(), be(8), f[10] as usize, f[11] as usize, be(12) as usize, be(14));
        let data = f.get(16..16 + len)?;
        self.0.retain(|_, e| e.2.elapsed() < Duration::from_secs(5));
        let e = self.0.entry((src, id)).or_insert_with(|| (vec![None; total], pmtu, Instant::now()));
        if e.0.len() != total || e.1 != pmtu || part >= total {
            return None;
        }
        e.0[part] = Some(data.to_vec());
        if e.0.iter().any(Option::is_none) {
            return None;
        }
        let (parts, pmtu, _) = self.0.remove(&(src, id))?;
        let mut out = Vec::new();
        for (i, p) in parts.into_iter().flatten().enumerate() {
            if out.len() != i * pmtu as usize {
                return None;
            }
            out.extend_from_slice(&p);
        }
        Some(out)
    }
}

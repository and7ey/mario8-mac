// Mush8 - shbatch.rs
// Constantin Clerc (c22dev) - 0.1
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (src, out) = (PathBuf::from(&args[0]), PathBuf::from(&args[1]));
    let compile_n: Option<usize> = args.iter().position(|a| a == "--compile").and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok());
    std::fs::create_dir_all(&out).unwrap();
    let mut files: Vec<PathBuf> = std::fs::read_dir(&src).unwrap().flatten().map(|e| e.path()).collect();
    files.sort();
    let mut ok = Vec::new();
    let mut fails: BTreeMap<String, usize> = BTreeMap::new();
    let mut warns: BTreeMap<String, usize> = BTreeMap::new();
    for f in &files {
        let blob = std::fs::read(f).unwrap();
        match nxshader::translate(&blob) {
            Ok(t) => {
                if let Some(w) = t.msl.lines().last().and_then(|l| l.strip_prefix("// warnings: ")) {
                    for x in w.split("; ") {
                        let key: String = x.split(|c: char| c.is_ascii_digit()).next().unwrap_or(x).to_string();
                        *warns.entry(key).or_default() += 1;
                    }
                }
                let o = out.join(f.file_stem().unwrap()).with_extension("metal");
                std::fs::write(&o, &t.msl).unwrap();
                ok.push(o);
            }
            Err(e) => {
                let key = e.to_string().split(" at ").next().unwrap_or("").to_string();
                *fails.entry(key).or_default() += 1;
            }
        }
    }
    println!("{} translated, {} failed", ok.len(), files.len() - ok.len());
    for (k, n) in &fails {
        println!("  FAIL {n:6} {k}");
    }
    for (k, n) in &warns {
        println!("  warn {n:6} {k}");
    }
    let Some(n) = compile_n else { return };
    let take = if n == 0 { ok.len() } else { n.min(ok.len()) };
    let step = (ok.len() / take.max(1)).max(1);
    let sample: Vec<PathBuf> = ok.iter().step_by(step).take(take).cloned().collect();
    let queue = Arc::new(Mutex::new(sample.clone()));
    let errors: Arc<Mutex<BTreeMap<String, (usize, String)>>> = Arc::default();
    let threads: Vec<_> = (0..10)
        .map(|_| {
            let (q, errs) = (queue.clone(), errors.clone());
            std::thread::spawn(move || {
                while let Some(p) = q.lock().unwrap().pop() {
                    let air = p.with_extension("air");
                    let r = std::process::Command::new("xcrun").args(["-sdk", "macosx", "metal", "-c"]).arg(&p).arg("-o").arg(&air).output().unwrap();
                    if !r.status.success() {
                        let text = String::from_utf8_lossy(&r.stderr);
                        let first = text.lines().find(|l| l.contains("error:")).unwrap_or("?").to_string();
                        let msg = first.split("error: ").nth(1).unwrap_or(&first).to_string();
                        let mut e = errs.lock().unwrap();
                        let ent = e.entry(msg).or_insert((0, p.display().to_string()));
                        ent.0 += 1;
                    }
                    let _ = std::fs::remove_file(&air);
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    let e = errors.lock().unwrap();
    let failed: usize = e.values().map(|v| v.0).sum();
    println!("compiled {} shaders: {} ok, {failed} errors", sample.len(), sample.len() - failed);
    for (msg, (n, ex)) in e.iter() {
        println!("  {n:5} {msg}  (e.g. {ex})");
    }
}

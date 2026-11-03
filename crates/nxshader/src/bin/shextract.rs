// Mush8 - shextract.rs
// Constantin Clerc (c22dev) - 0.1
use std::collections::BTreeMap;
use std::path::Path;

fn walk(dir: &Path, found: &mut BTreeMap<u64, Vec<u8>>, files: &mut usize) {
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, found, files);
        } else if let Ok(d) = std::fs::read(&p) {
            *files += 1;
            nxshader::corpus::scan(&d, found);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (src, out) = (Path::new(&args[0]), Path::new(&args[1]));
    let mut found = BTreeMap::new();
    let mut files = 0;
    walk(src, &mut found, &mut files);
    std::fs::create_dir_all(out).unwrap();
    for (h, code) in &found {
        std::fs::write(out.join(format!("{h:016x}.bin")), code).unwrap();
    }
    println!("{files} files scanned, {} unique shader programs", found.len());
}

// Mush8 - shdis.rs
// Constantin Clerc (c22dev) - 0.1
use nxshader::table::{InstName, Op, decode as decode_op, lookup};

fn decode(blob: &[u8]) -> Vec<(usize, u64, InstName)> {
    let code = &blob[0x80.min(blob.len())..];
    let mut out = Vec::new();
    for (i, c) in code.chunks_exact(8).enumerate() {
        if i % 4 == 0 {
            continue;
        }
        let w = u64::from_le_bytes(c.try_into().unwrap());
        let (name, _) = lookup(w);
        out.push((i * 8, w, name));
        let rel = ((((w >> 20) as i32) << 8) >> 8) + 8;
        if name == InstName::Bra && (-32..=0).contains(&rel) {
            break;
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--stats") {
        let mut counts: std::collections::HashMap<Op, usize> = std::collections::HashMap::new();
        let mut invalid = 0;
        for f in &args[1..] {
            for (_, w, n) in decode(&std::fs::read(f).unwrap()) {
                *counts.entry(decode_op(w).1).or_insert(0) += 1;
                if n == InstName::Invalid {
                    invalid += 1;
                }
            }
        }
        let mut v: Vec<_> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        println!("{} distinct opcodes, {invalid} invalid", v.len());
        for (n, c) in v {
            println!("{c:7} {n:?}");
        }
    } else {
        for (off, w, n) in decode(&std::fs::read(&args[0]).unwrap()) {
            println!("{off:#06x}: {w:016x} {n:?}");
        }
    }
}

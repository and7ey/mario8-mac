// Mush8 - main.rs
// Constantin Clerc (c22dev) - 0.1
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;
use yaxpeax_arch::{Decoder, U8Reader};
use yaxpeax_arm::armv8::a64::InstDecoder;

fn sysreg_name(w: u32) -> String {
    let (op0, op1, crn, crm, op2) = (2 + ((w >> 19) & 1), (w >> 16) & 7, (w >> 12) & 15, (w >> 8) & 15, (w >> 5) & 7);
    let known = match (op0, op1, crn, crm, op2) {
        (3, 3, 13, 0, 2) => "tpidr_el0",
        (3, 3, 13, 0, 3) => "tpidrro_el0",
        (3, 3, 14, 0, 0) => "cntfrq_el0",
        (3, 3, 14, 0, 1) => "cntpct_el0",
        (3, 3, 14, 0, 2) => "cntvct_el0",
        (3, 3, 0, 0, 1) => "ctr_el0",
        (3, 3, 0, 0, 7) => "dczid_el0",
        (3, 3, 4, 2, 0) => "nzcv",
        (3, 3, 4, 4, 0) => "fpcr",
        (3, 3, 4, 4, 1) => "fpsr",
        _ => "",
    };
    if known.is_empty() { format!("s{op0}_{op1}_c{crn}_c{crm}_{op2}") } else { known.into() }
}

fn may_use_x18(w: u32) -> bool {
    [0, 5, 10, 16].iter().any(|&s| (w >> s) & 31 == 18)
}

#[derive(Default)]
struct Report {
    svc: BTreeMap<u16, usize>,
    mrs: BTreeMap<String, usize>,
    msr: BTreeMap<String, usize>,
    sys: BTreeMap<String, usize>,
    x18: usize,
    x18_examples: Vec<(usize, String)>,
    words: usize,
}

fn scan(text: &[u8]) -> Report {
    let mut r = Report { words: text.len() / 4, ..Default::default() };
    let dec = InstDecoder::default();
    for (i, c) in text.chunks_exact(4).enumerate() {
        let w = u32::from_le_bytes(c.try_into().unwrap());
        if w & 0xFFE0_001F == 0xD400_0001 {
            *r.svc.entry(((w >> 5) & 0xFFFF) as u16).or_default() += 1;
        } else if w & 0xFFF0_0000 == 0xD530_0000 {
            *r.mrs.entry(sysreg_name(w)).or_default() += 1;
        } else if w & 0xFFF0_0000 == 0xD510_0000 {
            *r.msr.entry(sysreg_name(w)).or_default() += 1;
        } else if w & 0xFFF8_0000 == 0xD508_0000 {
            let (op1, crn, crm, op2) = ((w >> 16) & 7, (w >> 12) & 15, (w >> 8) & 15, (w >> 5) & 7);
            *r.sys.entry(format!("sys #{op1}, c{crn}, c{crm}, #{op2}")).or_default() += 1;
        }
        if may_use_x18(w) {
            let mut rd = U8Reader::new(c);
            if let Ok(inst) = dec.decode(&mut rd) {
                let s = inst.to_string();
                if s.split(|ch: char| !ch.is_ascii_alphanumeric()).any(|t| t == "x18" || t == "w18") {
                    r.x18 += 1;
                    if r.x18_examples.len() < 5 {
                        r.x18_examples.push((i * 4, s));
                    }
                }
            }
        }
    }
    r
}

fn main() -> Result<()> {
    let files: Vec<PathBuf> = std::env::args().skip(1).map(Into::into).collect();
    anyhow::ensure!(!files.is_empty(), "usage: nxscan <nso>...");
    for f in files {
        let nso = nxfs::nso::Nso::parse(&std::fs::read(&f).with_context(|| f.display().to_string())?)?;
        let r = scan(nso.text_bytes());
        println!("== {} ({} instructions)", f.display(), r.words);
        println!("  svc:  {}", r.svc.iter().map(|(k, v)| format!("{k:#04x}×{v}")).collect::<Vec<_>>().join(" "));
        println!("  mrs:  {:?}", r.mrs);
        println!("  msr:  {:?}", r.msr);
        println!("  sys:  {:?}", r.sys);
        println!("  x18:  {} uses", r.x18);
        for (off, s) in &r.x18_examples {
            println!("        {off:#010x}  {s}");
        }
    }
    Ok(())
}

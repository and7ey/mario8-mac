// Mush8 - classify.rs
// Constantin Clerc (c22dev) - 0.1
use anyhow::Result;
use capstone::arch::BuildsCapstone;
use capstone::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    IndirectBranch,
    CondBranch,
    PcRelative,
    WritesSp,
    InExclusive,
    Read,
    Write,
    ReadWrite,
}

pub struct Site {
    pub offset: usize,
    pub class: Class,
    pub text: String,
    pub uses: u32,
}

pub fn capstone() -> Result<Capstone> {
    Ok(Capstone::new().arm64().mode(arch::arm64::ArchMode::Arm).detail(true).build()?)
}

pub fn may_use_x18(w: u32) -> bool {
    [0, 5, 10, 16].iter().any(|&s| (w >> s) & 31 == 18)
}

fn is_load_exclusive(w: u32) -> bool {
    w & 0x3F80_0000 == 0x0800_0000 && (w >> 22) & 1 == 1
}
fn is_store_exclusive(w: u32) -> bool {
    w & 0x3F80_0000 == 0x0800_0000 && (w >> 22) & 1 == 0
}

pub fn gpr_index(name: &str) -> Option<u32> {
    match name {
        "sp" | "wsp" => Some(31),
        "fp" => Some(29),
        "lr" => Some(30),
        _ => {
            let n = name.strip_prefix('x').or_else(|| name.strip_prefix('w'))?;
            n.parse().ok().filter(|&i: &u32| i <= 30)
        }
    }
}

pub fn find_sites(text: &[u8]) -> Result<Vec<Site>> {
    let cs = capstone()?;
    let words: Vec<u32> = text.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();

    let mut in_excl = vec![false; words.len()];
    for i in 0..words.len() {
        if is_load_exclusive(words[i]) {
            for j in i + 1..(i + 16).min(words.len()) {
                in_excl[j] = true;
                if is_store_exclusive(words[j]) {
                    break;
                }
            }
        }
    }

    let mut sites = Vec::new();
    for (i, &w) in words.iter().enumerate() {
        if !may_use_x18(w) {
            continue;
        }
        let off = i * 4;
        let insns = cs.disasm_count(&text[off..off + 4], off as u64, 1)?;
        let Some(insn) = insns.iter().next() else { continue };
        let detail = cs.insn_detail(insn)?;
        let names = |regs: &[RegId]| regs.iter().filter_map(|r| cs.reg_name(*r)).collect::<Vec<_>>();
        let (rd, wr) = (names(detail.regs_read()), names(detail.regs_write()));
        let is18 = |n: &String| n == "x18" || n == "w18";
        let (r18, w18) = (rd.iter().any(is18), wr.iter().any(is18));
        if !r18 && !w18 {
            continue;
        }
        let mut uses = 0u32;
        for n in rd.iter().chain(&wr) {
            if let Some(g) = gpr_index(n).filter(|&g| g != 18) {
                uses |= 1 << g;
            }
        }
        let mnem = insn.mnemonic().unwrap_or("");
        let class = if in_excl[i] || is_load_exclusive(w) || is_store_exclusive(w) {
            Class::InExclusive
        } else if matches!(mnem, "br" | "blr" | "ret") {
            Class::IndirectBranch
        } else if matches!(mnem, "cbz" | "cbnz" | "tbz" | "tbnz") {
            Class::CondBranch
        } else if matches!(mnem, "adr" | "adrp") || (mnem.starts_with("ldr") && w & 0x3B00_0000 == 0x1800_0000) {
            Class::PcRelative
        } else if wr.iter().any(|n| n == "sp" || n == "wsp") {
            Class::WritesSp
        } else {
            match (r18, w18) {
                (true, true) => Class::ReadWrite,
                (true, false) => Class::Read,
                _ => Class::Write,
            }
        };
        let text = format!("{} {}", mnem, insn.op_str().unwrap_or(""));
        sites.push(Site { offset: off, class, text, uses });
    }
    Ok(sites)
}

pub fn report(text: &[u8], examples: usize) -> Result<()> {
    let sites = find_sites(text)?;
    let mut by: BTreeMap<Class, Vec<&Site>> = BTreeMap::new();
    for s in &sites {
        by.entry(s.class).or_default().push(s);
    }
    println!("  {} sites", sites.len());
    for (c, v) in &by {
        println!("  {:>6}  {:?}", v.len(), c);
        for s in v.iter().take(examples) {
            println!("            {:#010x}  {}", s.offset, s.text);
        }
    }
    let busy = sites.iter().filter(|s| s.uses & (3 << 16) == 3 << 16).count();
    println!("  sites touching both x16 and x17: {busy}");
    Ok(())
}

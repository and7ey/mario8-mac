// Mush8 - program.rs
// Constantin Clerc (c22dev) - 0.1
use crate::table::{InstName, InstProps, Op, decode};
use anyhow::{Result, bail};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug)]
pub struct Inst {
    pub addr: u32,
    pub raw: u64,
    pub name: InstName,
    pub op: Op,
    pub props: InstProps,
}

pub struct Program {
    pub insts: Vec<Inst>,
    pub targets: BTreeSet<u32>,
    pub converge: BTreeMap<u32, u32>,
}

pub fn branch_target(i: &Inst) -> u32 {
    let rel = (((i.raw >> 20) as i32) << 8) >> 8;
    let t = (i.addr as i64 + rel as i64 + 8) as u32;
    if t % 32 == 0 { t + 8 } else { t }
}

impl Program {
    pub fn decode(code: &[u8]) -> Result<Program> {
        let mut insts = Vec::new();
        for (i, c) in code.chunks_exact(8).enumerate() {
            if i % 4 == 0 {
                continue;
            }
            let raw = u64::from_le_bytes(c.try_into().unwrap());
            let (name, op, props) = decode(raw);
            let inst = Inst { addr: (i * 8) as u32, raw, name, op, props };
            let rel = ((((raw >> 20) as i32) << 8) >> 8) + 8;
            if name == InstName::Bra && (-32..=0).contains(&rel) {
                break;
            }
            insts.push(inst);
        }
        if insts.is_empty() {
            bail!("empty program");
        }
        let mut targets = BTreeSet::new();
        targets.insert(insts[0].addr);
        let mut converge = BTreeMap::new();
        let (mut ssy, mut pbk): (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        for i in &insts {
            while ssy.last() == Some(&i.addr) {
                ssy.pop();
            }
            while pbk.last() == Some(&i.addr) {
                pbk.pop();
            }
            match i.name {
                InstName::Bra => {
                    targets.insert(branch_target(i));
                }
                InstName::Ssy => {
                    let t = branch_target(i);
                    targets.insert(t);
                    ssy.push(t);
                }
                InstName::Pbk => {
                    let t = branch_target(i);
                    targets.insert(t);
                    pbk.push(t);
                }
                InstName::Sync => {
                    if let Some(&t) = ssy.last() {
                        converge.insert(i.addr, t);
                    }
                }
                InstName::Brk => {
                    if let Some(&t) = pbk.last() {
                        converge.insert(i.addr, t);
                    }
                }
                _ => {}
            }
        }
        Ok(Program { insts, targets, converge })
    }
}

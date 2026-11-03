// Mush8 - glsl.rs
// Constantin Clerc (c22dev) - 0.1
use crate::inst::*;
use crate::program::{Inst, Program, branch_target};
use crate::sph::{Imap, Sph, Stage};
use crate::table::{InstName, Op};
use anyhow::{Result, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TexKind {
    T2D,
    T2DArray,
    T3D,
    Cube,
    CubeArray,
    Depth2D,
    Depth2DArray,
    DepthCube,
}

impl TexKind {
    pub fn decl(self) -> &'static str {
        match self {
            TexKind::T2D => "sampler2D",
            TexKind::T2DArray => "sampler2DArray",
            TexKind::T3D => "sampler3D",
            TexKind::Cube => "samplerCube",
            TexKind::CubeArray => "samplerCubeArray",
            TexKind::Depth2D => "sampler2D",
            TexKind::Depth2DArray => "sampler2DArray",
            TexKind::DepthCube => "samplerCube",
        }
    }
    pub fn is_depth(self) -> bool {
        matches!(self, TexKind::Depth2D | TexKind::Depth2DArray | TexKind::DepthCube)
    }
    fn coords(self) -> usize {
        match self {
            TexKind::T3D | TexKind::Cube | TexKind::CubeArray | TexKind::DepthCube => 3,
            _ => 2,
        }
    }
    pub fn is_array(self) -> bool {
        matches!(self, TexKind::T2DArray | TexKind::CubeArray | TexKind::Depth2DArray)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Interface {

    pub textures: BTreeMap<u32, TexKind>,

    pub samplers: BTreeMap<u32, u32>,

    pub attributes: u32,

    pub outputs: u32,

    pub color_targets: u32,
    pub writes_depth: bool,
}

pub struct Translated {
    pub stage: Stage,
    pub source: String,
    pub entry: String,
    pub interface: Interface,
}

impl Translated {

    pub fn named(&self, suffix: &str) -> (String, String) {
        let mut src = self.source[PRELUDE.len()..].to_string();
        for name in ["VOut", "FIn", "FOut", self.entry.as_str()] {
            src = rename(&src, name, &format!("{name}_{suffix}"));
        }
        (src, format!("{}_{suffix}", self.entry))
    }
}

pub fn prelude() -> &'static str {
    PRELUDE
}

fn rename(src: &str, from: &str, to: &str) -> String {
    let word = |c: Option<char>| c.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
    let mut out = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(i) = rest.find(from) {
        let (before, after) = (rest[..i].chars().next_back(), rest[i + from.len()..].chars().next());
        out.push_str(&rest[..i]);
        out.push_str(if word(before) || word(after) { from } else { to });
        rest = &rest[i + from.len()..];
    }
    out.push_str(rest);
    out
}

const ATTR_POSITION: i32 = 0x70;
const ATTR_GENERIC: i32 = 0x80;
const ATTR_GENERIC_END: i32 = 0x280;
const ATTR_POINT_SIZE: i32 = 0x6c;
const ATTR_POINT_COORD: i32 = 0x2e0;
const ATTR_INSTANCE_ID: i32 = 0x2f8;
const ATTR_VERTEX_ID: i32 = 0x2fc;
const ATTR_FRONT_FACING: i32 = 0x3fc;

struct Gen<'a> {
    sph: &'a Sph,
    prog: &'a Program,
    body: String,
    regs: BTreeSet<i32>,
    preds: BTreeSet<i32>,
    uses_cc: bool,
    uses_lmem: bool,
    uses_point_coord: bool,
    iface: Interface,
    warnings: Vec<String>,

    handle_regs: BTreeMap<i32, u32>,
}

const RZ_: i32 = 255;

const TEXTURE_HANDLE_BANK: i32 = 2;

fn fimm(bits: u32) -> String {
    format!("as_float({bits:#x}u)")
}

impl<'a> Gen<'a> {
    fn line(&mut self, s: impl AsRef<str>) {
        self.body.push_str("        ");
        self.body.push_str(s.as_ref());
        self.body.push('\n');
    }

    fn r(&mut self, n: i32) -> String {
        if n == RZ_ {
            "0u".into()
        } else {
            self.regs.insert(n);
            format!("r{n}")
        }
    }
    fn fr(&mut self, n: i32) -> String {
        if n == RZ_ { "0.0f".into() } else { format!("as_float({})", self.r(n)) }
    }
    fn cb(&mut self, slot: i32, word: i32) -> String {
        format!("cb_at(res.cb[{slot}], {word})")
    }
    fn fcb(&mut self, slot: i32, word: i32) -> String {
        format!("as_float({})", self.cb(slot, word))
    }
    fn set(&mut self, d: i32, expr: String) {
        if d != RZ_ {
            self.regs.insert(d);
            self.line(format!("r{d} = {expr};"));
        }
    }
    fn setf(&mut self, d: i32, fexpr: String) {
        self.set(d, format!("as_uint({fexpr})"));
    }
    fn p(&mut self, n: i32, inv: bool) -> String {
        let base = if n == 7 {
            "true".to_string()
        } else {
            self.preds.insert(n);
            format!("p{n}")
        };
        if inv { format!("(!{base})") } else { base }
    }
    fn setp(&mut self, n: i32, expr: String) {
        if n != 7 {
            self.preds.insert(n);
            self.line(format!("p{n} = {expr};"));
        }
    }
    fn fmod(x: String, abs: bool, neg: bool) -> String {
        let x = if abs { format!("fabs({x})") } else { x };
        if neg { format!("(-{x})") } else { x }
    }
    fn sat(x: String, sat: bool) -> String {
        if sat { format!("m8_saturate({x})") } else { x }
    }
    fn set_zn(&mut self, d: String) {
        self.uses_cc = true;
        self.line(format!("zf = ({d}) == 0u; nf = int({d}) < 0;"));
    }

    fn label_target(&mut self, addr: u32) {
        self.line(format!("pc = {addr}u; continue;"));
    }

    fn attr_load(&mut self, addr: i32, output: bool) -> String {
        match (self.sph.stage, addr) {
            (Stage::Vertex, a) if output => self.vs_output_ref(a).map_or("0u".into(), |f| format!("as_uint({f})")),
            (Stage::Vertex, ATTR_GENERIC..ATTR_GENERIC_END) => {
                let (i, c) = ((addr - ATTR_GENERIC) / 16, (addr - ATTR_GENERIC) / 4 % 4);
                self.iface.attributes |= 1 << i;
                format!("a{i}[{c}]")
            }
            (Stage::Vertex, ATTR_INSTANCE_ID) => "iid".into(),
            (Stage::Vertex, ATTR_VERTEX_ID) => "vid".into(),
            (Stage::Fragment, a) => self.fs_input(a, false),
            _ => {
                self.warnings.push(format!("attribute load {addr:#x}"));
                "0u".into()
            }
        }
    }

    fn vs_output_ref(&mut self, addr: i32) -> Option<String> {
        let c = ["x", "y", "z", "w"];
        Some(match addr {
            ATTR_POSITION..=0x7c => format!("o.pos.{}", c[((addr - ATTR_POSITION) / 4) as usize]),
            ATTR_GENERIC..ATTR_GENERIC_END => {
                let (i, k) = ((addr - ATTR_GENERIC) / 16, (addr - ATTR_GENERIC) / 4 % 4);
                self.iface.outputs |= 1 << i;
                format!("o.v{i}.{}", c[k as usize])
            }
            ATTR_POINT_SIZE => "o.psize".into(),
            _ => return None,
        })
    }

    fn fs_input(&mut self, addr: i32, perspective_scale: bool) -> String {
        let c = ["x", "y", "z", "w"];
        match addr {
            ATTR_POSITION..=0x7c => format!("as_uint(in.pos.{})", c[((addr - ATTR_POSITION) / 4) as usize]),
            ATTR_GENERIC..ATTR_GENERIC_END => {
                let (i, k) = ((addr - ATTR_GENERIC) / 16, (addr - ATTR_GENERIC) / 4 % 4);
                self.iface.attributes |= 1 << i;
                let v = format!("in.v{i}.{}", c[k as usize]);

                if perspective_scale && self.sph.imap[i as usize] == Imap::Perspective {
                    format!("as_uint({v} * in.pos.w)")
                } else {
                    format!("as_uint({v})")
                }
            }
            ATTR_FRONT_FACING => "(front ? 0xffffffffu : 0u)".into(),
            ATTR_POINT_COORD | 0x2e4 => {
                self.uses_point_coord = true;
                format!("as_uint(pcoord.{})", if addr == ATTR_POINT_COORD { "x" } else { "y" })
            }
            _ => {
                self.warnings.push(format!("fragment input {addr:#x}"));
                "0u".into()
            }
        }
    }

    fn tex(&mut self, binding: i32, kind: TexKind) -> u32 {
        let b = ((binding - 8) / 2).max(0) as u32;
        let e = self.iface.textures.entry(b).or_insert(kind);
        if *e != kind {
            self.warnings.push(format!("texture binding {b} used as {:?} and {kind:?}", *e));
        }
        b
    }

    fn tsamp(&self, b: u32, kind: TexKind) -> String {
        format!("{}(t{b}, s{b})", kind.decl())
    }

    // size: 0 = u32, 1 = s32, 3 = f32 (AtomSize/AtomsSize agree on these values)
    fn atomic(&mut self, cell: &str, val: &str, size: u32, op: u32, d: i32) -> Result<()> {
        let plain = match (size, op) {
            (0, 0) | (0, 10) | (1, 0) | (1, 10) => Some(format!("atomicAdd({cell}, {val})")),
            (0, 1) => Some(format!("atomicMin({cell}, {val})")),
            (0, 2) => Some(format!("atomicMax({cell}, {val})")),
            (0, 3) | (1, 3) => Some(format!("atomicAdd({cell}, 1u)")),
            (0, 4) | (1, 4) => Some(format!("atomicAdd({cell}, 0xffffffffu)")),
            (0, 5) | (1, 5) => Some(format!("atomicAnd({cell}, {val})")),
            (0, 6) | (1, 6) => Some(format!("atomicOr({cell}, {val})")),
            (0, 7) | (1, 7) => Some(format!("atomicXor({cell}, {val})")),
            (0, 8) | (1, 8) | (3, 8) => Some(format!("atomicExchange({cell}, {val})")),
            _ => None,
        };
        if let Some(e) = plain {
            if d == RZ_ {
                self.line(format!("{e};"));
            } else {
                self.set(d, e);
            }
            return Ok(());
        }
        let next = match (size, op) {
            (1, 1) => format!("uint(min(int(aexp), int({val})))"),
            (1, 2) => format!("uint(max(int(aexp), int({val})))"),
            (3, 0) => format!("floatBitsToUint(uintBitsToFloat(aexp) + uintBitsToFloat({val}))"),
            (3, 1) => format!("floatBitsToUint(min(uintBitsToFloat(aexp), uintBitsToFloat({val})))"),
            (3, 2) => format!("floatBitsToUint(max(uintBitsToFloat(aexp), uintBitsToFloat({val})))"),
            _ => bail!("unsupported atomic size {size} op {op}"),
        };
        self.line(format!("{{ uint aexp = {cell}; uint aold; for (;;) {{ uint anv = {next}; aold = atomicCompSwap({cell}, aexp, anv); if (aold == aexp) break; aexp = aold; }}"));
        if d != RZ_ {
            self.set(d, "aold".into());
        }
        self.line("}");
        Ok(())
    }

    // 64-bit atomics take a register pair for the value and return a pair
    fn atomic64(&mut self, sb: i32, lo: &str, size: u32, op: u32, d: i32) -> Result<()> {
        let hi = if sb != RZ_ { self.r(sb + 1) } else { "0u".into() };
        let u = "U64Ref(ga & ~uint64_t(7)).data[0]";
        let i = "I64Ref(ga & ~uint64_t(7)).data[0]";
        let expr = match op {
            0 | 10 => format!("atomicAdd({u}, aval)"),
            1 if size == 5 => format!("uint64_t(atomicMin({i}, int64_t(aval)))"),
            2 if size == 5 => format!("uint64_t(atomicMax({i}, int64_t(aval)))"),
            1 => format!("atomicMin({u}, aval)"),
            2 => format!("atomicMax({u}, aval)"),
            3 => format!("atomicAdd({u}, uint64_t(1))"),
            4 => format!("atomicAdd({u}, ~uint64_t(0))"),
            5 => format!("atomicAnd({u}, aval)"),
            6 => format!("atomicOr({u}, aval)"),
            7 => format!("atomicXor({u}, aval)"),
            8 => format!("atomicExchange({u}, aval)"),
            _ => bail!("unsupported 64-bit atomic op {op}"),
        };
        self.line(format!("uint64_t aval = (uint64_t({hi}) << 32) | uint64_t({lo}); uint64_t aold = {expr};"));
        if d != RZ_ {
            self.set(d, "uint(aold)".into());
            self.set(d + 1, "uint(aold >> 32)".into());
        }
        Ok(())
    }

    fn write_components(&mut self, value: &str, mask: u32, dests: &[i32]) {
        let mut k = 0;
        for c in 0..4 {
            if mask >> c & 1 == 0 {
                continue;
            }
            if let Some(&d) = dests.get(k) {
                self.set(d, format!("as_uint({value}[{c}])"));
            }
            k += 1;
        }
    }

    fn coord(&mut self, regs: &[i32], n: usize) -> String {
        let v: Vec<String> = regs.iter().take(n).map(|&r| self.fr(r)).collect();
        match n {
            1 => v[0].clone(),
            2 => format!("float2({}, {})", v[0], v[1]),
            _ => format!("float3({}, {}, {})", v[0], v[1], v[2]),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn sample(&mut self, b: u32, kind: TexKind, coords: &[i32], array: Option<i32>, lod: Option<String>, bias: Option<String>, depth_ref: Option<i32>) -> String {
        let n = kind.coords();
        let mut v = Vec::new();
        for i in 0..n {
            v.push(self.fr(coords[i]));
        }
        if let Some(a) = array {
            let a = self.r(a);
            v.push(format!("float({a} & 0xffffu)"));
        }
        let c = match v.len() {
            1 => v[0].clone(),
            2 => format!("float2({}, {})", v[0], v[1]),
            3 => format!("float3({}, {}, {})", v[0], v[1], v[2]),
            _ => format!("float4({}, {}, {}, {})", v[0], v[1], v[2], v[3]),
        };
        let ts = self.tsamp(b, kind);
        let base = match (&lod, &bias) {
            (Some(l), _) => format!("textureLod({ts}, {c}, {l})"),
            (None, Some(bb)) => format!("texture({ts}, {c}, {bb})"),
            _ => format!("texture({ts}, {c})"),
        };
        if let Some(rf) = depth_ref {
            let rv = self.fr(rf);
            format!("float4(step({rv}, {base}.r))")
        } else if kind.is_depth() {
            format!("float4({base}.r)")
        } else {
            base
        }
    }

    fn inst(&mut self, i: &Inst) -> Result<()> {
        let w = i.raw;
        macro_rules! op {
            ($t:ident) => {
                $t(w)
            };
        }

        let fb_r = |g: &mut Self, r: i32| g.fr(r);
        match i.op {
            Op::Invalid => bail!("invalid instruction {w:016x} at {:#x}", i.addr),
            Op::Nop | Op::Depbar | Op::Ssy | Op::Pbk | Op::Cctl | Op::Cctll | Op::Cctlt => {}
            Op::Getlmembase => self.set(op!(InstGetlmembase).dest(), "0u".into()),
            Op::Cs2r => {
                let o = op!(InstCs2r);
                let compute = self.sph.stage == Stage::Compute;
                let v = match o.s_reg().0 {
                    0 => "simd_lane",
                    0x20 if compute => "(tid.x | tid.y << 16 | tid.z << 26)",
                    0x21 if compute => "tid.x",
                    0x22 if compute => "tid.y",
                    0x23 if compute => "tid.z",
                    0x25 if compute => "ctaid.x",
                    0x26 if compute => "ctaid.y",
                    0x27 if compute => "ctaid.z",
                    _ => "0u",
                };
                self.set(o.dest(), v.into());
            }
            Op::Bar if self.sph.stage == Stage::Compute => self.line("barrier();"),
            Op::Membar if self.sph.stage == Stage::Compute => self.line("memoryBarrier();"),
            Op::Bar | Op::Membar => {}

            Op::FaddR | Op::FaddI | Op::FaddC | Op::Fadd32i => {
                let (d, a, b, abs_a, abs_b, neg_a, neg_b, s) = match i.op {
                    Op::FaddR => {
                        let o = op!(InstFaddR);
                        (o.dest(), self.fr(o.src_a()), fb_r(self, o.src_b()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.sat())
                    }
                    Op::FaddI => {
                        let o = op!(InstFaddI);
                        (o.dest(), self.fr(o.src_a()), fimm((o.imm20() as u32) << 12), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.sat())
                    }
                    Op::FaddC => {
                        let o = op!(InstFaddC);
                        (o.dest(), self.fr(o.src_a()), self.fcb(o.cbuf_slot(), o.cbuf_offset()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.sat())
                    }
                    _ => {
                        let o = op!(InstFadd32i);
                        (o.dest(), self.fr(o.src_a()), fimm(o.imm32() as u32), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), false)
                    }
                };
                let e = Self::sat(format!("({} + {})", Self::fmod(a, abs_a, neg_a), Self::fmod(b, abs_b, neg_b)), s);
                self.setf(d, e);
            }
            Op::FmulR | Op::FmulI | Op::FmulC | Op::Fmul32i => {
                let (d, a, b, neg_a, scale, s) = match i.op {
                    Op::FmulR => {
                        let o = op!(InstFmulR);
                        (o.dest(), self.fr(o.src_a()), fb_r(self, o.src_b()), o.neg_a(), o.scale().0, o.sat())
                    }
                    Op::FmulI => {
                        let o = op!(InstFmulI);
                        (o.dest(), self.fr(o.src_a()), fimm((o.imm20() as u32) << 12), o.neg_a(), o.scale().0, o.sat())
                    }
                    Op::FmulC => {
                        let o = op!(InstFmulC);
                        (o.dest(), self.fr(o.src_a()), self.fcb(o.cbuf_slot(), o.cbuf_offset()), o.neg_a(), o.scale().0, o.sat())
                    }
                    _ => {
                        let o = op!(InstFmul32i);
                        (o.dest(), self.fr(o.src_a()), fimm(o.imm32() as u32), false, 0, o.sat())
                    }
                };
                let k = ["", " * 0.5f", " * 0.25f", " * 0.125f", " * 8.0f", " * 4.0f", " * 2.0f"].get(scale as usize).copied().unwrap_or("");
                let e = Self::sat(format!("({} * {}{k})", Self::fmod(a, false, neg_a), b), s);
                self.setf(d, e);
            }
            Op::FfmaR | Op::FfmaI | Op::FfmaC | Op::FfmaRc | Op::Ffma32i => {
                let (d, a, b, c, neg_a, neg_c, s) = match i.op {
                    Op::FfmaR => {
                        let o = op!(InstFfmaR);
                        (o.dest(), self.fr(o.src_a()), self.fr(o.src_b()), self.fr(o.src_c()), o.neg_a(), o.neg_c(), o.sat())
                    }
                    Op::FfmaI => {
                        let o = op!(InstFfmaI);
                        (o.dest(), self.fr(o.src_a()), fimm((o.imm20() as u32) << 12), self.fr(o.src_c()), o.neg_a(), o.neg_c(), o.sat())
                    }
                    Op::FfmaC => {
                        let o = op!(InstFfmaC);
                        (o.dest(), self.fr(o.src_a()), self.fcb(o.cbuf_slot(), o.cbuf_offset()), self.fr(o.src_c()), o.neg_a(), o.neg_c(), o.sat())
                    }
                    Op::FfmaRc => {
                        let o = op!(InstFfmaRc);
                        (o.dest(), self.fr(o.src_a()), self.fr(o.src_c()), self.fcb(o.cbuf_slot(), o.cbuf_offset()), o.neg_a(), o.neg_c(), o.sat())
                    }
                    _ => {
                        let o = op!(InstFfma32i);
                        (o.dest(), self.fr(o.src_a()), fimm(o.imm32() as u32), self.fr(o.dest()), o.neg_a(), o.neg_c(), o.sat())
                    }
                };
                let e = Self::sat(format!("m8_fma({}, {b}, {})", Self::fmod(a, false, neg_a), Self::fmod(c, false, neg_c)), s);
                self.setf(d, e);
            }
            Op::FmnmxR | Op::FmnmxI | Op::FmnmxC => {
                let (d, a, b, abs_a, abs_b, neg_a, neg_b, sp, spi) = match i.op {
                    Op::FmnmxR => {
                        let o = op!(InstFmnmxR);
                        (o.dest(), self.fr(o.src_a()), self.fr(o.src_b()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.src_pred(), o.src_pred_inv())
                    }
                    Op::FmnmxI => {
                        let o = op!(InstFmnmxI);
                        (o.dest(), self.fr(o.src_a()), fimm((o.imm20() as u32) << 12), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.src_pred(), o.src_pred_inv())
                    }
                    _ => {
                        let o = op!(InstFmnmxC);
                        (o.dest(), self.fr(o.src_a()), self.fcb(o.cbuf_slot(), o.cbuf_offset()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.src_pred(), o.src_pred_inv())
                    }
                };
                let (a, b) = (Self::fmod(a, abs_a, neg_a), Self::fmod(b, abs_b, neg_b));
                let p = self.p(sp, spi);
                self.setf(d, format!("({p} ? fmin({a}, {b}) : fmax({a}, {b}))"));
            }
            Op::Mufu => {
                let o = op!(InstMufu);
                let a = Self::fmod(self.fr(o.src_a()), o.abs_a(), o.neg_a());
                let e = match o.mufu_op() {
                    MufuOp::Cos => format!("cos({a})"),
                    MufuOp::Sin => format!("sin({a})"),
                    MufuOp::Ex2 => format!("exp2({a})"),
                    MufuOp::Lg2 => format!("log2({a})"),
                    MufuOp::Rcp | MufuOp::Rcp64h => format!("(1.0f / {a})"),
                    MufuOp::Rsq | MufuOp::Rsq64h => format!("m8_rsqrt({a})"),
                    MufuOp::Sqrt => format!("sqrt({a})"),
                    m => bail!("mufu op {m:?}"),
                };
                self.setf(o.dest(), Self::sat(e, o.sat()));
            }
            Op::RroR | Op::RroC => {
                let (d, b, abs_b, neg_b) = if i.op == Op::RroR {
                    let o = op!(InstRroR);
                    (o.dest(), self.fr(o.src_b()), o.abs_b(), o.neg_b())
                } else {
                    let o = op!(InstRroC);
                    (o.dest(), self.fcb(o.cbuf_slot(), o.cbuf_offset()), o.abs_b(), o.neg_b())
                };
                self.setf(d, Self::fmod(b, abs_b, neg_b));
            }
            Op::FsetpR | Op::FsetpI | Op::FsetpC => {
                let (dp, dpi, a, b, aa, ab, na, nb, cmp, sp, spi, bop) = match i.op {
                    Op::FsetpR => {
                        let o = op!(InstFsetpR);
                        (o.dest_pred(), o.dest_pred_inv(), self.fr(o.src_a()), self.fr(o.src_b()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.f_comp().0, o.src_pred(), o.src_pred_inv(), o.bop().0)
                    }
                    Op::FsetpI => {
                        let o = op!(InstFsetpI);
                        (o.dest_pred(), o.dest_pred_inv(), self.fr(o.src_a()), fimm((o.imm20() as u32) << 12), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.f_comp().0, o.src_pred(), o.src_pred_inv(), o.bop().0)
                    }
                    _ => {
                        let o = op!(InstFsetpC);
                        (o.dest_pred(), o.dest_pred_inv(), self.fr(o.src_a()), self.fcb(o.cbuf_slot(), o.cbuf_offset()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.f_comp().0, o.src_pred(), o.src_pred_inv(), o.bop().0)
                    }
                };
                let c = format!("fcmp({cmp}u, {}, {})", Self::fmod(a, aa, na), Self::fmod(b, ab, nb));
                let p = self.p(sp, spi);
                self.line(format!("{{ bool c = {c};"));
                self.setp(dp, format!("bop({bop}u, c, {p})"));
                self.setp(dpi, format!("bop({bop}u, !c, {p})"));
                self.line("}");
            }
            Op::FsetR | Op::FsetI | Op::FsetC => {
                let (d, a, b, aa, ab, na, nb, cmp, sp, spi, bop) = match i.op {
                    Op::FsetR => {
                        let o = op!(InstFsetR);
                        (o.dest(), self.fr(o.src_a()), self.fr(o.src_b()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.f_comp().0, o.src_pred(), o.src_pred_inv(), o.bop().0)
                    }
                    Op::FsetI => {
                        let o = op!(InstFsetI);
                        (o.dest(), self.fr(o.src_a()), fimm((o.imm20() as u32) << 12), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.f_comp().0, o.src_pred(), o.src_pred_inv(), o.bop().0)
                    }
                    _ => {
                        let o = op!(InstFsetC);
                        (o.dest(), self.fr(o.src_a()), self.fcb(o.cbuf_slot(), o.cbuf_offset()), o.abs_a(), o.abs_b(), o.neg_a(), o.neg_b(), o.f_comp().0, o.src_pred(), o.src_pred_inv(), o.bop().0)
                    }
                };
                let bf = (w >> 52) & 1 != 0;
                let p = self.p(sp, spi);
                let c = format!("bop({bop}u, fcmp({cmp}u, {}, {}), {p})", Self::fmod(a, aa, na), Self::fmod(b, ab, nb));
                let v = if bf { format!("({c} ? 0x3f800000u : 0u)") } else { format!("({c} ? 0xffffffffu : 0u)") };
                self.set(d, v);
            }
            Op::Fswzadd => {
                let o = op!(InstFswzadd);
                let (a, b) = (self.fr(o.src_a()), self.fr(o.src_b()));
                self.setf(o.dest(), format!("({a} + {b})"));
            }

            Op::Hadd2I => {
                let o = op!(InstHadd2I);
                let a = self.r(o.src_a());
                let imm = (((o.bimm_h1() as u32) << 6) << 16) | ((o.bimm_h0() as u32) << 6);
                self.set(o.dest(), format!("packHalf2x16(unpackHalf2x16({a}) + unpackHalf2x16({imm:#x}u))"));
            }
            Op::Hsetp2R => {
                let o = op!(InstHsetp2R);
                let (a, b) = (self.r(o.src_a()), self.r(o.src_b()));
                let p = self.p(o.src_pred(), o.src_pred_inv());
                let cmp = o.f_comp2().0;
                self.line(format!("{{ half2 ha = unpackHalf2x16({a}), hb = unpackHalf2x16({b}); bool c0 = fcmp({cmp}u, float(ha.x), float(hb.x)), c1 = fcmp({cmp}u, float(ha.y), float(hb.y));"));
                let bop = o.bop().0;
                if o.h_and() {
                    self.setp(o.dest_pred(), format!("bop({bop}u, c0 && c1, {p})"));
                    self.setp(o.dest_pred_inv(), format!("bop({bop}u, !(c0 && c1), {p})"));
                } else {
                    self.setp(o.dest_pred(), format!("bop({bop}u, c0, {p})"));
                    self.setp(o.dest_pred_inv(), format!("bop({bop}u, c1, {p})"));
                }
                self.line("}");
            }

            Op::IaddR | Op::IaddI | Op::IaddC | Op::Iadd32i => {
                let (d, a, b, avg, x, cc) = match i.op {
                    Op::IaddR => {
                        let o = op!(InstIaddR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.avg_mode().0, o.x(), o.write_cc())
                    }
                    Op::IaddI => {
                        let o = op!(InstIaddI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.avg_mode().0, o.x(), o.write_cc())
                    }
                    Op::IaddC => {
                        let o = op!(InstIaddC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.avg_mode().0, o.x(), o.write_cc())
                    }
                    _ => {
                        let o = op!(InstIadd32i);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", o.imm32() as u32), o.avg_mode().0, o.x(), o.write_cc())
                    }
                };
                let a = if avg == 2 { format!("(0u - {a})") } else { a };
                let b = if avg == 1 { format!("(0u - {b})") } else { b };
                let one = if avg == 3 { " + 1u" } else { "" };
                let carry = if x {
                    self.uses_cc = true;
                    " + uint(cf)"
                } else {
                    ""
                };
                if cc {
                    self.uses_cc = true;
                    self.line(format!("{{ ulong t = ulong({a}) + ulong({b}){one}{carry}; cf = (t >> 32) != 0ul; uint rr = uint(t); vf = (int((rr ^ {a}) & ~({a} ^ {b}))) < 0; zf = rr == 0u; nf = int(rr) < 0;"));
                    self.set(d, "rr".into());
                    self.line("}");
                } else {
                    self.set(d, format!("({a} + {b}{one}{carry})"));
                }
            }
            Op::Iadd3R | Op::Iadd3I | Op::Iadd3C => {
                let (d, a, b, c, na, nb, nc, lrs, ap, bp, cp) = match i.op {
                    Op::Iadd3R => {
                        let o = op!(InstIadd3R);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), self.r(o.src_c()), o.neg_a(), o.neg_b(), o.neg_c(), o.lrs().0, o.apart().0, o.bpart().0, o.cpart().0)
                    }
                    Op::Iadd3I => {
                        let o = op!(InstIadd3I);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), self.r(o.src_c()), o.neg_a(), o.neg_b(), o.neg_c(), 0, 0, 0, 0)
                    }
                    _ => {
                        let o = op!(InstIadd3C);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), self.r(o.src_c()), o.neg_a(), o.neg_b(), o.neg_c(), 0, 0, 0, 0)
                    }
                };
                let part = |x: String, p: u32| match p {
                    1 => format!("({x} & 0xffffu)"),
                    2 => format!("({x} >> 16)"),
                    _ => x,
                };
                let ng = |x: String, n: bool| if n { format!("(0u - {x})") } else { x };
                let (a, b, c) = (ng(part(a, ap), na), ng(part(b, bp), nb), ng(part(c, cp), nc));
                let ab = match lrs {
                    1 => format!("(({a} + {b}) >> 16)"),
                    2 => format!("(({a} + {b}) << 16)"),
                    _ => format!("({a} + {b})"),
                };
                self.set(d, format!("({ab} + {c})"));
            }
            Op::IscaddR | Op::IscaddI | Op::IscaddC => {
                let (d, a, b, sh, avg) = match i.op {
                    Op::IscaddR => {
                        let o = op!(InstIscaddR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.imm5(), o.avg_mode().0)
                    }
                    Op::IscaddI => {
                        let o = op!(InstIscaddI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.imm5(), o.avg_mode().0)
                    }
                    _ => {
                        let o = op!(InstIscaddC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.imm5(), o.avg_mode().0)
                    }
                };
                let a = if avg == 2 { format!("(0u - {a})") } else { a };
                let b = if avg == 1 { format!("(0u - {b})") } else { b };
                self.set(d, format!("(({a} << {sh}u) + {b})"));
            }
            Op::XmadR | Op::XmadI | Op::XmadC | Op::XmadRc => {
                let (d, a, b, c, cop, sa, sb, ha, hb, psl, mrg, x, write_cc) = match i.op {
                    Op::XmadR => {
                        let o = op!(InstXmadR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), self.r(o.src_c()), o.xmad_cop().0, o.a_signed(), o.b_signed(), o.hilo_a(), o.hilo_b(), o.psl(), o.mrg(), o.x(), o.write_cc())
                    }
                    Op::XmadI => {
                        let o = op!(InstXmadI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", o.imm16() as u32 & 0xffff), self.r(o.src_c()), o.xmad_cop().0, o.a_signed(), o.b_signed(), o.hilo_a(), false, o.psl(), o.mrg(), o.x(), o.write_cc())
                    }
                    Op::XmadC => {
                        let o = op!(InstXmadC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), self.r(o.src_c()), o.xmad_cop().0, o.a_signed(), o.b_signed(), o.hilo_a(), o.hilo_b(), o.psl(), o.mrg(), o.x(), o.write_cc())
                    }
                    _ => {
                        let o = op!(InstXmadRc);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_c()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.xmad_cop().0, o.a_signed(), o.b_signed(), o.hilo_a(), o.hilo_b(), false, false, o.x(), o.write_cc())
                    }
                };
                let carry = if x {
                    self.uses_cc = true;
                    "uint(cf)"
                } else {
                    "0xffffffffu"
                };
                let v = format!("xmad({a}, {b}, {c}, {cop}u, {sa}, {sb}, {ha}, {hb}, {psl}, {mrg}, {carry})");
                if write_cc {

                    self.uses_cc = true;
                    let p = format!("xmad({a}, {b}, 0u, 0u, {sa}, {sb}, {ha}, {hb}, {psl}, false, 0xffffffffu)");
                    self.line(format!("{{ uint xr = {v}; uint xp = {p}; uint xc = {c}; cf = xr < xp; vf = int((xr ^ xp) & ~(xp ^ xc)) < 0; zf = xr == 0u; nf = int(xr) < 0;"));
                    self.set(d, "xr".into());
                    self.line("}");
                } else {
                    self.set(d, v);
                }
            }
            Op::ImnmxR | Op::ImnmxI | Op::ImnmxC => {
                let (d, a, b, s, sp, spi) = match i.op {
                    Op::ImnmxR => {
                        let o = op!(InstImnmxR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.signed(), o.src_pred(), o.src_pred_inv())
                    }
                    Op::ImnmxI => {
                        let o = op!(InstImnmxI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.signed(), o.src_pred(), o.src_pred_inv())
                    }
                    _ => {
                        let o = op!(InstImnmxC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.signed(), o.src_pred(), o.src_pred_inv())
                    }
                };
                let p = self.p(sp, spi);
                let e = if s {
                    format!("({p} ? uint(min(int({a}), int({b}))) : uint(max(int({a}), int({b}))))")
                } else {
                    format!("({p} ? min({a}, {b}) : max({a}, {b}))")
                };
                self.set(d, e);
            }
            Op::IsetpR | Op::IsetpI | Op::IsetpC => {
                let (a, b, cmp, s, bop, sp, spi, dp, dpi) = match i.op {
                    Op::IsetpR => {
                        let o = op!(InstIsetpR);
                        (self.r(o.src_a()), self.r(o.src_b()), o.i_comp().0, o.signed(), o.bop().0, o.src_pred(), o.src_pred_inv(), o.dest_pred(), o.dest_pred_inv())
                    }
                    Op::IsetpI => {
                        let o = op!(InstIsetpI);
                        (self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.i_comp().0, o.signed(), o.bop().0, o.src_pred(), o.src_pred_inv(), o.dest_pred(), o.dest_pred_inv())
                    }
                    _ => {
                        let o = op!(InstIsetpC);
                        (self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.i_comp().0, o.signed(), o.bop().0, o.src_pred(), o.src_pred_inv(), o.dest_pred(), o.dest_pred_inv())
                    }
                };
                let p = self.p(sp, spi);
                self.line(format!("{{ bool c = icmp({cmp}u, {a}, {b}, {s});"));
                self.setp(dp, format!("bop({bop}u, c, {p})"));
                self.setp(dpi, format!("bop({bop}u, !c, {p})"));
                self.line("}");
            }
            Op::IsetR | Op::IsetI | Op::IsetC => {
                let (d, a, b, cmp, s, bop, sp, spi, bv) = match i.op {
                    Op::IsetR => {
                        let o = op!(InstIsetR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.i_comp().0, o.signed(), o.bop().0, o.src_pred(), o.src_pred_inv(), o.b_val())
                    }
                    Op::IsetI => {
                        let o = op!(InstIsetI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.i_comp().0, o.signed(), o.bop().0, o.src_pred(), o.src_pred_inv(), o.b_val())
                    }
                    _ => {
                        let o = op!(InstIsetC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.i_comp().0, o.signed(), o.bop().0, o.src_pred(), o.src_pred_inv(), o.b_val())
                    }
                };
                let p = self.p(sp, spi);
                let c = format!("bop({bop}u, icmp({cmp}u, {a}, {b}, {s}), {p})");
                self.set(d, if bv { format!("({c} ? 0x3f800000u : 0u)") } else { format!("({c} ? 0xffffffffu : 0u)") });
            }
            Op::SelR | Op::SelI | Op::SelC => {
                let (d, a, b, sp, spi) = match i.op {
                    Op::SelR => {
                        let o = op!(InstSelR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.src_pred(), o.src_pred_inv())
                    }
                    Op::SelI => {
                        let o = op!(InstSelI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.src_pred(), o.src_pred_inv())
                    }
                    _ => {
                        let o = op!(InstSelC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.src_pred(), o.src_pred_inv())
                    }
                };
                let p = self.p(sp, spi);
                self.set(d, format!("({p} ? {a} : {b})"));
            }

            Op::LopR | Op::LopI | Op::LopC | Op::Lop32i => {
                let (d, a, b, lop, na, nb, dp, pop, cc) = match i.op {
                    Op::LopR => {
                        let o = op!(InstLopR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.lop().0, o.neg_a(), o.neg_b(), o.dest_pred(), o.predicate_op().0, o.write_cc())
                    }
                    Op::LopI => {
                        let o = op!(InstLopI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.logic_op().0, o.neg_a(), o.neg_b(), o.dest_pred(), o.predicate_op().0, o.write_cc())
                    }
                    Op::LopC => {
                        let o = op!(InstLopC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.logic_op().0, o.neg_a(), o.neg_b(), o.dest_pred(), o.predicate_op().0, o.write_cc())
                    }
                    _ => {
                        let o = op!(InstLop32i);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", o.imm32() as u32), o.logic_op().0, o.neg_a(), o.neg_b(), 7, 0, o.write_cc())
                    }
                };
                let a = if na { format!("(~{a})") } else { a };
                let b = if nb { format!("(~{b})") } else { b };
                let e = match lop {
                    0 => format!("({a} & {b})"),
                    1 => format!("({a} | {b})"),
                    2 => format!("({a} ^ {b})"),
                    _ => b,
                };
                self.line(format!("{{ uint v = {e};"));
                self.setp(dp, match pop {
                    0 => "false".into(),
                    1 => "true".into(),
                    2 => "v == 0u".into(),
                    _ => "v != 0u".into(),
                });
                if cc {
                    self.set_zn("v".into());
                }
                self.set(d, "v".into());
                self.line("}");
            }
            Op::Lop3R | Op::Lop3I | Op::Lop3C => {
                let (d, a, b, c, lut, dp, pop) = match i.op {
                    Op::Lop3R => {
                        let o = op!(InstLop3R);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), self.r(o.src_c()), o.imm(), o.dest_pred(), o.predicate_op().0)
                    }
                    Op::Lop3I => {
                        let o = op!(InstLop3I);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), self.r(o.src_c()), o.imm(), 7, 0)
                    }
                    _ => {
                        let o = op!(InstLop3C);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), self.r(o.src_c()), o.imm(), 7, 0)
                    }
                };
                self.line(format!("{{ uint v = lop3({a}, {b}, {c}, {lut}u);"));
                self.setp(dp, match pop {
                    0 => "false".into(),
                    1 => "true".into(),
                    2 => "v == 0u".into(),
                    _ => "v != 0u".into(),
                });
                self.set(d, "v".into());
                self.line("}");
            }
            Op::ShlR | Op::ShlI | Op::ShlC => {
                let (d, a, b, m) = match i.op {
                    Op::ShlR => {
                        let o = op!(InstShlR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.m())
                    }
                    Op::ShlI => {
                        let o = op!(InstShlI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.m())
                    }
                    _ => {
                        let o = op!(InstShlC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.m())
                    }
                };
                let e = if m { format!("({a} << ({b} & 31u))") } else { format!("({b} >= 32u ? 0u : {a} << {b})") };
                self.set(d, e);
            }
            Op::ShrR | Op::ShrI | Op::ShrC => {
                let (d, a, b, s, m, brev) = match i.op {
                    Op::ShrR => {
                        let o = op!(InstShrR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.signed(), o.m(), o.brev())
                    }
                    Op::ShrI => {
                        let o = op!(InstShrI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.signed(), o.m(), o.brev())
                    }
                    _ => {
                        let o = op!(InstShrC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.signed(), o.m(), o.brev())
                    }
                };
                let a = if brev { format!("bitfieldReverse({a})") } else { a };
                let sh = if m { format!("({b} & 31u)") } else { format!("min({b}, 31u)") };
                let e = if s {
                    format!("uint(int({a}) >> {sh})")
                } else if m {
                    format!("({a} >> {sh})")
                } else {
                    format!("({b} >= 32u ? 0u : {a} >> {b})")
                };
                self.set(d, e);
            }
            Op::BfeR | Op::BfeI | Op::BfeC => {
                let (d, a, b, s) = match i.op {
                    Op::BfeR => {
                        let o = op!(InstBfeR);
                        (o.dest(), self.r(o.src_a()), self.r(o.src_b()), o.signed())
                    }
                    Op::BfeI => {
                        let o = op!(InstBfeI);
                        (o.dest(), self.r(o.src_a()), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.signed())
                    }
                    _ => {
                        let o = op!(InstBfeC);
                        (o.dest(), self.r(o.src_a()), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.signed())
                    }
                };
                self.set(d, format!("bfe({a}, {b}, {s})"));
            }

            Op::I2fR | Op::I2fI | Op::I2fC => {
                let (d, b, fmt, sel, abs, neg) = match i.op {
                    Op::I2fR => {
                        let o = op!(InstI2fR);
                        (o.dest(), self.r(o.src_b()), o.i_src_fmt().0, o.byte_sel().0, o.abs_b(), o.neg_b())
                    }
                    Op::I2fI => {
                        let o = op!(InstI2fI);
                        (o.dest(), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.i_src_fmt().0, o.byte_sel().0, o.abs_b(), o.neg_b())
                    }
                    _ => {
                        let o = op!(InstI2fC);
                        (o.dest(), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.i_src_fmt().0, o.byte_sel().0, o.abs_b(), o.neg_b())
                    }
                };
                self.setf(d, Self::fmod(format!("i2f({b}, {fmt}u, {sel}u)"), abs, neg));
            }
            Op::F2iR | Op::F2iI | Op::F2iC => {
                let (d, b, abs, neg, sh, dst, src, rnd) = match i.op {
                    Op::F2iR => {
                        let o = op!(InstF2iR);
                        (o.dest(), self.r(o.src_b()), o.abs_b(), o.neg_b(), o.sh(), o.i_dst_fmt().0, o.src_fmt().0, o.round_mode().0)
                    }
                    Op::F2iI => {
                        let o = op!(InstF2iI);
                        (o.dest(), format!("{:#x}u", (o.imm20() as u32) << 12), o.abs_b(), o.neg_b(), o.sh(), o.i_dst_fmt().0, o.src_fmt().0, o.round_mode().0)
                    }
                    _ => {
                        let o = op!(InstF2iC);
                        (o.dest(), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.abs_b(), o.neg_b(), o.sh(), o.i_dst_fmt().0, o.src_fmt().0, o.round_mode().0)
                    }
                };
                let x = Self::fmod(format!("tof32({b}, {src}u, {sh})"), abs, neg);
                self.set(d, format!("f2i({x}, {rnd}u, {dst}u)"));
            }
            Op::F2fR | Op::F2fI | Op::F2fC => {
                let (d, b, abs, neg, sh, dst, src, rnd, s) = match i.op {
                    Op::F2fR => {
                        let o = op!(InstF2fR);
                        (o.dest(), self.r(o.src_b()), o.abs_b(), o.neg_b(), o.sh(), o.dst_fmt().0, o.src_fmt().0, o.round_mode().0, o.sat())
                    }
                    Op::F2fI => {
                        let o = op!(InstF2fI);
                        (o.dest(), format!("{:#x}u", (o.imm20() as u32) << 12), o.abs_b(), o.neg_b(), o.sh(), o.dst_fmt().0, o.src_fmt().0, o.round_mode().0, o.sat())
                    }
                    _ => {
                        let o = op!(InstF2fC);
                        (o.dest(), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.abs_b(), o.neg_b(), o.sh(), o.dst_fmt().0, o.src_fmt().0, o.round_mode().0, o.sat())
                    }
                };
                let x = Self::sat(Self::fmod(format!("fround(tof32({b}, {src}u, {sh}), {rnd}u)"), abs, neg), s);
                if dst == 1 {
                    self.set(d, format!("as_uint(half2(half({x}), 0.0h))"));
                } else {
                    self.setf(d, x);
                }
            }
            Op::I2iR | Op::I2iI | Op::I2iC => {
                let (d, b, sel, src, dst, s, abs, neg) = match i.op {
                    Op::I2iR => {
                        let o = op!(InstI2iR);
                        (o.dest(), self.r(o.src_b()), o.byte_sel().0, o.i_src_fmt().0, o.i_dst_fmt().0, o.sat(), o.abs_b(), o.neg_b())
                    }
                    Op::I2iI => {
                        let o = op!(InstI2iI);
                        (o.dest(), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32), o.byte_sel().0, o.i_src_fmt().0, o.i_dst_fmt().0, o.sat(), o.abs_b(), o.neg_b())
                    }
                    _ => {
                        let o = op!(InstI2iC);
                        (o.dest(), self.cb(o.cbuf_slot(), o.cbuf_offset()), o.byte_sel().0, o.i_src_fmt().0, o.i_dst_fmt().0, o.sat(), o.abs_b(), o.neg_b())
                    }
                };
                self.set(d, format!("i2i({b}, {sel}u, {src}u, {dst}u, {s}, {abs}, {neg})"));
            }

            Op::MovR | Op::MovI | Op::MovC | Op::Mov32i => {
                let (d, v) = match i.op {
                    Op::MovR => {
                        let o = op!(InstMovR);
                        (o.dest(), self.r(o.src_a()))
                    }
                    Op::MovI => {
                        let o = op!(InstMovI);
                        (o.dest(), format!("{:#x}u", ((o.imm20() << 12) >> 12) as u32))
                    }
                    Op::MovC => {
                        let o = op!(InstMovC);
                        if o.cbuf_slot() == TEXTURE_HANDLE_BANK && o.cbuf_offset() * 4 >= 0x20 {
                            self.handle_regs.insert(o.dest(), ((o.cbuf_offset() * 4 - 0x20) / 8) as u32);
                        }
                        (o.dest(), self.cb(o.cbuf_slot(), o.cbuf_offset()))
                    }
                    _ => {
                        let o = op!(InstMov32i);
                        (o.dest(), format!("{:#x}u", o.imm32() as u32))
                    }
                };
                self.set(d, v);
            }
            Op::Ldc => {
                let o = op!(InstLdc);
                let mut a = self.r(o.src_a());

                let slot = if matches!(o.address_mode().0, 2 | 3) {
                    let s = format!("min({}u + ({a} >> 16), 17u)", o.cbuf_slot());
                    a = format!("({a} & 0xffffu)");
                    s
                } else {
                    o.cbuf_slot().to_string()
                };

                let off = (o.cbuf_offset() as u16 as i16) as i32;
                let addr = format!("({a} + {:#x}u)", off as u32);
                match o.ls_size().0 {
                    5 => {

                        self.line(format!("{{ UintRef cfgp = UintRef(res.cb[{slot}]); uint2 cv = uint2(cfgp.data[{addr} >> 2], cfgp.data[({addr} >> 2) + 1u]);"));
                        self.set(o.dest(), "cv.x".into());
                        if o.dest() != RZ_ {
                            self.set(o.dest() + 1, "cv.y".into());
                        }
                        self.line("}");
                    }
                    4 => self.set(o.dest(), format!("UintRef(res.cb[{slot}]).data[{addr} >> 2]")),
                    sz => {
                        let (bits, signed) = match sz {
                            0 => (8, false),
                            1 => (8, true),
                            2 => (16, false),
                            _ => (16, true),
                        };
                        self.set(o.dest(), format!("bfe(UintRef(res.cb[{slot}]).data[{addr} >> 2], ((({addr} & 3u) * 8u) | ({bits}u << 8)), {signed})"));
                    }
                }
            }
            Op::Ldl | Op::Stl | Op::Lds | Op::Sts => {
                self.uses_lmem = true;
                let (d, a, off, size) = match i.op {
                    Op::Ldl => {
                        let o = op!(InstLdl);
                        (o.dest(), self.r(o.src_a()), o.imm24(), o.ls_size().0)
                    }
                    Op::Stl => {
                        let o = op!(InstStl);
                        (o.dest(), self.r(o.src_a()), o.imm24(), o.ls_size().0)
                    }
                    Op::Lds => {
                        let o = op!(InstLds);
                        (o.dest(), self.r(o.src_a()), o.imm24(), o.ls_size().0)
                    }
                    _ => {
                        let o = op!(InstSts);
                        (o.dest(), self.r(o.src_a()), o.imm24(), o.ls_size().0)
                    }
                };
                let load = matches!(i.op, Op::Ldl | Op::Lds);
                let words = match size {
                    5 => 2,
                    6 => 4,
                    _ => 1,
                };

                self.line(format!("{{ uint la = ({a} + {:#x}u) >> 2;", off as u32));
                for k in 0..words {
                    let idx = format!("(la + {k}u) & LMEM_MASK");
                    if load {
                        self.line(format!("uint lv{k} = lmem[{idx}];"));
                    } else {
                        let v = self.r(if d == RZ_ { RZ_ } else { d + k });
                        self.line(format!("lmem[{idx}] = {v};"));
                    }
                }
                if load && d != RZ_ {
                    for k in 0..words {
                        self.set(d + k, format!("lv{k}"));
                    }
                }
                self.line("}");
            }
            Op::Ldg | Op::Stg => {

                let (d, ra, imm, size, e) = if i.op == Op::Ldg {
                    let o = op!(InstLdg);
                    (o.dest(), o.src_a(), o.imm24(), o.ls_size().0, o.e())
                } else {
                    let o = op!(InstStg);
                    (o.dest(), o.src_a(), o.imm24(), o.ls_size().0, o.e())
                };
                let imm = ((imm << 8) >> 8) as i64;
                let lo = self.r(ra);
                let hi = if e && ra != RZ_ { self.r(ra + 1) } else { "0u".into() };
                self.line(format!("{{ uint64_t ga = ((uint64_t({hi}) << 32) | uint64_t({lo})) + uint64_t({imm}); UintRef gp = gmem(res, ga & ~uint64_t(3));"));
                let words = match size {
                    5 => 2,
                    6 | 7 => 4,
                    _ => 1,
                };
                if i.op == Op::Ldg {
                    if size < 4 {
                        let (bits, signed) = match size {
                            0 => (8, false),
                            1 => (8, true),
                            2 => (16, false),
                            _ => (16, true),
                        };
                        self.set(d, format!("bfe(gp.data[0], ((uint(ga & 3ul) * 8u) | ({bits}u << 8)), {signed})"));
                    } else {

                        self.line(format!("uint gv[{words}];"));
                        for k in 0..words {
                            self.line(format!("gv[{k}] = gp.data[{k}];"));
                        }
                        for k in 0..words {
                            if d != RZ_ {
                                self.set(d + k, format!("gv[{k}]"));
                            }
                        }
                    }
                } else if size >= 4 {
                    for k in 0..words {
                        let v = self.r(if d == RZ_ { RZ_ } else { d + k });
                        self.line(format!("if (ga - res.heap_base < res.heap_size) gp.data[{k}] = {v};"));
                    }
                } else {
                    self.warnings.push("sub-word global stores ignored".into());
                }
                self.line("}");
            }
            Op::Atom => {
                let o = op!(InstAtom);
                let imm = ((o.imm20() << 12) >> 12) as i64;
                let lo = self.r(o.src_a());
                let hi = if o.e() && o.src_a() != RZ_ { self.r(o.src_a() + 1) } else { "0u".into() };
                let v = self.r(o.src_b());
                self.line(format!("{{ uint64_t ga = ((uint64_t({hi}) << 32) | uint64_t({lo})) + uint64_t({imm});"));
                self.line("if (ga - res.heap_base < res.heap_size) {");
                if matches!(o.size().0, 2 | 5) {
                    self.atomic64(o.src_b(), &v, o.size().0, o.op().0, o.dest())?;
                } else {
                    self.atomic("gmem(res, ga & ~uint64_t(3)).data[0]", &v, o.size().0, o.op().0, o.dest())?;
                }
                self.line("}");
                self.line("}");
            }
            Op::Atoms => {
                let o = op!(InstAtoms);
                self.uses_lmem = true;
                let a = self.r(o.src_a());
                let v = self.r(o.src_b());
                let idx = format!("((({a} + {:#x}u) >> 2) & LMEM_MASK)", o.imm22() as u32);
                self.atomic(&format!("lmem[{idx}]"), &v, o.atoms_size().0, o.atom_op().0, o.dest())?;
            }
            Op::Red => {
                let o = op!(InstRed);
                let imm = ((o.imm20() << 12) >> 12) as i64;
                let lo = self.r(o.src_a());
                let hi = if o.e() && o.src_a() != RZ_ { self.r(o.src_a() + 1) } else { "0u".into() };
                let v = self.r(o.src_b());
                self.line(format!("{{ uint64_t ga = ((uint64_t({hi}) << 32) | uint64_t({lo})) + uint64_t({imm});"));
                self.line("if (ga - res.heap_base < res.heap_size) {");
                if matches!(o.red_size().0, 2 | 5) {
                    self.atomic64(o.src_b(), &v, o.red_size().0, o.red_op().0, RZ_)?;
                } else {
                    self.atomic("gmem(res, ga & ~uint64_t(3)).data[0]", &v, o.red_size().0, o.red_op().0, RZ_)?;
                }
                self.line("}");
                self.line("}");
            }
            Op::AtomCas | Op::AtomsCas => bail!("atomic compare-and-swap is not supported at {:#x}", i.addr),
            Op::S2r => {
                let o = op!(InstS2r);
                let compute = self.sph.stage == Stage::Compute;
                let v = match o.s_reg().0 {
                    0 => "simd_lane",

                    0x20 if compute => "(tid.x | tid.y << 16 | tid.z << 26)",
                    0x21 if compute => "tid.x",
                    0x22 if compute => "tid.y",
                    0x23 if compute => "tid.z",

                    0x25 if compute => "ctaid.x",
                    0x26 if compute => "ctaid.y",
                    0x27 if compute => "ctaid.z",
                    _ => "0u",
                };
                self.set(o.dest(), v.into());
            }

            Op::Ald => {
                let o = op!(InstAld);
                let n = o.al_size().0 as i32 + 1;
                if o.src_a() != RZ_ {
                    self.warnings.push("indexed attribute load".into());
                }
                for k in 0..n {
                    let v = self.attr_load(o.imm11() + 4 * k, o.o());
                    self.set(o.dest() + k, v);
                }
            }
            Op::Ast => {
                let o = op!(InstAst);
                let n = o.al_size().0 as i32 + 1;
                if o.src_a() != RZ_ {
                    self.warnings.push("indexed attribute store".into());
                }
                for k in 0..n {
                    let v = self.fr(o.src_b() + k);
                    match self.vs_output_ref(o.imm11() + 4 * k) {
                        Some(lv) => self.line(format!("{lv} = {v};")),
                        None => self.warnings.push(format!("attribute store {:#x}", o.imm11() + 4 * k)),
                    }
                }
            }
            Op::Ipa => {
                let o = op!(InstIpa);
                let mut v = if o.idx() {
                    self.warnings.push("indexed ipa".into());
                    "0u".into()
                } else {
                    self.fs_input(o.imm10(), true)
                };
                let fixed = matches!(o.imm10(), ATTR_POSITION..=0x7c | ATTR_FRONT_FACING);
                if o.ipa_op() == IpaOp::Multiply && !fixed {
                    let b = self.fr(o.src_b());
                    v = format!("as_uint(as_float({v}) * {b})");
                }
                if o.sat() {
                    v = format!("as_uint(m8_saturate(as_float({v})))");
                }
                self.set(o.dest(), v);
            }
            Op::Isberd => {
                let o = op!(InstIsberd);
                let a = self.r(o.src_a());
                self.set(o.dest(), a);
            }
            Op::OutR | Op::OutI | Op::OutC => {
                self.warnings.push("geometry OUT".into());
            }

            Op::Texs => {
                let o = op!(InstTexs);
                self.texs(o)?;
            }
            Op::Tex => {
                let o = op!(InstTex);
                self.tex_full(o)?;
            }
            Op::Tlds => {
                let o = op!(InstTlds);
                self.tlds(o)?;
            }
            Op::Tld4 | Op::Tld4s => self.tld4(i)?,
            Op::TmmlB | Op::Tmml => {
                let (d, a, mask, dim, tid) = if i.op == Op::Tmml {
                    let o = op!(InstTmml);
                    (o.dest(), o.src_a(), o.w_mask() as u32, o.dim().0, o.tid_b())
                } else {

                    let o = op!(InstTmmlB);
                    let Some(&binding) = self.handle_regs.get(&o.src_b()) else {
                        self.warnings.push("unresolved bindless texture".into());
                        let dd = o.dest();
                        self.set(dd, "0u".into());
                        return Ok(());
                    };
                    (o.dest(), o.src_a(), o.w_mask() as u32, o.dim().0, 8 + 2 * binding as i32)
                };
                let kind = match dim {
                    3 => TexKind::T2DArray,
                    4 => TexKind::T3D,
                    6 => TexKind::Cube,
                    _ => TexKind::T2D,
                };
                let b = self.tex(tid, kind);
                let ra: Vec<i32> = (0..3).map(|k| if a == RZ_ { RZ_ } else { a + k }).collect();
                let coords: Vec<i32> = if kind.is_array() { ra[1..].to_vec() } else { ra };
                let c = self.coord(&coords, kind.coords());

                self.line(format!("{{ float l = calculate_clamped_lod(t{b}, s{b}, {c}); float4 v = float4(l, l, 0.0f, 0.0f);"));
                let dests: Vec<i32> = (0..4).map(|k| if d == RZ_ { RZ_ } else { d + k }).collect();
                self.write_components("v", mask, &dests);
                self.line("}");
            }
            Op::Txq | Op::TxqB => {
                let (d, a, mask, tid, query) = if i.op == Op::Txq {
                    let o = op!(InstTxq);
                    (o.dest(), o.src_a(), o.w_mask() as u32, o.tid_b(), o.tex_query().0)
                } else {
                    let o = op!(InstTxqB);
                    (o.dest(), o.src_a(), o.w_mask() as u32, 8, o.tex_query().0)
                };
                let binding = ((tid - 8) / 2).max(0) as u32;
                let kind = self.iface.textures.get(&binding).copied().unwrap_or(TexKind::T2D);
                let b = self.tex(tid, kind);
                let lod = self.r(a);
                if query == 1 {

                    let ts = self.tsamp(b, kind);
                    self.line(format!("{{ uint l = {lod}; ivec3 s = m8_texsize(t{b}, s{b}, int(l)); uint4 v = uint4(s, textureQueryLevels({ts}));"));
                } else {
                    self.line("{ uint4 v = uint4(0u);");
                }
                let dests: Vec<i32> = (0..4).map(|k| if d == RZ_ { RZ_ } else { d + k }).collect();
                let mut k = 0;
                for c in 0..4 {
                    if mask >> c & 1 != 0 {
                        if let Some(&dd) = dests.get(k) {
                            self.set(dd, format!("v[{c}]"));
                        }
                        k += 1;
                    }
                }
                self.line("}");
            }

            Op::Psetp => {
                let o = op!(InstPsetp);
                let (a, b, c) = (self.p(o.src_pred(), o.src_pred_inv()), self.p(o.src2_pred(), o.src2_pred_inv()), self.p(o.src1_pred(), o.src1_pred_inv()));
                let _ = c;
                let (ab, cop) = (o.bool_op_ab().0, o.bool_op_c().0);
                let src_c = self.p(o.src_pred(), o.src_pred_inv());
                let (pa, pb) = (self.p(o.src1_pred(), o.src1_pred_inv()), self.p(o.src2_pred(), o.src2_pred_inv()));
                let _ = (a, b);
                self.line(format!("{{ bool t = bop({ab}u, {pa}, {pb});"));
                self.setp(o.dest_pred(), format!("bop({cop}u, t, {src_c})"));
                self.setp(o.dest_pred_inv(), format!("bop({cop}u, !t, {src_c})"));
                self.line("}");
            }
            Op::Pset => {
                let o = op!(InstPset);
                let (pa, pb) = (self.p(o.src1_pred(), o.src1_pred_inv()), self.p(o.src2_pred(), o.src2_pred_inv()));
                let pc = self.p(o.src_pred(), o.src_pred_inv());
                let c = format!("bop({}u, bop({}u, {pa}, {pb}), {pc})", o.bool_op_c().0, o.bool_op_ab().0);
                self.set(o.dest(), if o.b_val() { format!("({c} ? 0x3f800000u : 0u)") } else { format!("({c} ? 0xffffffffu : 0u)") });
            }
            Op::Csetp => {
                let o = op!(InstCsetp);
                self.uses_cc = true;
                let p = self.p(o.src_pred(), o.src_pred_inv());
                let c = format!("ccc({}u, zf, nf, cf, vf)", o.ccc().0);
                self.setp(o.dest_pred(), format!("bop({}u, {c}, {p})", o.bop().0));
                self.setp(o.dest_pred_inv(), format!("bop({}u, !{c}, {p})", o.bop().0));
            }
            Op::Vsetp => {
                let o = op!(InstVsetp);
                self.setp(o.dest_pred(), "false".into());
                self.setp(o.dest_pred_inv(), "true".into());
            }
            Op::Vmad => {
                let o = op!(InstVmad);
                let (a, b, c) = (self.r(o.src_a()), self.r(o.src_b()), self.r(o.src_c()));
                self.set(o.dest(), format!("(({a} & 0xffffu) * ({b} & 0xffffu) + {c})"));
            }
            Op::Shfl => {
                let o = op!(InstShfl);
                let a = self.r(o.src_a());
                let lane = if o.b_fix_shfl() { format!("{}u", o.src_b_imm()) } else { self.r(o.src_b()) };
                let f = ["simd_shuffle", "simd_shuffle_up", "simd_shuffle_down", "simd_shuffle_xor"][o.shfl_mode().0 as usize & 3];
                self.set(o.dest(), format!("{f}({a}, uint({lane}))"));
            }

            Op::Bra => {
                let o = op!(InstBra);
                let t = branch_target(i);
                if o.ccc() != Ccc::T {
                    self.uses_cc = true;
                    self.line(format!("if (ccc({}u, zf, nf, cf, vf)) {{ pc = {t}u; continue; }}", o.ccc().0));
                } else {
                    self.label_target(t);
                }
            }
            Op::Sync | Op::Brk => match self.prog.converge.get(&i.addr) {
                Some(&t) => self.label_target(t),
                None => {
                    self.warnings.push(format!("unresolved {:?} at {:#x}", i.name, i.addr));
                    self.line("return epilogue();");
                }
            },
            Op::Exit => {
                let o = op!(InstExit);
                if o.ccc() != Ccc::T {
                    self.uses_cc = true;
                    self.line(format!("if (ccc({}u, zf, nf, cf, vf)) return epilogue();", o.ccc().0));
                } else {
                    self.line("return epilogue();");
                }
            }
            Op::Kil => {
                let o = op!(InstKil);
                if o.ccc() != Ccc::T {
                    self.uses_cc = true;
                    self.line(format!("if (ccc({}u, zf, nf, cf, vf)) discard;", o.ccc().0));
                } else {
                    self.line("discard;");
                }
            }
            other => bail!("unimplemented instruction {other:?} ({:?}) at {:#x}", i.name, i.addr),
        }
        Ok(())
    }

    fn texs(&mut self, o: InstTexs) -> Result<()> {
        let (dest, dest2) = (o.dest(), o.dest2());
        if dest == RZ_ && dest2 == RZ_ {
            return Ok(());
        }
        let mut ra = o.src_a();
        let mut rb = o.src_b();
        let mut next_a = |g: &mut Self| {
            let r = ra;
            if ra != RZ_ {
                ra += 1;
            }
            let _ = g;
            r
        };
        let mut a_regs = Vec::new();
        let mut b_regs = Vec::new();
        let t = o.target().0;
        let kind = match t {
            4..=6 => TexKind::Depth2D,
            9 => TexKind::Depth2DArray,
            7 | 8 => TexKind::T2DArray,
            10 | 11 => TexKind::T3D,
            12 | 13 => TexKind::Cube,
            _ => TexKind::T2D,
        };
        let mut lod = None;
        let mut depth_ref = None;
        let mut array = None;
        let take_b = |rb: &mut i32| {
            let r = *rb;
            if *rb != RZ_ {
                *rb += 1;
            }
            r
        };
        match t {
            7 | 8 | 9 => {

                array = Some(next_a(self));
                a_regs.push(next_a(self));
                b_regs.push(take_b(&mut rb));
                if t == 9 {
                    depth_ref = Some(take_b(&mut rb));
                }
                if t != 7 {
                    lod = Some("0.0f".to_string());
                }
            }
            0 => {
                a_regs.push(next_a(self));
                lod = Some("0.0f".into());
            }
            1 => {
                a_regs.push(next_a(self));
                b_regs.push(take_b(&mut rb));
            }
            2 => {
                a_regs.push(next_a(self));
                b_regs.push(take_b(&mut rb));
                lod = Some("0.0f".into());
            }
            3 | 4 | 10 | 12 => {
                a_regs.push(next_a(self));
                a_regs.push(next_a(self));
                let x = take_b(&mut rb);
                match t {
                    3 => {
                        let l = self.fr(x);
                        lod = Some(l);
                    }
                    4 => depth_ref = Some(x),
                    _ => b_regs.push(x),
                }
            }
            6 | 11 => {
                a_regs.push(next_a(self));
                a_regs.push(next_a(self));
                let x = take_b(&mut rb);
                if t == 6 { depth_ref = Some(x) } else { b_regs.push(x) }
                lod = Some("0.0f".into());
            }
            5 | 13 => {
                a_regs.push(next_a(self));
                a_regs.push(next_a(self));
                let x = take_b(&mut rb);
                let y = take_b(&mut rb);
                if t == 5 {
                    depth_ref = Some(x);
                    lod = Some(self.fr(y));
                } else {
                    b_regs.push(x);
                    lod = Some(self.fr(y));
                }
            }
            _ => bail!("texs target {t}"),
        }
        let coords: Vec<i32> = a_regs.into_iter().chain(b_regs).collect();
        let coords = if t == 0 { vec![coords[0], RZ_] } else { coords };
        let b = self.tex(o.tid_b(), kind);
        let v = self.sample(b, kind, &coords, array, lod, None, depth_ref);
        let mask = TEXS_MASK_LUT[if dest2 == RZ_ { 0 } else { 1 }][o.w_mask() as usize & 7];
        let dests = [dest, if dest == RZ_ { RZ_ } else { dest + 1 }, dest2, if dest2 == RZ_ { RZ_ } else { dest2 + 1 }];
        self.line(format!("{{ float4 v = {v};"));
        self.write_components("v", mask, &dests);
        self.line("}");
        Ok(())
    }

    fn tex_full(&mut self, o: InstTex) -> Result<()> {
        let d = o.dest();
        if d == RZ_ {
            return Ok(());
        }
        let (mut ra, mut rb) = (o.src_a(), o.src_b());
        let mut na = || {
            let r = ra;
            if ra != RZ_ {
                ra += 1;
            }
            r
        };
        let mut nb = || {
            let r = rb;
            if rb != RZ_ {
                rb += 1;
            }
            r
        };
        let dim = o.dim().0;
        let dc = o.dc();
        let kind = match (dim, dc) {
            (3 | 1, true) => TexKind::Depth2DArray,
            (6 | 7, true) => TexKind::DepthCube,
            (_, true) => TexKind::Depth2D,
            (3 | 1, false) => TexKind::T2DArray,
            (4 | 5, _) => TexKind::T3D,
            (6, _) => TexKind::Cube,
            (7, _) => TexKind::CubeArray,
            _ => TexKind::T2D,
        };
        let array = if matches!(dim, 1 | 3 | 5 | 7) { Some(na()) } else { None };
        let n = match dim {
            0 | 1 => 1,
            4 | 5 | 6 | 7 => 3,
            _ => 2,
        };
        let mut coords: Vec<i32> = (0..n).map(|_| na()).collect();
        if n == 1 {
            coords.push(RZ_);
        }
        let lodm = o.lod().0;
        let lod_reg = if lodm >= 2 { Some(nb()) } else { None };
        if o.aoffi() {
            let _ = nb();
            self.warnings.push("texture offsets ignored".into());
        }
        let depth_ref = if dc { Some(nb()) } else { None };
        let (lod, bias) = match lodm {
            1 => (Some("0.0f".to_string()), None),
            3 | 7 => (Some(self.fr(lod_reg.unwrap())), None),
            2 | 6 => (None, Some(self.fr(lod_reg.unwrap()))),
            _ => (None, None),
        };
        let b = self.tex(o.tid_b(), kind);
        let v = self.sample(b, kind, &coords, array, lod, bias, depth_ref);
        self.line(format!("{{ float4 v = {v};"));
        let dests: Vec<i32> = (0..4).map(|k| if d + k >= RZ_ { RZ_ } else { d + k }).collect();
        self.write_components("v", o.w_mask() as u32, &dests);
        self.line("}");
        Ok(())
    }

    fn tlds(&mut self, o: InstTlds) -> Result<()> {
        let (dest, dest2) = (o.dest(), o.dest2());
        if dest == RZ_ && dest2 == RZ_ {
            return Ok(());
        }
        let (mut ra, mut rb) = (o.src_a(), o.src_b());
        let mut na = || {
            let r = ra;
            if ra != RZ_ {
                ra += 1;
            }
            r
        };
        let mut nb = || {
            let r = rb;
            if rb != RZ_ {
                rb += 1;
            }
            r
        };
        let t = o.target().0;
        let (x, y, lod, kind, arr) = match t {
            0 => (na(), RZ_, None, TexKind::T2D, None),
            1 => (na(), RZ_, Some(nb()), TexKind::T2D, None),
            2 => (na(), nb(), None, TexKind::T2D, None),
            4 => (na(), na(), None, TexKind::T2D, None),
            5 | 0xc => (na(), na(), Some(nb()), TexKind::T2D, None),
            6 => (na(), na(), None, TexKind::T2D, None),
            7 => (na(), na(), None, TexKind::T3D, None),
            8 => {
                let x = nb();
                let y = nb();
                (x, y, None, TexKind::T2DArray, Some(na()))
            }
            _ => bail!("tlds target {t}"),
        };
        let b = self.tex(o.tid_b(), kind);
        let (xs, ys) = (self.r(x), self.r(y));
        let l = lod.map(|r| self.r(r)).unwrap_or("0u".into());
        let ts = self.tsamp(b, kind);
        let v = match (kind, arr) {
            (TexKind::T2DArray, Some(a)) => {
                let av = self.r(a);
                format!("texelFetch({ts}, ivec3(int({xs}), int({ys}), int({av})), int({l}))")
            }
            (TexKind::T3D, _) => format!("texelFetch({ts}, ivec3(int({xs}), int({ys}), 0), int({l}))"),
            _ => format!("texelFetch({ts}, ivec2(int({xs}), int({ys})), int({l}))"),
        };
        let mask = TEXS_MASK_LUT[if dest2 == RZ_ { 0 } else { 1 }][o.w_mask() as usize & 7];
        let dests = [dest, if dest == RZ_ { RZ_ } else { dest + 1 }, dest2, if dest2 == RZ_ { RZ_ } else { dest2 + 1 }];
        self.line(format!("{{ float4 v = {v};"));
        self.write_components("v", mask, &dests);
        self.line("}");
        Ok(())
    }

    fn tld4(&mut self, i: &Inst) -> Result<()> {
        let w = i.raw;
        let (d, d2, a, bsrc, comp, dc, tid, mask, two_dest) = if i.op == Op::Tld4s {
            let o = InstTld4s(w);
            (o.dest(), o.dest2(), o.src_a(), o.src_b(), o.tex_comp().0, o.dc(), o.tid_b(), 0xF, true)
        } else {
            let o = InstTld4(w);
            (o.dest(), RZ_, o.src_a(), o.src_b(), o.tex_comp().0, o.dc(), o.tid_b(), o.w_mask() as u32, false)
        };
        let kind = if dc { TexKind::Depth2D } else { TexKind::T2D };
        let b = self.tex(tid, kind);
        let coords = if two_dest && dc { vec![a, if a == RZ_ { RZ_ } else { a + 1 }] } else if two_dest { vec![a, bsrc] } else { vec![a, if a == RZ_ { RZ_ } else { a + 1 }] };
        let c = self.coord(&coords, 2);
        let comp_idx = comp & 3;
        let ts = self.tsamp(b, kind);
        let v = if dc {
            let rv = self.fr(bsrc);
            format!("gather_compare(t{b}, s{b}, {c}, {rv})")
        } else {
            format!("textureGather({ts}, {c}, {comp_idx})")
        };
        self.line(format!("{{ float4 v = {v};"));
        let dests: Vec<i32> = if two_dest {
            vec![d, if d == RZ_ { RZ_ } else { d + 1 }, d2, if d2 == RZ_ { RZ_ } else { d2 + 1 }]
        } else {
            (0..4).map(|k| if d == RZ_ { RZ_ } else { d + k }).collect()
        };
        self.write_components("v", mask, &dests);
        self.line("}");
        Ok(())
    }
}

const TEXS_MASK_LUT: [[u32; 8]; 2] = [
    [0b0001, 0b0010, 0b0100, 0b1000, 0b0011, 0b1001, 0b1010, 0b1100],
    [0b0111, 0b1011, 0b1101, 0b1110, 0b1111, 0b0000, 0b0000, 0b0000],
];

const PRELUDE: &str = r##"#version 450
#extension GL_EXT_buffer_reference : require
#extension GL_EXT_shader_explicit_arithmetic_types_int64 : require
#extension GL_EXT_shader_atomic_int64 : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_shuffle : require
#extension GL_KHR_shader_subgroup_shuffle_relative : require

#define float2 vec2
#define float3 vec3
#define float4 vec4
#define uint2 uvec2
#define uint3 uvec3
#define uint4 uvec4
#define int2 ivec2
#define int3 ivec3
#define int4 ivec4
#define half float
#define half2 vec2
#define half3 vec3
#define half4 vec4
#define packed_float2 vec2
#define packed_float3 vec3
#define packed_float4 vec4
#define packed_uint2 uvec2
#define packed_uint3 uvec3
#define packed_uint4 uvec4
#define packed_int2 ivec2
#define packed_int3 ivec3
#define packed_int4 ivec4
#define packed_ushort2 uvec2
#define packed_ushort3 uvec3
#define packed_ushort4 uvec4
#define packed_short2 ivec2
#define packed_short3 ivec3
#define packed_short4 ivec4
#define packed_half2 vec2
#define packed_half3 vec3
#define packed_half4 vec4
#define ulong uint64_t
#define rint roundEven
#define fmin min
#define fmax max
#define reverse_bits bitfieldReverse
#define fabs abs
#define front gl_FrontFacing
#define pcoord gl_PointCoord
#define simd_lane gl_SubgroupInvocationID

layout(buffer_reference, std430, buffer_reference_align = 4) buffer UintRef { uint data[]; };
layout(buffer_reference, std430, buffer_reference_align = 8) buffer U64Ref { uint64_t data[]; };
layout(buffer_reference, std430, buffer_reference_align = 8) buffer I64Ref { int64_t data[]; };

struct Res {
    uint64_t cb[18];
    uint64_t vb[16];
    uint vb_stride[16];
    uint vb_divisor[16];
    uint attr_format[16];
    uint attr_stream[16];
    uint attr_offset[16];
    int base_vertex;
    uint base_instance;
    uint flags;
    float alpha_ref;
    uint alpha_func;
    uint64_t heap_base;
    uint64_t heap_gpu;
    uint64_t heap_size;
    uint64_t zero_gpu;
};

layout(set = 0, binding = M8_RES_BINDING, std430) readonly buffer ResBuf { Res res; };

uint cb_at(uint64_t addr, uint word) { return UintRef(addr).data[word]; }

float as_float(uint x) { return uintBitsToFloat(x); }
uint as_uint(float x) { return floatBitsToUint(x); }
vec2 as_float2(uvec2 x) { return uintBitsToFloat(x); }
uvec2 as_uint2(vec2 x) { return floatBitsToUint(x); }
vec3 as_float3(uvec3 x) { return uintBitsToFloat(x); }
uvec3 as_uint3(vec3 x) { return floatBitsToUint(x); }
vec4 as_float4(uvec4 x) { return uintBitsToFloat(x); }
uvec4 as_uint4(vec4 x) { return floatBitsToUint(x); }
ivec4 as_int4(vec4 x) { return ivec4(x); }

float m8_saturate(float x) { return clamp(x, 0.0, 1.0); }
vec2 m8_saturate(vec2 x) { return clamp(x, vec2(0.0), vec2(1.0)); }
vec3 m8_saturate(vec3 x) { return clamp(x, vec3(0.0), vec3(1.0)); }
vec4 m8_saturate(vec4 x) { return clamp(x, vec4(0.0), vec4(1.0)); }
float m8_fma(float a, float b, float c) { return a * b + c; }
uint simd_shuffle(uint v, uint l) { return subgroupShuffle(v, l); }
uint simd_shuffle_up(uint v, uint l) { return subgroupShuffleUp(v, l); }
uint simd_shuffle_down(uint v, uint l) { return subgroupShuffleDown(v, l); }
uint simd_shuffle_xor(uint v, uint l) { return subgroupShuffleXor(v, l); }
vec2 m8_fma(vec2 a, vec2 b, vec2 c) { return a * b + c; }
vec3 m8_fma(vec3 a, vec3 b, vec3 c) { return a * b + c; }
vec4 m8_fma(vec4 a, vec4 b, vec4 c) { return a * b + c; }
float m8_rsqrt(float x) { return inversesqrt(x); }
vec2 m8_rsqrt(vec2 x) { return inversesqrt(x); }
vec3 m8_rsqrt(vec3 x) { return inversesqrt(x); }
vec4 m8_rsqrt(vec4 x) { return inversesqrt(x); }
ivec3 m8_texsize(texture2D t, sampler s, int l) { return ivec3(textureSize(sampler2D(t, s), l), 1); }
ivec3 m8_texsize(texture2DArray t, sampler s, int l) { return textureSize(sampler2DArray(t, s), l); }
ivec3 m8_texsize(texture3D t, sampler s, int l) { return textureSize(sampler3D(t, s), l); }
ivec3 m8_texsize(textureCube t, sampler s, int l) { return ivec3(textureSize(samplerCube(t, s), l), 1); }
ivec3 m8_texsize(textureCubeArray t, sampler s, int l) { return textureSize(samplerCubeArray(t, s), l); }
#ifdef GL_FRAGMENT_SHADER
float m8_implicit_lod(vec2 d, vec2 sz) {
    vec2 dx = dFdx(d) * sz, dy = dFdy(d) * sz;
    return clamp(0.5 * log2(max(dot(dx, dx), dot(dy, dy))), 0.0, 16.0);
}
float m8_implicit_lod(vec3 d, vec2 sz) {
    float l = max(length(dFdx(d)), length(dFdy(d)));
    return clamp(log2(l * max(sz.x, sz.y)), 0.0, 16.0);
}
float calculate_clamped_lod(texture2D t, sampler s, vec2 c) { return clamp(textureQueryLod(sampler2D(t, s), c).x, 0.0, 16.0); }
float calculate_clamped_lod(texture2DArray t, sampler s, vec2 c) { return m8_implicit_lod(c, vec2(textureSize(sampler2DArray(t, s), 0).xy)); }
float calculate_clamped_lod(texture3D t, sampler s, vec3 c) { return clamp(textureQueryLod(sampler3D(t, s), c).x, 0.0, 16.0); }
float calculate_clamped_lod(textureCube t, sampler s, vec3 c) { return clamp(textureQueryLod(samplerCube(t, s), c).x, 0.0, 16.0); }
float calculate_clamped_lod(textureCubeArray t, sampler s, vec3 c) { return m8_implicit_lod(c, vec2(textureSize(samplerCubeArray(t, s), 0).xy)); }
#else
float calculate_clamped_lod(texture2D t, sampler s, vec2 c) { return 0.0; }
float calculate_clamped_lod(texture2DArray t, sampler s, vec2 c) { return 0.0; }
float calculate_clamped_lod(texture3D t, sampler s, vec3 c) { return 0.0; }
float calculate_clamped_lod(textureCube t, sampler s, vec3 c) { return 0.0; }
float calculate_clamped_lod(textureCubeArray t, sampler s, vec3 c) { return 0.0; }
#endif
vec4 gather_compare(texture2D t, sampler s, vec2 c, float r) { return step(vec4(r), textureGather(sampler2D(t, s), c, 0)); }
vec4 gather_compare(texture2DArray t, sampler s, vec3 c, float r) { return step(vec4(r), textureGather(sampler2DArray(t, s), c, 0)); }

UintRef gmem(Res r, uint64_t ga) {
    uint64_t off = ga - r.heap_base;
    return UintRef(off < r.heap_size ? r.heap_gpu + off : r.zero_gpu);
}

bool bop(uint op, bool a, bool b) {
    return op == 0u ? (a && b) : op == 1u ? (a || b) : (a != b);
}
bool fcmp(uint op, float a, float b) {
    bool un = isnan(a) || isnan(b);
    switch (op) {
    case 0u: return false;
    case 1u: return a < b;
    case 2u: return a == b;
    case 3u: return a <= b;
    case 4u: return a > b;
    case 5u: return !un && a != b;
    case 6u: return a >= b;
    case 7u: return !un;
    case 8u: return un;
    case 9u: return un || a < b;
    case 10u: return un || a == b;
    case 11u: return un || a <= b;
    case 12u: return un || a > b;
    case 13u: return a != b;
    case 14u: return un || a >= b;
    default: return true;
    }
}
bool icmp(uint op, uint a, uint b, bool s) {
    int sa = int(a), sb = int(b);
    switch (op) {
    case 1u: return s ? sa < sb : a < b;
    case 2u: return a == b;
    case 3u: return s ? sa <= sb : a <= b;
    case 4u: return s ? sa > sb : a > b;
    case 5u: return a != b;
    case 6u: return s ? sa >= sb : a >= b;
    case 7u: return true;
    default: return false;
    }
}
bool ccc(uint c, bool zf, bool nf, bool cf, bool vf) {
    switch (c) {
    case 0u: return false;
    case 1u: return nf != vf;
    case 2u: return zf;
    case 3u: return zf || (nf != vf);
    case 4u: return !zf && (nf == vf);
    case 5u: return !zf;
    case 6u: return nf == vf;
    case 15u: return true;
    case 16u: return vf;
    case 17u: return cf;
    case 18u: return nf;
    case 19u: return !cf || zf;
    case 20u: return cf && !zf;
    case 21u: return !nf;
    case 22u: return !cf;
    case 23u: return !vf;
    default: return true;
    }
}
uint lop3(uint a, uint b, uint c, uint lut) {
    uint r = 0u;
    for (uint i = 0u; i < 8u; i++) {
        if (((lut >> i) & 1u) != 0u) {
            r |= (((i & 4u) != 0u) ? a : ~a) & (((i & 2u) != 0u) ? b : ~b) & (((i & 1u) != 0u) ? c : ~c);
        }
    }
    return r;
}
uint bfe(uint a, uint b, bool s) {
    uint pos = b & 0xffu, len = (b >> 8) & 0xffu;
    if (len == 0u) return 0u;
    if (pos >= 32u) return s && int(a) < 0 ? 0xffffffffu : 0u;
    len = min(len, 32u - pos);
    uint v = a >> pos;
    if (len < 32u) {
        uint m = (1u << len) - 1u;
        v &= m;
        if (s && (((v >> (len - 1u)) & 1u) != 0u)) v |= ~m;
    }
    return v;
}
uint ext16(uint x, bool high, bool s) {
    if (s) return high ? uint(int(x) >> 16) : uint(int(x << 16) >> 16);
    return high ? (x >> 16) : (x & 0xffffu);
}
uint xmad(uint a, uint b, uint c, uint mode, bool sa, bool sb, bool ha, bool hb, bool psl, bool mrg, uint carry) {
    uint ea = ext16(a, ha, sa), eb = ext16(b, hb, sb);
    uint p = ea * eb;
    if (psl) p <<= 16;
    switch (mode) {
    case 1u: c = c & 0xffffu; break;
    case 2u: c = c >> 16; break;
    case 3u: c = c - (((ea >> 31) << 16) + ((eb >> 31) << 16)); break;
    case 4u: c = c + (b << 16); break;
    default: break;
    }
    uint r = carry != 0xffffffffu ? p + carry : p + c;
    if (mrg) r = (r & 0xffffu) | (b << 16);
    return r;
}
float i2f(uint x, uint fmt, uint sel) {
    switch (fmt) {
    case 0u: return float((x >> (sel * 8u)) & 0xffu);
    case 1u: return float((x >> ((sel >> 1) * 16u)) & 0xffffu);
    case 4u: return float(int(x << (24u - sel * 8u)) >> 24);
    case 5u: return float(int(x << (16u - (sel >> 1) * 16u)) >> 16);
    case 6u: case 7u: return float(int(x));
    default: return float(x);
    }
}
float tof32(uint x, uint fmt, bool high) {
    if (fmt == 1u) {
        vec2 h = unpackHalf2x16(x);
        return high ? h.y : h.x;
    }
    return uintBitsToFloat(x);
}
float fround(float x, uint mode) {
    switch (mode) {
    case 4u: return roundEven(x);
    case 5u: return floor(x);
    case 6u: return ceil(x);
    case 7u: return trunc(x);
    default: return x;
    }
}
uint f2i(float x, uint rnd, uint fmt) {
    if (isnan(x)) return 0u;
    x = rnd == 1u ? floor(x) : rnd == 2u ? ceil(x) : rnd == 3u ? trunc(x) : roundEven(x);
    switch (fmt) {
    case 1u: return uint(clamp(x, 0.0f, 65535.0f));
    case 5u: return uint(int(clamp(x, -32768.0f, 32767.0f)));
    case 6u: case 7u: return x >= 2147483648.0f ? 0x7fffffffu : x <= -2147483648.0f ? 0x80000000u : uint(int(x));
    default: return x >= 4294967296.0f ? 0xffffffffu : x <= 0.0f ? 0u : uint(x);
    }
}
uint i2i(uint x, uint sel, uint src, uint dst, bool sat, bool ab, bool ng) {
    int v;
    switch (src) {
    case 0u: v = int((x >> (sel * 8u)) & 0xffu); break;
    case 1u: v = int((x >> ((sel >> 1) * 16u)) & 0xffffu); break;
    case 4u: v = int(x << (24u - sel * 8u)) >> 24; break;
    case 5u: v = int(x << (16u - (sel >> 1) * 16u)) >> 16; break;
    default: v = int(x); break;
    }
    if (ab) v = abs(v);
    if (ng) v = -v;
    if (sat) {
        switch (dst) {
        case 0u: v = clamp(v, 0, 255); break;
        case 1u: v = clamp(v, 0, 65535); break;
        case 4u: v = clamp(v, -128, 127); break;
        case 5u: v = clamp(v, -32768, 32767); break;
        default: break;
        }
    }
    return uint(v);
}

uint m8_ld32(uint64_t p) {
    uint64_t a = p & ~uint64_t(3);
    uint sh = (uint(p) & 3u) * 8u;
    uint lo = UintRef(a).data[0];
    return sh == 0u ? lo : (lo >> sh) | (UintRef(a + uint64_t(4)).data[0] << (32u - sh));
}
uint m8_ld16(uint64_t p) { return m8_ld32(p) & 0xffffu; }
uint m8_ld8(uint64_t p) { return m8_ld32(p) & 0xffu; }
int m8_s16(uint64_t p) { return int(m8_ld16(p) << 16u) >> 16; }
int m8_s8(uint64_t p) { return int(m8_ld8(p) << 24u) >> 24; }
float m8_f32(uint64_t p) { return uintBitsToFloat(m8_ld32(p)); }
vec2 m8_half2(uint64_t p) { return unpackHalf2x16(m8_ld32(p)); }
vec3 m8_half3(uint64_t p) { vec2 a = unpackHalf2x16(m8_ld32(p)); return vec3(a, unpackHalf2x16(m8_ld32(p + uint64_t(4))).x); }
vec4 m8_half4(uint64_t p) { return vec4(unpackHalf2x16(m8_ld32(p)), unpackHalf2x16(m8_ld32(p + uint64_t(4)))); }

uvec4 fetch_attr(Res r, uint i, uint vid, uint iid) {
    uint fmt = r.attr_format[i];
    if (fmt == 0u) return uvec4(0u, 0u, 0u, 0x3f800000u);
    uint s = r.attr_stream[i];
    uint div = r.vb_divisor[s];
    uint index = div == 0u ? vid : r.base_instance + iid / div;
    uint64_t p = r.vb[s] + uint64_t(index) * uint64_t(r.vb_stride[s]) + uint64_t(r.attr_offset[i]);
    vec4 f = vec4(0.0, 0.0, 0.0, 1.0);
    uvec4 u = uvec4(0u, 0u, 0u, 1u);
    bool integer = false;
    switch (fmt) {
    case 10u: f.x = m8_f32(p); break;
    case 22u: f.xy = vec2(m8_f32(p), m8_f32(p + uint64_t(4))); break;
    case 34u: f.xyz = vec3(m8_f32(p), m8_f32(p + uint64_t(4)), m8_f32(p + uint64_t(8))); break;
    case 46u: f = vec4(m8_f32(p), m8_f32(p + uint64_t(4)), m8_f32(p + uint64_t(8)), m8_f32(p + uint64_t(12))); break;
    case 11u: case 12u: u.x = m8_ld32(p); integer = true; break;
    case 23u: case 24u: u.xy = uvec2(m8_ld32(p), m8_ld32(p + uint64_t(4))); integer = true; break;
    case 35u: case 36u: u.xyz = uvec3(m8_ld32(p), m8_ld32(p + uint64_t(4)), m8_ld32(p + uint64_t(8))); integer = true; break;
    case 47u: case 48u: u = uvec4(m8_ld32(p), m8_ld32(p + uint64_t(4)), m8_ld32(p + uint64_t(8)), m8_ld32(p + uint64_t(12))); integer = true; break;
    case 5u: f.x = m8_half2(p).x; break;
    case 17u: f.xy = m8_half2(p); break;
    case 29u: f.xyz = m8_half3(p); break;
    case 41u: f = m8_half4(p); break;
    case 6u: f.x = float(m8_ld16(p)) / 65535.0; break;
    case 18u: f.xy = vec2(float(m8_ld16(p)), float(m8_ld16(p + uint64_t(2)))) / 65535.0; break;
    case 30u: f.xyz = vec3(float(m8_ld16(p)), float(m8_ld16(p + uint64_t(2))), float(m8_ld16(p + uint64_t(4)))) / 65535.0; break;
    case 42u: f = vec4(float(m8_ld16(p)), float(m8_ld16(p + uint64_t(2))), float(m8_ld16(p + uint64_t(4))), float(m8_ld16(p + uint64_t(6)))) / 65535.0; break;
    case 7u: f.x = max(float(m8_s16(p)) / 32767.0, -1.0); break;
    case 19u: f.xy = max(vec2(float(m8_s16(p)), float(m8_s16(p + uint64_t(2)))) / 32767.0, -1.0); break;
    case 31u: f.xyz = max(vec3(float(m8_s16(p)), float(m8_s16(p + uint64_t(2))), float(m8_s16(p + uint64_t(4)))) / 32767.0, -1.0); break;
    case 43u: f = max(vec4(float(m8_s16(p)), float(m8_s16(p + uint64_t(2))), float(m8_s16(p + uint64_t(4))), float(m8_s16(p + uint64_t(6)))) / 32767.0, -1.0); break;
    case 8u: u.x = m8_ld16(p); integer = true; break;
    case 20u: u.xy = uvec2(m8_ld16(p), m8_ld16(p + uint64_t(2))); integer = true; break;
    case 32u: u.xyz = uvec3(m8_ld16(p), m8_ld16(p + uint64_t(2)), m8_ld16(p + uint64_t(4))); integer = true; break;
    case 44u: u = uvec4(m8_ld16(p), m8_ld16(p + uint64_t(2)), m8_ld16(p + uint64_t(4)), m8_ld16(p + uint64_t(6))); integer = true; break;
    case 9u: u.x = uint(m8_s16(p)); integer = true; break;
    case 21u: u.xy = uvec2(uint(m8_s16(p)), uint(m8_s16(p + uint64_t(2)))); integer = true; break;
    case 33u: u.xyz = uvec3(uint(m8_s16(p)), uint(m8_s16(p + uint64_t(2))), uint(m8_s16(p + uint64_t(4)))); integer = true; break;
    case 45u: u = uvec4(uint(m8_s16(p)), uint(m8_s16(p + uint64_t(2))), uint(m8_s16(p + uint64_t(4))), uint(m8_s16(p + uint64_t(6)))); integer = true; break;
    case 1u: f.x = float(m8_ld8(p)) / 255.0; break;
    case 13u: f.xy = vec2(float(m8_ld8(p)), float(m8_ld8(p + uint64_t(1)))) / 255.0; break;
    case 25u: f.xyz = vec3(float(m8_ld8(p)), float(m8_ld8(p + uint64_t(1))), float(m8_ld8(p + uint64_t(2)))) / 255.0; break;
    case 37u: case 56u: f = vec4(float(m8_ld8(p)), float(m8_ld8(p + uint64_t(1))), float(m8_ld8(p + uint64_t(2))), float(m8_ld8(p + uint64_t(3)))) / 255.0; break;
    case 2u: f.x = max(float(m8_s8(p)) / 127.0, -1.0); break;
    case 14u: f.xy = max(vec2(float(m8_s8(p)), float(m8_s8(p + uint64_t(1)))) / 127.0, -1.0); break;
    case 26u: f.xyz = max(vec3(float(m8_s8(p)), float(m8_s8(p + uint64_t(1))), float(m8_s8(p + uint64_t(2)))) / 127.0, -1.0); break;
    case 38u: f = max(vec4(float(m8_s8(p)), float(m8_s8(p + uint64_t(1))), float(m8_s8(p + uint64_t(2))), float(m8_s8(p + uint64_t(3)))) / 127.0, -1.0); break;
    case 3u: u.x = m8_ld8(p); integer = true; break;
    case 15u: u.xy = uvec2(m8_ld8(p), m8_ld8(p + uint64_t(1))); integer = true; break;
    case 27u: u.xyz = uvec3(m8_ld8(p), m8_ld8(p + uint64_t(1)), m8_ld8(p + uint64_t(2))); integer = true; break;
    case 39u: u = uvec4(m8_ld8(p), m8_ld8(p + uint64_t(1)), m8_ld8(p + uint64_t(2)), m8_ld8(p + uint64_t(3))); integer = true; break;
    case 4u: u.x = uint(m8_s8(p)); integer = true; break;
    case 16u: u.xy = uvec2(uint(m8_s8(p)), uint(m8_s8(p + uint64_t(1)))); integer = true; break;
    case 28u: u.xyz = uvec3(uint(m8_s8(p)), uint(m8_s8(p + uint64_t(1))), uint(m8_s8(p + uint64_t(2)))); integer = true; break;
    case 40u: u = uvec4(uint(m8_s8(p)), uint(m8_s8(p + uint64_t(1))), uint(m8_s8(p + uint64_t(2))), uint(m8_s8(p + uint64_t(3)))); integer = true; break;
    case 81u: f.x = float(m8_ld8(p)); break;
    case 82u: f.x = float(m8_s8(p)); break;
    case 83u: f.x = float(m8_ld16(p)); break;
    case 84u: f.x = float(m8_s16(p)); break;
    case 85u: f.x = float(m8_ld32(p)); break;
    case 86u: f.x = float(int(m8_ld32(p))); break;
    case 87u: f.xy = vec2(float(m8_ld8(p)), float(m8_ld8(p + uint64_t(1)))); break;
    case 88u: f.xy = vec2(float(m8_s8(p)), float(m8_s8(p + uint64_t(1)))); break;
    case 89u: f.xy = vec2(float(m8_ld16(p)), float(m8_ld16(p + uint64_t(2)))); break;
    case 90u: f.xy = vec2(float(m8_s16(p)), float(m8_s16(p + uint64_t(2)))); break;
    case 91u: f.xy = vec2(float(m8_ld32(p)), float(m8_ld32(p + uint64_t(4)))); break;
    case 92u: f.xy = vec2(float(int(m8_ld32(p))), float(int(m8_ld32(p + uint64_t(4))))); break;
    case 93u: f.xyz = vec3(float(m8_ld8(p)), float(m8_ld8(p + uint64_t(1))), float(m8_ld8(p + uint64_t(2)))); break;
    case 94u: f.xyz = vec3(float(m8_s8(p)), float(m8_s8(p + uint64_t(1))), float(m8_s8(p + uint64_t(2)))); break;
    case 95u: f.xyz = vec3(float(m8_ld16(p)), float(m8_ld16(p + uint64_t(2))), float(m8_ld16(p + uint64_t(4)))); break;
    case 96u: f.xyz = vec3(float(m8_s16(p)), float(m8_s16(p + uint64_t(2))), float(m8_s16(p + uint64_t(4)))); break;
    case 97u: f.xyz = vec3(float(m8_ld32(p)), float(m8_ld32(p + uint64_t(4))), float(m8_ld32(p + uint64_t(8)))); break;
    case 98u: f.xyz = vec3(float(int(m8_ld32(p))), float(int(m8_ld32(p + uint64_t(4)))), float(int(m8_ld32(p + uint64_t(8))))); break;
    case 99u: f = vec4(float(m8_ld8(p)), float(m8_ld8(p + uint64_t(1))), float(m8_ld8(p + uint64_t(2))), float(m8_ld8(p + uint64_t(3)))); break;
    case 100u: f = vec4(float(m8_s8(p)), float(m8_s8(p + uint64_t(1))), float(m8_s8(p + uint64_t(2))), float(m8_s8(p + uint64_t(3)))); break;
    case 101u: f = vec4(float(m8_ld16(p)), float(m8_ld16(p + uint64_t(2))), float(m8_ld16(p + uint64_t(4))), float(m8_ld16(p + uint64_t(6)))); break;
    case 102u: f = vec4(float(m8_s16(p)), float(m8_s16(p + uint64_t(2))), float(m8_s16(p + uint64_t(4))), float(m8_s16(p + uint64_t(6)))); break;
    case 103u: f = vec4(float(m8_ld32(p)), float(m8_ld32(p + uint64_t(4))), float(m8_ld32(p + uint64_t(8))), float(m8_ld32(p + uint64_t(12)))); break;
    case 104u: f = vec4(float(int(m8_ld32(p))), float(int(m8_ld32(p + uint64_t(4)))), float(int(m8_ld32(p + uint64_t(8)))), float(int(m8_ld32(p + uint64_t(12))))); break;
    case 105u: case 106u: case 108u: {
        int v = int(m8_ld32(p));
        ivec4 c = ivec4((v << 22) >> 22, (v << 12) >> 22, (v << 2) >> 22, v >> 30);
        if (fmt == 105u) f = max(vec4(c) / vec4(511.0, 511.0, 511.0, 1.0), -1.0);
        else if (fmt == 108u) f = vec4(c);
        else { u = uvec4(c); integer = true; }
        break;
    }
    case 107u: { uint v = m8_ld32(p); f = vec4(float(v & 0x3ffu), float((v >> 10) & 0x3ffu), float((v >> 20) & 0x3ffu), float(v >> 30)); break; }
    case 61u: { uint v = m8_ld32(p); f = vec4(float(v & 0x3ffu), float((v >> 10) & 0x3ffu), float((v >> 20) & 0x3ffu), float(v >> 30)) / vec4(1023.0, 1023.0, 1023.0, 3.0); break; }
    case 62u: { uint v = m8_ld32(p); u = uvec4(v & 0x3ffu, (v >> 10) & 0x3ffu, (v >> 20) & 0x3ffu, v >> 30); integer = true; break; }
    default: break;
    }
    return integer ? u : floatBitsToUint(f);
}
"##;

pub fn translate(blob: &[u8]) -> Result<Translated> {
    translate_with(blob, 0, 0, 0)
}

pub fn translate_with(blob: &[u8], extra_outputs: u32, uint_targets: u32, sint_targets: u32) -> Result<Translated> {
    let sph = Sph::parse(&blob[0x30..]).ok_or_else(|| anyhow::anyhow!("bad SPH"))?;
    translate_code(&sph, &blob[0x80..], extra_outputs, uint_targets, sint_targets, [0; 3])
}

pub fn translate_compute(code: &[u8], local_memory: u32, local_size: [u32; 3]) -> Result<Translated> {
    let (sph, code) = Sph::split_compute(code, local_memory);
    translate_code(&sph, code, 0, 0, 0, local_size)
}

fn translate_code(sph: &Sph, code: &[u8], extra_outputs: u32, uint_targets: u32, sint_targets: u32, local_size: [u32; 3]) -> Result<Translated> {
    let prog = Program::decode(code)?;
    let mut g = Gen {
        sph,
        prog: &prog,
        body: String::new(),
        regs: BTreeSet::new(),
        preds: BTreeSet::new(),
        uses_cc: false,
        uses_lmem: false,
        uses_point_coord: false,
        iface: Interface::default(),
        warnings: Vec::new(),
        handle_regs: BTreeMap::new(),
    };
    for i in &prog.insts {
        if prog.targets.contains(&i.addr) {
            g.body.push_str(&format!("    case {}u:\n", i.addr));
        }

        let (pred, inv) = ((i.raw >> 16) & 7, (i.raw >> 19) & 1 != 0);
        let no_pred = i.props.has(crate::table::InstProps::NOPRED);
        let guarded = !no_pred && (pred != 7 || inv) && i.name != InstName::Ssy;
        if guarded {
            let p = g.p(pred as i32, inv);
            g.line(format!("if ({p}) {{"));
        }
        g.inst(i)?;
        if guarded {
            g.line("}");
        }
    }
    g.line("return epilogue();");

    let stage = sph.stage;
    if stage == Stage::Vertex {
        g.iface.outputs |= extra_outputs;
    }
    let entry = "main";
    let base: u32 = if stage == Stage::Fragment { 32 } else { 0 };
    let mut out = String::from(PRELUDE).replace("M8_RES_BINDING", &base.to_string());
    out.push('\n');

    for (slot, (b, k)) in g.iface.textures.clone().into_iter().enumerate() {
        writeln!(out, "layout(set = 0, binding = {}) uniform {} t{b};", base + 1 + b, k.decl().replacen("sampler", "texture", 1)).unwrap();
        writeln!(out, "layout(set = 1, binding = {}) uniform sampler s{b};", base + slot as u32).unwrap();
        g.iface.samplers.insert(b, slot as u32);
    }

    if stage == Stage::Vertex {
        for i in 0..32 {
            if g.iface.outputs >> i & 1 != 0 {
                writeln!(out, "layout(location = {i}) out vec4 v{i};").unwrap();
            }
        }
        out.push_str("struct VOut { vec4 pos;");
        for i in 0..32 {
            if g.iface.outputs >> i & 1 != 0 {
                write!(out, " vec4 v{i};").unwrap();
            }
        }
        out.push_str(" float psize; };\n");
        out.push_str("#define EPILOGUE do {\\\n  gl_Position = o.pos;\\\n  if ((res.flags & 1u) != 0u) gl_Position.y = -gl_Position.y;\\\n  if ((res.flags & 2u) != 0u) gl_Position.z = (gl_Position.z + gl_Position.w) * 0.5;\\\n");
        for i in 0..32 {
            if g.iface.outputs >> i & 1 != 0 {
                writeln!(out, "  v{i} = o.v{i};\\").unwrap();
            }
        }
        out.push_str("  gl_PointSize = o.psize;\\\n  return;\\\n} while (false)\n");
        out.push_str("#define vid gl_VertexIndex\n#define iid gl_InstanceIndex\n");
        out.push_str("void main() {\n    VOut o;\n    o.pos = vec4(0.0, 0.0, 0.0, 1.0);\n    o.psize = 1.0;\n");
        for i in 0..32 {
            if g.iface.outputs >> i & 1 != 0 {
                writeln!(out, "    o.v{i} = vec4(0.0, 0.0, 0.0, 1.0);").unwrap();
            }
        }
        for i in 0..16 {
            if g.iface.attributes >> i & 1 != 0 {
                writeln!(out, "    uint4 a{i} = fetch_attr(res, {i}u, gl_VertexIndex, gl_InstanceIndex);").unwrap();
            }
        }
    } else if stage == Stage::Compute {
        let ls = [local_size[0].clamp(1, 1024), local_size[1].clamp(1, 1024), local_size[2].clamp(1, 1024)];
        writeln!(out, "layout(local_size_x = {}, local_size_y = {}, local_size_z = {}) in;", ls[0], ls[1], ls[2]).unwrap();
        out.push_str("#define tid gl_LocalInvocationID\n#define ctaid gl_WorkGroupID\n");
        out.push_str("#define EPILOGUE do { return; } while (false)\n");
        if g.uses_lmem {
            let words = if sph.local_memory == 0 {
                g.warnings.push("shared memory size unknown, assuming 16 KiB".into());
                4096
            } else {
                (sph.local_memory / 4).clamp(1, 4096).next_power_of_two()
            };
            writeln!(out, "shared uint lmem[{words}];").unwrap();
        }
        out.push_str("void main() {\n");
    } else {
        for i in 0..32 {
            if g.iface.attributes >> i & 1 != 0 {
                let q = match sph.imap[i] {
                    Imap::Constant => "flat ",
                    Imap::ScreenLinear => "noperspective ",
                    _ => "",
                };
                writeln!(out, "layout(location = {i}) {q}in vec4 v{i};").unwrap();
            }
        }
        let mut targets: Vec<(usize, u32, &str, &str, &str)> = Vec::new();
        for rt in 0..8usize {
            let m = sph.omap_targets >> (rt * 4) & 0xF;
            if m != 0 {
                let (ty, conv, zero) = if uint_targets >> rt & 1 != 0 {
                    ("uvec4", "", "0u")
                } else if sint_targets >> rt & 1 != 0 {
                    ("ivec4", "int", "0")
                } else {
                    ("vec4", "as_float", "0.0f")
                };
                writeln!(out, "layout(location = {rt}) out {ty} c{rt};").unwrap();
                targets.push((rt, m, ty, conv, zero));
                g.iface.color_targets |= 1 << rt;
            }
        }
        let has_depth = sph.omap_depth;
        g.iface.writes_depth = has_depth;
        if targets.is_empty() {
            out.push_str("layout(location = 0) out vec4 c0;\n");
        }
        out.push_str("struct FIn { vec4 pos;");
        for i in 0..32 {
            if g.iface.attributes >> i & 1 != 0 {
                write!(out, " vec4 v{i};").unwrap();
            }
        }
        out.push_str(" };\nstruct FOut {");
        for (rt, _, ty, _, _) in &targets {
            write!(out, " {ty} c{rt};").unwrap();
        }
        if has_depth {
            out.push_str(" float depth;");
        }
        if targets.is_empty() {
            out.push_str(" vec4 c0;");
        }
        out.push_str(" };\n");
        out.push_str("#define EPILOGUE do {\\\n");
        let mut base = 0i32;
        for (rt, masks, ty, conv, zero) in &targets {
            let comps: Vec<String> = (0..4usize)
                .map(|k| {
                    if masks >> k & 1 != 0 {
                        g.regs.insert(base + k as i32);
                        format!("{conv}(r{})", base + k as i32)
                    } else {
                        (*zero).to_string()
                    }
                })
                .collect();
            writeln!(out, "  c{rt} = {ty}({});\\", comps.join(", ")).unwrap();
            base += 4;
        }
        if has_depth {
            let dr = sph.omap_targets.count_ones() as i32 + 1;
            g.regs.insert(dr);
            writeln!(out, "  gl_FragDepth = as_float(r{dr});\\").unwrap();
        }
        if targets.is_empty() {
            out.push_str("  c0 = vec4(0.0);\\\n");
        }
        if sph.omap_targets & 0xF != 0 && (uint_targets | sint_targets) & 1 == 0 {
            out.push_str("  if ((res.flags & 4u) != 0u && !fcmp(res.alpha_func, c0.w, res.alpha_ref)) discard;\\\n");
        }
        out.push_str("  return;\\\n} while (false)\n");
        out.push_str("void main() {\n    FIn fin;\n    fin.pos = gl_FragCoord;\n");
        for i in 0..32 {
            if g.iface.attributes >> i & 1 != 0 {
                writeln!(out, "    fin.v{i} = v{i};").unwrap();
            }
        }
    }

    for r in &g.regs {
        writeln!(out, "    uint r{r} = 0u;").unwrap();
    }
    for p in &g.preds {
        writeln!(out, "    bool p{p} = false;").unwrap();
    }
    if g.uses_cc {
        out.push_str("    bool zf = false, nf = false, cf = false, vf = false;\n");
    }
    if g.uses_lmem {
        let words = if stage == Stage::Compute && sph.local_memory == 0 {
            4096
        } else {
            (sph.local_memory / 4).clamp(1, 4096).next_power_of_two()
        };
        if stage == Stage::Compute {
            writeln!(out, "    const uint LMEM_MASK = {}u;", words - 1).unwrap();
        } else {
            writeln!(out, "    uint lmem[{words}];\n    const uint LMEM_MASK = {}u;", words - 1).unwrap();
        }
    }
    if stage == Stage::Fragment {
        out.push_str("    FOut r;\n");
    }
    writeln!(out, "    uint pc = {}u;\n    for (;;) {{\n    switch (pc) {{", prog.insts[0].addr).unwrap();
    let body = g.body.replace("in.", "fin.").replace("return epilogue();", "EPILOGUE;");
    out.push_str(&body);
    out.push_str("    default:\n        EPILOGUE;\n    }\n    }\n}\n");
    let mut interface = g.iface;

    if stage == Stage::Fragment && interface.color_targets == 0 && !interface.writes_depth {
        interface.color_targets = 1;
    }
    if !g.warnings.is_empty() {
        let mut w: Vec<_> = g.warnings.clone();
        w.sort();
        w.dedup();
        out.push_str(&format!("// warnings: {}\n", w.join("; ")));
    }
    Ok(Translated { stage, source: out, entry: entry.into(), interface })
}

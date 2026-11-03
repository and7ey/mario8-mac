// Mush8 - inst.rs
// Constantin Clerc (c22dev) - 0.1
#![allow(non_upper_case_globals, clippy::all, dead_code, unused_parens)]

pub const RZ: i32 = 255;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AlSize(pub u32);
impl AlSize {
    pub const _32: AlSize = AlSize(0);
    pub const _64: AlSize = AlSize(1);
    pub const _96: AlSize = AlSize(2);
    pub const _128: AlSize = AlSize(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AtomSize(pub u32);
impl AtomSize {
    pub const U32: AtomSize = AtomSize(0);
    pub const S32: AtomSize = AtomSize(1);
    pub const U64: AtomSize = AtomSize(2);
    pub const F32FtzRn: AtomSize = AtomSize(3);
    pub const F16x2FtzRn: AtomSize = AtomSize(4);
    pub const S64: AtomSize = AtomSize(5);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AtomOp(pub u32);
impl AtomOp {
    pub const Add: AtomOp = AtomOp(0);
    pub const Min: AtomOp = AtomOp(1);
    pub const Max: AtomOp = AtomOp(2);
    pub const Inc: AtomOp = AtomOp(3);
    pub const Dec: AtomOp = AtomOp(4);
    pub const And: AtomOp = AtomOp(5);
    pub const Or: AtomOp = AtomOp(6);
    pub const Xor: AtomOp = AtomOp(7);
    pub const Exch: AtomOp = AtomOp(8);
    pub const Safeadd: AtomOp = AtomOp(10);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AtomsSize(pub u32);
impl AtomsSize {
    pub const U32: AtomsSize = AtomsSize(0);
    pub const S32: AtomsSize = AtomsSize(1);
    pub const U64: AtomsSize = AtomsSize(2);
    pub const S64: AtomsSize = AtomsSize(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BarMode(pub u32);
impl BarMode {
    pub const Bar: BarMode = BarMode(0);
    pub const Result: BarMode = BarMode(1);
    pub const Warp: BarMode = BarMode(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BarOp(pub u32);
impl BarOp {
    pub const Sync: BarOp = BarOp(0);
    pub const Arv: BarOp = BarOp(1);
    pub const Red: BarOp = BarOp(2);
    pub const Scan: BarOp = BarOp(3);
    pub const SyncAll: BarOp = BarOp(4);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BarRedOp(pub u32);
impl BarRedOp {
    pub const Popc: BarRedOp = BarRedOp(0);
    pub const And: BarRedOp = BarRedOp(1);
    pub const Or: BarRedOp = BarRedOp(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Bpt(pub u32);
impl Bpt {
    pub const DrainIllegal: Bpt = Bpt(0);
    pub const Cal: Bpt = Bpt(1);
    pub const Pause: Bpt = Bpt(2);
    pub const Trap: Bpt = Bpt(3);
    pub const Int: Bpt = Bpt(4);
    pub const Drain: Bpt = Bpt(5);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Ccc(pub u32);
impl Ccc {
    pub const F: Ccc = Ccc(0);
    pub const Lt: Ccc = Ccc(1);
    pub const Eq: Ccc = Ccc(2);
    pub const Le: Ccc = Ccc(3);
    pub const Gt: Ccc = Ccc(4);
    pub const Ne: Ccc = Ccc(5);
    pub const Ge: Ccc = Ccc(6);
    pub const Num: Ccc = Ccc(7);
    pub const Nan: Ccc = Ccc(8);
    pub const Ltu: Ccc = Ccc(9);
    pub const Equ: Ccc = Ccc(10);
    pub const Leu: Ccc = Ccc(11);
    pub const Gtu: Ccc = Ccc(12);
    pub const Neu: Ccc = Ccc(13);
    pub const Geu: Ccc = Ccc(14);
    pub const T: Ccc = Ccc(15);
    pub const Off: Ccc = Ccc(16);
    pub const Lo: Ccc = Ccc(17);
    pub const Sff: Ccc = Ccc(18);
    pub const Ls: Ccc = Ccc(19);
    pub const Hi: Ccc = Ccc(20);
    pub const Sft: Ccc = Ccc(21);
    pub const Hs: Ccc = Ccc(22);
    pub const Oft: Ccc = Ccc(23);
    pub const CsmTa: Ccc = Ccc(24);
    pub const CsmTr: Ccc = Ccc(25);
    pub const CsmMx: Ccc = Ccc(26);
    pub const FcsmTa: Ccc = Ccc(27);
    pub const FcsmTr: Ccc = Ccc(28);
    pub const FcsmMx: Ccc = Ccc(29);
    pub const Rle: Ccc = Ccc(30);
    pub const Rgt: Ccc = Ccc(31);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CacheType(pub u32);
impl CacheType {
    pub const U: CacheType = CacheType(1);
    pub const C: CacheType = CacheType(2);
    pub const I: CacheType = CacheType(3);
    pub const Crs: CacheType = CacheType(4);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CctlOp(pub u32);
impl CctlOp {
    pub const Pf1: CctlOp = CctlOp(1);
    pub const Pf1_5: CctlOp = CctlOp(2);
    pub const Pf2: CctlOp = CctlOp(3);
    pub const Wb: CctlOp = CctlOp(4);
    pub const Iv: CctlOp = CctlOp(5);
    pub const Ivall: CctlOp = CctlOp(6);
    pub const Rs: CctlOp = CctlOp(7);
    pub const Rslb: CctlOp = CctlOp(9);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CctltOp(pub u32);
impl CctltOp {
    pub const Ivth: CctltOp = CctltOp(1);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BoolOp(pub u32);
impl BoolOp {
    pub const And: BoolOp = BoolOp(0);
    pub const Or: BoolOp = BoolOp(1);
    pub const Xor: BoolOp = BoolOp(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SReg(pub u32);
impl SReg {
    pub const LaneId: SReg = SReg(0);
    pub const Clock: SReg = SReg(1);
    pub const VirtCfg: SReg = SReg(2);
    pub const VirtId: SReg = SReg(3);
    pub const Pm0: SReg = SReg(4);
    pub const Pm1: SReg = SReg(5);
    pub const Pm2: SReg = SReg(6);
    pub const Pm3: SReg = SReg(7);
    pub const Pm4: SReg = SReg(8);
    pub const Pm5: SReg = SReg(9);
    pub const Pm6: SReg = SReg(10);
    pub const Pm7: SReg = SReg(11);
    pub const OrderingTicket: SReg = SReg(15);
    pub const PrimType: SReg = SReg(16);
    pub const InvocationId: SReg = SReg(17);
    pub const YDirection: SReg = SReg(18);
    pub const ThreadKill: SReg = SReg(19);
    pub const ShaderType: SReg = SReg(20);
    pub const DirectCbeWriteAddressLow: SReg = SReg(21);
    pub const DirectCbeWriteAddressHigh: SReg = SReg(22);
    pub const DirectCbeWriteEnabled: SReg = SReg(23);
    pub const MachineId0: SReg = SReg(24);
    pub const MachineId1: SReg = SReg(25);
    pub const MachineId2: SReg = SReg(26);
    pub const MachineId3: SReg = SReg(27);
    pub const Affinity: SReg = SReg(28);
    pub const InvocationInfo: SReg = SReg(29);
    pub const WScaleFactorXY: SReg = SReg(30);
    pub const WScaleFactorZ: SReg = SReg(31);
    pub const TId: SReg = SReg(32);
    pub const TIdX: SReg = SReg(33);
    pub const TIdY: SReg = SReg(34);
    pub const TIdZ: SReg = SReg(35);
    pub const CtaParam: SReg = SReg(36);
    pub const CtaIdX: SReg = SReg(37);
    pub const CtaIdY: SReg = SReg(38);
    pub const CtaIdZ: SReg = SReg(39);
    pub const Ntid: SReg = SReg(40);
    pub const CirQueueIncrMinusOne: SReg = SReg(41);
    pub const Nlatc: SReg = SReg(42);
    pub const Swinlo: SReg = SReg(48);
    pub const Swinsz: SReg = SReg(49);
    pub const Smemsz: SReg = SReg(50);
    pub const Smembanks: SReg = SReg(51);
    pub const LWinLo: SReg = SReg(52);
    pub const LWinSz: SReg = SReg(53);
    pub const LMemLoSz: SReg = SReg(54);
    pub const LMemHiOff: SReg = SReg(55);
    pub const EqMask: SReg = SReg(56);
    pub const LtMask: SReg = SReg(57);
    pub const LeMask: SReg = SReg(58);
    pub const GtMask: SReg = SReg(59);
    pub const GeMask: SReg = SReg(60);
    pub const RegAlloc: SReg = SReg(61);
    pub const CtxAddr: SReg = SReg(62);
    pub const GlobalErrorStatus: SReg = SReg(64);
    pub const WarpErrorStatus: SReg = SReg(66);
    pub const WarpErrorStatusClear: SReg = SReg(67);
    pub const PmHi0: SReg = SReg(72);
    pub const PmHi1: SReg = SReg(73);
    pub const PmHi2: SReg = SReg(74);
    pub const PmHi3: SReg = SReg(75);
    pub const PmHi4: SReg = SReg(76);
    pub const PmHi5: SReg = SReg(77);
    pub const PmHi6: SReg = SReg(78);
    pub const PmHi7: SReg = SReg(79);
    pub const ClockLo: SReg = SReg(80);
    pub const ClockHi: SReg = SReg(81);
    pub const GlobalTimerLo: SReg = SReg(82);
    pub const GlobalTimerHi: SReg = SReg(83);
    pub const HwTaskId: SReg = SReg(96);
    pub const CircularQueueEntryIndex: SReg = SReg(97);
    pub const CircularQueueEntryAddressLow: SReg = SReg(98);
    pub const CircularQueueEntryAddressHigh: SReg = SReg(99);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct RoundMode(pub u32);
impl RoundMode {
    pub const Rn: RoundMode = RoundMode(0);
    pub const Rm: RoundMode = RoundMode(1);
    pub const Rp: RoundMode = RoundMode(2);
    pub const Rz: RoundMode = RoundMode(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct FComp(pub u32);
impl FComp {
    pub const F: FComp = FComp(0);
    pub const Lt: FComp = FComp(1);
    pub const Eq: FComp = FComp(2);
    pub const Le: FComp = FComp(3);
    pub const Gt: FComp = FComp(4);
    pub const Ne: FComp = FComp(5);
    pub const Ge: FComp = FComp(6);
    pub const Num: FComp = FComp(7);
    pub const Nan: FComp = FComp(8);
    pub const Ltu: FComp = FComp(9);
    pub const Equ: FComp = FComp(10);
    pub const Leu: FComp = FComp(11);
    pub const Gtu: FComp = FComp(12);
    pub const Neu: FComp = FComp(13);
    pub const Geu: FComp = FComp(14);
    pub const T: FComp = FComp(15);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct IntegerRound(pub u32);
impl IntegerRound {
    pub const Pass: IntegerRound = IntegerRound(1);
    pub const Round: IntegerRound = IntegerRound(4);
    pub const Floor: IntegerRound = IntegerRound(5);
    pub const Ceil: IntegerRound = IntegerRound(6);
    pub const Trunc: IntegerRound = IntegerRound(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct IDstFmt(pub u32);
impl IDstFmt {
    pub const U16: IDstFmt = IDstFmt(1);
    pub const U32: IDstFmt = IDstFmt(2);
    pub const U64: IDstFmt = IDstFmt(3);
    pub const S16: IDstFmt = IDstFmt(5);
    pub const S32: IDstFmt = IDstFmt(6);
    pub const S64: IDstFmt = IDstFmt(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ISrcFmt(pub u32);
impl ISrcFmt {
    pub const U8: ISrcFmt = ISrcFmt(0);
    pub const U16: ISrcFmt = ISrcFmt(1);
    pub const U32: ISrcFmt = ISrcFmt(2);
    pub const U64: ISrcFmt = ISrcFmt(3);
    pub const S8: ISrcFmt = ISrcFmt(4);
    pub const S16: ISrcFmt = ISrcFmt(5);
    pub const S32: ISrcFmt = ISrcFmt(6);
    pub const S64: ISrcFmt = ISrcFmt(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ISrcDstFmt(pub u32);
impl ISrcDstFmt {
    pub const U8: ISrcDstFmt = ISrcDstFmt(0);
    pub const U16: ISrcDstFmt = ISrcDstFmt(1);
    pub const U32: ISrcDstFmt = ISrcDstFmt(2);
    pub const S8: ISrcDstFmt = ISrcDstFmt(4);
    pub const S16: ISrcDstFmt = ISrcDstFmt(5);
    pub const S32: ISrcDstFmt = ISrcDstFmt(6);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct RoundMode2(pub u32);
impl RoundMode2 {
    pub const Round: RoundMode2 = RoundMode2(0);
    pub const Floor: RoundMode2 = RoundMode2(1);
    pub const Ceil: RoundMode2 = RoundMode2(2);
    pub const Trunc: RoundMode2 = RoundMode2(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ChkModeF(pub u32);
impl ChkModeF {
    pub const Divide: ChkModeF = ChkModeF(0);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Fmz(pub u32);
impl Fmz {
    pub const Ftz: Fmz = Fmz(1);
    pub const Fmz: Fmz = Fmz(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct MultiplyScale(pub u32);
impl MultiplyScale {
    pub const NoScale: MultiplyScale = MultiplyScale(0);
    pub const D2: MultiplyScale = MultiplyScale(1);
    pub const D4: MultiplyScale = MultiplyScale(2);
    pub const D8: MultiplyScale = MultiplyScale(3);
    pub const M8: MultiplyScale = MultiplyScale(4);
    pub const M4: MultiplyScale = MultiplyScale(5);
    pub const M2: MultiplyScale = MultiplyScale(6);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct OFmt(pub u32);
impl OFmt {
    pub const F16: OFmt = OFmt(0);
    pub const F32: OFmt = OFmt(1);
    pub const MrgH0: OFmt = OFmt(2);
    pub const MrgH1: OFmt = OFmt(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct HalfSwizzle(pub u32);
impl HalfSwizzle {
    pub const F16: HalfSwizzle = HalfSwizzle(0);
    pub const F32: HalfSwizzle = HalfSwizzle(1);
    pub const H0H0: HalfSwizzle = HalfSwizzle(2);
    pub const H1H1: HalfSwizzle = HalfSwizzle(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ByteSel(pub u32);
impl ByteSel {
    pub const B0: ByteSel = ByteSel(0);
    pub const B1: ByteSel = ByteSel(1);
    pub const B2: ByteSel = ByteSel(2);
    pub const B3: ByteSel = ByteSel(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DstFmt(pub u32);
impl DstFmt {
    pub const F16: DstFmt = DstFmt(1);
    pub const F32: DstFmt = DstFmt(2);
    pub const F64: DstFmt = DstFmt(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AvgMode(pub u32);
impl AvgMode {
    pub const NoNeg: AvgMode = AvgMode(0);
    pub const NegB: AvgMode = AvgMode(1);
    pub const NegA: AvgMode = AvgMode(2);
    pub const PlusOne: AvgMode = AvgMode(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Lrs(pub u32);
impl Lrs {
    pub const None: Lrs = Lrs(0);
    pub const RightShift: Lrs = Lrs(1);
    pub const LeftShift: Lrs = Lrs(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct HalfSelect(pub u32);
impl HalfSelect {
    pub const B32: HalfSelect = HalfSelect(0);
    pub const H0: HalfSelect = HalfSelect(1);
    pub const H1: HalfSelect = HalfSelect(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct IComp(pub u32);
impl IComp {
    pub const F: IComp = IComp(0);
    pub const Lt: IComp = IComp(1);
    pub const Eq: IComp = IComp(2);
    pub const Le: IComp = IComp(3);
    pub const Gt: IComp = IComp(4);
    pub const Ne: IComp = IComp(5);
    pub const Ge: IComp = IComp(6);
    pub const T: IComp = IComp(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct XMode(pub u32);
impl XMode {
    pub const Xlo: XMode = XMode(1);
    pub const Xmed: XMode = XMode(2);
    pub const Xhi: XMode = XMode(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct IpaOp(pub u32);
impl IpaOp {
    pub const Pass: IpaOp = IpaOp(0);
    pub const Multiply: IpaOp = IpaOp(1);
    pub const Constant: IpaOp = IpaOp(2);
    pub const Sc: IpaOp = IpaOp(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct IBase(pub u32);
impl IBase {
    pub const Patch: IBase = IBase(1);
    pub const Prim: IBase = IBase(2);
    pub const Attr: IBase = IBase(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CacheOpLd(pub u32);
impl CacheOpLd {
    pub const Ca: CacheOpLd = CacheOpLd(0);
    pub const Cg: CacheOpLd = CacheOpLd(1);
    pub const Ci: CacheOpLd = CacheOpLd(2);
    pub const Cv: CacheOpLd = CacheOpLd(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CacheOpSt(pub u32);
impl CacheOpSt {
    pub const Wb: CacheOpSt = CacheOpSt(0);
    pub const Cg: CacheOpSt = CacheOpSt(1);
    pub const Ci: CacheOpSt = CacheOpSt(2);
    pub const Wt: CacheOpSt = CacheOpSt(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LsSize(pub u32);
impl LsSize {
    pub const U8: LsSize = LsSize(0);
    pub const S8: LsSize = LsSize(1);
    pub const U16: LsSize = LsSize(2);
    pub const S16: LsSize = LsSize(3);
    pub const B32: LsSize = LsSize(4);
    pub const B64: LsSize = LsSize(5);
    pub const B128: LsSize = LsSize(6);
    pub const UB128: LsSize = LsSize(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LsSize2(pub u32);
impl LsSize2 {
    pub const U8: LsSize2 = LsSize2(0);
    pub const S8: LsSize2 = LsSize2(1);
    pub const U16: LsSize2 = LsSize2(2);
    pub const S16: LsSize2 = LsSize2(3);
    pub const B32: LsSize2 = LsSize2(4);
    pub const B64: LsSize2 = LsSize2(5);
    pub const B128: LsSize2 = LsSize2(6);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AddressMode(pub u32);
impl AddressMode {
    pub const Il: AddressMode = AddressMode(1);
    pub const Is: AddressMode = AddressMode(2);
    pub const Isl: AddressMode = AddressMode(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CacheOp2(pub u32);
impl CacheOp2 {
    pub const Lu: CacheOp2 = CacheOp2(1);
    pub const Ci: CacheOp2 = CacheOp2(2);
    pub const Cv: CacheOp2 = CacheOp2(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PredicateOp(pub u32);
impl PredicateOp {
    pub const F: PredicateOp = PredicateOp(0);
    pub const T: PredicateOp = PredicateOp(1);
    pub const Z: PredicateOp = PredicateOp(2);
    pub const Nz: PredicateOp = PredicateOp(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LogicOp(pub u32);
impl LogicOp {
    pub const And: LogicOp = LogicOp(0);
    pub const Or: LogicOp = LogicOp(1);
    pub const Xor: LogicOp = LogicOp(2);
    pub const PassB: LogicOp = LogicOp(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Membar(pub u32);
impl Membar {
    pub const Cta: Membar = Membar(0);
    pub const Gl: Membar = Membar(1);
    pub const Sys: Membar = Membar(2);
    pub const Vc: Membar = Membar(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Ivall(pub u32);
impl Ivall {
    pub const Ivalld: Ivall = Ivall(1);
    pub const Ivallt: Ivall = Ivall(2);
    pub const Ivalltd: Ivall = Ivall(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct MufuOp(pub u32);
impl MufuOp {
    pub const Cos: MufuOp = MufuOp(0);
    pub const Sin: MufuOp = MufuOp(1);
    pub const Ex2: MufuOp = MufuOp(2);
    pub const Lg2: MufuOp = MufuOp(3);
    pub const Rcp: MufuOp = MufuOp(4);
    pub const Rsq: MufuOp = MufuOp(5);
    pub const Rcp64h: MufuOp = MufuOp(6);
    pub const Rsq64h: MufuOp = MufuOp(7);
    pub const Sqrt: MufuOp = MufuOp(8);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct OutType(pub u32);
impl OutType {
    pub const Emit: OutType = OutType(1);
    pub const Cut: OutType = OutType(2);
    pub const EmitThenCut: OutType = OutType(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PixMode(pub u32);
impl PixMode {
    pub const Covmask: PixMode = PixMode(1);
    pub const Covered: PixMode = PixMode(2);
    pub const Offset: PixMode = PixMode(3);
    pub const CentroidOffset: PixMode = PixMode(4);
    pub const MyIndex: PixMode = PixMode(5);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PMode(pub u32);
impl PMode {
    pub const F4e: PMode = PMode(1);
    pub const B4e: PMode = PMode(2);
    pub const Rc8: PMode = PMode(3);
    pub const Ecl: PMode = PMode(4);
    pub const Ecr: PMode = PMode(5);
    pub const Rc16: PMode = PMode(6);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct RedOp(pub u32);
impl RedOp {
    pub const Add: RedOp = RedOp(0);
    pub const Min: RedOp = RedOp(1);
    pub const Max: RedOp = RedOp(2);
    pub const Inc: RedOp = RedOp(3);
    pub const Dec: RedOp = RedOp(4);
    pub const And: RedOp = RedOp(5);
    pub const Or: RedOp = RedOp(6);
    pub const Xor: RedOp = RedOp(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct XModeShf(pub u32);
impl XModeShf {
    pub const Hi: XModeShf = XModeShf(1);
    pub const X: XModeShf = XModeShf(2);
    pub const Xhi: XModeShf = XModeShf(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct MaxShift(pub u32);
impl MaxShift {
    pub const U64: MaxShift = MaxShift(2);
    pub const S64: MaxShift = MaxShift(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ShflMode(pub u32);
impl ShflMode {
    pub const Idx: ShflMode = ShflMode(0);
    pub const Up: ShflMode = ShflMode(1);
    pub const Down: ShflMode = ShflMode(2);
    pub const Bfly: ShflMode = ShflMode(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Clamp(pub u32);
impl Clamp {
    pub const Ign: Clamp = Clamp(0);
    pub const Trap: Clamp = Clamp(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SuatomSize(pub u32);
impl SuatomSize {
    pub const U32: SuatomSize = SuatomSize(0);
    pub const S32: SuatomSize = SuatomSize(1);
    pub const U64: SuatomSize = SuatomSize(2);
    pub const F32FtzRn: SuatomSize = SuatomSize(3);
    pub const F16x2FtzRn: SuatomSize = SuatomSize(4);
    pub const S64: SuatomSize = SuatomSize(5);
    pub const Sd32: SuatomSize = SuatomSize(6);
    pub const Sd64: SuatomSize = SuatomSize(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SuDim(pub u32);
impl SuDim {
    pub const _1d: SuDim = SuDim(0);
    pub const _1dBuffer: SuDim = SuDim(1);
    pub const _1dArray: SuDim = SuDim(2);
    pub const _2d: SuDim = SuDim(3);
    pub const _2dArray: SuDim = SuDim(4);
    pub const _3d: SuDim = SuDim(5);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SuatomOp(pub u32);
impl SuatomOp {
    pub const Add: SuatomOp = SuatomOp(0);
    pub const Min: SuatomOp = SuatomOp(1);
    pub const Max: SuatomOp = SuatomOp(2);
    pub const Inc: SuatomOp = SuatomOp(3);
    pub const Dec: SuatomOp = SuatomOp(4);
    pub const And: SuatomOp = SuatomOp(5);
    pub const Or: SuatomOp = SuatomOp(6);
    pub const Xor: SuatomOp = SuatomOp(7);
    pub const Exch: SuatomOp = SuatomOp(8);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SuSize(pub u32);
impl SuSize {
    pub const U8: SuSize = SuSize(0);
    pub const S8: SuSize = SuSize(1);
    pub const U16: SuSize = SuSize(2);
    pub const S16: SuSize = SuSize(3);
    pub const B32: SuSize = SuSize(4);
    pub const B64: SuSize = SuSize(5);
    pub const B128: SuSize = SuSize(6);
    pub const UB128: SuSize = SuSize(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SuRgba(pub u32);
impl SuRgba {
    pub const R: SuRgba = SuRgba(1);
    pub const G: SuRgba = SuRgba(2);
    pub const Rg: SuRgba = SuRgba(3);
    pub const B: SuRgba = SuRgba(4);
    pub const Rb: SuRgba = SuRgba(5);
    pub const Gb: SuRgba = SuRgba(6);
    pub const Rgb: SuRgba = SuRgba(7);
    pub const A: SuRgba = SuRgba(8);
    pub const Ra: SuRgba = SuRgba(9);
    pub const Ga: SuRgba = SuRgba(10);
    pub const Rga: SuRgba = SuRgba(11);
    pub const Ba: SuRgba = SuRgba(12);
    pub const Rba: SuRgba = SuRgba(13);
    pub const Gba: SuRgba = SuRgba(14);
    pub const Rgba: SuRgba = SuRgba(15);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Lod(pub u32);
impl Lod {
    pub const Lz: Lod = Lod(1);
    pub const Lb: Lod = Lod(2);
    pub const Ll: Lod = Lod(3);
    pub const Lba: Lod = Lod(6);
    pub const Lla: Lod = Lod(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TexDim(pub u32);
impl TexDim {
    pub const _1d: TexDim = TexDim(0);
    pub const Array1d: TexDim = TexDim(1);
    pub const _2d: TexDim = TexDim(2);
    pub const Array2d: TexDim = TexDim(3);
    pub const _3d: TexDim = TexDim(4);
    pub const Array3d: TexDim = TexDim(5);
    pub const Cube: TexDim = TexDim(6);
    pub const ArrayCube: TexDim = TexDim(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TexsTarget(pub u32);
impl TexsTarget {
    pub const Texture1DLodZero: TexsTarget = TexsTarget(0);
    pub const Texture2D: TexsTarget = TexsTarget(1);
    pub const Texture2DLodZero: TexsTarget = TexsTarget(2);
    pub const Texture2DLodLevel: TexsTarget = TexsTarget(3);
    pub const Texture2DDepthCompare: TexsTarget = TexsTarget(4);
    pub const Texture2DLodLevelDepthCompare: TexsTarget = TexsTarget(5);
    pub const Texture2DLodZeroDepthCompare: TexsTarget = TexsTarget(6);
    pub const Texture2DArray: TexsTarget = TexsTarget(7);
    pub const Texture2DArrayLodZero: TexsTarget = TexsTarget(8);
    pub const Texture2DArrayLodZeroDepthCompare: TexsTarget = TexsTarget(9);
    pub const Texture3D: TexsTarget = TexsTarget(10);
    pub const Texture3DLodZero: TexsTarget = TexsTarget(11);
    pub const TextureCube: TexsTarget = TexsTarget(12);
    pub const TextureCubeLodLevel: TexsTarget = TexsTarget(13);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TldsTarget(pub u32);
impl TldsTarget {
    pub const Texture1DLodZero: TldsTarget = TldsTarget(0x0);
    pub const Texture1DLodLevel: TldsTarget = TldsTarget(0x1);
    pub const Texture2DLodZero: TldsTarget = TldsTarget(0x2);
    pub const Texture2DLodZeroOffset: TldsTarget = TldsTarget(0x4);
    pub const Texture2DLodLevel: TldsTarget = TldsTarget(0x5);
    pub const Texture2DLodZeroMultisample: TldsTarget = TldsTarget(0x6);
    pub const Texture3DLodZero: TldsTarget = TldsTarget(0x7);
    pub const Texture2DArrayLodZero: TldsTarget = TldsTarget(0x8);
    pub const Texture2DLodLevelOffset: TldsTarget = TldsTarget(0xc);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TexComp(pub u32);
impl TexComp {
    pub const R: TexComp = TexComp(0);
    pub const G: TexComp = TexComp(1);
    pub const B: TexComp = TexComp(2);
    pub const A: TexComp = TexComp(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TexOffset(pub u32);
impl TexOffset {
    pub const None: TexOffset = TexOffset(0);
    pub const Aoffi: TexOffset = TexOffset(1);
    pub const Ptp: TexOffset = TexOffset(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TexQuery(pub u32);
impl TexQuery {
    pub const TexHeaderDimension: TexQuery = TexQuery(1);
    pub const TexHeaderTextureType: TexQuery = TexQuery(2);
    pub const TexHeaderSamplerPos: TexQuery = TexQuery(5);
    pub const TexSamplerFilter: TexQuery = TexQuery(16);
    pub const TexSamplerLod: TexQuery = TexQuery(18);
    pub const TexSamplerWrap: TexQuery = TexQuery(20);
    pub const TexSamplerBorderColor: TexQuery = TexQuery(22);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct VectorSelect(pub u32);
impl VectorSelect {
    pub const U8B0: VectorSelect = VectorSelect(0);
    pub const U8B1: VectorSelect = VectorSelect(1);
    pub const U8B2: VectorSelect = VectorSelect(2);
    pub const U8B3: VectorSelect = VectorSelect(3);
    pub const U16H0: VectorSelect = VectorSelect(4);
    pub const U16H1: VectorSelect = VectorSelect(5);
    pub const U32: VectorSelect = VectorSelect(6);
    pub const S8B0: VectorSelect = VectorSelect(8);
    pub const S8B1: VectorSelect = VectorSelect(9);
    pub const S8B2: VectorSelect = VectorSelect(10);
    pub const S8B3: VectorSelect = VectorSelect(11);
    pub const S16H0: VectorSelect = VectorSelect(12);
    pub const S16H1: VectorSelect = VectorSelect(13);
    pub const S32: VectorSelect = VectorSelect(14);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct VideoOp(pub u32);
impl VideoOp {
    pub const Mrg16h: VideoOp = VideoOp(0);
    pub const Mrg16l: VideoOp = VideoOp(1);
    pub const Mrg8b0: VideoOp = VideoOp(2);
    pub const Mrg8b2: VideoOp = VideoOp(3);
    pub const Acc: VideoOp = VideoOp(4);
    pub const Min: VideoOp = VideoOp(5);
    pub const Max: VideoOp = VideoOp(6);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct VideoRed(pub u32);
impl VideoRed {
    pub const Acc: VideoRed = VideoRed(1);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LaneMask4(pub u32);
impl LaneMask4 {
    pub const Z: LaneMask4 = LaneMask4(1);
    pub const W: LaneMask4 = LaneMask4(2);
    pub const Zw: LaneMask4 = LaneMask4(3);
    pub const X: LaneMask4 = LaneMask4(4);
    pub const Xz: LaneMask4 = LaneMask4(5);
    pub const Xw: LaneMask4 = LaneMask4(6);
    pub const Xzw: LaneMask4 = LaneMask4(7);
    pub const Y: LaneMask4 = LaneMask4(8);
    pub const Yz: LaneMask4 = LaneMask4(9);
    pub const Yw: LaneMask4 = LaneMask4(10);
    pub const Yzw: LaneMask4 = LaneMask4(11);
    pub const Xy: LaneMask4 = LaneMask4(12);
    pub const Xyz: LaneMask4 = LaneMask4(13);
    pub const Xyw: LaneMask4 = LaneMask4(14);
    pub const Xyzw: LaneMask4 = LaneMask4(15);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ASelect4(pub u32);
impl ASelect4 {
    pub const _0000: ASelect4 = ASelect4(0);
    pub const _1111: ASelect4 = ASelect4(1);
    pub const _2222: ASelect4 = ASelect4(2);
    pub const _3333: ASelect4 = ASelect4(3);
    pub const _3210: ASelect4 = ASelect4(4);
    pub const _5432: ASelect4 = ASelect4(6);
    pub const _6543: ASelect4 = ASelect4(7);
    pub const _3201: ASelect4 = ASelect4(8);
    pub const _3012: ASelect4 = ASelect4(9);
    pub const _0213: ASelect4 = ASelect4(10);
    pub const _3120: ASelect4 = ASelect4(11);
    pub const _1230: ASelect4 = ASelect4(12);
    pub const _2310: ASelect4 = ASelect4(13);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BSelect4(pub u32);
impl BSelect4 {
    pub const _4444: BSelect4 = BSelect4(0);
    pub const _5555: BSelect4 = BSelect4(1);
    pub const _6666: BSelect4 = BSelect4(2);
    pub const _7777: BSelect4 = BSelect4(3);
    pub const _7654: BSelect4 = BSelect4(4);
    pub const _5432: BSelect4 = BSelect4(6);
    pub const _4321: BSelect4 = BSelect4(7);
    pub const _4567: BSelect4 = BSelect4(8);
    pub const _6745: BSelect4 = BSelect4(9);
    pub const _5476: BSelect4 = BSelect4(10);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct VideoScale(pub u32);
impl VideoScale {
    pub const Shr7: VideoScale = VideoScale(1);
    pub const Shr15: VideoScale = VideoScale(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct VoteMode(pub u32);
impl VoteMode {
    pub const All: VoteMode = VoteMode(0);
    pub const Any: VoteMode = VoteMode(1);
    pub const Eq: VoteMode = VoteMode(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct XmadCop(pub u32);
impl XmadCop {
    pub const Cfull: XmadCop = XmadCop(0);
    pub const Clo: XmadCop = XmadCop(1);
    pub const Chi: XmadCop = XmadCop(2);
    pub const Csfu: XmadCop = XmadCop(3);
    pub const Cbcc: XmadCop = XmadCop(4);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct XmadCop2(pub u32);
impl XmadCop2 {
    pub const Cfull: XmadCop2 = XmadCop2(0);
    pub const Clo: XmadCop2 = XmadCop2(1);
    pub const Chi: XmadCop2 = XmadCop2(2);
    pub const Csfu: XmadCop2 = XmadCop2(3);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ImadspASelect(pub u32);
impl ImadspASelect {
    pub const U32: ImadspASelect = ImadspASelect(0);
    pub const S32: ImadspASelect = ImadspASelect(1);
    pub const U24: ImadspASelect = ImadspASelect(2);
    pub const S24: ImadspASelect = ImadspASelect(3);
    pub const U16h0: ImadspASelect = ImadspASelect(4);
    pub const S16h0: ImadspASelect = ImadspASelect(5);
    pub const U16h1: ImadspASelect = ImadspASelect(6);
    pub const S16h1: ImadspASelect = ImadspASelect(7);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ImadspBSelect(pub u32);
impl ImadspBSelect {
    pub const U24: ImadspBSelect = ImadspBSelect(0);
    pub const S24: ImadspBSelect = ImadspBSelect(1);
    pub const U16h0: ImadspBSelect = ImadspBSelect(2);
    pub const S16h0: ImadspBSelect = ImadspBSelect(3);
}

#[derive(Clone, Copy, Debug)]
pub struct InstConditional(pub u64);
impl InstConditional {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstAl2p(pub u64);
impl InstAl2p {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn al_size(&self) -> AlSize { AlSize(((self.0 >> 47) & 0x3) as u32) }
    pub fn aio(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn imm11(&self) -> i32 { (((self.0 >> 20) & 0x7FF) as i32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 44) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstAld(pub u64);
impl InstAld {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm11(&self) -> i32 { (((self.0 >> 20) & 0x7FF) as i32) }
    pub fn p(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn o(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn al_size(&self) -> AlSize { AlSize(((self.0 >> 47) & 0x3) as u32) }
    pub fn phys(&self) -> bool { !self.p() && self.imm11() == 0 && self.src_a() != RZ }
}

#[derive(Clone, Copy, Debug)]
pub struct InstAst(pub u64);
impl InstAst {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm11(&self) -> i32 { (((self.0 >> 20) & 0x7FF) as i32) }
    pub fn p(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn al_size(&self) -> AlSize { AlSize(((self.0 >> 47) & 0x3) as u32) }
    pub fn phys(&self) -> bool { !self.p() && self.imm11() == 0 && self.src_a() != RZ }
}

#[derive(Clone, Copy, Debug)]
pub struct InstAtom(pub u64);
impl InstAtom {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm20(&self) -> i32 { (((self.0 >> 28) & 0xFFFFF) as i32) }
    pub fn size(&self) -> AtomSize { AtomSize(((self.0 >> 49) & 0x7) as u32) }
    pub fn op(&self) -> AtomOp { AtomOp(((self.0 >> 52) & 0xF) as u32) }
    pub fn e(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstAtomCas(pub u64);
impl InstAtomCas {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn bc_rz(&self) -> i32 { (((self.0 >> 50) & 0x3) as i32) }
    pub fn e(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstAtoms(pub u64);
impl InstAtoms {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm22(&self) -> i32 { (((self.0 >> 30) & 0x3FFFFF) as i32) }
    pub fn atoms_size(&self) -> AtomsSize { AtomsSize(((self.0 >> 28) & 0x3) as u32) }
    pub fn atom_op(&self) -> AtomOp { AtomOp(((self.0 >> 52) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstAtomsCas(pub u64);
impl InstAtomsCas {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn atoms_bc_rz(&self) -> i32 { (((self.0 >> 28) & 0x3) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstB2r(pub u64);
impl InstB2r {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 45) & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn mode(&self) -> BarMode { BarMode(((self.0 >> 32) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBar(pub u64);
impl InstBar {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm12(&self) -> i32 { (((self.0 >> 20) & 0xFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn bar_op(&self) -> BarOp { BarOp(((self.0 >> 32) & 0x7) as u32) }
    pub fn bar_red_op(&self) -> BarRedOp { BarRedOp(((self.0 >> 35) & 0x3) as u32) }
    pub fn a_fix_bar(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn b_fix_bar(&self) -> bool { (self.0 & 0x80000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBfeR(pub u64);
impl InstBfeR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn brev(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBfeI(pub u64);
impl InstBfeI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn brev(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBfeC(pub u64);
impl InstBfeC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn brev(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBfiR(pub u64);
impl InstBfiR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBfiI(pub u64);
impl InstBfiI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBfiC(pub u64);
impl InstBfiC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBfiRc(pub u64);
impl InstBfiRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBpt(pub u64);
impl InstBpt {
    pub fn imm20(&self) -> i32 { (((self.0 >> 20) & 0xFFFFF) as i32) }
    pub fn bpt(&self) -> Bpt { Bpt(((self.0 >> 6) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBra(pub u64);
impl InstBra {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn lmt(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn u(&self) -> bool { (self.0 & 0x80) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBrk(pub u64);
impl InstBrk {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstBrx(pub u64);
impl InstBrx {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn lmt(&self) -> bool { (self.0 & 0x40) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCal(pub u64);
impl InstCal {
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn inc(&self) -> bool { (self.0 & 0x40) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCctl(pub u64);
impl InstCctl {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm30(&self) -> i32 { (((self.0 >> 22) & 0x3FFFFFFF) as i32) }
    pub fn e(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn cache(&self) -> CacheType { CacheType(((self.0 >> 4) & 0x7) as u32) }
    pub fn cctl_op(&self) -> CctlOp { CctlOp((self.0 & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCctll(pub u64);
impl InstCctll {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm22(&self) -> i32 { (((self.0 >> 22) & 0x3FFFFF) as i32) }
    pub fn cache(&self) -> i32 { (((self.0 >> 4) & 0x3) as i32) }
    pub fn cctl_op(&self) -> CctlOp { CctlOp((self.0 & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCctlt(pub u64);
impl InstCctlt {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ts_idx13(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn cctlt_op(&self) -> CctltOp { CctltOp((self.0 & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCctltR(pub u64);
impl InstCctltR {
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cctlt_op(&self) -> CctltOp { CctltOp((self.0 & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCont(pub u64);
impl InstCont {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCset(pub u64);
impl InstCset {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc(((self.0 >> 8) & 0x1F) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCsetp(pub u64);
impl InstCsetp {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc(((self.0 >> 8) & 0x1F) as u32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstCs2r(pub u64);
impl InstCs2r {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn s_reg(&self) -> SReg { SReg(((self.0 >> 20) & 0xFF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDaddR(pub u64);
impl InstDaddR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDaddI(pub u64);
impl InstDaddI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDaddC(pub u64);
impl InstDaddC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDepbar(pub u64);
impl InstDepbar {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn le(&self) -> bool { (self.0 & 0x20000000) != 0 }
    pub fn sbid(&self) -> i32 { (((self.0 >> 26) & 0x7) as i32) }
    pub fn pend_cnt(&self) -> i32 { (((self.0 >> 20) & 0x3F) as i32) }
    pub fn imm6(&self) -> i32 { ((self.0 & 0x3F) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDfmaR(pub u64);
impl InstDfmaR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 50) & 0x3) as u32) }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDfmaI(pub u64);
impl InstDfmaI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 50) & 0x3) as u32) }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDfmaC(pub u64);
impl InstDfmaC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 50) & 0x3) as u32) }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDfmaRc(pub u64);
impl InstDfmaRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 50) & 0x3) as u32) }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDmnmxR(pub u64);
impl InstDmnmxR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDmnmxI(pub u64);
impl InstDmnmxI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDmnmxC(pub u64);
impl InstDmnmxC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDmulR(pub u64);
impl InstDmulR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDmulI(pub u64);
impl InstDmulI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDmulC(pub u64);
impl InstDmulC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDsetR(pub u64);
impl InstDsetR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDsetI(pub u64);
impl InstDsetI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDsetC(pub u64);
impl InstDsetC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDsetpR(pub u64);
impl InstDsetpR {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn abs_a(&self) -> bool { (self.0 & 0x80) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDsetpI(pub u64);
impl InstDsetpI {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn abs_a(&self) -> bool { (self.0 & 0x80) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstDsetpC(pub u64);
impl InstDsetpC {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn abs_a(&self) -> bool { (self.0 & 0x80) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstExit(pub u64);
impl InstExit {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
    pub fn keep_ref_cnt(&self) -> bool { (self.0 & 0x20) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstF2fR(pub u64);
impl InstF2fR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn dst_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 8) & 0x3) as u32) }
    pub fn src_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 10) & 0x3) as u32) }
    pub fn round_mode(&self) -> IntegerRound { IntegerRound(((((self.0 >> 40) & 0x4) as i32) | (((self.0 >> 39) & 0x3) as i32)) as u32) }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstF2fI(pub u64);
impl InstF2fI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn dst_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 8) & 0x3) as u32) }
    pub fn src_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 10) & 0x3) as u32) }
    pub fn round_mode(&self) -> IntegerRound { IntegerRound(((((self.0 >> 40) & 0x4) as i32) | (((self.0 >> 39) & 0x3) as i32)) as u32) }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstF2fC(pub u64);
impl InstF2fC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn dst_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 8) & 0x3) as u32) }
    pub fn src_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 10) & 0x3) as u32) }
    pub fn round_mode(&self) -> IntegerRound { IntegerRound(((((self.0 >> 40) & 0x4) as i32) | (((self.0 >> 39) & 0x3) as i32)) as u32) }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstF2iR(pub u64);
impl InstF2iR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn i_dst_fmt(&self) -> IDstFmt { IDstFmt(((((self.0 >> 10) & 0x4) as i32) | (((self.0 >> 8) & 0x3) as i32)) as u32) }
    pub fn src_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 10) & 0x3) as u32) }
    pub fn round_mode(&self) -> RoundMode2 { RoundMode2(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstF2iI(pub u64);
impl InstF2iI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn i_dst_fmt(&self) -> IDstFmt { IDstFmt(((((self.0 >> 10) & 0x4) as i32) | (((self.0 >> 8) & 0x3) as i32)) as u32) }
    pub fn src_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 10) & 0x3) as u32) }
    pub fn round_mode(&self) -> RoundMode2 { RoundMode2(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstF2iC(pub u64);
impl InstF2iC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn i_dst_fmt(&self) -> IDstFmt { IDstFmt(((((self.0 >> 10) & 0x4) as i32) | (((self.0 >> 8) & 0x3) as i32)) as u32) }
    pub fn src_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 10) & 0x3) as u32) }
    pub fn round_mode(&self) -> RoundMode2 { RoundMode2(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFaddR(pub u64);
impl InstFaddR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFaddI(pub u64);
impl InstFaddI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFaddC(pub u64);
impl InstFaddC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFadd32i(pub u64);
impl InstFadd32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn abs_b(&self) -> bool { (self.0 & 0x200000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFchkR(pub u64);
impl InstFchkR {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn chk_mode_f(&self) -> ChkModeF { ChkModeF(((self.0 >> 39) & 0x3F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFchkI(pub u64);
impl InstFchkI {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn chk_mode_f(&self) -> ChkModeF { ChkModeF(((self.0 >> 39) & 0x3F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFchkC(pub u64);
impl InstFchkC {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn chk_mode_f(&self) -> ChkModeF { ChkModeF(((self.0 >> 39) & 0x3F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFcmpR(pub u64);
impl InstFcmpR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFcmpI(pub u64);
impl InstFcmpI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFcmpC(pub u64);
impl InstFcmpC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFcmpRc(pub u64);
impl InstFcmpRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFfmaR(pub u64);
impl InstFfmaR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 53) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFfmaI(pub u64);
impl InstFfmaI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 53) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFfmaC(pub u64);
impl InstFfmaC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 53) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFfmaRc(pub u64);
impl InstFfmaRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 53) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFfma32i(pub u64);
impl InstFfma32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x200000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 53) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFloR(pub u64);
impl InstFloR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFloI(pub u64);
impl InstFloI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFloC(pub u64);
impl InstFloC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn sh(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFmnmxR(pub u64);
impl InstFmnmxR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFmnmxI(pub u64);
impl InstFmnmxI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFmnmxC(pub u64);
impl InstFmnmxC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFmulR(pub u64);
impl InstFmulR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 44) & 0x3) as u32) }
    pub fn scale(&self) -> MultiplyScale { MultiplyScale(((self.0 >> 41) & 0x7) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFmulI(pub u64);
impl InstFmulI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 44) & 0x3) as u32) }
    pub fn scale(&self) -> MultiplyScale { MultiplyScale(((self.0 >> 41) & 0x7) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFmulC(pub u64);
impl InstFmulC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 44) & 0x3) as u32) }
    pub fn scale(&self) -> MultiplyScale { MultiplyScale(((self.0 >> 41) & 0x7) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFmul32i(pub u64);
impl InstFmul32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 53) & 0x3) as u32) }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFsetR(pub u64);
impl InstFsetR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFsetC(pub u64);
impl InstFsetC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFsetI(pub u64);
impl InstFsetI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFsetpR(pub u64);
impl InstFsetpR {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x80) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFsetpI(pub u64);
impl InstFsetpI {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x80) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFsetpC(pub u64);
impl InstFsetpC {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x80) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 48) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstFswzadd(pub u64);
impl InstFswzadd {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn ndv(&self) -> bool { (self.0 & 0x4000000000) != 0 }
    pub fn pn_word(&self) -> i32 { (((self.0 >> 28) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstGetcrsptr(pub u64);
impl InstGetcrsptr {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstGetlmembase(pub u64);
impl InstGetlmembase {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHadd2R(pub u64);
impl InstHadd2R {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn b_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 28) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x40000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHadd2I(pub u64);
impl InstHadd2I {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn bimm_h0(&self) -> i32 { (((self.0 >> 20) & 0x3FF) as i32) }
    pub fn bimm_h1(&self) -> i32 { (((self.0 >> 47) & 0x200) as i32) | (((self.0 >> 30) & 0x1FF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHadd2C(pub u64);
impl InstHadd2C {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHadd232i(pub u64);
impl InstHadd232i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 53) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHfma2R(pub u64);
impl InstHfma2R {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn b_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 28) & 0x3) as u32) }
    pub fn c_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 35) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x40000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 37) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHfma2I(pub u64);
impl InstHfma2I {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn bimm_h0(&self) -> i32 { (((self.0 >> 20) & 0x3FF) as i32) }
    pub fn bimm_h1(&self) -> i32 { (((self.0 >> 47) & 0x200) as i32) | (((self.0 >> 30) & 0x1FF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn c_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 53) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 57) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHfma2C(pub u64);
impl InstHfma2C {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn c_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 53) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 57) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHfma2Rc(pub u64);
impl InstHfma2Rc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn c_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 53) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 57) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHfma232i(pub u64);
impl InstHfma232i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 57) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHmul2R(pub u64);
impl InstHmul2R {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn b_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 28) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x40000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHmul2I(pub u64);
impl InstHmul2I {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn bimm_h0(&self) -> i32 { (((self.0 >> 20) & 0x3FF) as i32) }
    pub fn bimm_h1(&self) -> i32 { (((self.0 >> 47) & 0x200) as i32) | (((self.0 >> 30) & 0x1FF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHmul2C(pub u64);
impl InstHmul2C {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn o_fmt(&self) -> OFmt { OFmt(((self.0 >> 49) & 0x3) as u32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHmul232i(pub u64);
impl InstHmul232i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 53) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn fmz(&self) -> Fmz { Fmz(((self.0 >> 55) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHset2R(pub u64);
impl InstHset2R {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn b_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 28) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x40000000) != 0 }
    pub fn bval(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn cmp(&self) -> FComp { FComp(((self.0 >> 35) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHset2I(pub u64);
impl InstHset2I {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn bimm_h0(&self) -> i32 { (((self.0 >> 20) & 0x3FF) as i32) }
    pub fn bimm_h1(&self) -> i32 { (((self.0 >> 47) & 0x200) as i32) | (((self.0 >> 30) & 0x1FF) as i32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn bval(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn cmp(&self) -> FComp { FComp(((self.0 >> 49) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHset2C(pub u64);
impl InstHset2C {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn bval(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn cmp(&self) -> FComp { FComp(((self.0 >> 49) & 0xF) as u32) }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn ftz(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHsetp2R(pub u64);
impl InstHsetp2R {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x40000000) != 0 }
    pub fn f_comp2(&self) -> FComp { FComp(((self.0 >> 35) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn h_and(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
    pub fn b_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 28) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHsetp2I(pub u64);
impl InstHsetp2I {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn bimm_h0(&self) -> i32 { (((self.0 >> 20) & 0x3FF) as i32) }
    pub fn bimm_h1(&self) -> i32 { (((self.0 >> 47) & 0x200) as i32) | (((self.0 >> 30) & 0x1FF) as i32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 49) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn h_and(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstHsetp2C(pub u64);
impl InstHsetp2C {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn abs_a(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn f_comp(&self) -> FComp { FComp(((self.0 >> 49) & 0xF) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn ftz(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn h_and(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn a_swizzle(&self) -> HalfSwizzle { HalfSwizzle(((self.0 >> 47) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstI2fR(pub u64);
impl InstI2fR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn i_src_fmt(&self) -> ISrcFmt { ISrcFmt(((((self.0 >> 11) & 0x4) as i32) | (((self.0 >> 10) & 0x3) as i32)) as u32) }
    pub fn dst_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 8) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstI2fI(pub u64);
impl InstI2fI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn i_src_fmt(&self) -> ISrcFmt { ISrcFmt(((((self.0 >> 11) & 0x4) as i32) | (((self.0 >> 10) & 0x3) as i32)) as u32) }
    pub fn dst_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 8) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstI2fC(pub u64);
impl InstI2fC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn round_mode(&self) -> RoundMode { RoundMode(((self.0 >> 39) & 0x3) as u32) }
    pub fn i_src_fmt(&self) -> ISrcFmt { ISrcFmt(((((self.0 >> 11) & 0x4) as i32) | (((self.0 >> 10) & 0x3) as i32)) as u32) }
    pub fn dst_fmt(&self) -> DstFmt { DstFmt(((self.0 >> 8) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstI2iR(pub u64);
impl InstI2iR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn i_dst_fmt(&self) -> ISrcDstFmt { ISrcDstFmt(((((self.0 >> 10) & 0x4) as i32) | (((self.0 >> 8) & 0x3) as i32)) as u32) }
    pub fn i_src_fmt(&self) -> ISrcDstFmt { ISrcDstFmt(((((self.0 >> 11) & 0x4) as i32) | (((self.0 >> 10) & 0x3) as i32)) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstI2iI(pub u64);
impl InstI2iI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn i_dst_fmt(&self) -> ISrcDstFmt { ISrcDstFmt(((((self.0 >> 10) & 0x4) as i32) | (((self.0 >> 8) & 0x3) as i32)) as u32) }
    pub fn i_src_fmt(&self) -> ISrcDstFmt { ISrcDstFmt(((((self.0 >> 11) & 0x4) as i32) | (((self.0 >> 10) & 0x3) as i32)) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstI2iC(pub u64);
impl InstI2iC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn i_dst_fmt(&self) -> ISrcDstFmt { ISrcDstFmt(((((self.0 >> 10) & 0x4) as i32) | (((self.0 >> 8) & 0x3) as i32)) as u32) }
    pub fn i_src_fmt(&self) -> ISrcDstFmt { ISrcDstFmt(((((self.0 >> 11) & 0x4) as i32) | (((self.0 >> 10) & 0x3) as i32)) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIaddR(pub u64);
impl InstIaddR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 48) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIaddI(pub u64);
impl InstIaddI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 48) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIaddC(pub u64);
impl InstIaddC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 48) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIadd32i(pub u64);
impl InstIadd32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 55) & 0x3) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIadd3R(pub u64);
impl InstIadd3R {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn lrs(&self) -> Lrs { Lrs(((self.0 >> 37) & 0x3) as u32) }
    pub fn apart(&self) -> HalfSelect { HalfSelect(((self.0 >> 35) & 0x3) as u32) }
    pub fn bpart(&self) -> HalfSelect { HalfSelect(((self.0 >> 33) & 0x3) as u32) }
    pub fn cpart(&self) -> HalfSelect { HalfSelect(((self.0 >> 31) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIadd3I(pub u64);
impl InstIadd3I {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIadd3C(pub u64);
impl InstIadd3C {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn neg_c(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIcmpR(pub u64);
impl InstIcmpR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIcmpI(pub u64);
impl InstIcmpI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIcmpC(pub u64);
impl InstIcmpC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIcmpRc(pub u64);
impl InstIcmpRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIde(pub u64);
impl InstIde {
    pub fn imm16(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
    pub fn di(&self) -> bool { (self.0 & 0x20) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIdpR(pub u64);
impl InstIdpR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn is_hi(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn src_a_sign(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn is_dp(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn src_b_sign(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIdpC(pub u64);
impl InstIdpC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn is_hi(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn src_a_sign(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn is_dp(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn src_b_sign(&self) -> bool { (self.0 & 0x800000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadR(pub u64);
impl InstImadR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadI(pub u64);
impl InstImadI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadC(pub u64);
impl InstImadC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadRc(pub u64);
impl InstImadRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 51) & 0x3) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImad32i(pub u64);
impl InstImad32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn b_signed(&self) -> bool { (self.0 & 0x200000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 55) & 0x3) as u32) }
    pub fn a_signed(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadspR(pub u64);
impl InstImadspR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn a_select(&self) -> ImadspASelect { ImadspASelect(((self.0 >> 48) & 0x7) as u32) }
    pub fn b_select(&self) -> ImadspBSelect { ImadspBSelect(((self.0 >> 53) & 0x3) as u32) }
    pub fn c_select(&self) -> ImadspASelect { ImadspASelect(((((self.0 >> 50) & 0x6) as i32) | (((self.0 >> 48) & 0x1) as i32)) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadspI(pub u64);
impl InstImadspI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn a_select(&self) -> ImadspASelect { ImadspASelect(((self.0 >> 48) & 0x7) as u32) }
    pub fn b_select(&self) -> ImadspBSelect { ImadspBSelect(((self.0 >> 53) & 0x3) as u32) }
    pub fn c_select(&self) -> ImadspASelect { ImadspASelect(((((self.0 >> 50) & 0x6) as i32) | (((self.0 >> 48) & 0x1) as i32)) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadspC(pub u64);
impl InstImadspC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn a_select(&self) -> ImadspASelect { ImadspASelect(((self.0 >> 48) & 0x7) as u32) }
    pub fn b_select(&self) -> ImadspBSelect { ImadspBSelect(((self.0 >> 53) & 0x3) as u32) }
    pub fn c_select(&self) -> ImadspASelect { ImadspASelect(((((self.0 >> 50) & 0x6) as i32) | (((self.0 >> 48) & 0x1) as i32)) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImadspRc(pub u64);
impl InstImadspRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn a_select(&self) -> ImadspASelect { ImadspASelect(((self.0 >> 48) & 0x7) as u32) }
    pub fn b_select(&self) -> ImadspBSelect { ImadspBSelect(((self.0 >> 53) & 0x3) as u32) }
    pub fn c_select(&self) -> ImadspASelect { ImadspASelect(((((self.0 >> 50) & 0x6) as i32) | (((self.0 >> 48) & 0x1) as i32)) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImnmxR(pub u64);
impl InstImnmxR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x_mode(&self) -> XMode { XMode(((self.0 >> 43) & 0x3) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImnmxI(pub u64);
impl InstImnmxI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x_mode(&self) -> XMode { XMode(((self.0 >> 43) & 0x3) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImnmxC(pub u64);
impl InstImnmxC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x_mode(&self) -> XMode { XMode(((self.0 >> 43) & 0x3) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImulR(pub u64);
impl InstImulR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImulI(pub u64);
impl InstImulI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImulC(pub u64);
impl InstImulC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x20000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstImul32i(pub u64);
impl InstImul32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn a_signed(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn b_signed(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn hilo(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIpa(pub u64);
impl InstIpa {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ipa_op(&self) -> IpaOp { IpaOp(((self.0 >> 54) & 0x3) as u32) }
    pub fn msi(&self) -> i32 { (((self.0 >> 52) & 0x3) as i32) }
    pub fn sat(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn idx(&self) -> bool { (self.0 & 0x4000000000) != 0 }
    pub fn imm10(&self) -> i32 { (((self.0 >> 28) & 0x3FF) as i32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 47) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIsberd(pub u64);
impl InstIsberd {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn al_size(&self) -> AlSize { AlSize(((self.0 >> 47) & 0x3) as u32) }
    pub fn i_base(&self) -> IBase { IBase(((self.0 >> 33) & 0x3) as u32) }
    pub fn o(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn p(&self) -> bool { (self.0 & 0x80000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIscaddR(pub u64);
impl InstIscaddR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn imm5(&self) -> i32 { (((self.0 >> 39) & 0x1F) as i32) }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 48) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIscaddI(pub u64);
impl InstIscaddI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn imm5(&self) -> i32 { (((self.0 >> 39) & 0x1F) as i32) }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 48) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIscaddC(pub u64);
impl InstIscaddC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn imm5(&self) -> i32 { (((self.0 >> 39) & 0x1F) as i32) }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 48) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIscadd32i(pub u64);
impl InstIscadd32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn imm5(&self) -> i32 { (((self.0 >> 53) & 0x1F) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIsetR(pub u64);
impl InstIsetR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIsetI(pub u64);
impl InstIsetI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIsetC(pub u64);
impl InstIsetC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn b_val(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIsetpR(pub u64);
impl InstIsetpR {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIsetpI(pub u64);
impl InstIsetpI {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstIsetpC(pub u64);
impl InstIsetpC {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn i_comp(&self) -> IComp { IComp(((self.0 >> 49) & 0x7) as u32) }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn bop(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstJcal(pub u64);
impl InstJcal {
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn inc(&self) -> bool { (self.0 & 0x40) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstJmp(pub u64);
impl InstJmp {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn lmt(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn u(&self) -> bool { (self.0 & 0x80) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstJmx(pub u64);
impl InstJmx {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn lmt(&self) -> bool { (self.0 & 0x40) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstKil(pub u64);
impl InstKil {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLd(pub u64);
impl InstLd {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 58) & 0x7) as i32) }
    pub fn cache_op(&self) -> CacheOpLd { CacheOpLd(((self.0 >> 56) & 0x3) as u32) }
    pub fn ls_size(&self) -> LsSize { LsSize(((self.0 >> 53) & 0x7) as u32) }
    pub fn e(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLdc(pub u64);
impl InstLdc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ls_size(&self) -> LsSize2 { LsSize2(((self.0 >> 48) & 0x7) as u32) }
    pub fn address_mode(&self) -> AddressMode { AddressMode(((self.0 >> 44) & 0x3) as u32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 36) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLdg(pub u64);
impl InstLdg {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ls_size(&self) -> LsSize { LsSize(((self.0 >> 48) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOpLd { CacheOpLd(((self.0 >> 46) & 0x3) as u32) }
    pub fn e(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLdl(pub u64);
impl InstLdl {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ls_size(&self) -> LsSize2 { LsSize2(((self.0 >> 48) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOp2 { CacheOp2(((self.0 >> 44) & 0x3) as u32) }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLds(pub u64);
impl InstLds {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ls_size(&self) -> LsSize2 { LsSize2(((self.0 >> 48) & 0x7) as u32) }
    pub fn u(&self) -> bool { (self.0 & 0x100000000000) != 0 }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLeaR(pub u64);
impl InstLeaR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn imm_u5(&self) -> i32 { (((self.0 >> 39) & 0x1F) as i32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLeaI(pub u64);
impl InstLeaI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn imm_u5(&self) -> i32 { (((self.0 >> 39) & 0x1F) as i32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLeaC(pub u64);
impl InstLeaC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn imm_u5(&self) -> i32 { (((self.0 >> 39) & 0x1F) as i32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLeaHiR(pub u64);
impl InstLeaHiR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x4000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x2000000000) != 0 }
    pub fn imm_u5(&self) -> i32 { (((self.0 >> 28) & 0x1F) as i32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLeaHiC(pub u64);
impl InstLeaHiC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x200000000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn imm_u5(&self) -> i32 { (((self.0 >> 51) & 0x1F) as i32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLepc(pub u64);
impl InstLepc {
}

#[derive(Clone, Copy, Debug)]
pub struct InstLongjmp(pub u64);
impl InstLongjmp {
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLopR(pub u64);
impl InstLopR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
    pub fn predicate_op(&self) -> PredicateOp { PredicateOp(((self.0 >> 44) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn lop(&self) -> LogicOp { LogicOp(((self.0 >> 41) & 0x3) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x8000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLopI(pub u64);
impl InstLopI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
    pub fn predicate_op(&self) -> PredicateOp { PredicateOp(((self.0 >> 44) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn logic_op(&self) -> LogicOp { LogicOp(((self.0 >> 41) & 0x3) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x8000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLopC(pub u64);
impl InstLopC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
    pub fn predicate_op(&self) -> PredicateOp { PredicateOp(((self.0 >> 44) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn logic_op(&self) -> LogicOp { LogicOp(((self.0 >> 41) & 0x3) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x8000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLop3R(pub u64);
impl InstLop3R {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
    pub fn predicate_op(&self) -> PredicateOp { PredicateOp(((self.0 >> 36) & 0x3) as u32) }
    pub fn x(&self) -> bool { (self.0 & 0x4000000000) != 0 }
    pub fn imm(&self) -> i32 { (((self.0 >> 28) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLop3I(pub u64);
impl InstLop3I {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x200000000000000) != 0 }
    pub fn imm(&self) -> i32 { (((self.0 >> 48) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLop3C(pub u64);
impl InstLop3C {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn imm(&self) -> i32 { (((self.0 >> 48) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstLop32i(pub u64);
impl InstLop32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn x(&self) -> bool { (self.0 & 0x200000000000000) != 0 }
    pub fn logic_op(&self) -> LogicOp { LogicOp(((self.0 >> 53) & 0x3) as u32) }
    pub fn neg_a(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstMembar(pub u64);
impl InstMembar {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn membar(&self) -> Membar { Membar(((self.0 >> 8) & 0x3) as u32) }
    pub fn ivall(&self) -> Ivall { Ivall((self.0 & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstMovR(pub u64);
impl InstMovR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn quad_mask(&self) -> i32 { (((self.0 >> 39) & 0xF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstMovI(pub u64);
impl InstMovI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn quad_mask(&self) -> i32 { (((self.0 >> 39) & 0xF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstMovC(pub u64);
impl InstMovC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn quad_mask(&self) -> i32 { (((self.0 >> 39) & 0xF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstMov32i(pub u64);
impl InstMov32i {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn quad_mask(&self) -> i32 { (((self.0 >> 12) & 0xF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstMufu(pub u64);
impl InstMufu {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn mufu_op(&self) -> MufuOp { MufuOp(((self.0 >> 20) & 0xF) as u32) }
    pub fn abs_a(&self) -> bool { (self.0 & 0x400000000000) != 0 }
    pub fn neg_a(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstNop(pub u64);
impl InstNop {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm16(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
    pub fn trig(&self) -> bool { (self.0 & 0x2000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc(((self.0 >> 8) & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstOutR(pub u64);
impl InstOutR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn out_type(&self) -> OutType { OutType(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstOutI(pub u64);
impl InstOutI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn out_type(&self) -> OutType { OutType(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstOutC(pub u64);
impl InstOutC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn out_type(&self) -> OutType { OutType(((self.0 >> 39) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstP2rR(pub u64);
impl InstP2rR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn ccpr(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstP2rI(pub u64);
impl InstP2rI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn ccpr(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstP2rC(pub u64);
impl InstP2rC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn ccpr(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPbk(pub u64);
impl InstPbk {
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPcnt(pub u64);
impl InstPcnt {
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPexit(pub u64);
impl InstPexit {
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPixld(pub u64);
impl InstPixld {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 45) & 0x7) as i32) }
    pub fn pix_mode(&self) -> PixMode { PixMode(((self.0 >> 31) & 0x7) as u32) }
    pub fn imm8(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPlongjmp(pub u64);
impl InstPlongjmp {
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPopcR(pub u64);
impl InstPopcR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPopcI(pub u64);
impl InstPopcI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPopcC(pub u64);
impl InstPopcC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPret(pub u64);
impl InstPret {
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn inc(&self) -> bool { (self.0 & 0x40) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPrmtR(pub u64);
impl InstPrmtR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn p_mode(&self) -> PMode { PMode(((self.0 >> 48) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPrmtI(pub u64);
impl InstPrmtI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn p_mode(&self) -> PMode { PMode(((self.0 >> 48) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPrmtC(pub u64);
impl InstPrmtC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn p_mode(&self) -> PMode { PMode(((self.0 >> 48) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPrmtRc(pub u64);
impl InstPrmtRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn p_mode(&self) -> PMode { PMode(((self.0 >> 48) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPset(pub u64);
impl InstPset {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn src2_pred(&self) -> i32 { (((self.0 >> 12) & 0x7) as i32) }
    pub fn src2_pred_inv(&self) -> bool { (self.0 & 0x8000) != 0 }
    pub fn src1_pred(&self) -> i32 { (((self.0 >> 29) & 0x7) as i32) }
    pub fn src1_pred_inv(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bool_op_ab(&self) -> BoolOp { BoolOp(((self.0 >> 24) & 0x3) as u32) }
    pub fn bool_op_c(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn b_val(&self) -> bool { (self.0 & 0x100000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstPsetp(pub u64);
impl InstPsetp {
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn src2_pred(&self) -> i32 { (((self.0 >> 12) & 0x7) as i32) }
    pub fn src2_pred_inv(&self) -> bool { (self.0 & 0x8000) != 0 }
    pub fn src1_pred(&self) -> i32 { (((self.0 >> 29) & 0x7) as i32) }
    pub fn src1_pred_inv(&self) -> bool { (self.0 & 0x100000000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn bool_op_ab(&self) -> BoolOp { BoolOp(((self.0 >> 24) & 0x3) as u32) }
    pub fn bool_op_c(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstR2b(pub u64);
impl InstR2b {
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn mode(&self) -> BarMode { BarMode(((self.0 >> 32) & 0x3) as u32) }
    pub fn name(&self) -> i32 { (((self.0 >> 28) & 0xF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstR2pR(pub u64);
impl InstR2pR {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn ccpr(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstR2pI(pub u64);
impl InstR2pI {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn ccpr(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstR2pC(pub u64);
impl InstR2pC {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn byte_sel(&self) -> ByteSel { ByteSel(((self.0 >> 41) & 0x3) as u32) }
    pub fn ccpr(&self) -> bool { (self.0 & 0x10000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstRam(pub u64);
impl InstRam {
}

#[derive(Clone, Copy, Debug)]
pub struct InstRed(pub u64);
impl InstRed {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm20(&self) -> i32 { (((self.0 >> 28) & 0xFFFFF) as i32) }
    pub fn red_size(&self) -> AtomSize { AtomSize(((self.0 >> 20) & 0x7) as u32) }
    pub fn red_op(&self) -> RedOp { RedOp(((self.0 >> 23) & 0x7) as u32) }
    pub fn e(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstRet(pub u64);
impl InstRet {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstRroR(pub u64);
impl InstRroR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn rro_op(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstRroI(pub u64);
impl InstRroI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn rro_op(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstRroC(pub u64);
impl InstRroC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn abs_b(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn neg_b(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn rro_op(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstRtt(pub u64);
impl InstRtt {
}

#[derive(Clone, Copy, Debug)]
pub struct InstS2r(pub u64);
impl InstS2r {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn s_reg(&self) -> SReg { SReg(((self.0 >> 20) & 0xFF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSam(pub u64);
impl InstSam {
}

#[derive(Clone, Copy, Debug)]
pub struct InstSelR(pub u64);
impl InstSelR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSelI(pub u64);
impl InstSelI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSelC(pub u64);
impl InstSelC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSetcrsptr(pub u64);
impl InstSetcrsptr {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSetlmembase(pub u64);
impl InstSetlmembase {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShfLR(pub u64);
impl InstShfLR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x_mode_shf(&self) -> XModeShf { XModeShf(((self.0 >> 48) & 0x3) as u32) }
    pub fn max_shift(&self) -> MaxShift { MaxShift(((self.0 >> 37) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShfRR(pub u64);
impl InstShfRR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x_mode_shf(&self) -> XModeShf { XModeShf(((self.0 >> 48) & 0x3) as u32) }
    pub fn max_shift(&self) -> MaxShift { MaxShift(((self.0 >> 37) & 0x3) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShfLI(pub u64);
impl InstShfLI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x_mode_shf(&self) -> XModeShf { XModeShf(((self.0 >> 48) & 0x3) as u32) }
    pub fn max_shift(&self) -> MaxShift { MaxShift(((self.0 >> 37) & 0x3) as u32) }
    pub fn imm6(&self) -> i32 { (((self.0 >> 20) & 0x3F) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShfRI(pub u64);
impl InstShfRI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn x_mode_shf(&self) -> XModeShf { XModeShf(((self.0 >> 48) & 0x3) as u32) }
    pub fn max_shift(&self) -> MaxShift { MaxShift(((self.0 >> 37) & 0x3) as u32) }
    pub fn imm6(&self) -> i32 { (((self.0 >> 20) & 0x3F) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShfl(pub u64);
impl InstShfl {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn src_b_imm(&self) -> i32 { (((self.0 >> 20) & 0x1F) as i32) }
    pub fn src_c_imm(&self) -> i32 { (((self.0 >> 34) & 0x1FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn shfl_mode(&self) -> ShflMode { ShflMode(((self.0 >> 30) & 0x3) as u32) }
    pub fn c_fix_shfl(&self) -> bool { (self.0 & 0x20000000) != 0 }
    pub fn b_fix_shfl(&self) -> bool { (self.0 & 0x10000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 48) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShlR(pub u64);
impl InstShlR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShlI(pub u64);
impl InstShlI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShlC(pub u64);
impl InstShlC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x80000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShrR(pub u64);
impl InstShrR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x_mode(&self) -> XMode { XMode(((self.0 >> 43) & 0x3) as u32) }
    pub fn brev(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShrI(pub u64);
impl InstShrI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn imm20(&self) -> i32 { (((self.0 >> 37) & 0x80000) as i32) | (((self.0 >> 20) & 0x7FFFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x_mode(&self) -> XMode { XMode(((self.0 >> 43) & 0x3) as u32) }
    pub fn brev(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstShrC(pub u64);
impl InstShrC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x_mode(&self) -> XMode { XMode(((self.0 >> 43) & 0x3) as u32) }
    pub fn brev(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn m(&self) -> bool { (self.0 & 0x8000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSsy(pub u64);
impl InstSsy {
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
    pub fn ca(&self) -> bool { (self.0 & 0x20) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSt(pub u64);
impl InstSt {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 58) & 0x7) as i32) }
    pub fn cache_op(&self) -> CacheOpSt { CacheOpSt(((self.0 >> 56) & 0x3) as u32) }
    pub fn ls_size(&self) -> LsSize { LsSize(((self.0 >> 53) & 0x7) as u32) }
    pub fn e(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn imm32(&self) -> i32 { ((self.0 >> 20) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstStg(pub u64);
impl InstStg {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ls_size(&self) -> LsSize2 { LsSize2(((self.0 >> 48) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOpSt { CacheOpSt(((self.0 >> 46) & 0x3) as u32) }
    pub fn e(&self) -> bool { (self.0 & 0x200000000000) != 0 }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstStl(pub u64);
impl InstStl {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ls_size(&self) -> LsSize2 { LsSize2(((self.0 >> 48) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOpSt { CacheOpSt(((self.0 >> 44) & 0x3) as u32) }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstStp(pub u64);
impl InstStp {
    pub fn wait(&self) -> bool { (self.0 & 0x80000000) != 0 }
    pub fn imm8(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSts(pub u64);
impl InstSts {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ls_size(&self) -> LsSize2 { LsSize2(((self.0 >> 48) & 0x7) as u32) }
    pub fn imm24(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuatomB(pub u64);
impl InstSuatomB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn size(&self) -> SuatomSize { SuatomSize(((self.0 >> 36) & 0x7) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn op(&self) -> SuatomOp { SuatomOp(((self.0 >> 29) & 0xF) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x10000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuatom(pub u64);
impl InstSuatom {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn size(&self) -> SuatomSize { SuatomSize(((self.0 >> 51) & 0x7) as u32) }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn op(&self) -> SuatomOp { SuatomOp(((self.0 >> 29) & 0xF) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x10000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuatomB2(pub u64);
impl InstSuatomB2 {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn size(&self) -> SuatomSize { SuatomSize(((self.0 >> 36) & 0x7) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn op(&self) -> SuatomOp { SuatomOp(((self.0 >> 29) & 0xF) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x10000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuatomCasB(pub u64);
impl InstSuatomCasB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn size(&self) -> SuatomSize { SuatomSize(((self.0 >> 36) & 0x7) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 30) & 0x7) as i32) }
    pub fn ba(&self) -> bool { (self.0 & 0x10000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuatomCas(pub u64);
impl InstSuatomCas {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn size(&self) -> SuatomSize { SuatomSize(((self.0 >> 51) & 0x7) as u32) }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 30) & 0x7) as i32) }
    pub fn ba(&self) -> bool { (self.0 & 0x10000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuldDB(pub u64);
impl InstSuldDB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn dest_pred2(&self) -> i32 { (((self.0 >> 30) & 0x7) as i32) }
    pub fn cache_op(&self) -> CacheOpLd { CacheOpLd(((self.0 >> 24) & 0x3) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x800000) != 0 }
    pub fn size(&self) -> SuSize { SuSize(((self.0 >> 20) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuldD(pub u64);
impl InstSuldD {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn dest_pred2(&self) -> i32 { (((self.0 >> 30) & 0x7) as i32) }
    pub fn cache_op(&self) -> CacheOpLd { CacheOpLd(((self.0 >> 24) & 0x3) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x800000) != 0 }
    pub fn size(&self) -> SuSize { SuSize(((self.0 >> 20) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuldB(pub u64);
impl InstSuldB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn dest_pred2(&self) -> i32 { (((self.0 >> 30) & 0x7) as i32) }
    pub fn cache_op(&self) -> CacheOpLd { CacheOpLd(((self.0 >> 24) & 0x3) as u32) }
    pub fn rgba(&self) -> SuRgba { SuRgba(((self.0 >> 20) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuld(pub u64);
impl InstSuld {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn dest_pred2(&self) -> i32 { (((self.0 >> 30) & 0x7) as i32) }
    pub fn cache_op(&self) -> CacheOpLd { CacheOpLd(((self.0 >> 24) & 0x3) as u32) }
    pub fn rgba(&self) -> SuRgba { SuRgba(((self.0 >> 20) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSuredB(pub u64);
impl InstSuredB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn op(&self) -> RedOp { RedOp(((self.0 >> 24) & 0x7) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x800000) != 0 }
    pub fn size(&self) -> SuatomSize { SuatomSize(((self.0 >> 20) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSured(pub u64);
impl InstSured {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn op(&self) -> RedOp { RedOp(((self.0 >> 24) & 0x7) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x800000) != 0 }
    pub fn size(&self) -> SuatomSize { SuatomSize(((self.0 >> 20) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSustDB(pub u64);
impl InstSustDB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOpSt { CacheOpSt(((self.0 >> 24) & 0x3) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x800000) != 0 }
    pub fn size(&self) -> SuSize { SuSize(((self.0 >> 20) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSustD(pub u64);
impl InstSustD {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOpSt { CacheOpSt(((self.0 >> 24) & 0x3) as u32) }
    pub fn ba(&self) -> bool { (self.0 & 0x800000) != 0 }
    pub fn size(&self) -> SuSize { SuSize(((self.0 >> 20) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSustB(pub u64);
impl InstSustB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOpSt { CacheOpSt(((self.0 >> 24) & 0x3) as u32) }
    pub fn rgba(&self) -> SuRgba { SuRgba(((self.0 >> 20) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSust(pub u64);
impl InstSust {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn clamp(&self) -> Clamp { Clamp(((self.0 >> 49) & 0x3) as u32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn dim(&self) -> SuDim { SuDim(((self.0 >> 33) & 0x7) as u32) }
    pub fn cache_op(&self) -> CacheOpSt { CacheOpSt(((self.0 >> 24) & 0x3) as u32) }
    pub fn rgba(&self) -> SuRgba { SuRgba(((self.0 >> 20) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstSync(pub u64);
impl InstSync {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn ccc(&self) -> Ccc { Ccc((self.0 & 0x1F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTex(pub u64);
impl InstTex {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn lc(&self) -> bool { (self.0 & 0x400000000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn lod(&self) -> Lod { Lod(((self.0 >> 55) & 0x7) as u32) }
    pub fn aoffi(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn dc(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn ndv(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTexB(pub u64);
impl InstTexB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn lcb(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn lodb(&self) -> Lod { Lod(((self.0 >> 37) & 0x7) as u32) }
    pub fn aoffib(&self) -> bool { (self.0 & 0x1000000000) != 0 }
    pub fn dc(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn ndv(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTexs(pub u64);
impl InstTexs {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn target(&self) -> TexsTarget { TexsTarget(((self.0 >> 53) & 0xF) as u32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 50) & 0x7) as i32) }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn dest2(&self) -> i32 { (((self.0 >> 28) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTld(pub u64);
impl InstTld {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn lod(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn toff(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn ms(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn cl(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTldB(pub u64);
impl InstTldB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn lod(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn toff(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn ms(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn cl(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTlds(pub u64);
impl InstTlds {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn target(&self) -> TldsTarget { TldsTarget(((self.0 >> 53) & 0xF) as u32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 50) & 0x7) as i32) }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn dest2(&self) -> i32 { (((self.0 >> 28) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTld4(pub u64);
impl InstTld4 {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn lc(&self) -> bool { (self.0 & 0x400000000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn tex_comp(&self) -> TexComp { TexComp(((self.0 >> 56) & 0x3) as u32) }
    pub fn toff(&self) -> TexOffset { TexOffset(((self.0 >> 54) & 0x3) as u32) }
    pub fn dc(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn ndv(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTld4B(pub u64);
impl InstTld4B {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn lc(&self) -> bool { (self.0 & 0x10000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn tex_comp(&self) -> TexComp { TexComp(((self.0 >> 38) & 0x3) as u32) }
    pub fn toff(&self) -> TexOffset { TexOffset(((self.0 >> 36) & 0x3) as u32) }
    pub fn dc(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn ndv(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTld4s(pub u64);
impl InstTld4s {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn tex_comp(&self) -> TexComp { TexComp(((self.0 >> 52) & 0x3) as u32) }
    pub fn aoffi(&self) -> bool { (self.0 & 0x8000000000000) != 0 }
    pub fn dc(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn dest2(&self) -> i32 { (((self.0 >> 28) & 0xFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTmml(pub u64);
impl InstTmml {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn ndv(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTmmlB(pub u64);
impl InstTmmlB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn ndv(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTxa(pub u64);
impl InstTxa {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn ndv(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTxd(pub u64);
impl InstTxd {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn lc(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn toff(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTxdB(pub u64);
impl InstTxdB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 51) & 0x7) as i32) }
    pub fn lc(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn toff(&self) -> bool { (self.0 & 0x800000000) != 0 }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn dim(&self) -> TexDim { TexDim(((self.0 >> 28) & 0x7) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTxq(pub u64);
impl InstTxq {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn tid_b(&self) -> i32 { (((self.0 >> 36) & 0x1FFF) as i32) }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn tex_query(&self) -> TexQuery { TexQuery(((self.0 >> 22) & 0x3F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstTxqB(pub u64);
impl InstTxqB {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn nodep(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn w_mask(&self) -> i32 { (((self.0 >> 31) & 0xF) as i32) }
    pub fn tex_query(&self) -> TexQuery { TexQuery(((self.0 >> 22) & 0x3F) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVabsdiff(pub u64);
impl InstVabsdiff {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn d_format(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 46) & 0x8) as i32) | (((self.0 >> 28) & 0x7) as i32)) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn video_op(&self) -> VideoOp { VideoOp(((self.0 >> 51) & 0x7) as u32) }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVabsdiff4(pub u64);
impl InstVabsdiff4 {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn v_red(&self) -> VideoRed { VideoRed(((self.0 >> 53) & 0x3) as u32) }
    pub fn lane_mask4(&self) -> LaneMask4 { LaneMask4(((((self.0 >> 49) & 0xC) as i32) | (((self.0 >> 36) & 0x3) as i32)) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
    pub fn src_b_fmt(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn src_a_fmt(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn d_format(&self) -> bool { (self.0 & 0x4000000000) != 0 }
    pub fn asel4(&self) -> ASelect4 { ASelect4(((self.0 >> 32) & 0xF) as u32) }
    pub fn bsel4(&self) -> BSelect4 { BSelect4(((self.0 >> 28) & 0xF) as u32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVadd(pub u64);
impl InstVadd {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm16(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 56) & 0x3) as u32) }
    pub fn d_format(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 46) & 0x8) as i32) | (((self.0 >> 28) & 0x7) as i32)) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn video_op(&self) -> VideoOp { VideoOp(((self.0 >> 51) & 0x7) as u32) }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVmad(pub u64);
impl InstVmad {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm16(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 46) & 0x8) as i32) | (((self.0 >> 28) & 0x7) as i32)) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn avg_mode(&self) -> AvgMode { AvgMode(((self.0 >> 53) & 0x3) as u32) }
    pub fn video_scale(&self) -> VideoScale { VideoScale(((self.0 >> 51) & 0x3) as u32) }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVmnmx(pub u64);
impl InstVmnmx {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm16(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn d_format(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 46) & 0x8) as i32) | (((self.0 >> 28) & 0x7) as i32)) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn video_op(&self) -> VideoOp { VideoOp(((self.0 >> 51) & 0x7) as u32) }
    pub fn mn(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVote(pub u64);
impl InstVote {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn vote_mode(&self) -> VoteMode { VoteMode(((self.0 >> 48) & 0x3) as u32) }
    pub fn vp_dest(&self) -> i32 { (((self.0 >> 45) & 0x7) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVotevtg(pub u64);
impl InstVotevtg {
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn vote_mode(&self) -> VoteMode { VoteMode(((self.0 >> 48) & 0x3) as u32) }
    pub fn imm28(&self) -> i32 { (((self.0 >> 20) & 0xFFFFFFF) as i32) }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVset(pub u64);
impl InstVset {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn v_comp(&self) -> IComp { IComp(((self.0 >> 54) & 0x7) as u32) }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 46) & 0x8) as i32) | (((self.0 >> 28) & 0x7) as i32)) as u32) }
    pub fn video_op(&self) -> VideoOp { VideoOp(((self.0 >> 51) & 0x7) as u32) }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVsetp(pub u64);
impl InstVsetp {
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn imm16(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 46) & 0x8) as i32) | (((self.0 >> 28) & 0x7) as i32)) as u32) }
    pub fn v_comp(&self) -> IComp { IComp(((((self.0 >> 45) & 0x4) as i32) | (((self.0 >> 43) & 0x3) as i32)) as u32) }
    pub fn bool_op(&self) -> BoolOp { BoolOp(((self.0 >> 45) & 0x3) as u32) }
    pub fn src_pred(&self) -> i32 { (((self.0 >> 39) & 0x7) as i32) }
    pub fn src_pred_inv(&self) -> bool { (self.0 & 0x40000000000) != 0 }
    pub fn dest_pred(&self) -> i32 { (((self.0 >> 3) & 0x7) as i32) }
    pub fn dest_pred_inv(&self) -> i32 { ((self.0 & 0x7) as i32) }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVshl(pub u64);
impl InstVshl {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn mv(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn d_format(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((self.0 >> 28) & 0x7) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn video_op(&self) -> VideoOp { VideoOp(((self.0 >> 51) & 0x7) as u32) }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstVshr(pub u64);
impl InstVshr {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn mv(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn d_format(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn a_select(&self) -> VectorSelect { VectorSelect(((((self.0 >> 45) & 0x8) as i32) | (((self.0 >> 36) & 0x7) as i32)) as u32) }
    pub fn b_select(&self) -> VectorSelect { VectorSelect(((self.0 >> 28) & 0x7) as u32) }
    pub fn sat(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn video_op(&self) -> VideoOp { VideoOp(((self.0 >> 51) & 0x7) as u32) }
    pub fn b_video(&self) -> bool { (self.0 & 0x4000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstXmadR(pub u64);
impl InstXmadR {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_b(&self) -> i32 { (((self.0 >> 20) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn hilo_a(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn xmad_cop(&self) -> XmadCop { XmadCop(((self.0 >> 50) & 0x7) as u32) }
    pub fn b_signed(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x4000000000) != 0 }
    pub fn mrg(&self) -> bool { (self.0 & 0x2000000000) != 0 }
    pub fn psl(&self) -> bool { (self.0 & 0x1000000000) != 0 }
    pub fn hilo_b(&self) -> bool { (self.0 & 0x800000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstXmadI(pub u64);
impl InstXmadI {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn imm16(&self) -> i32 { (((self.0 >> 20) & 0xFFFF) as i32) }
    pub fn hilo_a(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn xmad_cop(&self) -> XmadCop { XmadCop(((self.0 >> 50) & 0x7) as u32) }
    pub fn b_signed(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x4000000000) != 0 }
    pub fn mrg(&self) -> bool { (self.0 & 0x2000000000) != 0 }
    pub fn psl(&self) -> bool { (self.0 & 0x1000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstXmadC(pub u64);
impl InstXmadC {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn mrg(&self) -> bool { (self.0 & 0x100000000000000) != 0 }
    pub fn psl(&self) -> bool { (self.0 & 0x80000000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn hilo_a(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn hilo_b(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn xmad_cop(&self) -> XmadCop2 { XmadCop2(((self.0 >> 50) & 0x3) as u32) }
    pub fn b_signed(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct InstXmadRc(pub u64);
impl InstXmadRc {
    pub fn dest(&self) -> i32 { ((self.0 & 0xFF) as i32) }
    pub fn src_a(&self) -> i32 { (((self.0 >> 8) & 0xFF) as i32) }
    pub fn src_c(&self) -> i32 { (((self.0 >> 39) & 0xFF) as i32) }
    pub fn cbuf_slot(&self) -> i32 { (((self.0 >> 34) & 0x1F) as i32) }
    pub fn cbuf_offset(&self) -> i32 { (((self.0 >> 20) & 0x3FFF) as i32) }
    pub fn pred(&self) -> i32 { (((self.0 >> 16) & 0x7) as i32) }
    pub fn pred_inv(&self) -> bool { (self.0 & 0x80000) != 0 }
    pub fn write_cc(&self) -> bool { (self.0 & 0x800000000000) != 0 }
    pub fn x(&self) -> bool { (self.0 & 0x40000000000000) != 0 }
    pub fn hilo_a(&self) -> bool { (self.0 & 0x20000000000000) != 0 }
    pub fn hilo_b(&self) -> bool { (self.0 & 0x10000000000000) != 0 }
    pub fn xmad_cop(&self) -> XmadCop2 { XmadCop2(((self.0 >> 50) & 0x3) as u32) }
    pub fn b_signed(&self) -> bool { (self.0 & 0x2000000000000) != 0 }
    pub fn a_signed(&self) -> bool { (self.0 & 0x1000000000000) != 0 }
}

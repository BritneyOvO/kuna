//! (kuna `x86listingswitch`) Bounded i386 absolute-table switch successors for
//! the discovery Listing.
//!
//! The Listing walk gives a computed jump no successor, so on i386 every case
//! body behind `jmp dword ptr [idx*4 + table]` stays undecoded, and the AIF gap
//! walk later admits a case that opens with a prologue as a function. This
//! module runs once the ordinary walk (serial or parallel) has finished, over
//! that reconciled partition, and adds case bodies only where the bound on the
//! table index is proved locally:
//!
//!  * the jump's own p-code loads a 4-byte word from `table + idx * 4`, with
//!    `idx` a 4-byte register and `table` a constant, and no wrap at 32 bits;
//!  * a straight-line block `cmp ...; jcc out; jmp [...]` precedes it, where the
//!    `jcc`'s taken target leaves the block, every flag it reads is a comparison
//!    of that same `idx` register with a constant, nothing in the block writes
//!    any byte of `idx`, and no edge or function entry enters the block below
//!    its first instruction;
//!  * evaluated over every 32-bit value of `idx`, the values that fall through
//!    to the jump are exactly `[0, n)`;
//!  * each of the first `min(n, jumptablemax)` words is aligned, read-only,
//!    file-backed, and an address in the jump's executable section that is not
//!    a function entry, not inside the table and not inside another instruction.
//!
//! Anything short of that leaves the dispatch as it was. Admitted targets are
//! walked as instruction roots with the ordinary successor rules into an
//! isolated candidate, which is reconciled and re-examined round by round; the
//! candidate is published only when no round admits more and every admitted
//! proof still holds against the final partition. Otherwise the ordinary walk's
//! state is returned untouched.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::kuna_insnstore::InstructionStore;
use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{is_constant, key_of, keys_overlap, Key, MIN_ENTRIES};
use super::model::{DiscoveredFunction, FlowKind, Insn, InsnLite, Mnemonic, Reference, RefKind};
use super::walk::{self, CallbackEvidence, CallbackSink, FuncSink, InsnSink, RefSink, StepCtx, Successors, WalkState};
use super::xrefs::{decode, FullCapture, FullOp};

/// Rounds of admission before the candidate is abandoned as unstable.
pub(super) const MAX_ROUNDS: usize = 16;

/// Instructions the candidate may add before it is abandoned.
pub(super) const MAX_CANDIDATE_INSNS: usize = 1 << 22;

/// Instructions between the comparison and the dispatch, both included.
const MAX_BLOCK_INSNS: usize = 8;

const WIDTH: u32 = 4;
const MASK: u64 = 0xffff_ffff;

/// The read-only image the tables are read from, for one linked i386 object.
pub(super) struct SwitchTables<'a> {
    pool: PoolImage<'a>,
    data_space: Option<Rc<AddrSpace>>,
    max_entries: usize,
}

impl<'a> SwitchTables<'a> {
    /// `None` unless the option is on and `file` is a linked i386 ELF or PE.
    pub(super) fn new(file: &'a object::File<'a>, arch: &Architecture) -> Option<Self> {
        use object::Object;
        if !arch.analysis_x86listingswitch
            || file.architecture() != object::Architecture::I386
            || file.kind() == object::ObjectKind::Relocatable
            || !matches!(file.format(), object::BinaryFormat::Elf | object::BinaryFormat::Pe)
        {
            return None;
        }
        let max_entries = match arch.max_jumptable_size as usize {
            0 => super::kuna_switchtable::DEFAULT_MAX_ENTRIES,
            n => n,
        };
        Some(SwitchTables {
            pool: PoolImage::new(file)?,
            data_space: arch.manage().get_default_data_space().cloned(),
            max_entries,
        })
    }
}

/// One admitted dispatch: its guarded block and the cases read for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Proof {
    pub dispatch: u64,
    /// The block from the comparison to the dispatch, in address order.
    pub block: Vec<u64>,
    /// The case bodies, in table order, without repeats.
    pub cases: Vec<u64>,
}

/// The ordinary walk's state plus the candidate's additions, read as one.
struct View<'s> {
    base: &'s WalkState,
    cand: &'s Candidate,
}

#[derive(Default)]
struct Candidate {
    insns: BTreeMap<u64, Insn>,
    funcs: BTreeMap<u64, DiscoveredFunction>,
    refs: Vec<Reference>,
    callbacks: Vec<(u64, u64)>,
}

impl View<'_> {
    fn insn(&self, at: u64) -> Option<&Insn> {
        self.cand.insns.get(&at).or_else(|| self.base.insns.get(&at))
    }

    fn before(&self, at: u64) -> Option<&Insn> {
        let a = self.base.insns.range(..at).next_back().map(|(_, i)| i);
        let b = self.cand.insns.range(..at).next_back().map(|(_, i)| i);
        match (a, b) {
            (Some(a), Some(b)) => Some(if a.addr >= b.addr { a } else { b }),
            (a, b) => a.or(b),
        }
    }

    fn is_function(&self, at: u64) -> bool {
        self.base.funcs.contains_key(&at) || self.cand.funcs.contains_key(&at)
    }

    fn covered_inside(&self, at: u64) -> bool {
        self.insn(at).is_none()
            && self.before(at).is_some_and(|i| at < i.addr.wrapping_add(u64::from(i.len)))
    }

    fn all(&self) -> impl Iterator<Item = &Insn> {
        self.base.insns.iter().map(|(_, i)| i).chain(self.cand.insns.values())
    }
}

fn is_dispatch(insn: &Insn) -> bool {
    insn.flow.kind == FlowKind::ComputedJump && !insn.flow.is_call && insn.flows.is_empty()
}

/// Extend `st` with every provable switch's case bodies, or return it as it
/// was. The flag says whether anything was added.
pub(super) fn reconcile(ctx: &StepCtx<'_>, tables: &SwitchTables<'_>, mut st: WalkState) -> (WalkState, bool) {
    let base_dispatches: Vec<u64> = st.insns.iter().filter(|(_, i)| is_dispatch(i)).map(|(&a, _)| a).collect();
    if base_dispatches.is_empty() {
        return (st, false);
    }
    let mut cand = Candidate::default();
    let mut admitted: BTreeMap<u64, Proof> = BTreeMap::new();
    let mut stable = false;
    for round in 0..=MAX_ROUNDS {
        let fresh = {
            let view = View { base: &st, cand: &cand };
            let pending = base_dispatches
                .iter()
                .copied()
                .chain(cand.insns.values().filter(|i| is_dispatch(i)).map(|i| i.addr))
                .filter(|d| !admitted.contains_key(d));
            let proofs: Vec<Proof> = pending.filter_map(|d| prove(ctx, tables, &view, d)).collect();
            unbypassed(&view, proofs)
        };
        if fresh.is_empty() {
            stable = true;
            break;
        }
        if round == MAX_ROUNDS {
            break;
        }
        let mut roots = Vec::new();
        for proof in fresh {
            for &case in &proof.cases {
                roots.push(case);
                if ctx.detail.refs {
                    cand.refs.push(Reference { from: proof.dispatch, to: case, kind: RefKind::Code, op_index: None });
                }
            }
            admitted.insert(proof.dispatch, proof);
        }
        if !extend(ctx, &mut st, &mut cand, roots) {
            break;
        }
    }
    if !stable || admitted.is_empty() {
        return (st, false);
    }
    let still_valid = {
        let view = View { base: &st, cand: &cand };
        let again: Vec<Proof> = admitted.keys().filter_map(|&d| prove(ctx, tables, &view, d)).collect();
        let again = unbypassed(&view, again);
        again.len() == admitted.len() && again.iter().all(|p| admitted.get(&p.dispatch) == Some(p))
    };
    if !still_valid {
        return (st, false);
    }
    publish(&mut st, cand);
    (st, true)
}

fn publish(st: &mut WalkState, cand: Candidate) {
    for (addr, insn) in cand.insns {
        st.insns.insert(addr, insn);
    }
    for (entry, f) in cand.funcs {
        st.funcs.entry(entry).or_insert(f);
    }
    st.refs.extend(cand.refs);
    if !cand.callbacks.is_empty() {
        let mut evidence = CallbackEvidence::default();
        for (&target, &source) in &st.stack_callback_refs {
            evidence.file(source, target);
        }
        for (source, target) in cand.callbacks {
            evidence.file(source, target);
        }
        st.stack_callback_refs = evidence.into_refs();
    }
}

/// Walk `roots` with the ordinary successor rules into the candidate. `false`
/// when the candidate outgrew its budget.
fn extend(ctx: &StepCtx<'_>, st: &mut WalkState, cand: &mut Candidate, roots: Vec<u64>) -> bool {
    let mut insns = Overlay { base: &mut st.insns, cand: &mut cand.insns };
    let mut funcs = FuncOverlay { base: &st.funcs, cand: &mut cand.funcs };
    let mut refs = RefOverlay { want: ctx.detail.refs, edges: &mut cand.refs };
    let mut callbacks = CallbackOverlay { sites: &mut cand.callbacks };
    let mut work = Stack { todo: roots };
    while let Some(vma) = work.todo.pop() {
        walk::step(ctx, vma, &mut insns, &mut funcs, &mut refs, &mut callbacks, &mut work);
        if insns.cand.len() > MAX_CANDIDATE_INSNS {
            return false;
        }
    }
    true
}

struct Overlay<'a> {
    base: &'a mut InstructionStore,
    cand: &'a mut BTreeMap<u64, Insn>,
}

impl InsnSink for Overlay<'_> {
    fn decoded(&self, vma: u64) -> bool {
        self.base.contains_key(&vma) || self.cand.contains_key(&vma)
    }

    fn intern_mnemonic(&mut self, text: &str) -> Mnemonic {
        self.base.intern_mnemonic(text)
    }

    fn record(&mut self, insn: InsnLite) {
        self.cand.insert(insn.addr, Insn::from(insn));
    }
}

struct FuncOverlay<'a> {
    base: &'a BTreeMap<u64, DiscoveredFunction>,
    cand: &'a mut BTreeMap<u64, DiscoveredFunction>,
}

impl FuncSink for FuncOverlay<'_> {
    fn claim(&mut self, entry: u64) {
        if !self.base.contains_key(&entry) {
            self.cand.entry(entry).or_insert_with(|| walk::discovered(entry));
        }
    }
}

struct RefOverlay<'a> {
    want: bool,
    edges: &'a mut Vec<Reference>,
}

impl RefSink for RefOverlay<'_> {
    fn file(&mut self, from: u64, to: u64, kind: RefKind) {
        if self.want {
            self.edges.push(Reference { from, to, kind, op_index: None });
        }
    }
}

struct CallbackOverlay<'a> {
    sites: &'a mut Vec<(u64, u64)>,
}

impl CallbackSink for CallbackOverlay<'_> {
    fn file(&mut self, source: u64, target: u64) {
        self.sites.push((source, target));
    }
}

struct Stack {
    todo: Vec<u64>,
}

impl Successors for Stack {
    fn insn(&mut self, vma: u64) {
        self.todo.push(vma);
    }
    fn func(&mut self, entry: u64) {
        self.todo.push(entry);
    }
}

/// The proofs whose blocks nothing enters below their first instruction: no
/// edge from anything but the block predecessor, and no function entry.
fn unbypassed(view: &View<'_>, proofs: Vec<Proof>) -> Vec<Proof> {
    if proofs.is_empty() {
        return proofs;
    }
    let mut expected: HashMap<u64, u64> = HashMap::new();
    for p in &proofs {
        for pair in p.block.windows(2) {
            expected.insert(pair[1], pair[0]);
        }
    }
    let mut entered: BTreeSet<u64> = expected.keys().copied().filter(|&b| view.is_function(b)).collect();
    for insn in view.all() {
        for t in insn.flows.iter().copied().chain(insn.fall_through) {
            if expected.get(&t).is_some_and(|&pred| pred != insn.addr) {
                entered.insert(t);
            }
        }
    }
    proofs
        .into_iter()
        .filter(|p| !p.block[1..].iter().any(|b| entered.contains(b)))
        .collect()
}

/// The proof for the computed jump at `dispatch`, or `None`.
fn prove(ctx: &StepCtx<'_>, tables: &SwitchTables<'_>, view: &View<'_>, dispatch: u64) -> Option<Proof> {
    let data = tables.data_space.as_ref();
    let mut cap = FullCapture::default();
    let d = view.insn(dispatch)?;
    if !is_dispatch(d) || view.is_function(dispatch) {
        return None;
    }
    decode(ctx.translate, dispatch, ctx.code_space, &mut cap)?;
    let (table, idx) = table_load(cap.ops(), data)?;

    let mut block = vec![dispatch];
    let mut cur = dispatch;
    let mut guard: Option<(u64, Vec<FullOp>, usize, Vec<Key>)> = None;
    // The register whose value bounds the index, and whether one zero
    // extension has already been stepped over to reach it.
    let mut key = idx;
    let mut extended = false;
    let mut pending: Option<(Key, Vec<(Key, Pred)>)> = None;
    let (preds, compared) = loop {
        if block.len() >= MAX_BLOCK_INSNS {
            return None;
        }
        let p = view.before(cur)?;
        if p.addr.wrapping_add(u64::from(p.len)) != cur || p.fall_through != Some(cur) || p.flow.is_call {
            return None;
        }
        let len = decode(ctx.translate, p.addr, ctx.code_space, &mut cap)?;
        if len != p.len {
            return None;
        }
        let ops = cap.ops();
        let plain = p.flow.kind == FlowKind::Fallthrough && !has_effects(ops);
        block.push(p.addr);
        cur = p.addr;
        if let Some((narrow, preds)) = &pending {
            if !plain {
                return None;
            }
            if writes(ops, data, |k| keys_overlap(k, key)) {
                if zero_extension(ops, data, key) != Some(*narrow) {
                    return None;
                }
                break (preds.clone(), *narrow);
            }
            if writes(ops, data, |k| keys_overlap(k, *narrow)) {
                return None;
            }
            continue;
        }
        if writes(ops, data, |k| keys_overlap(k, key)) {
            match zero_extension(ops, data, key) {
                Some(src) if plain && !extended => {
                    key = src;
                    extended = true;
                }
                _ => return None,
            }
        }
        match &guard {
            None if p.flow.kind == FlowKind::ConditionalBranch => {
                let at = ops.iter().rposition(|op| op.opcode == OpCode::CPUI_CBRANCH)?;
                let reads = condition_reads(&ops[..at], &ops[at], data)?;
                if ops[at + 1..].iter().any(|op| op.out.is_some()) || p.flows.len() != 1 {
                    return None;
                }
                guard = Some((p.addr, ops.to_vec(), at, reads));
            }
            Some((_, _, _, reads)) if writes(ops, data, |k| reads.iter().any(|&r| keys_overlap(k, r))) => {
                if !plain {
                    return None;
                }
                if let Some(preds) = comparisons(ops, data, key, reads) {
                    break (preds, key);
                }
                // `movzx eax,si; cmp si,7`: the comparison bounds a narrower
                // register the index was zero-extended from above it.
                if extended {
                    return None;
                }
                let found = narrower_reads(ops, data, key)
                    .into_iter()
                    .find_map(|narrow| comparisons(ops, data, narrow, reads).map(|preds| (narrow, preds)));
                pending = Some(found?);
            }
            _ if !plain => return None,
            _ => {}
        }
    };
    block.reverse();
    let (jcc, branch, at, reads) = guard?;
    let taken = *view.insn(jcc)?.flows.first()?;
    if (block[0]..=dispatch).contains(&taken) {
        return None;
    }
    let n = reaching_prefix(&branch, at, &reads, &preds, compared, data)?;
    if n < MIN_ENTRIES {
        return None;
    }
    let count = n.min(tables.max_entries);
    if table.checked_add(4 * count as u64)? > MASK + 1 {
        return None;
    }
    let section = ctx.exec_ranges.iter().copied().find(|&(lo, hi)| dispatch >= lo && dispatch < hi)?;
    let table_end = table + 4 * count as u64;
    let mut cases = Vec::with_capacity(count);
    for i in 0..count as u64 {
        let word = tables.pool.word_at(table + 4 * i)?;
        if word < section.0 || word >= section.1 || (table..table_end).contains(&word) {
            return None;
        }
        if view.is_function(word) || view.covered_inside(word) {
            return None;
        }
        if !cases.contains(&word) {
            cases.push(word);
        }
    }
    Some(Proof { dispatch, block, cases })
}

/// What a dispatch value is while its own p-code is folded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lin {
    Const(u64),
    Reg(Key),
    Affine { base: u64, idx: Key, scale: u64 },
    Entry { base: u64, idx: Key, scale: u64 },
}

/// The `(table, idx)` of a jump whose own p-code is `BRANCHIND LOAD(table +
/// idx * 4)` over 4-byte values, and nothing else that writes a register.
fn table_load(ops: &[FullOp], data: Option<&Rc<AddrSpace>>) -> Option<(u64, Key)> {
    let ram = data.map(|d| d.get_index())?;
    let mut env: Vec<(Key, Lin)> = Vec::new();
    let value = |vn: &VarnodeData, env: &[(Key, Lin)]| -> Option<Lin> {
        if is_constant(vn) {
            return Some(Lin::Const(vn.offset & MASK));
        }
        let key = key_of(vn, data)?;
        if let Some((_, v)) = env.iter().rev().find(|(k, _)| *k == key) {
            return Some(*v);
        }
        match key {
            Key::Reg(_, WIDTH) if !env.iter().any(|(k, _)| keys_overlap(*k, key)) => Some(Lin::Reg(key)),
            _ => None,
        }
    };
    for op in ops {
        let get = |i: usize| op.ins.get(i).and_then(|vn| value(vn, &env));
        if op.opcode == OpCode::CPUI_BRANCHIND {
            return match get(0)? {
                Lin::Entry { base, idx, scale: 4 } => Some((base, idx)),
                _ => None,
            };
        }
        let out = op.out.as_ref()?;
        if out.size != WIDTH {
            return None;
        }
        let key = key_of(out, data).filter(|k| matches!(k, Key::Tmp(..)))?;
        let v = match op.opcode {
            OpCode::CPUI_COPY => get(0)?,
            OpCode::CPUI_INT_MULT => match (get(0)?, get(1)?) {
                (Lin::Reg(idx), Lin::Const(c)) | (Lin::Const(c), Lin::Reg(idx)) => Lin::Affine { base: 0, idx, scale: c },
                _ => return None,
            },
            OpCode::CPUI_INT_LEFT => match (get(0)?, get(1)?) {
                (Lin::Reg(idx), Lin::Const(s)) if s < 32 => Lin::Affine { base: 0, idx, scale: 1 << s },
                _ => return None,
            },
            OpCode::CPUI_INT_ADD => match (get(0)?, get(1)?) {
                (Lin::Affine { base, idx, scale }, Lin::Const(c)) | (Lin::Const(c), Lin::Affine { base, idx, scale }) => {
                    Lin::Affine { base: base.wrapping_add(c) & MASK, idx, scale }
                }
                (Lin::Const(a), Lin::Const(b)) => Lin::Const(a.wrapping_add(b) & MASK),
                _ => return None,
            },
            OpCode::CPUI_LOAD => {
                if !is_constant(op.ins.first()?) || op.ins[0].offset != u64::try_from(ram).ok()? {
                    return None;
                }
                match get(1)? {
                    Lin::Affine { base, idx, scale } => Lin::Entry { base, idx, scale },
                    _ => return None,
                }
            }
            _ => return None,
        };
        env.retain(|(k, _)| !keys_overlap(*k, key));
        env.push((key, v));
    }
    None
}

fn width_mask(width: u32) -> u64 {
    if width >= 8 { u64::MAX } else { (1u64 << (8 * width)) - 1 }
}

fn writes(ops: &[FullOp], data: Option<&Rc<AddrSpace>>, hit: impl Fn(Key) -> bool) -> bool {
    ops.iter().any(|op| op.out.as_ref().and_then(|vn| key_of(vn, data)).is_some_and(&hit))
}

/// The narrower register `ops` zero-extend into exactly `key` (`movzx eax,al`),
/// when that is the only write to any byte of `key`.
fn zero_extension(ops: &[FullOp], data: Option<&Rc<AddrSpace>>, key: Key) -> Option<Key> {
    let mut hits = ops.iter().filter(|op| {
        op.out.as_ref().and_then(|vn| key_of(vn, data)).is_some_and(|k| keys_overlap(k, key))
    });
    let op = hits.next()?;
    if hits.next().is_some() || op.opcode != OpCode::CPUI_INT_ZEXT || key_of(op.out.as_ref()?, data)? != key {
        return None;
    }
    let src = key_of(op.ins.first()?, data)?;
    matches!(src, Key::Reg(_, w) if w < WIDTH).then_some(src)
}

/// The registers narrower than `key` that `ops` read, in order.
fn narrower_reads(ops: &[FullOp], data: Option<&Rc<AddrSpace>>, key: Key) -> Vec<Key> {
    let Key::Reg(_, width) = key else { return Vec::new() };
    let mut out = Vec::new();
    for vn in ops.iter().flat_map(|op| op.ins.iter()) {
        if let Some(k @ Key::Reg(_, w)) = key_of(vn, data) {
            if w < width && !out.contains(&k) {
                out.push(k);
            }
        }
    }
    out
}

/// Does an instruction in the block do anything but compute register values
/// and store? Calls and user ops are not looked through.
fn has_effects(ops: &[FullOp]) -> bool {
    ops.iter().any(|op| {
        matches!(
            op.opcode,
            OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_RETURN
        )
    })
}

/// The registers the branch condition is computed from, when the condition is
/// computed only by ops [`eval`] models.
fn condition_reads(ops: &[FullOp], branch: &FullOp, data: Option<&Rc<AddrSpace>>) -> Option<Vec<Key>> {
    let mut written: Vec<Key> = Vec::new();
    let mut reads: Vec<Key> = Vec::new();
    for op in ops {
        if !modeled(op.opcode) {
            return None;
        }
        for vn in &op.ins {
            if is_constant(vn) {
                continue;
            }
            let k = key_of(vn, data)?;
            if !written.contains(&k) && !reads.contains(&k) {
                if !matches!(k, Key::Reg(..)) {
                    return None;
                }
                reads.push(k);
            }
        }
        written.push(key_of(op.out.as_ref()?, data)?);
    }
    let cond = branch.ins.get(1)?;
    let k = key_of(cond, data)?;
    if !written.contains(&k) && !reads.contains(&k) {
        if !matches!(k, Key::Reg(..)) {
            return None;
        }
        reads.push(k);
    }
    (!reads.is_empty()).then_some(reads)
}

fn modeled(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
    )
}

/// A flag as a function of the index: `idx OP c`, unsigned, at 32 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pred {
    Less(u64),
    LessEq(u64),
    Greater(u64),
    GreaterEq(u64),
    Equal(u64),
    NotEqual(u64),
}

impl Pred {
    fn at(self, x: u64) -> bool {
        match self {
            Pred::Less(c) => x < c,
            Pred::LessEq(c) => x <= c,
            Pred::Greater(c) => x > c,
            Pred::GreaterEq(c) => x >= c,
            Pred::Equal(c) => x == c,
            Pred::NotEqual(c) => x != c,
        }
    }

    fn constant(self) -> u64 {
        match self {
            Pred::Less(c) | Pred::LessEq(c) | Pred::Greater(c) | Pred::GreaterEq(c) | Pred::Equal(c) | Pred::NotEqual(c) => c,
        }
    }
}

/// The comparison instruction's definition of each of `reads`, as a predicate
/// on the untouched `idx` register; `None` unless every one is such.
fn comparisons(ops: &[FullOp], data: Option<&Rc<AddrSpace>>, idx: Key, reads: &[Key]) -> Option<Vec<(Key, Pred)>> {
    let Key::Reg(_, width) = idx else { return None };
    let mask = width_mask(width);
    #[derive(Clone, Copy)]
    enum Sym {
        Idx,
        Const(u64),
        Diff(u64),
        Flag(Pred),
    }
    let mut env: Vec<(Key, Sym)> = Vec::new();
    for op in ops {
        let Some(out) = op.out.as_ref() else { continue };
        let Some(key) = key_of(out, data) else { return None };
        let get = |i: usize| -> Option<Sym> {
            let vn = op.ins.get(i)?;
            if is_constant(vn) {
                return (vn.size == width).then_some(Sym::Const(vn.offset & mask));
            }
            let k = key_of(vn, data)?;
            if let Some((_, s)) = env.iter().rev().find(|(e, _)| *e == k) {
                return Some(*s);
            }
            (k == idx).then_some(Sym::Idx)
        };
        let sym = match (op.opcode, get(0), get(1)) {
            (OpCode::CPUI_COPY, Some(s), _) if out.size == width || matches!(s, Sym::Flag(_)) => Some(s),
            (OpCode::CPUI_INT_SUB, Some(Sym::Idx), Some(Sym::Const(c))) if out.size == width => Some(Sym::Diff(c)),
            (OpCode::CPUI_INT_LESS, Some(Sym::Idx), Some(Sym::Const(c))) => Some(Sym::Flag(Pred::Less(c))),
            (OpCode::CPUI_INT_LESS, Some(Sym::Const(c)), Some(Sym::Idx)) => Some(Sym::Flag(Pred::Greater(c))),
            (OpCode::CPUI_INT_LESSEQUAL, Some(Sym::Idx), Some(Sym::Const(c))) => Some(Sym::Flag(Pred::LessEq(c))),
            (OpCode::CPUI_INT_LESSEQUAL, Some(Sym::Const(c)), Some(Sym::Idx)) => Some(Sym::Flag(Pred::GreaterEq(c))),
            (OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL, a, b) => {
                let eq = |c: u64| if op.opcode == OpCode::CPUI_INT_EQUAL { Pred::Equal(c) } else { Pred::NotEqual(c) };
                match (a, b) {
                    (Some(Sym::Idx), Some(Sym::Const(c))) | (Some(Sym::Const(c)), Some(Sym::Idx)) => Some(Sym::Flag(eq(c))),
                    (Some(Sym::Diff(c)), Some(Sym::Const(k))) | (Some(Sym::Const(k)), Some(Sym::Diff(c))) => {
                        Some(Sym::Flag(eq(c.wrapping_add(k) & mask)))
                    }
                    _ => None,
                }
            }
            _ => None,
        };
        if keys_overlap(key, idx) {
            return None;
        }
        env.retain(|(k, _)| !keys_overlap(*k, key));
        if let Some(s) = sym {
            env.push((key, s));
        }
    }
    reads
        .iter()
        .map(|&r| match env.iter().rev().find(|(k, _)| *k == r) {
            Some(&(_, Sym::Flag(p))) => Some((r, p)),
            _ => None,
        })
        .collect()
}

/// `n` when the index values that fall through the guard to the dispatch are
/// exactly `[0, n)`. Every predicate changes value only at `c` or `c + 1`, so
/// the condition is evaluated at each such point, its neighbours, and one value
/// inside each gap between them.
fn reaching_prefix(
    branch: &[FullOp],
    at: usize,
    reads: &[Key],
    preds: &[(Key, Pred)],
    index: Key,
    data: Option<&Rc<AddrSpace>>,
) -> Option<usize> {
    let Key::Reg(_, width) = index else { return None };
    let mask = width_mask(width);
    let mut points: BTreeSet<u64> = [0, mask].into_iter().collect();
    for &(_, p) in preds {
        let c = p.constant();
        for x in [c.wrapping_sub(1), c, c.wrapping_add(1)] {
            points.insert(x & mask);
        }
    }
    let marks: Vec<u64> = points.iter().copied().collect();
    let mut samples: BTreeSet<u64> = points;
    for pair in marks.windows(2) {
        if pair[0] + 1 < pair[1] {
            samples.insert(pair[0] + 1);
        }
    }
    let reaches = |x: u64| -> Option<bool> {
        let mut env: Vec<(Key, u64)> = reads
            .iter()
            .map(|&r| preds.iter().find(|(k, _)| *k == r).map(|&(_, p)| (r, u64::from(p.at(x)))))
            .collect::<Option<_>>()?;
        Some(!eval(&branch[..at], &branch[at], &mut env, data)?)
    };
    let mut n: Option<u64> = None;
    for x in samples {
        match (n, reaches(x)?) {
            (None, false) => n = Some(x),
            (Some(_), true) => return None,
            _ => {}
        }
    }
    usize::try_from(n?).ok()
}

/// The branch condition, given the values of the registers it reads.
fn eval(ops: &[FullOp], branch: &FullOp, env: &mut Vec<(Key, u64)>, data: Option<&Rc<AddrSpace>>) -> Option<bool> {
    let known = |vn: &VarnodeData, env: &[(Key, u64)]| -> Option<u64> {
        if is_constant(vn) {
            return Some(vn.offset);
        }
        let k = key_of(vn, data)?;
        env.iter().rev().find(|(e, _)| *e == k).map(|&(_, v)| v)
    };
    for op in ops {
        let get = |i: usize| known(op.ins.get(i)?, env);
        let v = match op.opcode {
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => get(0)?,
            OpCode::CPUI_BOOL_NEGATE => u64::from(get(0)? == 0),
            OpCode::CPUI_BOOL_AND => u64::from(get(0)? != 0 && get(1)? != 0),
            OpCode::CPUI_BOOL_OR => u64::from(get(0)? != 0 || get(1)? != 0),
            OpCode::CPUI_BOOL_XOR => u64::from((get(0)? != 0) != (get(1)? != 0)),
            OpCode::CPUI_INT_AND => get(0)? & get(1)?,
            OpCode::CPUI_INT_OR => get(0)? | get(1)?,
            OpCode::CPUI_INT_XOR => get(0)? ^ get(1)?,
            OpCode::CPUI_INT_EQUAL => u64::from(get(0)? == get(1)?),
            OpCode::CPUI_INT_NOTEQUAL => u64::from(get(0)? != get(1)?),
            _ => return None,
        };
        let out = op.out.as_ref()?;
        let key = key_of(out, data)?;
        let mask = if out.size >= 8 { u64::MAX } else { (1u64 << (8 * out.size)) - 1 };
        env.retain(|(k, _)| !keys_overlap(*k, key));
        env.push((key, v & mask));
    }
    known(branch.ins.get(1)?, env).map(|v| v != 0)
}

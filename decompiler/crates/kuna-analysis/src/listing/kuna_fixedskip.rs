//! (kuna) A skip to the fall-through that the instruction itself decides.
//!
//! SPARC lifts `call target` with its delay slot inlined as
//! `o7 = inst_start; didrestore = 0; <delay slot>; call target;
//! if (didrestore == 0) goto inst_next; return [o7]`. A `restore` (or any write
//! of `%o7`) in the delay slot sets `didrestore = 1`, which makes the call a tail
//! call; any other delay slot leaves it 0, and control comes back to the next
//! instruction. [`super::classify`] reads ops one at a time, so it saw a call
//! whose last op is a `RETURN` (no fall-through) with a branch target at the next
//! instruction, and the walk claimed that instruction as a new function.
//!
//! [`SkipFold`] follows the constants an instruction writes to its registers and
//! temporaries, and [`SkipFold::apply`] settles each `CBRANCH` to the
//! fall-through whose condition is one of them before the ops are classified: a
//! skip that is always taken ends the instruction there, so it falls through, and
//! one that is never taken is dropped. Ops after a p-code-relative branch are not
//! straight-line, so an instruction that has one is left as it is.

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::model::RawOp;

/// A register or temporary, by space index, offset and size.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Key {
    space: i32,
    offset: u64,
    size: u32,
}

/// The constants one instruction's ops have written so far, and the verdicts of
/// its conditional branches whose conditions are among them.
#[derive(Default)]
pub(crate) struct SkipFold {
    known: Vec<(Key, u64)>,
    decided: Vec<(usize, bool)>,
    seen: usize,
    relative: bool,
}

impl SkipFold {
    /// Start a new instruction.
    pub(crate) fn begin(&mut self) {
        self.known.clear();
        self.decided.clear();
        self.seen = 0;
        self.relative = false;
    }

    /// Follow the next emitted op.
    pub(crate) fn observe(&mut self, opcode: OpCode, out: Option<&VarnodeData>, ins: &[VarnodeData]) {
        let at = self.seen;
        self.seen += 1;
        if self.relative {
            return;
        }
        if matches!(opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) {
            if ins.first().is_some_and(is_constant) {
                self.relative = true;
            } else if opcode == OpCode::CPUI_CBRANCH {
                if let Some(cond) = ins.get(1).and_then(|vn| self.value(vn)) {
                    self.decided.push((at, cond != 0));
                }
            }
            return;
        }
        let Some(out) = out else { return };
        let value = match opcode {
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => ins.first().and_then(|vn| self.value(vn)),
            OpCode::CPUI_INT_EQUAL => self.pair(ins).map(|(a, b)| u64::from(a == b)),
            OpCode::CPUI_INT_NOTEQUAL => self.pair(ins).map(|(a, b)| u64::from(a != b)),
            OpCode::CPUI_BOOL_NEGATE => ins.first().and_then(|vn| self.value(vn)).map(|v| u64::from(v & 1 == 0)),
            OpCode::CPUI_BOOL_AND => self.pair(ins).map(|(a, b)| a & b & 1),
            OpCode::CPUI_BOOL_OR => self.pair(ins).map(|(a, b)| (a | b) & 1),
            OpCode::CPUI_BOOL_XOR => self.pair(ins).map(|(a, b)| (a ^ b) & 1),
            _ => None,
        };
        let Some(key) = key_of(out) else { return };
        self.known.retain(|(k, _)| !overlaps(*k, key));
        if let Some(v) = value.filter(|_| key.size <= 8) {
            self.known.push((key, v & mask(key.size)));
        }
    }

    /// Settle the skips of the instruction whose ops are `ops` (exactly the ops
    /// observed since [`SkipFold::begin`]) and whose fall-through is `fall`.
    pub(crate) fn apply(&self, ops: &mut Vec<RawOp>, fall: u64) {
        if self.relative || self.decided.is_empty() || ops.len() != self.seen {
            return;
        }
        let mut end = ops.len();
        let mut never = Vec::new();
        for &(at, taken) in &self.decided {
            let skip = ops[at].in0.as_ref().is_some_and(|t| !is_constant(t) && t.offset == fall);
            if !skip {
                continue;
            }
            if taken {
                end = at;
                break;
            }
            never.push(at);
        }
        ops.truncate(end);
        for at in never.into_iter().rev() {
            ops.remove(at);
        }
    }

    fn value(&self, vn: &VarnodeData) -> Option<u64> {
        if vn.size > 8 {
            return None;
        }
        if is_constant(vn) {
            return Some(vn.offset & mask(vn.size));
        }
        let key = key_of(vn)?;
        self.known.iter().rev().find(|(k, _)| *k == key).map(|&(_, v)| v)
    }

    fn pair(&self, ins: &[VarnodeData]) -> Option<(u64, u64)> {
        Some((self.value(ins.first()?)?, self.value(ins.get(1)?)?))
    }
}

fn is_constant(vn: &VarnodeData) -> bool {
    vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
}

/// A register or temporary; memory is not followed.
fn key_of(vn: &VarnodeData) -> Option<Key> {
    let space = vn.space.as_ref()?;
    let tracked = space.get_type() == spacetype::IPTR_INTERNAL || space.get_name() == "register";
    tracked.then(|| Key { space: space.get_index(), offset: vn.offset, size: vn.size })
}

fn overlaps(a: Key, b: Key) -> bool {
    a.space == b.space
        && a.offset < b.offset.saturating_add(u64::from(b.size))
        && b.offset < a.offset.saturating_add(u64::from(a.size))
}

fn mask(size: u32) -> u64 {
    if size >= 8 { u64::MAX } else { (1u64 << (size * 8)) - 1 }
}

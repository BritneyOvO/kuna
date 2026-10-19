//! Opt-in recovery of Go's implicit slice/array bounds guards (goboundscheck).
//! Removes a conditional edge only when its direct successor diverges and has
//! exactly one call to a compiler runtime bounds-panic entry point, with no
//! memory stores. The access remains in p-code. This is structural source-level
//! recovery: the emitted C does not retain Go panic semantics on invalid indices.

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockKind;
use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;
use kuna_base::types::int4;

pub fn is_go_bounds_name(name: &str) -> bool {
    let name = name.strip_suffix(".abi0").or_else(|| name.strip_suffix("_abi0")).unwrap_or(name);
    let normalized;
    let name = if let Some(leaf) = name.strip_prefix("runtime_") {
        normalized = format!("runtime.{leaf}");
        normalized.as_str()
    } else { name };
    matches!(name, "runtime.panicBounds" | "runtime.panicIndex" | "runtime.panicIndexU"
        | "runtime.panicSliceAlen" | "runtime.panicSliceAlenU"
        | "runtime.panicSliceAcap" | "runtime.panicSliceAcapU"
        | "runtime.panicSliceB" | "runtime.panicSliceBU"
        | "runtime.panicSlice3Alen" | "runtime.panicSlice3AlenU"
        | "runtime.panicSlice3Acap" | "runtime.panicSlice3AcapU"
        | "runtime.panicSlice3B" | "runtime.panicSlice3BU"
        | "runtime.panicSlice3C" | "runtime.panicSlice3CU")
}

fn callee_name(op: OpId, data: &Funcdata) -> Option<&str> {
    let idx = data.get_call_specs_index(op)?;
    let nm = data.get_call_specs(idx).get_name();
    if nm.is_empty() {
        None
    } else {
        Some(nm)
    }
}

fn block_is_go_bounds_handler(bb: BlockId, data: &Funcdata) -> bool {
    let mut matched = false;
    for op in data.bb_ops(bb) {
        let oc = data.obank().get(op).expect("goboundscheck: op").code();
        if oc == OpCode::CPUI_STORE {
            return false;
        }
        if !matches!(
            oc,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
        ) {
            continue;
        }
        if matched {
            return false; // a second call: the arm does more than panic
        }
        match callee_name(op, data) {
            Some(name) if is_go_bounds_name(name) => matched = true,
            _ => return false,
        }
    }
    matched
}

pub struct ActionRemoveGoBoundsCheck {
    base: ActionBase,
    enabled: bool,
}

impl ActionRemoveGoBoundsCheck {
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionRemoveGoBoundsCheck {
        ActionRemoveGoBoundsCheck { base: ActionBase::new(0, "removegoboundscheck", g), enabled }
    }
}

impl Action for ActionRemoveGoBoundsCheck {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionRemoveGoBoundsCheck {
            base: self.base.clone(),
            enabled: self.enabled,
        }))
    }

    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !self.enabled && !data.get_arch().strip_go_bounds_check {
            return 0; // P0 assertion not set
        }
        let size = data.bblocks_get_size();
        for i in 0..size {
            let h = data.bblocks_get_block(i);
            if !matches!(data.bblocks_ref().block(h).kind(), BlockKind::Basic(_)) {
                continue;
            }
            if data.bblocks_ref().block(h).size_out() != 2 {
                continue;
            }
            let cb = match data.bb_op_tail(h) {
                Some(op) => op,
                None => continue,
            };
            if data.obank().get(cb).expect("goboundscheck: cb").code() != OpCode::CPUI_CBRANCH {
                continue;
            }
            let cond = data.obank().get(cb).and_then(|op| op.get_in(1));
            let Some(cond) = cond else { continue };
            let Some(def) = data.vbank().get(cond).and_then(|vn| vn.get_def()) else { continue };
            let Some(compare) = data.obank().get(def) else { continue };
            if !matches!(compare.code(), OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_BOOL_NEGATE | OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL) {
                continue;
            }
            let mut victim: Option<int4> = None;
            for j in 0..2 {
                let out = data.bblocks_ref().block(h).get_out(j);
                if !matches!(data.bblocks_ref().block(out).kind(), BlockKind::Basic(_)) {
                    continue;
                }
                if data.bblocks_ref().block(out).size_out() != 0 {
                    continue;
                }
                if block_is_go_bounds_handler(out, data) {
                    victim = Some(j);
                    break;
                }
            }
            let idx = match victim {
                Some(j) => j,
                None => continue,
            };
            data.remove_branch(h, idx).expect("ActionRemoveGoBoundsCheck: removeBranch");
            data.remove_unreachable_blocks(false, true)
                .expect("ActionRemoveGoBoundsCheck: removeUnreachableBlocks");
            self.base.count += 1;
            return 1;
        }
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_runtime_names_only() {
        for name in ["runtime.panicBounds", "runtime.panicIndex", "runtime.panicSlice3B.abi0", "runtime_panicBounds"] {
            assert!(is_go_bounds_name(name));
        }
        for name in ["panicIndex", "example.runtime.panicIndex", "runtime.gopanic",
            "runtime.panicIndexExtra", "runtime.panicBounds.abi1", ""] {
            assert!(!is_go_bounds_name(name));
        }
    }
}

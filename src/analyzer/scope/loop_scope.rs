use std::{collections::BTreeMap, rc::Rc};

use hakana_code_info::{t_union::TUnion, var_name::VarName};
use rustc_hash::{FxHashMap, FxHashSet};

use super::control_action::ControlAction;

#[derive(Clone, Debug)]
pub struct LoopScope {
    pub iteration_count: usize,

    pub parent_context_vars: BTreeMap<VarName, Rc<TUnion>>,

    pub redefined_loop_vars: FxHashMap<VarName, TUnion>,

    pub possibly_redefined_loop_vars: FxHashMap<VarName, TUnion>,

    pub possibly_redefined_loop_parent_vars: FxHashMap<VarName, Rc<TUnion>>,

    pub possibly_defined_loop_parent_vars: FxHashMap<VarName, TUnion>,

    pub final_actions: FxHashSet<ControlAction>,
}

impl LoopScope {
    pub fn new(parent_context_vars: BTreeMap<VarName, Rc<TUnion>>) -> Self {
        Self {
            parent_context_vars,
            iteration_count: 0,
            redefined_loop_vars: FxHashMap::default(),
            possibly_redefined_loop_vars: FxHashMap::default(),
            possibly_redefined_loop_parent_vars: FxHashMap::default(),
            possibly_defined_loop_parent_vars: FxHashMap::default(),
            final_actions: FxHashSet::default(),
        }
    }
}

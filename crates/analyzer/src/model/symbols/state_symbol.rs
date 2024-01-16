use std::collections::HashSet;
use witcherscript::attribs::StateSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StateSymbol {
    path: StateSymbolPath,
    pub specifiers: HashSet<StateSpecifier>,
    pub base_state_name: Option<String>
}

impl Symbol for StateSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::State
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl StateSymbol {
    pub fn new(path: StateSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new(),
            base_state_name: None
        }
    }


    pub fn state_name(&self) -> &str {
        &self.path.state_name
    }

    pub fn parent_class_path(&self) -> &SymbolPath {
        &self.path.parent_class_path
    }
}
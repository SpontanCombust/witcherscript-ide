use std::collections::HashSet;
use abs_path::AbsPath;
use witcherscript::attribs::StateSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StateSymbol {
    path: StateSymbolPath,
    decl_file_path: AbsPath,
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
    pub fn new(path: StateSymbolPath, decl_file_path: AbsPath) -> Self {
        Self {
            path,
            decl_file_path,
            specifiers: HashSet::new(),
            base_state_name: None
        }
    }


    pub fn decl_file_path(&self) -> &AbsPath {
        &self.decl_file_path
    }

    pub fn state_name(&self) -> &str {
        &self.path.state_name
    }

    pub fn parent_class_path(&self) -> &SymbolPath {
        &self.path.parent_class_path
    }
}
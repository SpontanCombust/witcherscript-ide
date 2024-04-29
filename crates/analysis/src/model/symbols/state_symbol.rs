use std::{collections::HashSet, path::{Path, PathBuf}};
use lsp_types as lsp;
use witcherscript::attribs::StateSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StateSymbol {
    path: StateSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    label_range: lsp::Range,
    pub specifiers: HashSet<StateSpecifier>,
    pub base_state_name: Option<String>,
    /*//TODO base_state_path can be known only after the class tree can be inspected 
    the base state can belong to a super class of the statemachine class
    and its name cannot be deduced from the context of state declaration itself*/
    pub base_state_path: Option<StateSymbolPath> 
}

impl Symbol for StateSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::State
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimarySymbol for StateSymbol {
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl LocatableSymbol for StateSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl StateSymbol {
    pub fn new(path: StateSymbolPath, local_source_path: PathBuf, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            local_source_path,
            range,
            label_range,
            specifiers: HashSet::new(),
            base_state_name: None,
            base_state_path: None
        }
    }


    pub fn state_name(&self) -> &str {
        &self.path.state_name
    }

    pub fn parent_class_path(&self) -> &SymbolPath {
        &self.path.parent_class_path
    }

    pub fn parent_class_name(&self) -> &str {
        self.path.parent_class_path.components().last().unwrap().name
    }
}
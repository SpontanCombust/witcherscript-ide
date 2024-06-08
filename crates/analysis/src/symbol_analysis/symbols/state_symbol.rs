use witcherscript::attribs::StateSpecifier;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StateSymbol {
    path: StateSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<StateSpecifier>,
    pub base_state_name: Option<String>,
}

impl Symbol for StateSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::State
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for StateSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for StateSymbol {
    
}

impl StateSymbol {
    // CScriptableState is not actually a state, but a class!
    // I know, confusing as hell, just like this entire language...
    pub const DEFAULT_STATE_BASE_NAME: &'static str = "CScriptableState";

    pub fn new(path: StateSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new(),
            base_state_name: None
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
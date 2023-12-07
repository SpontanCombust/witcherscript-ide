use std::collections::HashSet;
use uuid::Uuid;
use witcherscript::attribs::FunctionParameterSpecifier;
use super::{SymbolType, ERROR_SYMBOL_ID, SymbolData, Symbol};


#[derive(Debug, Clone)]
pub struct FunctionParameterSymbolData {
    pub specifiers: HashSet<FunctionParameterSpecifier>,
    pub type_id: Uuid
}

impl Default for FunctionParameterSymbolData {
    fn default() -> Self {
        Self { 
            specifiers: HashSet::new(),
            type_id: ERROR_SYMBOL_ID,
        }
    }
}

impl SymbolData for FunctionParameterSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Parameter;
}

pub type FunctionParameterSymbol = Symbol<FunctionParameterSymbolData>;
use std::collections::HashSet;
use uuid::Uuid;
use witcherscript::attribs::StructSpecifier;
use super::{Symbol, SymbolType, MemberVarSymbol, SymbolData};


#[derive(Debug, Clone, Default)]
pub struct StructSymbolData {
    pub specifiers: HashSet<StructSpecifier>,
    pub member_var_ids: Vec<Uuid>,
}

impl SymbolData for StructSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Struct;
}

pub type StructSymbol = Symbol<StructSymbolData>;

impl StructSymbol {
    #[must_use]
    pub fn add_member_var(&mut self, name: &str) -> MemberVarSymbol {
        let s = MemberVarSymbol::new_with_default(name, self.id);
        self.data.member_var_ids.push(s.id);
        s
    }
}
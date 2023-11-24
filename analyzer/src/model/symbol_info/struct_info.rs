use uuid::Uuid;
use witcherscript::attribs::StructSpecifier;
use super::{SymbolInfo, SymbolType, MemberVarInfo, GlobalSymbolInfo};

pub struct StructInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<StructSpecifier>,
    pub fields: Vec<MemberVarInfo>,
}

impl StructInfo {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            fields: Vec::new()
        }
    }
}

impl SymbolInfo for StructInfo {
    const TYPE: SymbolType = SymbolType::Struct;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbolInfo for StructInfo {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}
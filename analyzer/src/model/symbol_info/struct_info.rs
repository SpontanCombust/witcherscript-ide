use uuid::Uuid;
use witcherscript::attribs::StructSpecifier;
use super::{Symbol, SymbolType, MemberVarSymbol, GlobalSymbol};


#[derive(Debug, Clone)]
pub struct StructSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<StructSpecifier>,
    pub fields: Vec<MemberVarSymbol>,
}

impl StructSymbol {
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

impl Symbol for StructSymbol {
    const TYPE: SymbolType = SymbolType::Struct;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbol for StructSymbol {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}
use uuid::Uuid;
use witcherscript::attribs::StructSpecifier;
use super::{Symbol, SymbolType, MemberVarSymbol};


#[derive(Debug, Clone)]
pub struct StructSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<StructSpecifier>,
    pub member_vars: Vec<MemberVarSymbol>,
}

impl StructSymbol {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            member_vars: Vec::new()
        }
    }


    pub fn add_member_var(&mut self, name: &str) -> &mut MemberVarSymbol {
        self.member_vars.push(MemberVarSymbol::new(self.symbol_id, name));
        self.member_vars.last_mut().unwrap()
    }
}

impl Symbol for StructSymbol {
    const TYPE: SymbolType = SymbolType::Struct;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        self.script_id
    }
}

use uuid::Uuid;
use super::{Symbol, SymbolType};


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub members: Vec<EnumMemberSymbol>,
}

impl EnumSymbol {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            members: Vec::new(),
        }
    }
}

impl Symbol for EnumSymbol {
    const TYPE: SymbolType = SymbolType::Enum;

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


#[derive(Debug, Clone)]
pub struct EnumMemberSymbol {
    enum_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub value: i32
}

impl EnumMemberSymbol {
    pub fn new(enum_info: &EnumSymbol, name: &str) -> Self {
        let value = enum_info.members
            .last()
            .map(|m| m.value)
            .unwrap_or(-1) + 1;

        Self {
            enum_id: enum_info.symbol_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            value
        }
    }
}

impl Symbol for EnumMemberSymbol {
    const TYPE: SymbolType = SymbolType::EnumMember;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        self.enum_id
    }
}
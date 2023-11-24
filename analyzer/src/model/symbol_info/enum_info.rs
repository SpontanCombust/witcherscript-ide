use uuid::Uuid;

use super::{SymbolInfo, SymbolType, GlobalSymbolInfo, ChildSymbolInfo};

pub struct EnumInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub members: Vec<EnumMemberInfo>,
}

impl EnumInfo {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            members: Vec::new(),
        }
    }
}

impl SymbolInfo for EnumInfo {
    const TYPE: SymbolType = SymbolType::Enum;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbolInfo for EnumInfo {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}


pub struct EnumMemberInfo {
    enum_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub value: i32
}

impl EnumMemberInfo {
    pub fn new(enum_info: &EnumInfo, name: &str, value: Option<i32>) -> Self {
        let value = if let Some(v) = value { 
            v 
        } else { 
            enum_info.members
                .last()
                .map(|m| m.value)
                .unwrap_or(-1) + 1 
        };

        Self {
            enum_id: enum_info.symbol_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            value
        }
    }
}

impl SymbolInfo for EnumMemberInfo {
    const TYPE: SymbolType = SymbolType::EnumMember;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbolInfo for EnumMemberInfo {
    fn parent_symbol_id(&self) -> Uuid {
        self.enum_id
    }
}
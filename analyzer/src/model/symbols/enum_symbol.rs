use uuid::Uuid;
use super::{Symbol, SymbolType};


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub member_ids: Vec<Uuid>,
}

impl EnumSymbol {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            member_ids: Vec::new(),
        }
    }


    #[must_use]
    pub fn add_member(&mut self, name: &str) -> EnumMemberSymbol {
        let m = EnumMemberSymbol::new(self.symbol_id, name);
        // if let Some(value) = value {
        //     m.value = value;
        // } else {
        //     m.value = self.member_ids.last().map(|l| l.value).unwrap_or(-1) + 1;
        // }

        self.member_ids.push(m.symbol_id);
        m
    }
}

//TODO EnumSymbolBuilder

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
    pub fn new(enum_id: Uuid, name: &str) -> Self {
        Self {
            enum_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            value: 0
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
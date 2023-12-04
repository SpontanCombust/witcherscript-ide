use uuid::Uuid;
use super::{Symbol, SymbolType, SymbolData};


#[derive(Debug, Clone, Default)]
pub struct EnumSymbolData {
    pub member_ids: Vec<Uuid>,
}

impl SymbolData for EnumSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Enum;
}

pub type EnumSymbol = Symbol<EnumSymbolData>;

impl EnumSymbol {
    #[must_use]
    pub fn add_member(&mut self, name: &str /*, value: i32*/) -> EnumMemberSymbol {
        let m = EnumMemberSymbol::new(name, self.id, EnumMemberSymbolData::new());
        self.data.member_ids.push(m.id);
        m
    }
}



#[derive(Debug, Clone, Default)]
pub struct EnumMemberSymbolData {
    // pub value: i32
}

impl EnumMemberSymbolData {
    pub fn new(/*value: i32*/) -> Self {
        Self {
            // value
        }
    }
}

impl SymbolData for EnumMemberSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::EnumMember;
}

pub type EnumMemberSymbol = Symbol<EnumMemberSymbolData>;


/* 
pub struct EnumSymbolBuilder {
    sym: EnumSymbol,
    members: Vec<EnumMemberSymbol>,
    prev_val: i32
}

impl EnumSymbolBuilder {
    pub fn new(script_id: Uuid, enum_name: &str) -> Self {
        Self { 
            sym: EnumSymbol::new_with_default(enum_name, script_id), 
            members: Vec::new(), 
            prev_val: -1
        }
    }

    pub fn member(&mut self, name: &str, value: Option<i32>) -> &mut Self {
        let member_value = if let Some(value) = value {
            value
        } else {
            self.prev_val + 1
        };

        let member = self.sym.add_member(name, member_value);
        
        self.prev_val = member_value;
        self.members.push(member);
        self
    }

    pub fn finish(self) -> (EnumSymbol, Vec<EnumMemberSymbol>) {
        (self.sym, self.members)
    } 
}
*/
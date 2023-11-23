use uuid::Uuid;
use super::{SymbolInfo, SymbolType, MemberVarInfo, GlobalSymbolInfo};

pub struct StructInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    fields: Vec<MemberVarInfo>
}

impl StructInfo {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            fields: Vec::new()
        }
    }


    pub fn fields(&self) -> &[MemberVarInfo] {
        &self.fields
    }

    pub fn add_field(&mut self, field: MemberVarInfo) {
        self.fields.push(field);
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
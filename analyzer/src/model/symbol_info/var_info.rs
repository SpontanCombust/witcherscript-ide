use uuid::Uuid;
use witcherscript::attribs::MemberVarSpecifier;
use super::{SymbolInfo, SymbolType, ChildSymbolInfo};


pub struct MemberVarInfo {
    owner_id: Uuid,
    symbol_id: Uuid,
    name: String,
    specifiers: Vec<MemberVarSpecifier>,
    type_id: Uuid,
}

impl MemberVarInfo {
    pub fn new(owner_info: &impl SymbolInfo, name: &str, type_id: Uuid) -> Self {
        Self {
            owner_id: owner_info.symbol_id(),
            symbol_id: Uuid::new_v4(),
            specifiers: Vec::new(),
            name: name.to_owned(),
            type_id,
        }
    }


    pub fn specifiers(&self) -> &[MemberVarSpecifier] {
        &self.specifiers
    }

    pub fn add_specifier(&mut self, specifier: MemberVarSpecifier) {
        self.specifiers.push(specifier)
    }

    
    pub fn type_id(&self) -> Uuid {
        self.type_id
    }
}

impl SymbolInfo for MemberVarInfo {
    const TYPE: SymbolType = SymbolType::Field;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbolInfo for MemberVarInfo {
    fn parent_symbol_id(&self) -> Uuid {
        self.owner_id
    }
}



pub struct VarInfo {
    func_id: Uuid,
    symbol_id: Uuid,
    name: String,
    type_id: Uuid,
}

impl VarInfo {
    //TODO &FunctionInfo 
    pub fn new(func_info: &impl SymbolInfo, name: &str, type_id: Uuid) -> Self {
        Self {
            func_id: func_info.symbol_id(),
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            type_id
        }
    }


    pub fn type_id(&self) -> Uuid {
        self.type_id
    }
}

impl SymbolInfo for VarInfo {
    const TYPE: SymbolType = SymbolType::Variable;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbolInfo for VarInfo {
    fn parent_symbol_id(&self) -> Uuid {
        self.func_id
    }
}
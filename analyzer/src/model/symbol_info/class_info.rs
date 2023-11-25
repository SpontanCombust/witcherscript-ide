use uuid::Uuid;
use witcherscript::attribs::{ClassSpecifier, StateSpecifier};
use super::{MemberVarInfo, MemberFunctionInfo, EventInfo, SymbolInfo, SymbolType, GlobalSymbolInfo, TypeParameterInfo};


#[derive(Debug, Clone)]
pub struct ClassInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String, // doesn't include possible type parameter 
    type_param: Option<TypeParameterInfo>, // the only generic type in WS i.e. array<T> takes only one type argument
    full_name: String, // includes type parameter (if there is any)
    pub specifiers: Vec<ClassSpecifier>,
    pub base_id: Option<Uuid>,
    pub member_vars: Vec<MemberVarInfo>,
    pub member_funcs: Vec<MemberFunctionInfo>,
    pub events: Vec<EventInfo>,
}

impl ClassInfo {
    pub fn new(script_id: Uuid, name: &str, type_param: Option<TypeParameterInfo>) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            full_name: if let Some(t) = &type_param { format!("{}<{}>", name, t.name()) } else { name.to_owned() },
            type_param,
            specifiers: Vec::new(),
            base_id: None,
            member_vars: Vec::new(),
            member_funcs: Vec::new(),
            events: Vec::new(),
        }
    }

    /*
    /// Returns new symbol for this class, but with all generics replaced with actual types.
    /// Returns Err if this class doesn't take any type arguments.
    /// 
    /// Due to an extremly sparse use of generics in WS this should be enough. There is only "array", 
    /// which doesn't have any members that also take type arguments. Besides, it you cannot declare 
    /// your own generic classes. 
    pub fn with_generic_substituted(&self, substitute: Uuid) -> Result<Self, String> {
        if self.type_param.is_none() {
            return Err(format!("Class {} doesn't take any type arguments.", self.name));
        }

        let symbol_id = Uuid::new_v4();
        let name = 
    }
    */
}

impl SymbolInfo for ClassInfo {
    const TYPE: SymbolType = SymbolType::Class;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.full_name.as_str()
    }
}

impl GlobalSymbolInfo for ClassInfo {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}



#[derive(Debug, Clone)]
pub struct StateInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<StateSpecifier>,
    pub parent_id: Uuid,
    pub base_id: Option<Uuid>,
    pub member_vars: Vec<MemberVarInfo>,
    pub member_funcs: Vec<MemberFunctionInfo>,
    pub events: Vec<EventInfo>,
}

impl StateInfo {
    pub fn new(script_id: Uuid, name: &str, parent_id: Uuid) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            parent_id,
            base_id: None,
            member_vars: Vec::new(),
            member_funcs: Vec::new(),
            events: Vec::new(),
        }
    }
}

impl SymbolInfo for StateInfo {
    const TYPE: SymbolType = SymbolType::State;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbolInfo for StateInfo {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}
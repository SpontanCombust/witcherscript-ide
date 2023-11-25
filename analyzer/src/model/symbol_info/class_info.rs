use uuid::Uuid;
use witcherscript::attribs::{ClassSpecifier, StateSpecifier};
use super::{MemberVarInfo, MemberFunctionInfo, EventInfo, SymbolInfo, SymbolType, GlobalSymbolInfo, TypeParameterInfo};


#[derive(Debug, Clone)]
pub struct ClassInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String, // doesn't include possible type parameter 
    type_param: Option<TypeParameterInfo>, // the only generic type in WS i.e. array<T> takes only one type argument
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
            type_param,
            specifiers: Vec::new(),
            base_id: None,
            member_vars: Vec::new(),
            member_funcs: Vec::new(),
            events: Vec::new(),
        }
    }

    
    pub fn type_param(&self) -> &Option<TypeParameterInfo> {
        &self.type_param
    }

    /// Returns new symbol for this class, but with all generics replaced with actual types.
    /// Returns Err if this class doesn't take any type arguments.
    /// 
    /// Due to an extremly sparse use of generics in WS (there's only the "array" type and you can't declare your own generic types)
    /// the approach taken here to handling them is extremely minimalisitc. If at some point for some reason
    /// there will exist a need for universal generic support this will need to be rewritten. 
    pub fn with_generic_substituted(&self, substitute_id: Uuid, substitute_name: &str) -> Result<Self, String> {
        if self.type_param.is_none() {
            return Err(format!("Class {} doesn't take any type arguments.", self.name));
        }

        let type_param_id = self.type_param.as_ref().unwrap().symbol_id();
        let new_symbol_id = Uuid::new_v4();
        let name = format!("{}<{}>", self.name, substitute_name);
        let subst_members = self.member_funcs.iter()
                                .map(|m| m.with_type_substituted(new_symbol_id, (type_param_id, substitute_id)))
                                .collect::<Vec<_>>();

        // array doesn't have any properties exposed other than member functions

        Ok(Self {
            symbol_id: new_symbol_id,
            name,
            type_param: None,
            member_funcs: subst_members,
            ..self.clone()
        })
    }
}

impl SymbolInfo for ClassInfo {
    const TYPE: SymbolType = SymbolType::Class;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
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
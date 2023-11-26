use uuid::Uuid;
use super::{MemberFunctionSymbol, Symbol, SymbolType, TypeParameterSymbol, ClassSymbol, NATIVE_SYMBOL_SCRIPT_ID, GlobalSymbol};

// Used for the "array" type
#[derive(Debug, Clone)]
pub struct GenericTypeSymbol {
    symbol_id: Uuid,
    name: String,
    full_name: String,
    type_param: TypeParameterSymbol,
    pub funcs: Vec<MemberFunctionSymbol>
}

impl GenericTypeSymbol {
    pub fn new(name: &str, type_param: TypeParameterSymbol) -> Self {
        Self {
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            full_name: format!("{}<{}>", name, type_param.name()),
            type_param,
            funcs: Vec::new()
        }
    }


    pub fn type_param(&self) -> &TypeParameterSymbol {
        &self.type_param
    }

    /// Creates a concrete class type by replacing all occurances of generic type parameter with a given type
    /// 
    /// Due to an extremly sparse use of generics in WS (there's only the "array" type and you can't declare your own generic types)
    /// the approach taken here to handling them is extremely minimalisitc. If at some point for some reason
    /// there will exist a need for universal generic support this will need to be vastly improved. 
    pub fn new_concrete_class(&self, concrete_type_id: Uuid, concrete_type_name: &str) -> ClassSymbol {
        let name = format!("{}<{}>", self.name, concrete_type_name);
        let mut sym = ClassSymbol::new(NATIVE_SYMBOL_SCRIPT_ID, &name);
        sym.member_funcs = self.funcs.iter()
                            .map(|m| m.with_type_substituted(sym.symbol_id(), self.type_param.symbol_id(), concrete_type_id))
                            .collect::<Vec<_>>();

        // array doesn't have any properties exposed other than member functions
        sym
    }
}

impl Symbol for GenericTypeSymbol {
    const TYPE: SymbolType = SymbolType::Class;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.full_name.as_str()
    }
}

impl GlobalSymbol for GenericTypeSymbol {
    fn script_id(&self) -> Uuid {
        NATIVE_SYMBOL_SCRIPT_ID
    }
}
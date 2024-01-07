pub use super::symbols::*;


#[derive(Debug, Clone)]
pub enum SymbolVariant {
    Primitive(PrimitiveTypeSymbol),
    Enum(EnumSymbol),
    EnumMember(EnumMemberSymbol),
    Struct(StructSymbol),
    Class(ClassSymbol),
    State(StateSymbol),
    Array(ArrayTypeSymbol),
    GlobalFunc(GlobalFunctionSymbol),
    MemberFunc(MemberFunctionSymbol),
    Event(EventSymbol),
    FuncParam(FunctionParameterSymbol),
    GlobalVar(GlobalVarSymbol),
    MemberVar(MemberVarSymbol),
    Autobind(AutobindSymbol),
    LocalVar(LocalVarSymbol),
}

impl SymbolVariant {
    pub fn as_dyn(&self) -> &dyn Symbol {
        match self {
            SymbolVariant::Primitive(v) => v,
            SymbolVariant::Enum(v) => v,
            SymbolVariant::EnumMember(v) => v,
            SymbolVariant::Struct(v) => v,
            SymbolVariant::Class(v) => v,
            SymbolVariant::State(v) => v,
            SymbolVariant::Array(v) => v,
            SymbolVariant::GlobalFunc(v) => v,
            SymbolVariant::MemberFunc(v) => v,
            SymbolVariant::Event(v) => v,
            SymbolVariant::FuncParam(v) => v,
            SymbolVariant::GlobalVar(v) => v,
            SymbolVariant::MemberVar(v) => v,
            SymbolVariant::Autobind(v) => v,
            SymbolVariant::LocalVar(v) => v,
        }
    }


    pub fn into_primitive(self) -> Option<PrimitiveTypeSymbol> {
        if let SymbolVariant::Primitive(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_primitive(&self) -> Option<&PrimitiveTypeSymbol> {
        if let SymbolVariant::Primitive(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_primitive_mut(&mut self) -> Option<&mut PrimitiveTypeSymbol> {
        if let SymbolVariant::Primitive(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_enum(self) -> Option<EnumSymbol> {
        if let SymbolVariant::Enum(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_enum(&self) -> Option<&EnumSymbol> {
        if let SymbolVariant::Enum(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_enum_mut(&mut self) -> Option<&mut EnumSymbol> {
        if let SymbolVariant::Enum(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_enum_member(self) -> Option<EnumMemberSymbol> {
        if let SymbolVariant::EnumMember(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_enum_member(&self) -> Option<&EnumMemberSymbol> {
        if let SymbolVariant::EnumMember(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_enum_member_mut(&mut self) -> Option<&mut EnumMemberSymbol> {
        if let SymbolVariant::EnumMember(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_struct(self) -> Option<StructSymbol> {
        if let SymbolVariant::Struct(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_struct(&self) -> Option<&StructSymbol> {
        if let SymbolVariant::Struct(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_struct_mut(&mut self) -> Option<&mut StructSymbol> {
        if let SymbolVariant::Struct(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_class(self) -> Option<ClassSymbol> {
        if let SymbolVariant::Class(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_class(&self) -> Option<&ClassSymbol> {
        if let SymbolVariant::Class(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_class_mut(&mut self) -> Option<&mut ClassSymbol> {
        if let SymbolVariant::Class(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_state(self) -> Option<StateSymbol> {
        if let SymbolVariant::State(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_state(&self) -> Option<&StateSymbol> {
        if let SymbolVariant::State(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_state_mut(&mut self) -> Option<&mut StateSymbol> {
        if let SymbolVariant::State(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_array(self) -> Option<ArrayTypeSymbol> {
        if let SymbolVariant::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&ArrayTypeSymbol> {
        if let SymbolVariant::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut ArrayTypeSymbol> {
        if let SymbolVariant::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_global_func(self) -> Option<GlobalFunctionSymbol> {
        if let SymbolVariant::GlobalFunc(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_global_func(&self) -> Option<&GlobalFunctionSymbol> {
        if let SymbolVariant::GlobalFunc(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_global_func_mut(&mut self) -> Option<&mut GlobalFunctionSymbol> {
        if let SymbolVariant::GlobalFunc(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_member_func(self) -> Option<MemberFunctionSymbol> {
        if let SymbolVariant::MemberFunc(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_member_func(&self) -> Option<&MemberFunctionSymbol> {
        if let SymbolVariant::MemberFunc(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_member_func_mut(&mut self) -> Option<&mut MemberFunctionSymbol> {
        if let SymbolVariant::MemberFunc(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_event(self) -> Option<EventSymbol> {
        if let SymbolVariant::Event(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_event(&self) -> Option<&EventSymbol> {
        if let SymbolVariant::Event(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_event_mut(&mut self) -> Option<&mut EventSymbol> {
        if let SymbolVariant::Event(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_func_param(self) -> Option<FunctionParameterSymbol> {
        if let SymbolVariant::FuncParam(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_func_param(&self) -> Option<&FunctionParameterSymbol> {
        if let SymbolVariant::FuncParam(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_func_param_mut(&mut self) -> Option<&mut FunctionParameterSymbol> {
        if let SymbolVariant::FuncParam(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_global_var(self) -> Option<GlobalVarSymbol> {
        if let SymbolVariant::GlobalVar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_global_var(&self) -> Option<&GlobalVarSymbol> {
        if let SymbolVariant::GlobalVar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_global_var_mut(&mut self) -> Option<&mut GlobalVarSymbol> {
        if let SymbolVariant::GlobalVar(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_member_var(self) -> Option<MemberVarSymbol> {
        if let SymbolVariant::MemberVar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_member_var(&self) -> Option<&MemberVarSymbol> {
        if let SymbolVariant::MemberVar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_member_var_mut(&mut self) -> Option<&mut MemberVarSymbol> {
        if let SymbolVariant::MemberVar(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_autobind(self) -> Option<AutobindSymbol> {
        if let SymbolVariant::Autobind(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_autobind(&self) -> Option<&AutobindSymbol> {
        if let SymbolVariant::Autobind(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_autobind_mut(&mut self) -> Option<&mut AutobindSymbol> {
        if let SymbolVariant::Autobind(v) = self {
            Some(v)
        } else {
            None
        }
    }


    pub fn into_local_var(self) -> Option<LocalVarSymbol> {
        if let SymbolVariant::LocalVar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_local_var(&self) -> Option<&LocalVarSymbol> {
        if let SymbolVariant::LocalVar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_local_var_mut(&mut self) -> Option<&mut LocalVarSymbol> {
        if let SymbolVariant::LocalVar(v) = self {
            Some(v)
        } else {
            None
        }
    }
}


impl From<PrimitiveTypeSymbol> for SymbolVariant {
    fn from(value: PrimitiveTypeSymbol) -> Self {
        Self::Primitive(value)
    }
}

impl From<EnumSymbol> for SymbolVariant {
    fn from(value: EnumSymbol) -> Self {
        Self::Enum(value)
    }
}

impl From<EnumMemberSymbol> for SymbolVariant {
    fn from(value: EnumMemberSymbol) -> Self {
        Self::EnumMember(value)
    }
}

impl From<StructSymbol> for SymbolVariant {
    fn from(value: StructSymbol) -> Self {
        Self::Struct(value)
    }
}

impl From<ClassSymbol> for SymbolVariant {
    fn from(value: ClassSymbol) -> Self {
        Self::Class(value)
    }
}

impl From<StateSymbol> for SymbolVariant {
    fn from(value: StateSymbol) -> Self {
        Self::State(value)
    }
}

impl From<ArrayTypeSymbol> for SymbolVariant {
    fn from(value: ArrayTypeSymbol) -> Self {
        Self::Array(value)
    }
}

impl From<GlobalFunctionSymbol> for SymbolVariant {
    fn from(value: GlobalFunctionSymbol) -> Self {
        Self::GlobalFunc(value)
    }
}

impl From<MemberFunctionSymbol> for SymbolVariant {
    fn from(value: MemberFunctionSymbol) -> Self {
        Self::MemberFunc(value)
    }
}

impl From<EventSymbol> for SymbolVariant {
    fn from(value: EventSymbol) -> Self {
        Self::Event(value)
    }
}

impl From<FunctionParameterSymbol> for SymbolVariant {
    fn from(value: FunctionParameterSymbol) -> Self {
        Self::FuncParam(value)
    }
}

impl From<GlobalVarSymbol> for SymbolVariant {
    fn from(value: GlobalVarSymbol) -> Self {
        Self::GlobalVar(value)
    }
}

impl From<MemberVarSymbol> for SymbolVariant {
    fn from(value: MemberVarSymbol) -> Self {
        Self::MemberVar(value)
    }
}

impl From<AutobindSymbol> for SymbolVariant {
    fn from(value: AutobindSymbol) -> Self {
        Self::Autobind(value)
    }
}

impl From<LocalVarSymbol> for SymbolVariant {
    fn from(value: LocalVarSymbol) -> Self {
        Self::LocalVar(value)
    }
}
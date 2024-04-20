use strum_macros::{EnumIs, EnumTryAs};
use super::symbols::*;


#[derive(Debug, Clone, EnumIs, EnumTryAs)]
pub enum SymbolVariant {
    // types
    Class(ClassSymbol),
    State(StateSymbol),
    Struct(StructSymbol),
    Enum(EnumSymbol),
    Array(ArrayTypeSymbol),

    // callables
    GlobalFunc(GlobalFunctionSymbol),
    MemberFunc(MemberFunctionSymbol),
    Event(EventSymbol),

    // data
    Primitive(PrimitiveTypeSymbol),
    EnumVariant(EnumVariantSymbol),
    FuncParam(FunctionParameterSymbol),
    GlobalVar(GlobalVarSymbol),
    MemberVar(MemberVarSymbol),
    Autobind(AutobindSymbol),
    LocalVar(LocalVarSymbol),
    SpecialVar(SpecialVarSymbol)
}

impl SymbolVariant {
    pub fn as_dyn(&self) -> &dyn Symbol {
        match self {
            SymbolVariant::Primitive(v) => v,
            SymbolVariant::Enum(v) => v,
            SymbolVariant::EnumVariant(v) => v,
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
            SymbolVariant::SpecialVar(v) => v
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

impl From<EnumVariantSymbol> for SymbolVariant {
    fn from(value: EnumVariantSymbol) -> Self {
        Self::EnumVariant(value)
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

impl From<SpecialVarSymbol> for SymbolVariant {
    fn from(value: SpecialVarSymbol) -> Self {
        Self::SpecialVar(value)
    }
}
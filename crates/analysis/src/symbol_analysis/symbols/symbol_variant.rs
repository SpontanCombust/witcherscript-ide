use strum_macros::{EnumIs, EnumTryAs};
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Clone, EnumIs, EnumTryAs)]
pub enum SymbolVariant {
    // types
    Class(ClassSymbol),
    State(StateSymbol),
    Struct(StructSymbol),
    Enum(EnumSymbol),

    Array(ArrayTypeSymbol),
    ArrayFunc(ArrayTypeFunctionSymbol),
    ArrayFuncParam(ArrayTypeFunctionParameterSymbol),

    // callables
    GlobalFunc(GlobalFunctionSymbol),
    MemberFunc(MemberFunctionSymbol),
    Event(EventSymbol),
    Constructor(ConstructorSymbol),

    // data
    Primitive(PrimitiveTypeSymbol),
    EnumVariant(EnumVariantSymbol),
    FuncParam(FunctionParameterSymbol),
    GlobalVar(GlobalVarSymbol),
    MemberVar(MemberVarSymbol),
    Autobind(AutobindSymbol),
    LocalVar(LocalVarSymbol),
    ThisVar(ThisVarSymbol),
    SuperVar(SuperVarSymbol),
    ParentVar(ParentVarSymbol),
    VirtualParentVar(VirtualParentVarSymbol),
}

impl std::fmt::Debug for SymbolVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class(s) => s.fmt(f),
            Self::State(s) => s.fmt(f),
            Self::Struct(s) => s.fmt(f),
            Self::Enum(s) => s.fmt(f),
            Self::Array(s) => s.fmt(f),
            Self::ArrayFunc(s) => s.fmt(f),
            Self::ArrayFuncParam(s) => s.fmt(f),
            Self::GlobalFunc(s) => s.fmt(f),
            Self::MemberFunc(s) => s.fmt(f),
            Self::Event(s) => s.fmt(f),
            Self::Constructor(s) => s.fmt(f),
            Self::Primitive(s) => s.fmt(f),
            Self::EnumVariant(s) => s.fmt(f),
            Self::FuncParam(s) => s.fmt(f),
            Self::GlobalVar(s) => s.fmt(f),
            Self::MemberVar(s) => s.fmt(f),
            Self::Autobind(s) => s.fmt(f),
            Self::LocalVar(s) => s.fmt(f),
            Self::ThisVar(s) => s.fmt(f),
            Self::SuperVar(s) => s.fmt(f),
            Self::ParentVar(s) => s.fmt(f),
            Self::VirtualParentVar(s) => s.fmt(f),
        }
    }
}

impl SymbolVariant {
    pub fn typ(&self) -> SymbolType {
        match self {
            SymbolVariant::Class(s) => s.typ(),
            SymbolVariant::State(s) => s.typ(),
            SymbolVariant::Struct(s) => s.typ(),
            SymbolVariant::Enum(s) => s.typ(),
            SymbolVariant::Array(s) => s.typ(),
            SymbolVariant::ArrayFunc(s) => s.typ(),
            SymbolVariant::ArrayFuncParam(s) => s.typ(),
            SymbolVariant::GlobalFunc(s) => s.typ(),
            SymbolVariant::MemberFunc(s) => s.typ(),
            SymbolVariant::Event(s) => s.typ(),
            SymbolVariant::Constructor(s) => s.typ(),
            SymbolVariant::Primitive(s) => s.typ(),
            SymbolVariant::EnumVariant(s) => s.typ(),
            SymbolVariant::FuncParam(s) => s.typ(),
            SymbolVariant::GlobalVar(s) => s.typ(),
            SymbolVariant::MemberVar(s) => s.typ(),
            SymbolVariant::Autobind(s) => s.typ(),
            SymbolVariant::LocalVar(s) => s.typ(),
            SymbolVariant::ThisVar(s) => s.typ(),
            SymbolVariant::SuperVar(s) => s.typ(),
            SymbolVariant::ParentVar(s) => s.typ(),
            SymbolVariant::VirtualParentVar(s) => s.typ(),
        }
    }

    pub fn path(&self) -> &SymbolPath {
        match self {
            SymbolVariant::Class(s) => s.path(),
            SymbolVariant::State(s) => s.path(),
            SymbolVariant::Struct(s) => s.path(),
            SymbolVariant::Enum(s) => s.path(),
            SymbolVariant::Array(s) => s.path(),
            SymbolVariant::ArrayFunc(s) => s.path(),
            SymbolVariant::ArrayFuncParam(s) => s.path(),
            SymbolVariant::GlobalFunc(s) => s.path(),
            SymbolVariant::MemberFunc(s) => s.path(),
            SymbolVariant::Event(s) => s.path(),
            SymbolVariant::Constructor(s) => s.path(),
            SymbolVariant::Primitive(s) => s.path(),
            SymbolVariant::EnumVariant(s) => s.path(),
            SymbolVariant::FuncParam(s) => s.path(),
            SymbolVariant::GlobalVar(s) => s.path(),
            SymbolVariant::MemberVar(s) => s.path(),
            SymbolVariant::Autobind(s) => s.path(),
            SymbolVariant::LocalVar(s) => s.path(),
            SymbolVariant::ThisVar(s) => s.path(),
            SymbolVariant::SuperVar(s) => s.path(),
            SymbolVariant::ParentVar(s) => s.path(),
            SymbolVariant::VirtualParentVar(s) => s.path(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            SymbolVariant::Class(s) => s.name(),
            SymbolVariant::State(s) => s.name(),
            SymbolVariant::Struct(s) => s.name(),
            SymbolVariant::Enum(s) => s.name(),
            SymbolVariant::Array(s) => s.name(),
            SymbolVariant::ArrayFunc(s) => s.name(),
            SymbolVariant::ArrayFuncParam(s) => s.name(),
            SymbolVariant::GlobalFunc(s) => s.name(),
            SymbolVariant::MemberFunc(s) => s.name(),
            SymbolVariant::Event(s) => s.name(),
            SymbolVariant::Constructor(s) => s.name(),
            SymbolVariant::Primitive(s) => s.name(),
            SymbolVariant::EnumVariant(s) => s.name(),
            SymbolVariant::FuncParam(s) => s.name(),
            SymbolVariant::GlobalVar(s) => s.name(),
            SymbolVariant::MemberVar(s) => s.name(),
            SymbolVariant::Autobind(s) => s.name(),
            SymbolVariant::LocalVar(s) => s.name(),
            SymbolVariant::ThisVar(s) => s.name(),
            SymbolVariant::SuperVar(s) => s.name(),
            SymbolVariant::ParentVar(s) => s.name(),
            SymbolVariant::VirtualParentVar(s) => s.name(),
        }
    }

    pub fn location(&self) -> Option<&SymbolLocation> {
        match self {
            SymbolVariant::Class(s) => Some(s.location()),
            SymbolVariant::State(s) => Some(s.location()),
            SymbolVariant::Struct(s) => Some(s.location()),
            SymbolVariant::Enum(s) => Some(s.location()),
            SymbolVariant::Array(_) => None,
            SymbolVariant::ArrayFunc(_) => None,
            SymbolVariant::ArrayFuncParam(_) => None,
            SymbolVariant::GlobalFunc(s) => Some(s.location()),
            SymbolVariant::MemberFunc(s) => Some(s.location()),
            SymbolVariant::Event(s) => Some(s.location()),
            SymbolVariant::Constructor(s) => Some(s.location()),
            SymbolVariant::Primitive(_) => None,
            SymbolVariant::EnumVariant(s) => Some(s.location()),
            SymbolVariant::FuncParam(s) => Some(s.location()),
            SymbolVariant::GlobalVar(_) => None,
            SymbolVariant::MemberVar(s) => Some(s.location()),
            SymbolVariant::Autobind(s) => Some(s.location()),
            SymbolVariant::LocalVar(s) => Some(s.location()),
            SymbolVariant::ThisVar(_) => None,
            SymbolVariant::SuperVar(_) => None,
            SymbolVariant::ParentVar(_) => None,
            SymbolVariant::VirtualParentVar(_) => None,
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

impl From<ArrayTypeFunctionSymbol> for SymbolVariant {
    fn from(value: ArrayTypeFunctionSymbol) -> Self {
        Self::ArrayFunc(value)
    }
}

impl From<ArrayTypeFunctionParameterSymbol> for SymbolVariant {
    fn from(value: ArrayTypeFunctionParameterSymbol) -> Self {
        Self::ArrayFuncParam(value)
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

impl From<ConstructorSymbol> for SymbolVariant {
    fn from(value: ConstructorSymbol) -> Self {
        Self::Constructor(value)
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

impl From<ThisVarSymbol> for SymbolVariant {
    fn from(value: ThisVarSymbol) -> Self {
        Self::ThisVar(value)
    }
}

impl From<SuperVarSymbol> for SymbolVariant {
    fn from(value: SuperVarSymbol) -> Self {
        Self::SuperVar(value)
    }
}

impl From<ParentVarSymbol> for SymbolVariant {
    fn from(value: ParentVarSymbol) -> Self {
        Self::ParentVar(value)
    }
}

impl From<VirtualParentVarSymbol> for SymbolVariant {
    fn from(value: VirtualParentVarSymbol) -> Self {
        Self::VirtualParentVar(value)
    }
}
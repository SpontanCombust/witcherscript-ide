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
    MemberFuncInjector(MemberFunctionInjectorSymbol),
    MemberFuncReplacer(MemberFunctionReplacerSymbol),
    GlobalFuncReplacer(GlobalFunctionReplacerSymbol),
    MemberFuncWrapper(MemberFunctionWrapperSymbol),

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
    StateSuperVar(StateSuperVarSymbol),
    ParentVar(ParentVarSymbol),
    VirtualParentVar(VirtualParentVarSymbol),
    MemberVarInjector(MemberVarInjectorSymbol)
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
            Self::StateSuperVar(s) => s.fmt(f),
            Self::ParentVar(s) => s.fmt(f),
            Self::VirtualParentVar(s) => s.fmt(f),

            Self::MemberFuncInjector(s) => s.fmt(f),
            Self::MemberFuncReplacer(s) => s.fmt(f),
            Self::GlobalFuncReplacer(s) => s.fmt(f),
            Self::MemberFuncWrapper(s) => s.fmt(f),
            Self::MemberVarInjector(s) => s.fmt(f)
        }
    }
}

impl SymbolVariant {
    pub fn typ(&self) -> SymbolType {
        match self {
            Self::Class(s) => s.typ(),
            Self::State(s) => s.typ(),
            Self::Struct(s) => s.typ(),
            Self::Enum(s) => s.typ(),
            Self::Array(s) => s.typ(),
            Self::ArrayFunc(s) => s.typ(),
            Self::ArrayFuncParam(s) => s.typ(),
            Self::GlobalFunc(s) => s.typ(),
            Self::MemberFunc(s) => s.typ(),
            Self::Event(s) => s.typ(),
            Self::Constructor(s) => s.typ(),
            Self::Primitive(s) => s.typ(),
            Self::EnumVariant(s) => s.typ(),
            Self::FuncParam(s) => s.typ(),
            Self::GlobalVar(s) => s.typ(),
            Self::MemberVar(s) => s.typ(),
            Self::Autobind(s) => s.typ(),
            Self::LocalVar(s) => s.typ(),
            Self::ThisVar(s) => s.typ(),
            Self::SuperVar(s) => s.typ(),
            Self::StateSuperVar(s) => s.typ(),
            Self::ParentVar(s) => s.typ(),
            Self::VirtualParentVar(s) => s.typ(),

            Self::MemberFuncInjector(s) => s.typ(),
            Self::MemberFuncReplacer(s) => s.typ(),
            Self::GlobalFuncReplacer(s) => s.typ(),
            Self::MemberFuncWrapper(s) => s.typ(),
            Self::MemberVarInjector(s) => s.typ()
        }
    }

    pub fn path(&self) -> &SymbolPath {
        match self {
            Self::Class(s) => s.path(),
            Self::State(s) => s.path(),
            Self::Struct(s) => s.path(),
            Self::Enum(s) => s.path(),
            Self::Array(s) => s.path(),
            Self::ArrayFunc(s) => s.path(),
            Self::ArrayFuncParam(s) => s.path(),
            Self::GlobalFunc(s) => s.path(),
            Self::MemberFunc(s) => s.path(),
            Self::Event(s) => s.path(),
            Self::Constructor(s) => s.path(),
            Self::Primitive(s) => s.path(),
            Self::EnumVariant(s) => s.path(),
            Self::FuncParam(s) => s.path(),
            Self::GlobalVar(s) => s.path(),
            Self::MemberVar(s) => s.path(),
            Self::Autobind(s) => s.path(),
            Self::LocalVar(s) => s.path(),
            Self::ThisVar(s) => s.path(),
            Self::SuperVar(s) => s.path(),
            Self::StateSuperVar(s) => s.path(),
            Self::ParentVar(s) => s.path(),
            Self::VirtualParentVar(s) => s.path(),

            Self::MemberFuncInjector(s) => s.path(),
            Self::MemberFuncReplacer(s) => s.path(),
            Self::GlobalFuncReplacer(s) => s.path(),
            Self::MemberFuncWrapper(s) => s.path(),
            Self::MemberVarInjector(s) => s.path()
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Class(s) => s.name(),
            Self::State(s) => s.name(),
            Self::Struct(s) => s.name(),
            Self::Enum(s) => s.name(),
            Self::Array(s) => s.name(),
            Self::ArrayFunc(s) => s.name(),
            Self::ArrayFuncParam(s) => s.name(),
            Self::GlobalFunc(s) => s.name(),
            Self::MemberFunc(s) => s.name(),
            Self::Event(s) => s.name(),
            Self::Constructor(s) => s.name(),
            Self::Primitive(s) => s.name(),
            Self::EnumVariant(s) => s.name(),
            Self::FuncParam(s) => s.name(),
            Self::GlobalVar(s) => s.name(),
            Self::MemberVar(s) => s.name(),
            Self::Autobind(s) => s.name(),
            Self::LocalVar(s) => s.name(),
            Self::ThisVar(s) => s.name(),
            Self::SuperVar(s) => s.name(),
            Self::StateSuperVar(s) => s.name(),
            Self::ParentVar(s) => s.name(),
            Self::VirtualParentVar(s) => s.name(),

            Self::MemberFuncInjector(s) => s.name(),
            Self::MemberFuncReplacer(s) => s.name(),
            Self::GlobalFuncReplacer(s) => s.name(),
            Self::MemberFuncWrapper(s) => s.name(),
            Self::MemberVarInjector(s) => s.name()
        }
    }

    pub fn location(&self) -> Option<&SymbolLocation> {
        match self {
            Self::Class(s) => Some(s.location()),
            Self::State(s) => Some(s.location()),
            Self::Struct(s) => Some(s.location()),
            Self::Enum(s) => Some(s.location()),
            Self::Array(_) => None,
            Self::ArrayFunc(_) => None,
            Self::ArrayFuncParam(_) => None,
            Self::GlobalFunc(s) => Some(s.location()),
            Self::MemberFunc(s) => Some(s.location()),
            Self::Event(s) => Some(s.location()),
            Self::Constructor(s) => Some(s.location()),
            Self::Primitive(_) => None,
            Self::EnumVariant(s) => Some(s.location()),
            Self::FuncParam(s) => Some(s.location()),
            Self::GlobalVar(_) => None,
            Self::MemberVar(s) => Some(s.location()),
            Self::Autobind(s) => Some(s.location()),
            Self::LocalVar(s) => Some(s.location()),
            Self::ThisVar(_) => None,
            Self::SuperVar(_) => None,
            Self::StateSuperVar(_) => None,
            Self::ParentVar(_) => None,
            Self::VirtualParentVar(_) => None,

            Self::MemberFuncInjector(s) => Some(s.location()),
            Self::MemberFuncReplacer(s) => Some(s.location()),
            Self::GlobalFuncReplacer(s) => Some(s.location()),
            Self::MemberFuncWrapper(s) => Some(s.location()),
            Self::MemberVarInjector(s) => Some(s.location())
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

impl From<StateSuperVarSymbol> for SymbolVariant {
    fn from(value: StateSuperVarSymbol) -> Self {
        Self::StateSuperVar(value)
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

impl From<MemberFunctionInjectorSymbol> for SymbolVariant {
    fn from(value: MemberFunctionInjectorSymbol) -> Self {
        Self::MemberFuncInjector(value)
    }
}

impl From<MemberFunctionReplacerSymbol> for SymbolVariant {
    fn from(value: MemberFunctionReplacerSymbol) -> Self {
        Self::MemberFuncReplacer(value)
    }
}

impl From<GlobalFunctionReplacerSymbol> for SymbolVariant {
    fn from(value: GlobalFunctionReplacerSymbol) -> Self {
        Self::GlobalFuncReplacer(value)
    }
}

impl From<MemberFunctionWrapperSymbol> for SymbolVariant {
    fn from(value: MemberFunctionWrapperSymbol) -> Self {
        Self::MemberFuncWrapper(value)
    }
}

impl From<MemberVarInjectorSymbol> for SymbolVariant {
    fn from(value: MemberVarInjectorSymbol) -> Self {
        Self::MemberVarInjector(value)
    }
}
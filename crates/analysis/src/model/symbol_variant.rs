use std::path::Path;
use strum_macros::{EnumIs, EnumTryAs};
use lsp_types as lsp;
use super::symbols::*;


#[derive(Clone, EnumIs, EnumTryAs)]
pub enum SymbolVariant {
    // types
    Class(ClassSymbol),
    State(StateSymbol),
    Struct(StructSymbol),
    Enum(EnumSymbol),
    Array(ArrayTypeSymbol), //TODO maybe rework array symbol so only one set of symbols has to persist and specialized types get generated dynamically

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

impl std::fmt::Debug for SymbolVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class(s) => s.fmt(f),
            Self::State(s) => s.fmt(f),
            Self::Struct(s) => s.fmt(f),
            Self::Enum(s) => s.fmt(f),
            Self::Array(s) => s.fmt(f),
            Self::GlobalFunc(s) => s.fmt(f),
            Self::MemberFunc(s) => s.fmt(f),
            Self::Event(s) => s.fmt(f),
            Self::Primitive(s) => s.fmt(f),
            Self::EnumVariant(s) => s.fmt(f),
            Self::FuncParam(s) => s.fmt(f),
            Self::GlobalVar(s) => s.fmt(f),
            Self::MemberVar(s) => s.fmt(f),
            Self::Autobind(s) => s.fmt(f),
            Self::LocalVar(s) => s.fmt(f),
            Self::SpecialVar(s) => s.fmt(f),
        }
    }
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

    pub fn range(&self) -> Option<lsp::Range> {
        match self {
            SymbolVariant::Class(s) => Some(s.range()),
            SymbolVariant::State(s) => Some(s.range()),
            SymbolVariant::Struct(s) => Some(s.range()),
            SymbolVariant::Enum(s) => Some(s.range()),
            SymbolVariant::Array(_) => None,
            SymbolVariant::GlobalFunc(s) => Some(s.range()),
            SymbolVariant::MemberFunc(s) => Some(s.range()),
            SymbolVariant::Event(s) => Some(s.range()),
            SymbolVariant::Primitive(_) => None,
            SymbolVariant::EnumVariant(s) => Some(s.range()),
            SymbolVariant::FuncParam(s) => Some(s.range()),
            SymbolVariant::GlobalVar(_) => None,
            SymbolVariant::MemberVar(s) => Some(s.range()),
            SymbolVariant::Autobind(s) => Some(s.range()),
            SymbolVariant::LocalVar(s) => Some(s.range()),
            SymbolVariant::SpecialVar(_) => None,
        }
    }

    pub fn label_range(&self) -> Option<lsp::Range> {
        match self {
            SymbolVariant::Class(s) => Some(s.label_range()),
            SymbolVariant::State(s) => Some(s.label_range()),
            SymbolVariant::Struct(s) => Some(s.label_range()),
            SymbolVariant::Enum(s) => Some(s.label_range()),
            SymbolVariant::Array(_) => None,
            SymbolVariant::GlobalFunc(s) => Some(s.label_range()),
            SymbolVariant::MemberFunc(s) => Some(s.label_range()),
            SymbolVariant::Event(s) => Some(s.label_range()),
            SymbolVariant::Primitive(_) => None,
            SymbolVariant::EnumVariant(s) => Some(s.label_range()),
            SymbolVariant::FuncParam(s) => Some(s.label_range()),
            SymbolVariant::GlobalVar(_) => None,
            SymbolVariant::MemberVar(s) => Some(s.label_range()),
            SymbolVariant::Autobind(s) => Some(s.label_range()),
            SymbolVariant::LocalVar(s) => Some(s.label_range()),
            SymbolVariant::SpecialVar(_) => None,
        }
    }

    pub fn local_source_path(&self) -> Option<&Path> {
        match self {
            SymbolVariant::Class(s) => Some(s.local_source_path()),
            SymbolVariant::State(s) => Some(s.local_source_path()),
            SymbolVariant::Struct(s) => Some(s.local_source_path()),
            SymbolVariant::Enum(s) => Some(s.local_source_path()),
            SymbolVariant::Array(_) => None,
            SymbolVariant::GlobalFunc(s) => Some(s.local_source_path()),
            SymbolVariant::MemberFunc(_) => None,
            SymbolVariant::Event(_) => None,
            SymbolVariant::Primitive(_) => None,
            SymbolVariant::EnumVariant(_) => None,
            SymbolVariant::FuncParam(_) => None,
            SymbolVariant::GlobalVar(_) => None,
            SymbolVariant::MemberVar(_) => None,
            SymbolVariant::Autobind(_) => None,
            SymbolVariant::LocalVar(_) => None,
            SymbolVariant::SpecialVar(_) => None,
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
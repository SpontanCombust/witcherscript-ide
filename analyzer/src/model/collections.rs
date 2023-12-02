use uuid::Uuid;
use std::collections::HashMap;
use super::symbols::*;


#[derive(Debug, Clone, Copy)]
pub struct SymbolTableValue {
    pub id: Uuid,
    pub typ: SymbolType
}

impl SymbolTableValue {
    pub fn from_symbol<S: SymbolData>(sym: &Symbol<S>) -> Self {
        Self {
            id: sym.id(),
            typ: sym.typ()
        }
    }
}

//TODO make into struct
// introduce scopes arranged in a stack
pub type SymbolTable = HashMap<String, SymbolTableValue>;


#[derive(Debug, Clone, Default)]
pub struct SymbolDb {
    pub primitives: HashMap<Uuid, PrimitiveTypeSymbol>,
    pub enums: HashMap<Uuid, EnumSymbol>,
    pub structs: HashMap<Uuid, StructSymbol>,
    pub classes: HashMap<Uuid, ClassSymbol>,
    pub states: HashMap<Uuid, StateSymbol>,
    pub arrays: HashMap<Uuid, ArrayTypeSymbol>,

    pub enum_members: HashMap<Uuid, EnumMemberSymbol>,

    pub global_funcs: HashMap<Uuid, GlobalFunctionSymbol>,
    pub member_funcs: HashMap<Uuid, MemberFunctionSymbol>,
    pub events: HashMap<Uuid, EventSymbol>,

    pub params: HashMap<Uuid, FunctionParameterSymbol>,

    pub global_vars: HashMap<Uuid, GlobalVarSymbol>,
    pub member_vars: HashMap<Uuid, MemberVarSymbol>,
    pub autobinds: HashMap<Uuid, AutobindSymbol>,
    pub local_vars: HashMap<Uuid, LocalVarSymbol>
}

impl SymbolDb {
    pub fn new() -> Self {
        Self::default()
    }
}
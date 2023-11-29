use uuid::Uuid;
use std::collections::HashMap;
use super::symbols::*;


pub struct SymbolTableValue {
    pub id: Uuid,
    pub typ: SymbolType
}

pub type SymbolTable = HashMap<String, SymbolTableValue>;


pub struct SymbolDb {
    pub primitives: HashMap<Uuid, PrimitiveTypeSymbol>,
    pub global_vars: HashMap<Uuid, GlobalVarSymbol>,
    pub arrays: HashMap<Uuid, ArrayTypeSymbol>,
    pub classes: HashMap<Uuid, ClassSymbol>,
    pub states: HashMap<Uuid, StateSymbol>,
    pub structs: HashMap<Uuid, StructSymbol>,
    pub enums: HashMap<Uuid, EnumSymbol>,
    pub global_funcs: HashMap<Uuid, GlobalFunctionSymbol>,
}
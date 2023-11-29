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

impl SymbolDb {
    pub fn insert(&mut self, entry: SymbolDbEntry) {
        match entry {
            SymbolDbEntry::Primitive(val) => { self.primitives.insert(val.symbol_id(), val); },
            SymbolDbEntry::GlobalVar(val) => { self.global_vars.insert(val.symbol_id(), val); },
            SymbolDbEntry::Array(val) => { self.arrays.insert(val.symbol_id(), val); },
            SymbolDbEntry::Class(val) => { self.classes.insert(val.symbol_id(), val); },
            SymbolDbEntry::State(val) => { self.states.insert(val.symbol_id(), val); },
            SymbolDbEntry::Struct(val) => { self.structs.insert(val.symbol_id(), val); },
            SymbolDbEntry::Enum(val) => { self.enums.insert(val.symbol_id(), val); },
            SymbolDbEntry::GlobalFunction(val) => { self.global_funcs.insert(val.symbol_id(), val); },
        };
    }
}

pub enum SymbolDbEntry {
    Primitive(PrimitiveTypeSymbol),
    GlobalVar(GlobalVarSymbol),
    Array(ArrayTypeSymbol),
    Class(ClassSymbol),
    State(StateSymbol),
    Struct(StructSymbol),
    Enum(EnumSymbol),
    GlobalFunction(GlobalFunctionSymbol)
}

impl From<PrimitiveTypeSymbol> for SymbolDbEntry {
    fn from(value: PrimitiveTypeSymbol) -> Self {
        SymbolDbEntry::Primitive(value)
    }
}

impl From<GlobalVarSymbol> for SymbolDbEntry {
    fn from(value: GlobalVarSymbol) -> Self {
        SymbolDbEntry::GlobalVar(value)
    }
}

impl From<ArrayTypeSymbol> for SymbolDbEntry {
    fn from(value: ArrayTypeSymbol) -> Self {
        SymbolDbEntry::Array(value)
    }
}

impl From<ClassSymbol> for SymbolDbEntry {
    fn from(value: ClassSymbol) -> Self {
        SymbolDbEntry::Class(value)
    }
}

impl From<StateSymbol> for SymbolDbEntry {
    fn from(value: StateSymbol) -> Self {
        SymbolDbEntry::State(value)
    }
}

impl From<StructSymbol> for SymbolDbEntry {
    fn from(value: StructSymbol) -> Self {
        SymbolDbEntry::Struct(value)
    }
}

impl From<EnumSymbol> for SymbolDbEntry {
    fn from(value: EnumSymbol) -> Self {
        SymbolDbEntry::Enum(value)
    }
}

impl From<GlobalFunctionSymbol> for SymbolDbEntry {
    fn from(value: GlobalFunctionSymbol) -> Self {
        SymbolDbEntry::GlobalFunction(value)
    }
}
use thiserror::Error;
use uuid::Uuid;
use std::{collections::HashMap, borrow::Cow};
use super::symbols::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SymbolTableKey<'t> {
    name: Cow<'t, str>,
    category: SymbolCategory
}

impl<'t> SymbolTableKey<'t> {
    pub fn new<S>(name: S, category: SymbolCategory) -> Self 
    where S: Into<Cow<'t, str>> {
        Self {
            name: name.into(),
            category
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct SymbolTableValue {
    /// Symbol Uuid
    pub id: Uuid,
    /// Type of the symbol
    pub typ: SymbolType,
}

impl SymbolTableValue {
    pub fn new(id: Uuid, typ: SymbolType) -> Self {
        Self {
            id, typ
        }
    }
}


type SymbolTableScope<'t> = HashMap<SymbolTableKey<'t>, SymbolTableValue>;


/// A scope aware map of symbols with access to their identifiers by name and symbol category.
#[derive(Debug, Clone)]
pub struct SymbolTable<'t> {
    stack: Vec<SymbolTableScope<'t>>,
}


#[derive(Debug, Clone, Error)]
pub enum SymbolTableError {
    #[error("global var already exists for name {0:?}")]
    GlobalVarAlreadyExists(String, SymbolTableValue),
    #[error("type already exists for name {0:?}")]
    TypeAlreadyExists(String, SymbolTableValue),
    #[error("data already exists in the same scope for name {0:?}")]
    DataAlreadyExists(String, SymbolTableValue),
    #[error("callable already exists in the same scope for name {0:?}")]
    CallableAlreadyExists(String, SymbolTableValue),
}

impl<'t> SymbolTable<'t> {
    pub fn new() -> Self {
        Self {
            stack: vec![SymbolTableScope::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.stack.push(SymbolTableScope::new())
    }

    pub fn pop_scope(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    
    /// Get the value and relative scope it is contained in (0 means this scope, higher means parent scopes)
    fn get_with_rel_scope<S>(&self, name: S, category: SymbolCategory) -> Option<(&SymbolTableValue, usize)> 
    where S: Into<Cow<'t, str>> {
        let k = SymbolTableKey::new(name, category);
        for (i, level) in self.stack.iter().rev().enumerate() {
            let v = level.get(&k);
            if v.is_some() {
                return v.map(|opt| (opt, i));
            }
        }

        None
    }

    pub fn contains<S>(&self, name: S, category: SymbolCategory) -> bool
    where S: Into<Cow<'t, str>> {
        self.get_with_rel_scope(name, category).is_some()
    }

    pub fn get<S>(&self, name: S, category: SymbolCategory) -> Option<&SymbolTableValue> 
    where S: Into<Cow<'t, str>> {
        self.get_with_rel_scope(name, category).map(|v| v.0)
    }
    
    /// If a symbol with that configuration can be inserted, returns None.
    /// Otherwise, returns the reason as to why that can't be done.
    pub fn can_insert(&self, name: &str, typ: SymbolType) -> Option<SymbolTableError> {
        use SymbolTableError::*;

        let exist_data = self.get_with_rel_scope(name, SymbolCategory::Data);
        if let Some((data_val, _)) = exist_data {
            // global var names are prohibited from being re-used in any context
            // globals are actually already defined on the lexel level (in the WS compiler)
            if data_val.typ == SymbolType::GlobalVar {
                return Some(GlobalVarAlreadyExists(name.to_string(), data_val.clone()));
            }
        }
        
        let cat = typ.category();
        match cat {
            SymbolCategory::Type |
            SymbolCategory::Data => {
                // If there is a type defined with that name, always fail.
                if let Some((typ_val, _)) = self.get_with_rel_scope(name, SymbolCategory::Type) {
                    return Some(TypeAlreadyExists(name.to_string(), typ_val.clone()));
                }
                // If there is a var or constant defined, but it exists in a higher scope, allow to obstruct it.
                // In the case of types, they can only be defined in the global scope. So when they check against
                // data, they check against defined enum members.
                else if let Some((data_val, data_scope)) = exist_data {
                    if data_scope == 0 {
                        return Some(DataAlreadyExists(name.to_string(), data_val.clone()));
                    }
                } 
            },
            SymbolCategory::Callable => {
                // Callables only need to check against other callables in the same scope.
                // They have the advantage of being able to be easily distinguished from the other two categories
                // - they are always used with `()` operator. Functions in WS are not first-class objects afaik.
                if let Some((callable_val, callable_scope)) = self.get_with_rel_scope(name, SymbolCategory::Callable) {
                    if callable_scope == 0 {
                        return Some(CallableAlreadyExists(name.to_string(), callable_val.clone()));
                    }
                }                 
            },
        }

        None
    }

    /// Inserts a symbol mapping into the table.
    /// Make sure to check with `can_insert` beforehand.
    pub fn insert(&mut self, name: &str, id: Uuid, typ: SymbolType) {
        let cat = typ.category();
        if cat == SymbolCategory::Type {
            // always insert types into the highest (global) scope
            self.stack.first_mut().unwrap().insert(
                SymbolTableKey::new(name.to_string(), cat), 
                SymbolTableValue::new(id, typ)
            );
        } else {
            // the rest of symbols can be scope dependant
            self.stack.last_mut().unwrap().insert(
                SymbolTableKey::new(name.to_string(), cat), 
                SymbolTableValue::new(id, typ)
            );
        }
    }
}




#[derive(Debug, Clone, Default)]
pub struct SymbolDb {
    map: HashMap<Uuid, SymbolDbMapValue>
}

#[derive(Debug, Clone)]
enum SymbolDbMapValue {
    Primitive(PrimitiveTypeSymbol),
    Enum(EnumSymbol),
    Struct(StructSymbol),
    Class(ClassSymbol),
    State(StateSymbol),
    Array(ArrayTypeSymbol),
    EnumMember(EnumMemberSymbol),
    GlobalFunc(GlobalFunctionSymbol),
    MemberFunc(MemberFunctionSymbol),
    Event(EventSymbol),
    FuncParam(FunctionParameterSymbol),
    GlobalVar(GlobalVarSymbol),
    MemberVar(MemberVarSymbol),
    Autobind(AutobindSymbol),
    LocalVar(LocalVarSymbol),
}

impl SymbolDb {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn insert_primitive(&mut self, sym: PrimitiveTypeSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::Primitive(sym));
    }

    pub fn get_primitive(&self, id: Uuid) -> Option<&PrimitiveTypeSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::Primitive(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_primitive_mut(&mut self, id: Uuid) -> Option<&mut PrimitiveTypeSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::Primitive(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_enum(&mut self, sym: EnumSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::Enum(sym));
    }

    pub fn get_enum(&self, id: Uuid) -> Option<&EnumSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::Enum(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_enum_mut(&mut self, id: Uuid) -> Option<&mut EnumSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::Enum(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_struct(&mut self, sym: StructSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::Struct(sym));
    }

    pub fn get_struct(&self, id: Uuid) -> Option<&StructSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::Struct(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_struct_mut(&mut self, id: Uuid) -> Option<&mut StructSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::Struct(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_class(&mut self, sym: ClassSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::Class(sym));
    }

    pub fn get_class(&self, id: Uuid) -> Option<&ClassSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::Class(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_class_mut(&mut self, id: Uuid) -> Option<&mut ClassSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::Class(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_state(&mut self, sym: StateSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::State(sym));
    }

    pub fn get_state(&self, id: Uuid) -> Option<&StateSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::State(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_state_mut(&mut self, id: Uuid) -> Option<&mut StateSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::State(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_array(&mut self, sym: ArrayTypeSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::Array(sym));
    }

    pub fn get_array(&self, id: Uuid) -> Option<&ArrayTypeSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::Array(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_array_mut(&mut self, id: Uuid) -> Option<&mut ArrayTypeSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::Array(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_enum_member(&mut self, sym: EnumMemberSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::EnumMember(sym));
    }

    pub fn get_enum_member(&self, id: Uuid) -> Option<&EnumMemberSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::EnumMember(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_enum_member_mut(&mut self, id: Uuid) -> Option<&mut EnumMemberSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::EnumMember(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_global_func(&mut self, sym: GlobalFunctionSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::GlobalFunc(sym));
    }

    pub fn get_global_func(&self, id: Uuid) -> Option<&GlobalFunctionSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::GlobalFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_global_func_mut(&mut self, id: Uuid) -> Option<&mut GlobalFunctionSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::GlobalFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_member_func(&mut self, sym: MemberFunctionSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::MemberFunc(sym));
    }

    pub fn get_member_func(&self, id: Uuid) -> Option<&MemberFunctionSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::MemberFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_member_func_mut(&mut self, id: Uuid) -> Option<&mut MemberFunctionSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::MemberFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_event(&mut self, sym: EventSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::Event(sym));
    }

    pub fn get_event(&self, id: Uuid) -> Option<&EventSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::Event(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_event_mut(&mut self, id: Uuid) -> Option<&mut EventSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::Event(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_func_param(&mut self, sym: FunctionParameterSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::FuncParam(sym));
    }

    pub fn get_func_param(&self, id: Uuid) -> Option<&FunctionParameterSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::FuncParam(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_func_param_mut(&mut self, id: Uuid) -> Option<&mut FunctionParameterSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::FuncParam(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_global_var(&mut self, sym: GlobalVarSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::GlobalVar(sym));
    }

    pub fn get_global_var(&self, id: Uuid) -> Option<&GlobalVarSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::GlobalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_global_var_mut(&mut self, id: Uuid) -> Option<&mut GlobalVarSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::GlobalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_member_var(&mut self, sym: MemberVarSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::MemberVar(sym));
    }

    pub fn get_member_var(&self, id: Uuid) -> Option<&MemberVarSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::MemberVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_member_var_mut(&mut self, id: Uuid) -> Option<&mut MemberVarSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::MemberVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_autobind(&mut self, sym: AutobindSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::Autobind(sym));
    }

    pub fn get_autobind(&self, id: Uuid) -> Option<&AutobindSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::Autobind(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_autobind_mut(&mut self, id: Uuid) -> Option<&mut AutobindSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::Autobind(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn insert_local_var(&mut self, sym: LocalVarSymbol) {
        self.map.insert(sym.id(), SymbolDbMapValue::LocalVar(sym));
    }

    pub fn get_local_var(&self, id: Uuid) -> Option<&LocalVarSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolDbMapValue::LocalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_local_var_mut(&mut self, id: Uuid) -> Option<&mut LocalVarSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolDbMapValue::LocalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }


    pub fn remove(&mut self, id: Uuid) {
        self.map.remove(&id);
    }
}



/// For mapping a sumbol to its children.
pub type SymbolAssociations = HashMap<Uuid, Vec<Uuid>>;
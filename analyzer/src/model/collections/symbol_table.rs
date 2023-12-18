use std::collections::HashMap;
use uuid::Uuid;
use crate::model::symbols::*;


/// Contains information about symbols in the workspace 
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    map: HashMap<Uuid, SymbolTableValue>
}

#[derive(Debug, Clone)]
enum SymbolTableValue {
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

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn insert_primitive(&mut self, sym: PrimitiveTypeSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::Primitive(sym));
    }

    pub fn get_primitive(&self, id: Uuid) -> Option<&PrimitiveTypeSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::Primitive(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_primitive_mut(&mut self, id: Uuid) -> Option<&mut PrimitiveTypeSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::Primitive(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_primitive(&mut self, id: Uuid) -> Option<PrimitiveTypeSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::Primitive(v) => Some(v),
                _ => {
                    // put the symbol back in the map if it was queried with wrong type
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_enum(&mut self, sym: EnumSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::Enum(sym));
    }

    pub fn get_enum(&self, id: Uuid) -> Option<&EnumSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::Enum(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_enum_mut(&mut self, id: Uuid) -> Option<&mut EnumSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::Enum(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_enum(&mut self, id: Uuid) -> Option<EnumSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::Enum(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_struct(&mut self, sym: StructSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::Struct(sym));
    }

    pub fn get_struct(&self, id: Uuid) -> Option<&StructSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::Struct(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_struct_mut(&mut self, id: Uuid) -> Option<&mut StructSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::Struct(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_struct(&mut self, id: Uuid) -> Option<StructSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::Struct(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_class(&mut self, sym: ClassSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::Class(sym));
    }

    pub fn get_class(&self, id: Uuid) -> Option<&ClassSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::Class(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_class_mut(&mut self, id: Uuid) -> Option<&mut ClassSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::Class(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_class(&mut self, id: Uuid) -> Option<ClassSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::Class(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_state(&mut self, sym: StateSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::State(sym));
    }

    pub fn get_state(&self, id: Uuid) -> Option<&StateSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::State(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_state_mut(&mut self, id: Uuid) -> Option<&mut StateSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::State(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_state(&mut self, id: Uuid) -> Option<StateSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::State(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_array(&mut self, sym: ArrayTypeSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::Array(sym));
    }

    pub fn get_array(&self, id: Uuid) -> Option<&ArrayTypeSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::Array(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_array_mut(&mut self, id: Uuid) -> Option<&mut ArrayTypeSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::Array(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_array(&mut self, id: Uuid) -> Option<ArrayTypeSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::Array(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_enum_member(&mut self, sym: EnumMemberSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::EnumMember(sym));
    }

    pub fn get_enum_member(&self, id: Uuid) -> Option<&EnumMemberSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::EnumMember(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_enum_member_mut(&mut self, id: Uuid) -> Option<&mut EnumMemberSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::EnumMember(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_enum_member(&mut self, id: Uuid) -> Option<EnumMemberSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::EnumMember(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_global_func(&mut self, sym: GlobalFunctionSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::GlobalFunc(sym));
    }

    pub fn get_global_func(&self, id: Uuid) -> Option<&GlobalFunctionSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::GlobalFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_global_func_mut(&mut self, id: Uuid) -> Option<&mut GlobalFunctionSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::GlobalFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_global_func(&mut self, id: Uuid) -> Option<GlobalFunctionSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::GlobalFunc(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_member_func(&mut self, sym: MemberFunctionSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::MemberFunc(sym));
    }

    pub fn get_member_func(&self, id: Uuid) -> Option<&MemberFunctionSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::MemberFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_member_func_mut(&mut self, id: Uuid) -> Option<&mut MemberFunctionSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::MemberFunc(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_member_func(&mut self, id: Uuid) -> Option<MemberFunctionSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::MemberFunc(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_event(&mut self, sym: EventSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::Event(sym));
    }

    pub fn get_event(&self, id: Uuid) -> Option<&EventSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::Event(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_event_mut(&mut self, id: Uuid) -> Option<&mut EventSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::Event(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_event(&mut self, id: Uuid) -> Option<EventSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::Event(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_func_param(&mut self, sym: FunctionParameterSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::FuncParam(sym));
    }

    pub fn get_func_param(&self, id: Uuid) -> Option<&FunctionParameterSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::FuncParam(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_func_param_mut(&mut self, id: Uuid) -> Option<&mut FunctionParameterSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::FuncParam(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_func_param(&mut self, id: Uuid) -> Option<FunctionParameterSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::FuncParam(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_global_var(&mut self, sym: GlobalVarSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::GlobalVar(sym));
    }

    pub fn get_global_var(&self, id: Uuid) -> Option<&GlobalVarSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::GlobalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_global_var_mut(&mut self, id: Uuid) -> Option<&mut GlobalVarSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::GlobalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_global_var(&mut self, id: Uuid) -> Option<GlobalVarSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::GlobalVar(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_member_var(&mut self, sym: MemberVarSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::MemberVar(sym));
    }

    pub fn get_member_var(&self, id: Uuid) -> Option<&MemberVarSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::MemberVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_member_var_mut(&mut self, id: Uuid) -> Option<&mut MemberVarSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::MemberVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_member_var(&mut self, id: Uuid) -> Option<MemberVarSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::MemberVar(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_autobind(&mut self, sym: AutobindSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::Autobind(sym));
    }

    pub fn get_autobind(&self, id: Uuid) -> Option<&AutobindSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::Autobind(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_autobind_mut(&mut self, id: Uuid) -> Option<&mut AutobindSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::Autobind(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_autobind(&mut self, id: Uuid) -> Option<AutobindSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::Autobind(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }


    pub fn insert_local_var(&mut self, sym: LocalVarSymbol) {
        self.map.insert(sym.id(), SymbolTableValue::LocalVar(sym));
    }

    pub fn get_local_var(&self, id: Uuid) -> Option<&LocalVarSymbol> {
        if let Some(sym) = self.map.get(&id) {
            match sym {
                SymbolTableValue::LocalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_local_var_mut(&mut self, id: Uuid) -> Option<&mut LocalVarSymbol> {
        if let Some(sym) = self.map.get_mut(&id) {
            match sym {
                SymbolTableValue::LocalVar(v) => Some(v),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn remove_local_var(&mut self, id: Uuid) -> Option<LocalVarSymbol> {
        if let Some(sym) = self.map.remove(&id) {
            match sym {
                SymbolTableValue::LocalVar(v) => Some(v),
                _ => {
                    self.map.insert(id, sym);
                    None
                }
            }
        } else {
            None
        }
    }
}

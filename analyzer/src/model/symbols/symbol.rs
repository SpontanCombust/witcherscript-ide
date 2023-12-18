use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Symbol<T> 
where T: SymbolData {
    pub(super) id: Uuid,
    //TODO optional node id?
    pub(super) name: String,
    pub(super) parent_id: Uuid,
    //TODO relative_span - a span relative to parent scope
    pub data: T
}

impl<T: SymbolData> Symbol<T> {
    pub fn new(name: &str, parent_id: Uuid, data: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            parent_id,
            data,
        }
    }

    pub fn new_with_default(name: &str, parent_id: Uuid) -> Self 
    where T: Default {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            parent_id,
            data: T::default(),
        }
    }

    
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn parent_id(&self) -> Uuid {
        self.parent_id
    }

    pub fn typ(&self) -> SymbolType {
        T::SYMBOL_TYPE
    }
}

pub trait SymbolData {
    const SYMBOL_TYPE: SymbolType;
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Type,
    Enum,
    Struct,
    Class,
    State,
    Array,
    
    EnumMember,

    GlobalFunction,
    MemberFunction,
    Event,

    Parameter,

    GlobalVar,
    MemberVar,
    Autobind,
    LocalVar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolCategory {
    Type,
    Data,
    Callable
}

impl SymbolType {
    pub fn category(&self) -> SymbolCategory {
        use SymbolType::*;
        match self {
            Type | Enum | Struct | Class | State | Array => SymbolCategory::Type,
            EnumMember | Parameter | GlobalVar | MemberVar | Autobind | LocalVar => SymbolCategory::Data,
            GlobalFunction | MemberFunction | Event => SymbolCategory::Callable,
        }
    }
}
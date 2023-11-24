use uuid::{Uuid, uuid};

mod basic_type_info;
mod enum_info;
mod struct_info;
mod var_info;
mod param_info;
mod function_info;
mod class_info;

pub use basic_type_info::*;
pub use enum_info::*;
pub use struct_info::*;
pub use var_info::*;
pub use param_info::*;
pub use function_info::*;
pub use class_info::*;


pub trait SymbolInfo {
    const TYPE: SymbolType;
    
    fn symbol_id(&self) -> Uuid;
    //TODO symbol_span(&self) -> Span;
    fn name(&self) -> &str;
}

pub enum SymbolType {
    Type,
    Class,
    Struct,
    State,
    Field,
    Enum,
    EnumMember,
    Function,
    Method,
    Event,
    Parameter,
    Variable
}

pub trait GlobalSymbolInfo: SymbolInfo {
    fn script_id(&self) -> Uuid;
}

pub trait ChildSymbolInfo: SymbolInfo {
    fn parent_symbol_id(&self) -> Uuid;
}

//TODO manually prepare UUIDs for native types 
pub const ERROR_SYMBOL_ID: Uuid         = uuid!("00000000-0000-0000-0000-000000000000");
pub const NATIVE_SYMBOL_SCRIPT_ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
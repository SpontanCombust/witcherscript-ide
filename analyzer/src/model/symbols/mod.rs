use uuid::{Uuid, uuid};

mod basic_type_symbol;
mod enum_symbol;
mod struct_symbol;
mod var_symbol;
mod func_param_symbol;
mod function_symbol;
mod class_symbol;
mod type_param_symbol;

pub use basic_type_symbol::*;
pub use enum_symbol::*;
pub use struct_symbol::*;
pub use var_symbol::*;
pub use func_param_symbol::*;
pub use function_symbol::*;
pub use class_symbol::*;
pub use type_param_symbol::*;


pub trait Symbol {
    const TYPE: SymbolType;
    
    fn symbol_id(&self) -> Uuid;
    //TODO symbol_span(&self) -> Span;
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Copy)]
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

pub trait GlobalSymbol: Symbol {
    fn script_id(&self) -> Uuid;
}

pub trait ChildSymbol: Symbol {
    fn parent_symbol_id(&self) -> Uuid;
}

//TODO manually prepare UUIDs for native types 
pub const ERROR_SYMBOL_ID: Uuid         = uuid!("00000000-0000-0000-0000-000000000000");
pub const NATIVE_SYMBOL_SCRIPT_ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
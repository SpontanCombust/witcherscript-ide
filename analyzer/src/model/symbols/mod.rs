use uuid::{Uuid, uuid};

mod basic_type_symbol;
mod enum_symbol;
mod struct_symbol;
mod var_symbol;
mod func_param_symbol;
mod function_symbol;
mod class_symbol;
mod state_symbol;
mod type_param_symbol;
mod generic_type_symbol;

pub use basic_type_symbol::*;
pub use enum_symbol::*;
pub use struct_symbol::*;
pub use var_symbol::*;
pub use func_param_symbol::*;
pub use function_symbol::*;
pub use class_symbol::*;
pub use type_param_symbol::*;
pub use generic_type_symbol::*;
pub use state_symbol::*;


pub trait Symbol {
    const TYPE: SymbolType;
    
    /// Unique identifier of the symbol
    fn symbol_id(&self) -> Uuid;
    /// Name of the symbol to be used in the symbol table
    fn symbol_name(&self) -> &str;
    /// Identifier of the symbol higher in the symbol tree
    /// If self is a global symbol it should return script identifier or NATIVE_SYMBOL_SCRIPT_ID
    fn parent_symbol_id(&self) -> Uuid;
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

//TODO manually prepare UUIDs for native types 
pub const ERROR_SYMBOL_ID: Uuid         = uuid!("00000000-0000-0000-0000-000000000000");
pub const NATIVE_SYMBOL_SCRIPT_ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
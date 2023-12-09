use uuid::{Uuid, uuid};

mod symbol;
mod primitive_type_symbol;
mod enum_symbol;
mod struct_symbol;
mod var_symbol;
mod func_param_symbol;
mod function_symbol;
mod class_symbol;
mod state_symbol;
mod array_type_symbol;

pub use symbol::*;
pub use primitive_type_symbol::*;
pub use enum_symbol::*;
pub use struct_symbol::*;
pub use var_symbol::*;
pub use func_param_symbol::*;
pub use function_symbol::*;
pub use class_symbol::*;
pub use array_type_symbol::*;
pub use state_symbol::*;


pub const ERROR_SYMBOL_ID: Uuid         = uuid!("00000000-0000-0000-0000-000000000000");
pub const NATIVE_SYMBOL_SCRIPT_ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
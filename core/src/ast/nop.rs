use super::{functions::FunctionStatement, classes::ClassStatement, module::ModuleStatement};

// Empty type essentially representing an orphaned/trailing semicolon
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nop;

impl From<Nop> for FunctionStatement {
    fn from(_: Nop) -> Self {
        FunctionStatement::Nop
    }
}

impl From<Nop> for ClassStatement {
    fn from(_: Nop) -> Self {
        ClassStatement::Nop
    }
}

impl From<Nop> for ModuleStatement {
    fn from(_: Nop) -> Self {
        ModuleStatement::Nop
    }
}

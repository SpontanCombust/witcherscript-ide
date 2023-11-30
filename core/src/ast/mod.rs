use std::fmt::Debug;
use crate::{SyntaxNode, NamedSyntaxNode, Script};


mod expressions;
mod functions;
mod classes;
mod loops;
mod conditionals;
mod vars;
mod structs;
mod enums;
mod states;
mod nop;
mod visitor;

pub use expressions::*;
pub use functions::*;
pub use classes::*;
pub use loops::*;
pub use conditionals::*;
pub use vars::*;
pub use structs::*;
pub use enums::*;
pub use states::*;
pub use nop::*;
pub use visitor::*;




#[derive(Debug, Clone)]
pub enum ScriptStatement<'script> {
    Function(SyntaxNode<'script, GlobalFunctionDeclaration>),
    Class(SyntaxNode<'script, ClassDeclaration>),
    State(SyntaxNode<'script, StateDeclaration>),
    Struct(SyntaxNode<'script, StructDeclaration>),
    Enum(SyntaxNode<'script, EnumDeclaration>),
    Nop
}

impl SyntaxNode<'_, ScriptStatement<'_>> {
    pub fn value(&self) -> ScriptStatement {
        let s = self.tree_node.kind();
        match s {
            GlobalFunctionDeclaration::NODE_NAME => ScriptStatement::Function(self.clone().into()),
            ClassDeclaration::NODE_NAME => ScriptStatement::Class(self.clone().into()),
            StateDeclaration::NODE_NAME => ScriptStatement::State(self.clone().into()),
            StructDeclaration::NODE_NAME => ScriptStatement::Struct(self.clone().into()),
            EnumDeclaration::NODE_NAME => ScriptStatement::Enum(self.clone().into()),
            Nop::NODE_NAME => ScriptStatement::Nop,
            _ => panic!("Unknown script statement: {}", s)
        }
    }
}

impl Debug for SyntaxNode<'_, ScriptStatement<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}

impl StatementTraversal for SyntaxNode<'_, ScriptStatement<'_>> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.value() {
            ScriptStatement::Function(s) => s.accept(visitor),
            ScriptStatement::Class(s) => s.accept(visitor),
            ScriptStatement::State(s) => s.accept(visitor),
            ScriptStatement::Struct(s) => s.accept(visitor),
            ScriptStatement::Enum(s) => s.accept(visitor),
            ScriptStatement::Nop => visitor.visit_nop_stmt(),
        }
    }
}


impl NamedSyntaxNode for Script {
    const NODE_NAME: &'static str = "script";
}

impl SyntaxNode<'_, Script> {
    pub fn statements(&self) -> impl Iterator<Item = SyntaxNode<'_, ScriptStatement>> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, Script> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Script{:?}", &self.statements().collect::<Vec<_>>())
    }
}

impl StatementTraversal for SyntaxNode<'_, Script> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        self.statements().for_each(|s| s.accept(visitor));
    }
}
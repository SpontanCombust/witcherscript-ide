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
mod error;
mod unnamed;

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
pub use error::*;
pub use unnamed::*;



#[derive(Debug, Clone)]
pub enum ScriptStatement<'script> {
    Function(GlobalFunctionDeclarationNode<'script>),
    Class(ClassDeclarationNode<'script>),
    State(StateDeclarationNode<'script>),
    Struct(StructDeclarationNode<'script>),
    Enum(EnumDeclarationNode<'script>),
    Nop
}

pub type ScriptStatementNode<'script> = SyntaxNode<'script, ScriptStatement<'script>>;

impl ScriptStatementNode<'_> {
    pub fn value(&self) -> ScriptStatement {
        let s = self.tree_node.kind();
        match s {
            GlobalFunctionDeclarationNode::NODE_KIND => ScriptStatement::Function(self.clone().into()),
            ClassDeclarationNode::NODE_KIND => ScriptStatement::Class(self.clone().into()),
            StateDeclarationNode::NODE_KIND => ScriptStatement::State(self.clone().into()),
            StructDeclarationNode::NODE_KIND => ScriptStatement::Struct(self.clone().into()),
            EnumDeclarationNode::NODE_KIND => ScriptStatement::Enum(self.clone().into()),
            NopNode::NODE_KIND => ScriptStatement::Nop,
            _ => panic!("Unknown script statement: {}", s)
        }
    }
}

impl Debug for ScriptStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}

impl StatementTraversal for ScriptStatementNode<'_> {
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


pub type ScriptNode<'script> = SyntaxNode<'script, Script>;

impl NamedSyntaxNode for ScriptNode<'_> {
    const NODE_KIND: &'static str = "script";
}

impl ScriptNode<'_> {
    pub fn statements(&self) -> impl Iterator<Item = ScriptStatementNode> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for ScriptNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stmts = self.statements().collect::<Vec<_>>();
        if f.alternate() {
            write!(f, "Script{:#?}", stmts)
        } else {
            write!(f, "Script{:?}", stmts)
        }
    }
}

impl StatementTraversal for ScriptNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_script(self) {
            self.statements().for_each(|s| s.accept(visitor));
        }
    }
}
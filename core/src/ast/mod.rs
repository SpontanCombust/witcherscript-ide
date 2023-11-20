use crate::{SyntaxNode, NamedSyntaxNode, Script};
use self::{
    functions::FunctionDeclaration, 
    classes::ClassDeclaration, 
    states::StateDeclaration, 
    structs::StructDeclaration, 
    enums::EnumDeclaration, 
    nop::Nop
};


pub mod expressions;
pub mod functions;
pub mod classes;
pub mod loops;
pub mod conditionals;
pub mod vars;
pub mod structs;
pub mod enums;
pub mod states;
pub mod nop;


#[derive(Debug, Clone)]
pub enum ScriptStatement<'script> {
    Function(SyntaxNode<'script, FunctionDeclaration>),
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
            FunctionDeclaration::NODE_NAME => ScriptStatement::Function(self.clone().into()),
            ClassDeclaration::NODE_NAME => ScriptStatement::Class(self.clone().into()),
            StateDeclaration::NODE_NAME => ScriptStatement::State(self.clone().into()),
            StructDeclaration::NODE_NAME => ScriptStatement::Struct(self.clone().into()),
            EnumDeclaration::NODE_NAME => ScriptStatement::Enum(self.clone().into()),
            Nop::NODE_NAME => ScriptStatement::Nop,
            _ => panic!("Unknown script statement: {}", s)
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
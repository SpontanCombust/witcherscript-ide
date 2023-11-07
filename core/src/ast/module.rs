use std::path::PathBuf;

use super::{
    functions::FunctionDeclaration,
    enums::EnumDeclaration,
    classes::ClassDeclaration,
    states::StateDeclaration,
    structs::StructDeclaration, 
    span::Spanned
};


#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub path: PathBuf,
    pub body: Spanned<ModuleBody>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleStatement {
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    State(StateDeclaration),
    Struct(StructDeclaration),
    Enum(EnumDeclaration),
    Nop
}

pub type ModuleBody = Vec<Spanned<ModuleStatement>>;
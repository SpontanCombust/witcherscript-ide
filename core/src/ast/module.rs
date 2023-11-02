use std::path::PathBuf;

use super::{
    functions::FunctionDeclaration,
    enums::EnumDeclaration,
    classes::ClassDeclaration,
    states::StateDeclaration,
    structs::StructDeclaration
};


#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub path: PathBuf,
    pub body: ModuleBody
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleStatement {
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    StateDeclaration(StateDeclaration),
    StructDeclaration(StructDeclaration),
    EnumDeclaration(EnumDeclaration),
    Nop
}

pub type ModuleBody = Vec<ModuleStatement>;
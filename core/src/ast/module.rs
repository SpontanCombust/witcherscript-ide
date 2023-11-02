use std::path::PathBuf;

use super::{
    functions::FunctionDeclaration,
    enums::EnumDeclaration,
    classes::ClassDeclaration,
    states::StateDeclaration,
    structs::StructDeclaration
};


pub struct Module {
    pub path: PathBuf,
    pub body: ModuleBody
}

pub enum ModuleStatement {
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    StateDeclaration(StateDeclaration),
    StructDeclaration(StructDeclaration),
    EnumDeclaration(EnumDeclaration)
}

pub type ModuleBody = Vec<ModuleStatement>;
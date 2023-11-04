use bitmask_enum::bitmask;

use super::{vars::*, functions::FunctionDeclaration, identifier::Identifier, structs::*};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessModifier {
    Private,
    Protected,
    Public
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    pub imported: bool,
    pub specifiers: ClassSpecifiers,
    pub name: Identifier,
    pub base_class: Option<Identifier>,
    pub body: ClassBody,
}

#[bitmask(u8)]
pub enum ClassSpecifiers {
    Abstract,
    Statemachine
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassAutobind {
    pub access_modifier: Option<AccessModifier>,
    pub optional: bool,
    pub name: Identifier,
    pub autobind_type: TypeAnnotation,
    pub value: Option<String>, // if None it's a "single"
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassStatement {
    Var(MemberVarDeclaration),
    Default(MemberDefaultValue),
    Hint(MemberHint),
    Autobind(ClassAutobind),
    Method(FunctionDeclaration),
    Nop
}

pub type ClassBody = Vec<ClassStatement>;
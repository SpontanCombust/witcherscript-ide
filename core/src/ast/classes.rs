use bitmask_enum::bitmask;

use super::{vars::*, functions::FunctionDeclaration, identifier::Identifier, structs::*, span::Spanned};

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
    pub name: Spanned<Identifier>,
    pub base_class: Option<Spanned<Identifier>>,
    pub body: Spanned<ClassBody>,
}

#[bitmask(u8)]
pub enum ClassSpecifiers {
    Abstract,
    Statemachine
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassAutobind {
    pub access_modifier: Option<Spanned<AccessModifier>>,
    pub optional: bool,
    pub name: Spanned<Identifier>,
    pub autobind_type: Spanned<TypeAnnotation>,
    pub value: Spanned<ClassAutobindValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassAutobindValue {
    Single,
    Concrete(String)
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

pub type ClassBody = Vec<Spanned<ClassStatement>>;
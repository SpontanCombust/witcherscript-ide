use bitmask_enum::bitmask;

use super::{literal::*, vars::*, functions::FunctionDeclaration, identifier::Identifier};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessModifier {
    Private,
    Protected,
    Public
}

pub struct ClassDeclaration {
    pub imported: bool,
    pub specifiers: ClassSpecifiers,
    pub name: Identifier,
    pub extended_class: Option<TypeAnnotation>,
    pub body: ClassBody,
}

#[bitmask]
pub enum ClassSpecifiers {
    Abstract,
    Statemachine
}

pub struct ClassAutobind {
    pub access_modifier: Option<AccessModifier>,
    pub optional: bool,
    pub name: Identifier,
    pub autobind_type: TypeAnnotation,
    pub value: Option<LiteralOrIdentifier>, // if None it's a "single"
}

pub enum ClassStatement {
    MemberDeclaration(VarDeclaration),
    MemberDefaultValue {
        member: Identifier,
        value: LiteralOrIdentifier
    },
    MemberHint {
        member: Identifier,
        value: LiteralOrIdentifier
    },
    Autobind(ClassAutobind),
    MethodDeclaration(FunctionDeclaration),
    Nop
}

pub type ClassBody = Vec<ClassStatement>;
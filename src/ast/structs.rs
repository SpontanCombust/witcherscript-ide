use bitmask_enum::bitmask;
use super::{vars::VarDeclaration, literal::Literal};


pub struct StructDeclaration {
    pub specifiers: StructSpecifiers,
    pub name: String,
    pub body: StructBody
}

#[bitmask]
pub enum StructSpecifiers {
    Import
}

pub enum StructStatement {
    MemberDeclaration(VarDeclaration),
    MemberDefaultValue {
        member: String,
        value: Literal
    },
}

pub type StructBody = Vec<StructStatement>;

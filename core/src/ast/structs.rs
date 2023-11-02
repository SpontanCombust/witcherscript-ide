use super::{vars::VarDeclaration, literal::*, identifier::Identifier};


#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub imported: bool,
    pub name: Identifier,
    pub body: StructBody
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructStatement {
    MemberDeclaration(VarDeclaration),
    MemberDefaultValue {
        member: Identifier,
        value: LiteralOrIdentifier
    },
}

pub type StructBody = Vec<StructStatement>;

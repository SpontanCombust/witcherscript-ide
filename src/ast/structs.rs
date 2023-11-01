use super::{vars::VarDeclaration, literal::*, identifier::Identifier};


pub struct StructDeclaration {
    pub imported: bool,
    pub name: Identifier,
    pub body: StructBody
}

pub enum StructStatement {
    MemberDeclaration(VarDeclaration),
    MemberDefaultValue {
        member: Identifier,
        value: LiteralOrIdentifier
    },
}

pub type StructBody = Vec<StructStatement>;

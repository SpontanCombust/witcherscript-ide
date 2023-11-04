use super::{vars::*, literal::*, identifier::Identifier};


#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub imported: bool,
    pub name: Identifier,
    pub body: StructBody
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructStatement {
    Var(MemberVarDeclaration),
    Default(MemberDefaultValue),
    Hint(MemberHint),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberDefaultValue {
    pub member: Identifier,
    pub value: LiteralOrIdentifier
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberHint {
    pub member: Identifier,
    pub value: String
}

pub type StructBody = Vec<StructStatement>;

use super::{vars::*, literal::*, identifier::Identifier, span::Spanned};


#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub imported: bool,
    pub name: Spanned<Identifier>,
    pub body: Spanned<StructBody>
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructStatement {
    Var(MemberVarDeclaration),
    Default(MemberDefaultValue),
    Hint(MemberHint),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberDefaultValue {
    pub member: Spanned<Identifier>,
    pub value: Spanned<LiteralOrIdentifier>
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberHint {
    pub member: Spanned<Identifier>,
    pub value: Spanned<String>
}

pub type StructBody = Vec<Spanned<StructStatement>>;

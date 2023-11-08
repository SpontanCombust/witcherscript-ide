use crate::lexing::{Identifier, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub name: Spanned<Identifier>,
    pub body: Spanned<EnumBody>
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclarationValue {
    pub name: Spanned<Identifier>,
    pub int_value: Option<Spanned<i32>>
}

pub type EnumBody = Vec<Spanned<EnumDeclarationValue>>;
use crate::lexing::{Identifier, Spanned};
use super::{classes::AccessModifier, expressions::Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub name: Spanned<Identifier>,
    pub generic_argument: Option<Spanned<Identifier>> // only used for arrays
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclaration {
    pub names: Vec<Spanned<Identifier>>,
    pub var_type: Spanned<TypeAnnotation>,
    pub init_value: Option<Box<Spanned<Expression>>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberVarDeclaration {
    pub imported: bool,
    pub access_modifier: Option<Spanned<AccessModifier>>,
    pub specifiers: Spanned<Vec<Spanned<VarSpecifier>>>,
    pub names: Vec<Spanned<Identifier>>,
    pub var_type: Spanned<TypeAnnotation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarSpecifier {
    Const,
    Editable,
    Inlined,
    Saved,
}
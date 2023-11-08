use crate::lexing::{Identifier, Spanned};
use super::{classes::AccessModifier, expressions::Expression};

use bitmask_enum::bitmask;


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
    pub specifiers: VarSpecifiers,
    pub names: Vec<Spanned<Identifier>>,
    pub var_type: Spanned<TypeAnnotation>,
}

#[bitmask(u8)] //TODO maybe just use a Vec...
pub enum VarSpecifiers {
    Const,
    Editable,
    Inlined,
    Saved,
}
use super::{classes::AccessModifier, identifier::Identifier, expressions::Expression};
use bitmask_enum::bitmask;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAnnotation {
    pub name: Identifier,
    pub generic_argument: Option<Identifier> // only used for arrays
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclaration {
    pub names: Vec<Identifier>,
    pub var_type: TypeAnnotation,
    pub init_value: Option<Box<Expression>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberVarDeclaration {
    pub imported: bool,
    pub access_modifier: Option<AccessModifier>,
    pub specifiers: VarSpecifiers,
    pub names: Vec<Identifier>,
    pub var_type: TypeAnnotation,
}

#[bitmask(u8)]
pub enum VarSpecifiers {
    Const,
    Editable,
    Inlined,
    Saved,
}
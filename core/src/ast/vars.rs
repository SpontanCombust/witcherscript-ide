use super::{classes::AccessModifier, identifier::Identifier};
use bitmask_enum::bitmask;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeAnnotation {
    pub name: Identifier,
    pub generic_argument: Option<Identifier> // only used for arrays
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarDeclaration {
    pub imported: bool,
    pub access_modifier: Option<AccessModifier>,
    pub specifiers: VarSpecifiers,

    pub names: Vec<Identifier>,
    pub var_type: TypeAnnotation
}

#[bitmask(u8)]
pub enum VarSpecifiers {
    Const,
    Editable,
    Inlined,
    Saved,
}
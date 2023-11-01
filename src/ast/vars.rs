use super::classes::AccessModifier;
use bitmask_enum::bitmask;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeAnnotation {
    pub name: String,
    pub generic_argument: Option<String> // only used for arrays
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarDeclaration {
    pub imported: bool,
    pub access_modifier: Option<AccessModifier>,
    pub specifiers: VarSpecifiers,

    pub name: String,
    pub var_type: TypeAnnotation
}

#[bitmask]
pub enum VarSpecifiers {
    Const,
    Editable,
    Inlined,
    Saved,
}
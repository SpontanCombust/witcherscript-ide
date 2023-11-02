use super::identifier::Identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDeclaration {
    pub name: Identifier,
    pub values: Vec<EnumDeclarationValue>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDeclarationValue {
    pub name: Identifier,
    pub int_value: Option<i32>
}
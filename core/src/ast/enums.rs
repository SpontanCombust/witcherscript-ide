use super::identifier::Identifier;

pub struct EnumDeclaration {
    pub name: Identifier,
    pub values: Vec<EnumDeclarationValue>
}

pub struct EnumDeclarationValue {
    pub name: Identifier,
    pub int_value: Option<i32>
}
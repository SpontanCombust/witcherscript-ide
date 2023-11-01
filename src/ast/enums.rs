pub struct EnumDeclaration {
    pub name: String,
    pub values: Vec<EnumDeclarationValue>
}

pub struct EnumDeclarationValue {
    pub name: String,
    pub int_value: Option<i32>
}
use super::{vars::VarDeclaration, literal::Literal};


pub struct StructDeclaration {
    pub imported: bool,
    pub name: String,
    pub body: StructBody
}

pub enum StructStatement {
    MemberDeclaration(VarDeclaration),
    MemberDefaultValue {
        member: String,
        value: Literal
    },
}

pub type StructBody = Vec<StructStatement>;

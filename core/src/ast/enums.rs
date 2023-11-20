use crate::{tokens::{Identifier, LiteralInt}, NamedSyntaxNode, SyntaxNode};


#[derive(Debug, Clone)]
pub struct EnumDeclaration;

impl NamedSyntaxNode for EnumDeclaration {
    const NODE_NAME: &'static str = "enum_decl_stmt";
}

impl SyntaxNode<'_, EnumDeclaration> {
    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn definition(&self) -> SyntaxNode<'_, EnumBlock> {
        self.field_child("definition").unwrap().into()
    }
}



#[derive(Debug, Clone)]
pub struct EnumBlock;

impl NamedSyntaxNode for EnumBlock {
    const NODE_NAME: &'static str = "enum_block";
}

impl SyntaxNode<'_, EnumBlock> {
    pub fn values(&self) -> impl Iterator<Item = SyntaxNode<'_, EnumDeclarationValue>> {
        self.children(true).map(|n| n.into())
    }
}



#[derive(Debug, Clone)]
pub struct EnumDeclarationValue;

impl NamedSyntaxNode for EnumDeclarationValue {
    const NODE_NAME: &'static str = "enum_decl_value";
}

impl SyntaxNode<'_, EnumDeclarationValue> {
    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, LiteralInt> {
        self.field_child("value").unwrap().into()
    }
}

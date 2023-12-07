use std::fmt::Debug;
use crate::{tokens::{Identifier, LiteralInt}, NamedSyntaxNode, SyntaxNode};
use super::{StatementTraversal, StatementVisitor};


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

impl Debug for SyntaxNode<'_, EnumDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumDeclaration")
            .field("name", &self.name())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, EnumDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_enum_decl(self) {
            self.definition().members().for_each(|s| s.accept(visitor));
        }
        visitor.exit_enum_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct EnumBlock;

impl NamedSyntaxNode for EnumBlock {
    const NODE_NAME: &'static str = "enum_block";
}

impl SyntaxNode<'_, EnumBlock> {
    pub fn members(&self) -> impl Iterator<Item = SyntaxNode<'_, EnumMemberDeclaration>> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, EnumBlock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnumBlock{:?}", self.members().collect::<Vec<_>>())
    }
}



#[derive(Debug, Clone)]
pub struct EnumMemberDeclaration;

impl NamedSyntaxNode for EnumMemberDeclaration {
    const NODE_NAME: &'static str = "enum_decl_value";
}

impl SyntaxNode<'_, EnumMemberDeclaration> {
    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn value(&self) -> Option<SyntaxNode<'_, LiteralInt>> {
        self.field_child("value").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, EnumMemberDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumMemberDeclaration")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, EnumMemberDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_enum_member_decl(self);
    }
}
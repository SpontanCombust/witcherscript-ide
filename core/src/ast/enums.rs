use std::fmt::Debug;
use crate::{tokens::{IdentifierNode, LiteralIntNode}, NamedSyntaxNode, SyntaxNode};
use super::{StatementTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct EnumDeclaration;

pub type EnumDeclarationNode<'script> = SyntaxNode<'script, EnumDeclaration>;

impl NamedSyntaxNode for EnumDeclarationNode<'_> {
    const NODE_NAME: &'static str = "enum_decl_stmt";
}

impl EnumDeclarationNode<'_> {
    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn definition(&self) -> EnumBlockNode {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for EnumDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumDeclaration")
            .field("name", &self.name())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for EnumDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_enum_decl(self) {
            self.definition().members().for_each(|s| s.accept(visitor));
        }
        visitor.exit_enum_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct EnumBlock;

pub type EnumBlockNode<'script> = SyntaxNode<'script, EnumBlock>;

impl NamedSyntaxNode for EnumBlockNode<'_> {
    const NODE_NAME: &'static str = "enum_block";
}

impl EnumBlockNode<'_> {
    pub fn members(&self) -> impl Iterator<Item = EnumMemberDeclarationNode> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for EnumBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let members = self.members().collect::<Vec<_>>();
        if f.alternate() {
            write!(f, "EnumBlock{:#?}", members)
        } else {
            write!(f, "EnumBlock{:?}", members)
        }
    }
}



#[derive(Debug, Clone)]
pub struct EnumMemberDeclaration;

pub type EnumMemberDeclarationNode<'script> = SyntaxNode<'script, EnumMemberDeclaration>;

impl NamedSyntaxNode for EnumMemberDeclarationNode<'_> {
    const NODE_NAME: &'static str = "enum_decl_value";
}

impl EnumMemberDeclarationNode<'_> {
    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn value(&self) -> Option<LiteralIntNode> {
        self.field_child("value").map(|n| n.into())
    }
}

impl Debug for EnumMemberDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumMemberDeclaration")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl StatementTraversal for EnumMemberDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_enum_member_decl(self);
    }
}
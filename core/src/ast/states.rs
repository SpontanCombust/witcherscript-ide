use std::fmt::Debug;
use crate::{tokens::Identifier, NamedSyntaxNode, SyntaxNode, attribs::StateSpecifier};
use super::{ClassBlock, StatementTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct StateDeclaration;

impl NamedSyntaxNode for StateDeclaration {
    const NODE_NAME: &'static str = "state_decl_stmt";
}

impl SyntaxNode<'_, StateDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, StateSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn parent(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("parent").unwrap().into()
    }

    pub fn base(&self) -> Option<SyntaxNode<'_, Identifier>> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> SyntaxNode<'_, ClassBlock> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, StateDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("parent", &self.parent())
            .field("base", &self.base())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, StateDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_state_decl(self);
        if visitor.should_visit_inner() {
            self.definition().statements().for_each(|s| s.accept(visitor));
        }
    }
}
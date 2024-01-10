use std::fmt::Debug;
use crate::{tokens::IdentifierNode, NamedSyntaxNode, SyntaxNode, attribs::StateSpecifierNode, AnyNode};
use super::{StatementTraversal, StatementVisitor, ClassBlockNode};


#[derive(Debug, Clone)]
pub struct StateDeclaration;

pub type StateDeclarationNode<'script> = SyntaxNode<'script, StateDeclaration>;

impl NamedSyntaxNode for StateDeclarationNode<'_> {
    const NODE_KIND: &'static str = "state_decl_stmt";
}

impl StateDeclarationNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = StateSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn parent(&self) -> IdentifierNode {
        self.field_child("parent").unwrap().into()
    }

    pub fn base(&self) -> Option<IdentifierNode> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> ClassBlockNode {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for StateDeclarationNode<'_> {
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

impl<'script> TryFrom<AnyNode<'script>> for StateDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for StateDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_state_decl(self) {
            self.definition().statements().for_each(|s| s.accept(visitor));
        }
        visitor.exit_state_decl(self);
    }
}
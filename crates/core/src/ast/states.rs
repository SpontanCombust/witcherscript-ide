use std::fmt::Debug;
use crate::{attribs::SpecifierNode, tokens::IdentifierNode, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct StateDeclaration;
    pub struct StateBlock;
}


pub type StateDeclarationNode<'script> = SyntaxNode<'script, tags::StateDeclaration>;

impl NamedSyntaxNode for StateDeclarationNode<'_> {
    const NODE_KIND: &'static str = "state_decl";
}

impl<'script> StateDeclarationNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn parent(&self) -> IdentifierNode<'script> {
        self.field_child("parent").unwrap().into()
    }

    pub fn base(&self) -> Option<IdentifierNode<'script>> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> StateBlockNode<'script> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for StateDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("StateDeclaration {}", self.range().debug()))
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

impl SyntaxNodeTraversal for StateDeclarationNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        let tp = visitor.visit_state_decl(self);
        if tp.traverse_definition {
            self.definition().accept(visitor, ());
        }
        visitor.exit_state_decl(self);
    }
}


pub type StateBlockNode<'script> = SyntaxNode<'script, tags::StateBlock>;

impl NamedSyntaxNode for StateBlockNode<'_> {
    const NODE_KIND: &'static str = "state_def";
}

impl<'script> StateBlockNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = ClassPropertyNode<'script>> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for StateBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("StateBlock {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StateBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for StateBlockNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        self.iter().for_each(|s| s.accept(visitor, DeclarationTraversalContext::StateDefinition));
    }
}
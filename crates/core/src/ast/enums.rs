use std::fmt::Debug;
use crate::{tokens::{IdentifierNode, LiteralHexNode, LiteralIntNode}, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct EnumDeclaration;
    pub struct EnumBlock;
    pub struct EnumVariantDeclaration;
}


pub type EnumDeclarationNode<'script> = SyntaxNode<'script, tags::EnumDeclaration>;

impl NamedSyntaxNode for EnumDeclarationNode<'_> {
    const NODE_KIND: &'static str = "enum_decl_stmt";
}

impl<'script> EnumDeclarationNode<'script> {
    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn definition(&self) -> EnumBlockNode<'script> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for EnumDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("EnumDeclaration {}", self.range().debug()))
            .field("name", &self.name())
            .field("definition", &self.definition())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for EnumDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for EnumDeclarationNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        let tp = visitor.visit_enum_decl(self);
        if tp.traverse_definition {
            self.definition().accept(visitor, ());
        }
        visitor.exit_enum_decl(self);
    }
}



pub type EnumBlockNode<'script> = SyntaxNode<'script, tags::EnumBlock>;

impl NamedSyntaxNode for EnumBlockNode<'_> {
    const NODE_KIND: &'static str = "enum_block";
}

impl<'script> EnumBlockNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = EnumVariantDeclarationNode<'script>> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for EnumBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("EnumBlock {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for EnumBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for EnumBlockNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        self.iter().for_each(|s| s.accept(visitor, ()));
    }
}


pub type EnumVariantDeclarationNode<'script> = SyntaxNode<'script, tags::EnumVariantDeclaration>;

impl NamedSyntaxNode for EnumVariantDeclarationNode<'_> {
    const NODE_KIND: &'static str = "enum_decl_variant";
}

impl<'script> EnumVariantDeclarationNode<'script> {
    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn value(&self) -> Option<EnumVariantValue<'script>> {
        self.field_child("value").map(|n| {
            let kind = n.tree_node.kind();
            match kind {
                LiteralIntNode::NODE_KIND => EnumVariantValue::Int(n.into()),
                LiteralHexNode::NODE_KIND => EnumVariantValue::Hex(n.into()),
                _ => panic!("Unknown enum variant value kind: {} {}", kind, self.range().debug())
            }
        })
    }
}

impl Debug for EnumVariantDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("EnumVariantDeclaration {}", self.range().debug()))
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for EnumVariantDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for EnumVariantDeclarationNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        visitor.visit_enum_variant_decl(self);
    }
}


#[derive(Clone)]
pub enum EnumVariantValue<'script> {
    Int(LiteralIntNode<'script>),
    Hex(LiteralHexNode<'script>)
}

impl Debug for EnumVariantValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => f.debug_maybe_alternate(n),
            Self::Hex(n) => f.debug_maybe_alternate(n)
        }
    }
}
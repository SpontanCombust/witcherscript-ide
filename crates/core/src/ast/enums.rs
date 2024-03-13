use std::fmt::Debug;
use crate::{tokens::{IdentifierNode, LiteralHexNode, LiteralIntNode}, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::{StatementTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct EnumDeclaration;

pub type EnumDeclarationNode<'script> = SyntaxNode<'script, EnumDeclaration>;

impl NamedSyntaxNode for EnumDeclarationNode<'_> {
    const NODE_KIND: &'static str = "enum_decl_stmt";
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

impl StatementTraversal for EnumDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_enum_decl(self) {
            self.definition().accept(visitor);
        }
        visitor.exit_enum_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct EnumBlock;

pub type EnumBlockNode<'script> = SyntaxNode<'script, EnumBlock>;

impl NamedSyntaxNode for EnumBlockNode<'_> {
    const NODE_KIND: &'static str = "enum_block";
}

impl EnumBlockNode<'_> {
    pub fn variants(&self) -> impl Iterator<Item = EnumVariantDeclarationNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for EnumBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("EnumBlock {}", self.range().debug()), 
            &self.variants().collect::<Vec<_>>()
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

impl StatementTraversal for EnumBlockNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        self.variants().for_each(|s| s.accept(visitor));
    }
}


#[derive(Debug, Clone)]
pub struct EnumVariantDeclaration;

pub type EnumVariantDeclarationNode<'script> = SyntaxNode<'script, EnumVariantDeclaration>;

impl NamedSyntaxNode for EnumVariantDeclarationNode<'_> {
    const NODE_KIND: &'static str = "enum_decl_variant";
}

impl EnumVariantDeclarationNode<'_> {
    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn value(&self) -> Option<EnumVariantValue> {
        self.field_child("value").map(|n| {
            let kind = n.tree_node.kind();
            match kind {
                LiteralIntNode::NODE_KIND => EnumVariantValue::Int(n.into()),
                LiteralHexNode::NODE_KIND => EnumVariantValue::Hex(n.into()),
                _ => panic!("Unknown enum variant value kind: {}", kind)
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

impl StatementTraversal for EnumVariantDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
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
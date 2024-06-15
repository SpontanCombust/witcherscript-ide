use std::fmt::Debug;
use crate::{attribs::*, tokens::*, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct StructDeclaration;
    pub struct StructBlock;
    pub struct MemberDefaultsBlock;
    pub struct MemberDefaultsBlockAssignment;
    pub struct MemberDefaultValue; 
    pub struct MemberHint; 
}


pub type StructDeclarationNode<'script> = SyntaxNode<'script, tags::StructDeclaration>;

impl NamedSyntaxNode for StructDeclarationNode<'_> {
    const NODE_KIND: &'static str = "struct_decl";
}

impl<'script> StructDeclarationNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn definition(&self) -> StructBlockNode<'script> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for StructDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("StructDeclaration {}", self.range().debug()))
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("definition", &self.definition())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StructDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for StructDeclarationNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        let tp = visitor.visit_struct_decl(self);
        if tp.traverse_definition {
            self.definition().accept(visitor, ());
        }
        visitor.exit_struct_decl(self);
    }
}



pub type StructBlockNode<'script> = SyntaxNode<'script, tags::StructBlock>;

impl NamedSyntaxNode for StructBlockNode<'_> {
    const NODE_KIND: &'static str = "struct_def";
}

impl<'script> StructBlockNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = StructPropertyNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for StructBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("StructBlock {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StructBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for StructBlockNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        self.iter().for_each(|s| s.accept(visitor, DeclarationTraversalContext::StructDefinition));
    }
}



#[derive(Clone)]
pub enum StructProperty<'script> {
    Var(MemberVarDeclarationNode<'script>),
    Default(MemberDefaultValueNode<'script>),
    DefaultsBlock(MemberDefaultsBlockNode<'script>),
    Hint(MemberHintNode<'script>),
    Nop(NopNode<'script>)
}

impl Debug for StructProperty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(n) => f.debug_maybe_alternate(n),
            Self::Default(n) => f.debug_maybe_alternate(n),
            Self::DefaultsBlock(n) => f.debug_maybe_alternate(n),
            Self::Hint(n) => f.debug_maybe_alternate(n),
            Self::Nop(n) => f.debug_maybe_alternate(n),
        }
    }
}

pub type StructPropertyNode<'script> = SyntaxNode<'script, StructProperty<'script>>;

impl<'script> StructPropertyNode<'script> {
    pub fn value(self) -> StructProperty<'script> {
        match self.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND => StructProperty::Var(self.into()),
            MemberDefaultValueNode::NODE_KIND => StructProperty::Default(self.into()),
            MemberDefaultsBlockNode::NODE_KIND => StructProperty::DefaultsBlock(self.into()),
            MemberHintNode::NODE_KIND => StructProperty::Hint(self.into()),
            NopNode::NODE_KIND => StructProperty::Nop(self.into()),
            _ => panic!("Unknown struct property type: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for StructPropertyNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StructPropertyNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }
        
        match value.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND     |
            MemberDefaultValueNode::NODE_KIND       |
            MemberDefaultsBlockNode::NODE_KIND      |
            MemberHintNode::NODE_KIND               |
            NopNode::NODE_KIND                      => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for StructPropertyNode<'_> {
    type TraversalCtx = DeclarationTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        match self.clone().value() {
            StructProperty::Var(s) => s.accept(visitor, ctx),
            StructProperty::Default(s) => s.accept(visitor, ctx),
            StructProperty::DefaultsBlock(s) => s.accept(visitor, ctx),
            StructProperty::Hint(s) => s.accept(visitor, ctx),
            StructProperty::Nop(_) => {},
        }
    }
}



pub type MemberDefaultsBlockNode<'script> = SyntaxNode<'script, tags::MemberDefaultsBlock>;

impl NamedSyntaxNode for MemberDefaultsBlockNode<'_> {
    const NODE_KIND: &'static str = "member_default_val_block";
}

impl<'script> MemberDefaultsBlockNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = MemberDefaultsBlockAssignmentNode<'script>> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for MemberDefaultsBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("MemberDefaultsBlock {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberDefaultsBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for MemberDefaultsBlockNode<'_> {
    type TraversalCtx = DeclarationTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_member_defaults_block(self, ctx);
        if tp.traverse {
            self.iter().for_each(|n| n.accept(visitor, ()));
        }
        visitor.exit_member_defaults_block(self, ctx);
    }
}



pub type MemberDefaultsBlockAssignmentNode<'script> = SyntaxNode<'script, tags::MemberDefaultsBlockAssignment>;

impl NamedSyntaxNode for MemberDefaultsBlockAssignmentNode<'_> {
    const NODE_KIND: &'static str = "member_default_val_block_assign";
}

impl<'script> MemberDefaultsBlockAssignmentNode<'script> {
    pub fn member(&self) -> IdentifierNode<'script> {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> ExpressionNode<'script> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for MemberDefaultsBlockAssignmentNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("MemberDefaultsBlockAssignment {}", self.range().debug()))
            .field("member", &self.member())
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberDefaultsBlockAssignmentNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for MemberDefaultsBlockAssignmentNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        let tp = visitor.visit_member_defaults_block_assignment(self);
        if tp.traverse_value {
            self.value().accept(visitor, ExpressionTraversalContext::MemberDefaultValue);
        }
        visitor.exit_member_defaults_block_assignment(self);
    }
}



pub type MemberDefaultValueNode<'script> = SyntaxNode<'script, tags::MemberDefaultValue>;

impl NamedSyntaxNode for MemberDefaultValueNode<'_> {
    const NODE_KIND: &'static str = "member_default_val";
}

impl<'script> MemberDefaultValueNode<'script> {
    pub fn member(&self) -> IdentifierNode<'script> {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> ExpressionNode<'script> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for MemberDefaultValueNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("MemberDefaultValue {}", self.range().debug()))
            .field("member", &self.member())
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberDefaultValueNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for MemberDefaultValueNode<'_> {
    type TraversalCtx = DeclarationTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_member_default_val(self, ctx);
        if tp.traverse_value {
            self.value().accept(visitor, ExpressionTraversalContext::MemberDefaultValue);
        }
        visitor.exit_member_default_val(self, ctx);
    }
}



pub type MemberHintNode<'script> = SyntaxNode<'script, tags::MemberHint>;

impl NamedSyntaxNode for MemberHintNode<'_> {
    const NODE_KIND: &'static str = "member_hint";
}

impl<'script> MemberHintNode<'script> {
    pub fn member(&self) -> IdentifierNode<'script> {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> LiteralStringNode<'script> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for MemberHintNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("MemberHint {}", self.range().debug()))
            .field("member", &self.member())
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberHintNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for MemberHintNode<'_> {
    type TraversalCtx = DeclarationTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        visitor.visit_member_hint(self, ctx);
    }
}

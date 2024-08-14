use crate::{tokens::*, debug::*, AnyNode, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct MemberDefaultsBlock;
    pub struct MemberDefaultsBlockAssignment;
    pub struct MemberDefaultValue; 
    pub struct MemberHint; 
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

impl std::fmt::Debug for MemberDefaultsBlockNode<'_> {
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
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_member_defaults_block(self, ctx);

        if tp.any() {
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((assign, _)) if tp.traverse_assignments => {
                        let assign: MemberDefaultsBlockAssignmentNode = assign.into();

                        assign.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
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

impl std::fmt::Debug for MemberDefaultsBlockAssignmentNode<'_> {
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
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_member_defaults_block_assignment(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::MemberDefaultValue);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((value, Some("value"))) if tp.traverse_value => {
                        let value: ExpressionNode = value.into();

                        value.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();  
        }

        visitor.exit_member_defaults_block_assignment(self, ctx);
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

impl std::fmt::Debug for MemberDefaultValueNode<'_> {
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
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_member_default_val(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::MemberDefaultValue);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((value, Some("value"))) if tp.traverse_value => {
                        let value: ExpressionNode = value.into();

                        value.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
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

impl std::fmt::Debug for MemberHintNode<'_> {
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
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_member_hint(self, ctx);

        if tp.any() {
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_member_hint(self, ctx);
    }
}

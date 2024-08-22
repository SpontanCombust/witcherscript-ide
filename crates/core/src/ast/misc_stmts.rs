use crate::{debug::*, tokens::UnnamedNode, AnyNode, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct BreakStatement;
    pub struct ContinueStatement;
    pub struct ReturnStatement;
    pub struct DeleteStatement;
    pub struct CompoundStatement;
}


pub type BreakStatementNode<'script> = SyntaxNode<'script, tags::BreakStatement>;

impl NamedSyntaxNode for BreakStatementNode<'_> {
    const NODE_KIND: &'static str = "break_stmt";
}

impl BreakStatementNode<'_> {}

impl std::fmt::Debug for BreakStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BreakStatement {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for BreakStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for BreakStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_break_stmt(self, ctx);

        if tp.any() {
            for ch in self.children_detailed() {
                match ch {
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_break_stmt(self, ctx);
    }
}



pub type ContinueStatementNode<'script> = SyntaxNode<'script, tags::ContinueStatement>;

impl NamedSyntaxNode for ContinueStatementNode<'_> {
    const NODE_KIND: &'static str = "continue_stmt";
}

impl ContinueStatementNode<'_> {}

impl std::fmt::Debug for ContinueStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContinueStatement {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ContinueStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ContinueStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_continue_stmt(self, ctx);

        if tp.any() {
            for ch in self.children_detailed() {
                match ch {
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_continue_stmt(self, ctx);
    }
}



pub type ReturnStatementNode<'script> = SyntaxNode<'script, tags::ReturnStatement>;

impl NamedSyntaxNode for ReturnStatementNode<'_> {
    const NODE_KIND: &'static str = "return_stmt";
}

impl<'script> ReturnStatementNode<'script> {
    pub fn value(&self) -> Option<ExpressionNode<'script>> {
        self.first_child(true).map(|n| n.unsafe_into())
    }
}

impl std::fmt::Debug for ReturnStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("ReturnStatement {}", self.range().debug()))
            .field(&self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ReturnStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ReturnStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_return_stmt(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::ReturnStatement);

            for ch in self.children_detailed() {
                match ch {
                    Ok((value, _)) if value.is_named() && tp.traverse_value => {
                        let value: ExpressionNode = value.unsafe_into();
                        value.accept(visitor, ctx);
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_return_stmt(self, ctx);
    }
}



pub type DeleteStatementNode<'script> = SyntaxNode<'script, tags::DeleteStatement>;

impl NamedSyntaxNode for DeleteStatementNode<'_> {
    const NODE_KIND: &'static str = "delete_stmt";
}

impl<'script> DeleteStatementNode<'script> {
    pub fn value(&self) -> ExpressionNode<'script> {
        self.first_child(true).unwrap().unsafe_into()
    }
}

impl std::fmt::Debug for DeleteStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("DeleteStatement {}", self.range().debug()))
            .field(&self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for DeleteStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for DeleteStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_delete_stmt(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::DeleteStatement);

            for ch in self.children_detailed() {
                match ch {
                    Ok((value, _)) if value.is_named() && tp.traverse_value => {
                        let value: ExpressionNode = value.unsafe_into();
                        value.accept(visitor, ctx);
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_delete_stmt(self, ctx);
    }
}



pub type CompoundStatementNode<'script> = SyntaxNode<'script, tags::CompoundStatement>;

impl NamedSyntaxNode for CompoundStatementNode<'_> {
    const NODE_KIND: &'static str = "compound_stmt";
}

impl<'script> CompoundStatementNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = FunctionStatementNode<'script>> {
        self.named_children().map(|n| n.unsafe_into())
    }
}

impl std::fmt::Debug for CompoundStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("CompoundStatement {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for CompoundStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for CompoundStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_compound_stmt(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::CompoundStatement);

            for ch in self.children_detailed() {
                match ch {
                    Ok((stmt, _)) if stmt.is_named() && tp.traverse_statements => {
                        let stmt: FunctionStatementNode = stmt.unsafe_into();
                        stmt.accept(visitor, ctx);
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_compound_stmt(self, ctx);
    }
}
use std::fmt::Debug;
use crate::{AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct ForLoop;
    pub struct WhileLoop;
    pub struct DoWhileLoop;
}


pub type ForLoopNode<'script> = SyntaxNode<'script, tags::ForLoop>;

impl NamedSyntaxNode for ForLoopNode<'_> {
    const NODE_KIND: &'static str = "for_stmt";
}

impl<'script> ForLoopNode<'script> {
    pub fn init(&self) -> Option<ExpressionNode<'script>> {
        self.field_child("init").map(|n| n.unsafe_into())
    }

    pub fn cond(&self) -> Option<ExpressionNode<'script>> {
        self.field_child("cond").map(|n| n.unsafe_into())
    }

    pub fn iter(&self) -> Option<ExpressionNode<'script>> {
        self.field_child("iter").map(|n| n.unsafe_into())
    }

    pub fn body(&self) -> FunctionStatementNode<'script> {
        self.field_child("body").unwrap().unsafe_into()
    }
}

impl Debug for ForLoopNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("ForLoop {}", self.range().debug()))
            .field("init", &self.init())
            .field("cond", &self.cond())
            .field("iter", &self.iter())
            .field("body", &self.body())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ForLoopNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ForLoopNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_for_stmt(self, ctx);

        if tp.any() {
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((init, Some("init"))) if tp.traverse_init => {
                        let init: ExpressionNode = init.unsafe_into();

                        ctx.push(TraversalContext::ForLoopInit);
                        init.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((cond, Some("cond"))) if tp.traverse_cond => {
                        let cond: ExpressionNode = cond.unsafe_into();

                        ctx.push(TraversalContext::ForLoopCond);
                        cond.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((iter, Some("iter"))) if tp.traverse_iter => {
                        let iter: ExpressionNode = iter.unsafe_into();

                        ctx.push(TraversalContext::ForLoopIter);
                        iter.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((body, Some("body"))) if tp.traverse_body => {
                        let body: FunctionStatementNode = body.unsafe_into();

                        ctx.push(TraversalContext::ForLoopBody);
                        body.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_for_stmt(self, ctx);
    }
}



pub type WhileLoopNode<'script> = SyntaxNode<'script, tags::WhileLoop>;

impl NamedSyntaxNode for WhileLoopNode<'_> {
    const NODE_KIND: &'static str = "while_stmt";
}

impl<'script> WhileLoopNode<'script> {
    pub fn cond(&self) -> ExpressionNode<'script> {
        self.field_child("cond").unwrap().unsafe_into()
    }

    pub fn body(&self) -> FunctionStatementNode<'script> {
        self.field_child("body").unwrap().unsafe_into()
    }
}

impl Debug for WhileLoopNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("WhileLoop {}", self.range().debug()))
            .field("cond", &self.cond())
            .field("body", &self.body())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for WhileLoopNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for WhileLoopNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_while_stmt(self, ctx);

        if tp.any() {
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((cond, Some("cond"))) if tp.traverse_cond => {
                        let cond: ExpressionNode = cond.unsafe_into();

                        ctx.push(TraversalContext::WhileLoopCond);
                        cond.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((body, Some("body"))) if tp.traverse_body => {
                        let body: FunctionStatementNode = body.unsafe_into();

                        ctx.push(TraversalContext::WhileLoopBody);
                        body.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_while_stmt(self, ctx);
    }
}



pub type DoWhileLoopNode<'script> = SyntaxNode<'script, tags::DoWhileLoop>;

impl NamedSyntaxNode for DoWhileLoopNode<'_> {
    const NODE_KIND: &'static str = "do_while_stmt";
}

impl<'script> DoWhileLoopNode<'script> {
    pub fn cond(&self) -> ExpressionNode<'script> {
        self.field_child("cond").unwrap().unsafe_into()
    }

    pub fn body(&self) -> FunctionStatementNode<'script> {
        self.field_child("body").unwrap().unsafe_into()
    }
}

impl Debug for DoWhileLoopNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("DoWhileLoop {}", self.range().debug()))
            .field("cond", &self.cond())
            .field("body", &self.body())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for DoWhileLoopNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for DoWhileLoopNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_do_while_stmt(self, ctx);

        if tp.any() {
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((cond, Some("cond"))) if tp.traverse_cond => {
                        let cond: ExpressionNode = cond.unsafe_into();

                        ctx.push(TraversalContext::DoWhileLoopCond);
                        cond.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((body, Some("body"))) if tp.traverse_body => {
                        let body: FunctionStatementNode = body.unsafe_into();

                        ctx.push(TraversalContext::DoWhileLoopBody);
                        body.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_do_while_stmt(self, ctx);
    }
}
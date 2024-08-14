use crate::{ast::{SyntaxNodeTraversal, SyntaxNodeVisitor, TraversalContext, TraversalContextStack}, AnyNode, NamedSyntaxNode, SyntaxNode};


mod tags {
    pub struct Error;
}

#[derive(Debug, Clone)]
pub enum SyntaxError<'script> {
    /// Corresponds to a named or unnamed leaf node that was inserted by tree-sitter to recover from syntax error.
    Missing(AnyNode<'script>),
    /// Corresponds to a parent node of at least one node that could not fit into the syntax.
    Invalid(ErrorNode<'script>)
}


pub type ErrorNode<'script> = SyntaxNode<'script, tags::Error>;

impl NamedSyntaxNode for ErrorNode<'_> {
    const NODE_KIND: &'static str = "ERROR";
}

impl std::fmt::Debug for ErrorNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorNode").finish()
    }
}

impl<'script> SyntaxNodeTraversal for ErrorNode<'script> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_error(self, ctx);
        ctx.push(TraversalContext::Error);
        if tp.traverse {
            for res in self.children_detailed() {
                match res {
                    Ok((n, _)) => n.accept(visitor, ctx),
                    Err(e) => e.accept(visitor, ctx)
                }
            }
        }
        ctx.pop();
        visitor.exit_error(self, ctx);
    }
}
mod policies;
mod visitor;
mod contexts;

pub use policies::*;
pub use visitor::*;
pub use contexts::*;


/// Traverse an syntax node using left-recursion.
pub trait SyntaxNodeTraversal {
    type TraversalCtx;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx);
}

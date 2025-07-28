mod policies;
mod visitor;
mod contexts;
mod visitor_chain;

pub use policies::*;
pub use visitor::*;
pub use contexts::*;
pub use visitor_chain::*;


/// Traverse an syntax node using left-recursion.
pub trait SyntaxNodeTraversal {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack);
}

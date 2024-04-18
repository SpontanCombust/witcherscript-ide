mod policies;
mod visitor;
mod visitor_decorators;

pub use policies::*;
pub use visitor::*;
pub use visitor_decorators::*;


/// Traverse an expression node using left-recursion.
pub trait SyntaxTraversal {
    fn accept<V: SyntaxVisitor>(&self, visitor: &mut V);
}


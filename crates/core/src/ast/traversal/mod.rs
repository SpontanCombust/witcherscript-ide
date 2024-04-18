mod policies;
mod visitor;

pub use policies::*;
pub use visitor::*;


/// Traverse an expression node using left-recursion.
pub trait SyntaxTraversal {
    fn accept<V: SyntaxVisitor>(&self, visitor: &mut V);
}


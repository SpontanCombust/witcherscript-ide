mod policies;
mod visitor;

pub use policies::*;
pub use visitor::*;


/// Traverse an expression node using left-recursion.
pub trait ExpressionTraversal {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V);
}

/// Traverse a statement node using left-recursion.
pub trait DeclarationTraversal {
    fn accept<V: DeclarationVisitor>(&self, visitor: &mut V);
}

/// Traverse a statement node using left-recursion.
pub trait StatementTraversal {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V);
}

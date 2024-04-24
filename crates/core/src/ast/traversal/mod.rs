mod policies;
mod visitor;
mod contexts;

pub use policies::*;
pub use visitor::*;
pub use contexts::*;


/// Traverse an expression node using left-recursion.
pub trait ExpressionTraversal {
    type TraversalCtx;

    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx);
}

/// Traverse a statement node using left-recursion.
pub trait DeclarationTraversal {
    type TraversalCtx;

    fn accept<V: DeclarationVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx);
}

/// Traverse a statement node using left-recursion.
pub trait StatementTraversal {
    type TraversalCtx;

    fn accept<V: StatementVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx);
}

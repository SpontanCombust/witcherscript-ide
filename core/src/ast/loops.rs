use std::fmt::Debug;
use crate::{NamedSyntaxNode, SyntaxNode};
use super::{Expression, FunctionStatement, StatementTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct ForLoop;

impl NamedSyntaxNode for ForLoop {
    const NODE_NAME: &'static str = "for_stmt";
}

impl SyntaxNode<'_, ForLoop> {
    pub fn init(&self) -> Option<SyntaxNode<'_, Expression>> {
        self.field_child("init").map(|n| n.into())
    }

    pub fn cond(&self) -> Option<SyntaxNode<'_, Expression>> {
        self.field_child("cond").map(|n| n.into())
    }

    pub fn iter(&self) -> Option<SyntaxNode<'_, Expression>> {
        self.field_child("iter").map(|n| n.into())
    }

    pub fn body(&self) -> SyntaxNode<'_, FunctionStatement> {
        self.field_child("body").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, ForLoop> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForLoop")
            .field("init", &self.init())
            .field("cond", &self.cond())
            .field("iter", &self.iter())
            .field("body", &self.body())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, ForLoop> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_for_stmt(self);
        self.body().accept(visitor);
    }
}



#[derive(Debug, Clone)]
pub struct WhileLoop;

impl NamedSyntaxNode for WhileLoop {
    const NODE_NAME: &'static str = "while_stmt";
}

impl SyntaxNode<'_, WhileLoop> {
    pub fn cond(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> SyntaxNode<'_, FunctionStatement> {
        self.field_child("body").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, WhileLoop> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WhileLoop")
            .field("cond", &self.cond())
            .field("body", &self.body())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, WhileLoop> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_while_stmt(self);
        self.body().accept(visitor);
    }
}



#[derive(Debug, Clone)]
pub struct DoWhileLoop;

impl NamedSyntaxNode for DoWhileLoop {
    const NODE_NAME: &'static str = "do_while_stmt";
}

impl SyntaxNode<'_, DoWhileLoop> {
    pub fn cond(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> SyntaxNode<'_, FunctionStatement> {
        self.field_child("body").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, DoWhileLoop> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoWhileLoop")
            .field("cond", &self.cond())
            .field("body", &self.body())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, DoWhileLoop> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_do_while_stmt(self);
        self.body().accept(visitor);
    }
}
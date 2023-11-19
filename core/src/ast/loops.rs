use crate::{NamedSyntaxNode, SyntaxNode};
use super::{expressions::Expression, functions::FunctionStatement};

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
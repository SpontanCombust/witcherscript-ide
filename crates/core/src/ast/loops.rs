use std::fmt::Debug;
use crate::{AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::{StatementTraversal, StatementVisitor, ExpressionNode, FunctionStatementNode};


mod tags {
    pub struct ForLoop;
    pub struct WhileLoop;
    pub struct DoWhileLoop;
}


pub type ForLoopNode<'script> = SyntaxNode<'script, tags::ForLoop>;

impl NamedSyntaxNode for ForLoopNode<'_> {
    const NODE_KIND: &'static str = "for_stmt";
}

impl ForLoopNode<'_> {
    pub fn init(&self) -> Option<ExpressionNode> {
        self.field_child("init").map(|n| n.into())
    }

    pub fn cond(&self) -> Option<ExpressionNode> {
        self.field_child("cond").map(|n| n.into())
    }

    pub fn iter(&self) -> Option<ExpressionNode> {
        self.field_child("iter").map(|n| n.into())
    }

    pub fn body(&self) -> FunctionStatementNode {
        self.field_child("body").unwrap().into()
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
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for ForLoopNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_for_stmt(self) {
            self.body().accept(visitor);
        }
    }
}



pub type WhileLoopNode<'script> = SyntaxNode<'script, tags::WhileLoop>;

impl NamedSyntaxNode for WhileLoopNode<'_> {
    const NODE_KIND: &'static str = "while_stmt";
}

impl WhileLoopNode<'_> {
    pub fn cond(&self) -> ExpressionNode {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> FunctionStatementNode {
        self.field_child("body").unwrap().into()
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
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for WhileLoopNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_while_stmt(self) {
            self.body().accept(visitor);
        }
    }
}



pub type DoWhileLoopNode<'script> = SyntaxNode<'script, tags::DoWhileLoop>;

impl NamedSyntaxNode for DoWhileLoopNode<'_> {
    const NODE_KIND: &'static str = "do_while_stmt";
}

impl DoWhileLoopNode<'_> {
    pub fn cond(&self) -> ExpressionNode {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> FunctionStatementNode {
        self.field_child("body").unwrap().into()
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
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for DoWhileLoopNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_do_while_stmt(self) {
            self.body().accept(visitor);
        }
    }
}
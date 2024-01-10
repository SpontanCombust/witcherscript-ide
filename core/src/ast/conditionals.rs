use std::fmt::Debug;
use crate::{NamedSyntaxNode, SyntaxNode, AnyNode};
use super::{StatementTraversal, StatementVisitor, ExpressionNode, FunctionStatementNode};


#[derive(Debug, Clone)]
pub struct IfConditional;

pub type IfConditionalNode<'script> = SyntaxNode<'script, IfConditional>;

impl NamedSyntaxNode for IfConditionalNode<'_> {
    const NODE_KIND: &'static str = "if_stmt";
}

impl IfConditionalNode<'_> {
    pub fn cond(&self) -> ExpressionNode {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> FunctionStatementNode {
        self.field_child("body").unwrap().into()
    }

    pub fn else_body(&self) -> Option<FunctionStatementNode> {
        self.field_child("else").map(|n| n.into())
    }
}

impl Debug for IfConditionalNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IfConditional")
            .field("cond", &self.cond())
            .field("body", &self.body())
            .field("else", &self.else_body())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for IfConditionalNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for IfConditionalNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_if_stmt(self);
        self.body().accept(visitor);
        self.else_body().map(|s| { s.accept(visitor) });
    }
}



#[derive(Debug, Clone)]
pub struct SwitchConditional;

pub type SwitchConditionalNode<'script> = SyntaxNode<'script, SwitchConditional>;

impl NamedSyntaxNode for SwitchConditionalNode<'_> {
    const NODE_KIND: &'static str = "switch_stmt";
}

impl SwitchConditionalNode<'_> {
    pub fn matched_expr(&self) -> ExpressionNode {
        self.field_child("matched_expr").unwrap().into()
    }

    pub fn cases(&self) -> impl Iterator<Item = SwitchConditionalCaseNode> {
        self.field_children("cases").map(|n| n.into())
    }

    pub fn default(&self) -> Option<SwitchConditionalDefaultNode> {
        self.field_child("default").map(|n| n.into())
    }
}

impl Debug for SwitchConditionalNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchConditional")
            .field("matched_expr", &self.matched_expr())
            .field("cases", &self.cases().collect::<Vec<_>>())
            .field("default", &self.default())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt(self);
        self.cases().for_each(|s| s.accept(visitor));
        self.default().map(|s| { s.accept(visitor) });
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalCase;

pub type SwitchConditionalCaseNode<'script> = SyntaxNode<'script, SwitchConditionalCase>;

impl NamedSyntaxNode for SwitchConditionalCaseNode<'_> {
    const NODE_KIND: &'static str = "switch_case";
}

impl SwitchConditionalCaseNode<'_> {
    pub fn value(&self) -> ExpressionNode {
        self.field_child("value").unwrap().into()
    }

    pub fn body(&self) -> impl Iterator<Item = FunctionStatementNode> {
        self.field_children("body").map(|n| n.into())
    }
}

impl Debug for SwitchConditionalCaseNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchConditionalCase")
            .field("value", &self.value())
            .field("body", &self.body().collect::<Vec<_>>())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalCaseNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalCaseNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt_case(self);
        self.body().for_each(|s| s.accept(visitor));
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalDefault;

pub type SwitchConditionalDefaultNode<'script> = SyntaxNode<'script, SwitchConditionalDefault>;

impl NamedSyntaxNode for SwitchConditionalDefaultNode<'_> {
    const NODE_KIND: &'static str = "switch_default";
}

impl SwitchConditionalDefaultNode<'_> {
    pub fn body(&self) -> impl Iterator<Item = FunctionStatementNode> {
        self.field_children("body").map(|n| n.into())
    }
}

impl Debug for SwitchConditionalDefaultNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchConditionalDefault")
            .field("body", &self.body().collect::<Vec<_>>())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalDefaultNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalDefaultNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt_default(self);
        self.body().for_each(|s| s.accept(visitor));
    }
}
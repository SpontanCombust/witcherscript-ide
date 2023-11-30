use std::fmt::Debug;
use crate::{NamedSyntaxNode, SyntaxNode};
use super::{Expression, FunctionStatement, StatementTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct IfConditional;

impl NamedSyntaxNode for IfConditional {
    const NODE_NAME: &'static str = "if_stmt";
}

impl SyntaxNode<'_, IfConditional> {
    pub fn cond(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> SyntaxNode<'_, FunctionStatement> {
        self.field_child("body").unwrap().into()
    }

    pub fn else_body(&self) -> Option<SyntaxNode<'_, FunctionStatement>> {
        self.field_child("else").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, IfConditional> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IfConditional")
            .field("cond", &self.cond())
            .field("body", &self.body())
            .field("else", &self.else_body())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, IfConditional> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_if_stmt(self);
        if visitor.should_visit_inner() {
            self.body().accept(visitor);
            self.else_body().map(|s| { s.accept(visitor) });
        }
    }
}



#[derive(Debug, Clone)]
pub struct SwitchConditional;

impl NamedSyntaxNode for SwitchConditional {
    const NODE_NAME: &'static str = "switch_stmt";
}

impl SyntaxNode<'_, SwitchConditional> {
    pub fn matched_expr(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("matched_expr").unwrap().into()
    }

    pub fn cases(&self) -> impl Iterator<Item = SyntaxNode<'_, SwitchConditionalCase>> {
        self.field_children("cases").map(|n| n.into())
    }

    pub fn default(&self) -> Option<SyntaxNode<'_, SwitchConditionalDefault>> {
        self.field_child("default").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, SwitchConditional> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchConditional")
            .field("matched_expr", &self.matched_expr())
            .field("cases", &self.cases().collect::<Vec<_>>())
            .field("default", &self.default())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, SwitchConditional> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt(self);
        if visitor.should_visit_inner() {
            self.cases().for_each(|s| s.accept(visitor));
            self.default().map(|s| { s.accept(visitor) });
        }
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalCase;

impl NamedSyntaxNode for SwitchConditionalCase {
    const NODE_NAME: &'static str = "switch_case";
}

impl SyntaxNode<'_, SwitchConditionalCase> {
    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("value").unwrap().into()
    }

    pub fn body(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionStatement>> {
        self.field_children("body").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, SwitchConditionalCase> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchConditionalCase")
            .field("value", &self.value())
            .field("body", &self.body().collect::<Vec<_>>())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, SwitchConditionalCase> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt_case(self);
        if visitor.should_visit_inner() {
            self.body().for_each(|s| s.accept(visitor));
        }
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalDefault;

impl NamedSyntaxNode for SwitchConditionalDefault {
    const NODE_NAME: &'static str = "switch_default";
}

impl SyntaxNode<'_, SwitchConditionalDefault> {
    pub fn body(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionStatement>> {
        self.field_children("body").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, SwitchConditionalDefault> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchConditionalDefault")
            .field("body", &self.body().collect::<Vec<_>>())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, SwitchConditionalDefault> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt_default(self);
        if visitor.should_visit_inner() {
            self.body().for_each(|s| s.accept(visitor));
        }
    }
}
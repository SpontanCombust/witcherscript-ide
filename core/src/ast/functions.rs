use std::fmt::Debug;
use crate::{SyntaxNode, NamedSyntaxNode, tokens::Identifier, attribs::*};
use super::*;


#[derive(Debug, Clone)]
pub struct EventDeclaration;

impl NamedSyntaxNode for EventDeclaration {
    const NODE_NAME: &'static str = "event_decl_stmt";
}

impl SyntaxNode<'_, EventDeclaration> {
    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionParameterGroup>> {
        self.field_children("params").map(|n| n.into())
    }

    pub fn definition(&self) -> Option<SyntaxNode<'_, FunctionBlock>> {
        self.field_child("definition").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, EventDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventDeclaration")
            .field("name", &self.name())
            .field("params", &self.params().collect::<Vec<_>>())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, EventDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_event_decl(self) {
            self.params().for_each(|p| p.accept(visitor));
            self.definition().map(|s| s.accept(visitor));
        }
    }
}



#[derive(Debug, Clone)]
pub struct GlobalFunctionDeclaration;

impl NamedSyntaxNode for GlobalFunctionDeclaration {
    const NODE_NAME: &'static str = "global_func_decl_stmt";
}

impl SyntaxNode<'_, GlobalFunctionDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, GlobalFunctionSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn flavour(&self) -> Option<SyntaxNode<'_, GlobalFunctionFlavour>> {
        self.field_child("flavour").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionParameterGroup>> {
        self.field_children("params").map(|n| n.into())
    }

    pub fn return_type(&self) -> Option<SyntaxNode<'_, TypeAnnotation>> {
        self.field_child("return_type").map(|n| n.into())
    }

    pub fn definition(&self) -> Option<SyntaxNode<'_, FunctionBlock>> {
        self.field_child("definition").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, GlobalFunctionDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalFunctionDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("flavour", &self.flavour())
            .field("name", &self.name())
            .field("params", &self.params().collect::<Vec<_>>())
            .field("return_type", &self.return_type())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, GlobalFunctionDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_global_func_decl(self) {
            self.params().for_each(|p| p.accept(visitor));
            self.definition().map(|s| s.accept(visitor));
        }
    }
}



#[derive(Debug, Clone)]
pub struct MemberFunctionDeclaration;

impl NamedSyntaxNode for MemberFunctionDeclaration {
    const NODE_NAME: &'static str = "member_func_decl_stmt";
}

impl SyntaxNode<'_, MemberFunctionDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, MemberFunctionSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn flavour(&self) -> Option<SyntaxNode<'_, MemberFunctionFlavour>> {
        self.field_child("flavour").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionParameterGroup>> {
        self.field_children("params").map(|n| n.into())
    }

    pub fn return_type(&self) -> Option<SyntaxNode<'_, TypeAnnotation>> {
        self.field_child("return_type").map(|n| n.into())
    }

    pub fn definition(&self) -> Option<SyntaxNode<'_, FunctionBlock>> {
        self.field_child("definition").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, MemberFunctionDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberFunctionDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("flavour", &self.flavour())
            .field("name", &self.name())
            .field("params", &self.params().collect::<Vec<_>>())
            .field("return_type", &self.return_type())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, MemberFunctionDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_member_func_decl(self) {
            self.params().for_each(|p| p.accept(visitor));
            self.definition().map(|s| s.accept(visitor));
        }
    }
}




#[derive(Debug, Clone)]
pub struct FunctionParameterGroup;

impl NamedSyntaxNode for FunctionParameterGroup {
    const NODE_NAME: &'static str = "func_param_group";
}

impl SyntaxNode<'_, FunctionParameterGroup> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionParameterSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn names(&self) -> impl Iterator<Item = SyntaxNode<'_, Identifier>> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn param_type(&self) -> SyntaxNode<'_, TypeAnnotation> {
        self.field_child("param_type").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, FunctionParameterGroup> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionParameterGroup")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("names", &self.names().collect::<Vec<_>>())
            .field("param_type", &self.param_type())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, FunctionParameterGroup> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_func_param_group(self);
    }
}



#[derive(Debug, Clone)]
pub enum FunctionStatement<'script> {
    Var(SyntaxNode<'script, VarDeclaration>),
    Expr(SyntaxNode<'script, ExpressionStatement>),
    For(SyntaxNode<'script, ForLoop>),
    While(SyntaxNode<'script, WhileLoop>),
    DoWhile(SyntaxNode<'script, DoWhileLoop>),
    If(SyntaxNode<'script, IfConditional>),
    Switch(SyntaxNode<'script, SwitchConditional>),
    Break(SyntaxNode<'script, BreakStatement>),
    Continue(SyntaxNode<'script, ContinueStatement>),
    Return(SyntaxNode<'script, ReturnStatement>),
    Delete(SyntaxNode<'script, DeleteStatement>),
    Block(SyntaxNode<'script, FunctionBlock>),
    Nop,
}

impl SyntaxNode<'_, FunctionStatement<'_>> {
    pub fn value(&self) -> FunctionStatement {
        match self.tree_node.kind() {
            VarDeclaration::NODE_NAME => FunctionStatement::Var(self.clone().into()),
            ExpressionStatement::NODE_NAME => FunctionStatement::Expr(self.clone().into()),
            ForLoop::NODE_NAME => FunctionStatement::For(self.clone().into()),
            WhileLoop::NODE_NAME => FunctionStatement::While(self.clone().into()),
            DoWhileLoop::NODE_NAME => FunctionStatement::DoWhile(self.clone().into()),
            IfConditional::NODE_NAME => FunctionStatement::If(self.clone().into()),
            SwitchConditional::NODE_NAME => FunctionStatement::Switch(self.clone().into()),
            BreakStatement::NODE_NAME => FunctionStatement::Break(self.clone().into()),
            ContinueStatement::NODE_NAME => FunctionStatement::Continue(self.clone().into()),
            ReturnStatement::NODE_NAME => FunctionStatement::Return(self.clone().into()),
            DeleteStatement::NODE_NAME => FunctionStatement::Delete(self.clone().into()),
            FunctionBlock::NODE_NAME => FunctionStatement::Block(self.clone().into()),
            Nop::NODE_NAME => FunctionStatement::Nop,
            _ => panic!("Unknown function statement type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, FunctionStatement<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}

impl StatementTraversal for SyntaxNode<'_, FunctionStatement<'_>> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.value() {
            FunctionStatement::Var(s) => s.accept(visitor),
            FunctionStatement::Expr(s) => s.accept(visitor),
            FunctionStatement::For(s) => s.accept(visitor),
            FunctionStatement::While(s) => s.accept(visitor),
            FunctionStatement::DoWhile(s) => s.accept(visitor),
            FunctionStatement::If(s) => s.accept(visitor),
            FunctionStatement::Switch(s) => s.accept(visitor),
            FunctionStatement::Break(s) => s.accept(visitor),
            FunctionStatement::Continue(s) => s.accept(visitor),
            FunctionStatement::Return(s) => s.accept(visitor),
            FunctionStatement::Delete(s) => s.accept(visitor),
            FunctionStatement::Block(s) => s.accept(visitor),
            FunctionStatement::Nop => visitor.visit_nop_stmt(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionBlock;

impl NamedSyntaxNode for FunctionBlock {
    const NODE_NAME: &'static str = "func_block";
}

impl SyntaxNode<'_, FunctionBlock> {
    pub fn statements(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionStatement>> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, FunctionBlock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionBlock{:?}", self.statements().collect::<Vec<_>>())
    }
}

impl StatementTraversal for SyntaxNode<'_, FunctionBlock> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        self.statements().for_each(|s| s.accept(visitor));
    }
}



#[derive(Debug, Clone)]
pub struct BreakStatement;

impl NamedSyntaxNode for BreakStatement {
    const NODE_NAME: &'static str = "break_stmt";
}

impl SyntaxNode<'_, BreakStatement> {}

impl Debug for SyntaxNode<'_, BreakStatement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BreakStatement")
    }
}

impl StatementTraversal for SyntaxNode<'_, BreakStatement> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_break_stmt(self);
    }
}



#[derive(Debug, Clone)]
pub struct ContinueStatement;

impl NamedSyntaxNode for ContinueStatement {
    const NODE_NAME: &'static str = "continue_stmt";
}

impl SyntaxNode<'_, ContinueStatement> {}

impl Debug for SyntaxNode<'_, ContinueStatement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContinueStatement")
    }
}

impl StatementTraversal for SyntaxNode<'_, ContinueStatement> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_continue_stmt(self);
    }
}



#[derive(Debug, Clone)]
pub struct ReturnStatement;

impl NamedSyntaxNode for ReturnStatement {
    const NODE_NAME: &'static str = "return_stmt";
}

impl SyntaxNode<'_, ReturnStatement> {
    pub fn value(&self) -> Option<SyntaxNode<'_, Expression>> {
        self.first_child(true).map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, ReturnStatement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReturnStatement")
            .field(&self.value())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, ReturnStatement> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_return_stmt(self);
    }
}



#[derive(Debug, Clone)]
pub struct DeleteStatement;

impl NamedSyntaxNode for DeleteStatement {
    const NODE_NAME: &'static str = "delete_stmt";
}

impl SyntaxNode<'_, DeleteStatement> {
    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, DeleteStatement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DeleteStatement")
            .field(&self.value())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, DeleteStatement> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_delete_stmt(self);
    }
}
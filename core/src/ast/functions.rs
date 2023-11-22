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


#[derive(Debug, Clone)]
pub struct FunctionDeclaration;

impl NamedSyntaxNode for FunctionDeclaration {
    const NODE_NAME: &'static str = "func_decl_stmt";
}

impl SyntaxNode<'_, FunctionDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn flavour(&self) -> Option<SyntaxNode<'_, FunctionFlavour>> {
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

impl Debug for SyntaxNode<'_, FunctionDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("flavour", &self.flavour())
            .field("name", &self.name())
            .field("params", &self.params().collect::<Vec<_>>())
            .field("return_type", &self.return_type())
            .field("definition", &self.definition())
            .finish()
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
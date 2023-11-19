use crate::{SyntaxNode, NamedSyntaxNode};
use super::{vars::*, expressions::*, nop::Nop, loops::*, conditionals::*};

/*
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub imported: bool,
    pub access_modifier: Option<Spanned<AccessModifier>>,
    pub specifiers: Spanned<Vec<Spanned<FunctionSpecifier>>>,
    pub speciality: Spanned<FunctionSpeciality>,

    pub name: Spanned<Identifier>,
    pub params: Spanned<Vec<Spanned<FunctionParameterGroup>>>,
    pub return_type: Option<Spanned<TypeAnnotation>>,
    pub body: Option<Spanned<FunctionBody>> // if there is no body it doesn't have a definition
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionSpecifier {
    Final,
    Latent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionSpeciality {
    Entry,
    Event,
    Exec,
    Quest,
    Timer,
    Storyscene,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameterGroup {
    pub names: Vec<Spanned<Identifier>>,
    pub optional: bool,
    pub output: bool,
    pub param_type: Spanned<TypeAnnotation>
}
*/

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
    Nop(SyntaxNode<'script, Nop>)
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
            Nop::NODE_NAME => FunctionStatement::Nop(self.clone().into()),
            _ => panic!("Unknown function statement type: {}", self.tree_node.kind())
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
        self.children().map(|n| n.into())
    }
}



#[derive(Debug, Clone)]
pub struct BreakStatement;

impl NamedSyntaxNode for BreakStatement {
    const NODE_NAME: &'static str = "break_stmt";
}

impl SyntaxNode<'_, BreakStatement> {}



#[derive(Debug, Clone)]
pub struct ContinueStatement;

impl NamedSyntaxNode for ContinueStatement {
    const NODE_NAME: &'static str = "continue_stmt";
}

impl SyntaxNode<'_, ContinueStatement> {}



#[derive(Debug, Clone)]
pub struct ReturnStatement;

impl NamedSyntaxNode for ReturnStatement {
    const NODE_NAME: &'static str = "return_stmt";
}

impl SyntaxNode<'_, ReturnStatement> {
    pub fn value(&self) -> Option<SyntaxNode<'_, Expression>> {
        self.first_child().map(|n| n.into())
    }
}



#[derive(Debug, Clone)]
pub struct DeleteStatement;

impl NamedSyntaxNode for DeleteStatement {
    const NODE_NAME: &'static str = "delete_stmt";
}

impl SyntaxNode<'_, DeleteStatement> {
    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.first_child().unwrap().into()
    }
}
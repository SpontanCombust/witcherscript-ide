use std::str::FromStr;

use crate::{SyntaxNode, NamedSyntaxNode, tokens::Keyword};
use super::{vars::*, expressions::*, nop::Nop, loops::*, conditionals::*, classes::AccessModifier};

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

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameterGroup {
    pub names: Vec<Spanned<Identifier>>,
    pub optional: bool,
    pub output: bool,
    pub param_type: Spanned<TypeAnnotation>
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionParameterSpecifier {
    Optional,
    Out
}

impl NamedSyntaxNode for FunctionParameterSpecifier {
    const NODE_NAME: &'static str = "func_param_specifier";
}

impl SyntaxNode<'_, FunctionParameterSpecifier> {
    pub fn value(&self) -> FunctionSpecifier {

    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionFlavour {
    Function,
    Entry,
    Event,
    Exec,
    Quest,
    Timer,
    Storyscene,
}

impl SyntaxNode<'_, FunctionFlavour> {
    pub fn value(&self) -> FunctionFlavour {
        match self.tree_node.kind() {
            "func_flavour_function" => FunctionFlavour::Function,
            "func_flavour_entry" => FunctionFlavour::Entry,
            "func_flavour_event" => FunctionFlavour::Event,
            "func_flavour_exec" => FunctionFlavour::Exec,
            "func_flavour_quest" => FunctionFlavour::Quest,
            "func_flavour_timer" => FunctionFlavour::Timer,
            "func_flavour_storyscene" => FunctionFlavour::Storyscene,
            _ => panic!("Unknown function flavour")
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionSpecifier {
    AccessModifier(AccessModifier),
    Import,
    Final,
    Latent,
}

impl NamedSyntaxNode for FunctionSpecifier {
    const NODE_NAME: &'static str = "func_specifier";
}

impl SyntaxNode<'_, FunctionSpecifier> {
    pub fn value(&self) -> FunctionSpecifier {
        let s = self.tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Private => return FunctionSpecifier::AccessModifier(AccessModifier::Private),
                Keyword::Protected => return FunctionSpecifier::AccessModifier(AccessModifier::Protected),
                Keyword::Public => return FunctionSpecifier::AccessModifier(AccessModifier::Public),
                Keyword::Import => return FunctionSpecifier::Import,
                Keyword::Final => return FunctionSpecifier::Final,
                Keyword::Latent => return FunctionSpecifier::Latent,
                _ => {}
            }
        }

        panic!("Unknown function specifier: {}", s)
    }
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
        self.children(Some(true)).map(|n| n.into())
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
        self.first_child(Some(true)).map(|n| n.into())
    }
}



#[derive(Debug, Clone)]
pub struct DeleteStatement;

impl NamedSyntaxNode for DeleteStatement {
    const NODE_NAME: &'static str = "delete_stmt";
}

impl SyntaxNode<'_, DeleteStatement> {
    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.first_child(Some(true)).unwrap().into()
    }
}
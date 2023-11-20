use crate::{NamedSyntaxNode, SyntaxNode, tokens::*, attribs::*};
use super::*;


#[derive(Debug, Clone)]
pub struct ClassDeclaration;

impl NamedSyntaxNode for ClassDeclaration {
    const NODE_NAME: &'static str = "class_decl_stmt";
}

impl SyntaxNode<'_, ClassDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, ClassSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn base(&self) -> Option<SyntaxNode<'_, Identifier>> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> SyntaxNode<'_, ClassBlock> {
        self.field_child("definition").unwrap().into()
    }
}


#[derive(Debug, Clone)]
pub struct ClassBlock;

impl NamedSyntaxNode for ClassBlock {
    const NODE_NAME: &'static str = "class_block";
}

impl SyntaxNode<'_, ClassBlock> {
    pub fn statements(&self) -> impl Iterator<Item = SyntaxNode<'_, ClassStatement>> {
        self.children(true).map(|n| n.into())
    }
}



#[derive(Debug, Clone)]
pub enum ClassStatement<'script> {
    Var(SyntaxNode<'script, MemberVarDeclaration>),
    Default(SyntaxNode<'script, MemberDefaultValue>),
    Hint(SyntaxNode<'script, MemberHint>),
    Autobind(SyntaxNode<'script, ClassAutobind>),
    Method(SyntaxNode<'script, FunctionDeclaration>),
    Nop
}

impl SyntaxNode<'_, ClassStatement<'_>> {
    pub fn value(&self) -> ClassStatement {
        match self.tree_node.kind() {
            MemberVarDeclaration::NODE_NAME => ClassStatement::Var(self.clone().into()),
            MemberDefaultValue::NODE_NAME => ClassStatement::Default(self.clone().into()),
            MemberHint::NODE_NAME => ClassStatement::Hint(self.clone().into()),
            ClassAutobind::NODE_NAME => ClassStatement::Autobind(self.clone().into()),
            FunctionDeclaration::NODE_NAME => ClassStatement::Method(self.clone().into()),
            Nop::NODE_NAME => ClassStatement::Nop,
            _ => panic!("Unknown class statement type: {}", self.tree_node.kind())
        }
    }
}


#[derive(Debug, Clone)]
pub struct ClassAutobind;

impl NamedSyntaxNode for ClassAutobind {
    const NODE_NAME: &'static str = "class_autobind_stmt";
}

impl SyntaxNode<'_, ClassAutobind> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, ClassAutobindSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn autobind_type(&self) -> SyntaxNode<'_, TypeAnnotation> {
        self.field_child("autobind_type").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, ClassAutobindValue> {
        self.field_child("value").unwrap().into()
    }
}


#[derive(Debug, Clone)]
pub enum ClassAutobindValue<'script> {
    Single,
    Concrete(SyntaxNode<'script, LiteralString>)
}

impl SyntaxNode<'_, ClassAutobindValue<'_>> {
    pub fn value(&self) -> ClassAutobindValue {
        let child = self.first_child(false).unwrap();
        let s = child.tree_node.kind();
        if s == LiteralString::NODE_NAME {
            return ClassAutobindValue::Concrete(child.into());
        } else if s == "single" {
            return ClassAutobindValue::Single;
        } else {
            panic!("Unknown class autobind value type: {}", s);
        }
    }
} 

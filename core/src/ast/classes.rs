use std::str::FromStr;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::{Keyword, Identifier, LiteralString}};
use super::{vars::*, structs::*, functions::FunctionDeclaration, nop::Nop};


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
        self.children(Some(true)).map(|n| n.into())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassSpecifier {
    Import,
    Abstract,
    Statemachine
}

impl NamedSyntaxNode for ClassSpecifier {
    const NODE_NAME: &'static str = "class_specifier";
}

impl SyntaxNode<'_, ClassSpecifier> {
    pub fn value(&self) -> ClassSpecifier {
        let s = self.first_child(None).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Import => return ClassSpecifier::Import,
                Keyword::Abstract => return ClassSpecifier::Abstract,
                Keyword::Statemachine => return ClassSpecifier::Statemachine,
                _ => {}
            }
        }

        panic!("Unknown class specifier: {}", s);
    }
}




#[derive(Debug, Clone)]
pub enum ClassStatement<'script> {
    Var(SyntaxNode<'script, MemberVarDeclaration>),
    Default(SyntaxNode<'script, MemberDefaultValue>),
    Hint(SyntaxNode<'script, MemberHint>),
    Autobind(SyntaxNode<'script, ClassAutobind>),
    Method(SyntaxNode<'script, FunctionDeclaration>),
    Nop(SyntaxNode<'script, Nop>)
}

impl SyntaxNode<'_, ClassStatement<'_>> {
    pub fn value(&self) -> ClassStatement {
        match self.tree_node.kind() {
            MemberVarDeclaration::NODE_NAME => ClassStatement::Var(self.clone().into()),
            MemberDefaultValue::NODE_NAME => ClassStatement::Default(self.clone().into()),
            MemberHint::NODE_NAME => ClassStatement::Hint(self.clone().into()),
            ClassAutobind::NODE_NAME => ClassStatement::Autobind(self.clone().into()),
            FunctionDeclaration::NODE_NAME => ClassStatement::Method(self.clone().into()),
            Nop::NODE_NAME => ClassStatement::Nop(self.clone().into()),
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
        let child = self.first_child(None).unwrap();
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassAutobindSpecifier {
    AccessModifier(AccessModifier),
    Optional
}

impl NamedSyntaxNode for ClassAutobindSpecifier {
    const NODE_NAME: &'static str = "class_autobind_specifier";
}

impl SyntaxNode<'_, ClassAutobindSpecifier> {
    pub fn value(&self) -> ClassAutobindSpecifier {
        let s = self.first_child(None).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Private => return ClassAutobindSpecifier::AccessModifier(AccessModifier::Private),
                Keyword::Protected => return ClassAutobindSpecifier::AccessModifier(AccessModifier::Protected),
                Keyword::Public => return ClassAutobindSpecifier::AccessModifier(AccessModifier::Public),
                Keyword::Optional => return ClassAutobindSpecifier::Optional,
                _ => {}
            }
        }

        panic!("Unknown class autobind specifier: {}", s)
    }
}



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessModifier {
    Private,
    Protected,
    Public
}
use std::str::FromStr;

use crate::{tokens::{Identifier, Keyword, LiteralString}, NamedSyntaxNode, SyntaxNode};
use super::{vars::MemberVarDeclaration, expressions::Expression, nop::Nop};


#[derive(Debug, Clone)]
pub struct StructDeclaration;

impl NamedSyntaxNode for StructDeclaration {
    const NODE_NAME: &'static str = "struct_decl_stmt";
}

impl SyntaxNode<'_, StructDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, StructSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn definition(&self) -> SyntaxNode<'_, StructBlock> {
        self.field_child("definition").unwrap().into()
    }
}



#[derive(Debug, Clone)]
pub struct StructBlock;

impl NamedSyntaxNode for StructBlock {
    const NODE_NAME: &'static str = "struct_block";
}

impl SyntaxNode<'_, StructBlock> {
    pub fn statements(&self) -> impl Iterator<Item = SyntaxNode<'_, StructStatement>> {
        self.children(true).map(|n| n.into())
    }
}



#[derive(Debug, Clone)]
pub enum StructStatement<'script> {
    Var(SyntaxNode<'script, MemberVarDeclaration>),
    Default(SyntaxNode<'script, MemberDefaultValue>),
    Hint(SyntaxNode<'script, MemberHint>),
    Nop
}

impl SyntaxNode<'_, StructStatement<'_>> {
    pub fn value(&self) -> StructStatement {
        match self.tree_node.kind() {
            MemberVarDeclaration::NODE_NAME => StructStatement::Var(self.clone().into()),
            MemberDefaultValue::NODE_NAME => StructStatement::Default(self.clone().into()),
            MemberHint::NODE_NAME => StructStatement::Hint(self.clone().into()),
            Nop::NODE_NAME => StructStatement::Nop,
            _ => panic!("Unknown struct statement type: {}", self.tree_node.kind())
        }
    }
}



#[derive(Debug, Clone)]
pub struct MemberDefaultValue; 

impl NamedSyntaxNode for MemberDefaultValue {
    const NODE_NAME: &'static str = "member_default_val_stmt";
}

impl SyntaxNode<'_, MemberDefaultValue> {
    pub fn member(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("value").unwrap().into()
    }
}



#[derive(Debug, Clone)]
pub struct MemberHint; 

impl NamedSyntaxNode for MemberHint {
    const NODE_NAME: &'static str = "member_hint_stmt";
}

impl SyntaxNode<'_, MemberHint> {
    pub fn member(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, LiteralString> {
        self.field_child("value").unwrap().into()
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructSpecifier {
    Import
}

impl NamedSyntaxNode for StructSpecifier {
    const NODE_NAME: &'static str = "struct_specifier";
}

impl SyntaxNode<'_, StructSpecifier> {
    pub fn value(&self) -> StructSpecifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Import => return StructSpecifier::Import,
                _ => {}
            }
        }

        panic!("Unknown struct specifier: {}", s)
    }
}
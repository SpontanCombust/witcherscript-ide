use std::str::FromStr;
use crate::{tokens::{Identifier, Keyword}, NamedSyntaxNode, SyntaxNode};
use super::classes::ClassBlock;


#[derive(Debug, Clone)]
pub struct StateDeclaration;

impl NamedSyntaxNode for StateDeclaration {
    const NODE_NAME: &'static str = "state_decl_stmt";
}

impl SyntaxNode<'_, StateDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, StateSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn parent(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("parent").unwrap().into()
    }

    pub fn base(&self) -> Option<SyntaxNode<'_, Identifier>> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> SyntaxNode<'_, ClassBlock> {
        self.field_child("definition").unwrap().into()
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpecifier {
    Import,
    Abstract
}

impl NamedSyntaxNode for StateSpecifier {
    const NODE_NAME: &'static str = "state_specifier";
}

impl SyntaxNode<'_, StateSpecifier> {
    pub fn value(&self) -> StateSpecifier {
        let s = self.tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Import => return StateSpecifier::Import,
                Keyword::Abstract => return StateSpecifier::Abstract,
                _ => {}
            }
        }

        panic!("Unknown state specifier: {}", s);
    }
}
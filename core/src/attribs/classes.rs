use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::Keyword};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessModifier {
    Private,
    Protected,
    Public
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
        let s = self.first_child(false).unwrap().tree_node.kind();
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

impl Debug for SyntaxNode<'_, ClassSpecifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
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
        let s = self.first_child(false).unwrap().tree_node.kind();
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

impl Debug for SyntaxNode<'_, ClassAutobindSpecifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}
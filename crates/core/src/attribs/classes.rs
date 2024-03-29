use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::Keyword, AnyNode, DebugMaybeAlternate};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessModifier {
    Private,
    Protected,
    Public
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassSpecifier {
    Import,
    Abstract,
    Statemachine
}

pub type ClassSpecifierNode<'script> = SyntaxNode<'script, ClassSpecifier>;

impl NamedSyntaxNode for ClassSpecifierNode<'_> {
    const NODE_KIND: &'static str = "class_specifier";
}

impl ClassSpecifierNode<'_> {
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

impl Debug for ClassSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ClassSpecifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutobindSpecifier {
    AccessModifier(AccessModifier),
    Optional
}

pub type AutobindSpecifierNode<'script> = SyntaxNode<'script, AutobindSpecifier>;

impl NamedSyntaxNode for AutobindSpecifierNode<'_> {
    const NODE_KIND: &'static str = "class_autobind_specifier";
}

impl AutobindSpecifierNode<'_> {
    pub fn value(&self) -> AutobindSpecifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Private => return AutobindSpecifier::AccessModifier(AccessModifier::Private),
                Keyword::Protected => return AutobindSpecifier::AccessModifier(AccessModifier::Protected),
                Keyword::Public => return AutobindSpecifier::AccessModifier(AccessModifier::Public),
                Keyword::Optional => return AutobindSpecifier::Optional,
                _ => {}
            }
        }

        panic!("Unknown class autobind specifier: {}", s)
    }
}

impl Debug for AutobindSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for AutobindSpecifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}
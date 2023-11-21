use std::str::FromStr;
use crate::{NamedSyntaxNode, tokens::Keyword, SyntaxNode};
use super::AccessModifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionParameterSpecifier {
    Optional,
    Out
}

impl NamedSyntaxNode for FunctionParameterSpecifier {
    const NODE_NAME: &'static str = "func_param_specifier";
}

impl SyntaxNode<'_, FunctionParameterSpecifier> {
    pub fn value(&self) -> FunctionParameterSpecifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Optional => return FunctionParameterSpecifier::Optional,
                Keyword::Out => return FunctionParameterSpecifier::Out,
                _ => {}
            }
        }

        panic!("Unknown function parameter specifier: {}", s)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionFlavour {
    Entry,
    Exec,
    Quest,
    Timer,
    Storyscene,
}

impl NamedSyntaxNode for FunctionFlavour {
    const NODE_NAME: &'static str = "func_flavour";
}

impl SyntaxNode<'_, FunctionFlavour> {
    pub fn value(&self) -> FunctionFlavour {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Entry => return FunctionFlavour::Entry,
                Keyword::Exec => return FunctionFlavour::Exec,
                Keyword::Quest => return FunctionFlavour::Quest,
                Keyword::Timer => return FunctionFlavour::Timer,
                Keyword::Storyscene => return FunctionFlavour::Storyscene,
                _ => {}
            }
        }

        panic!("Unknown function flavour")
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
        let s = self.first_child(false).unwrap().tree_node.kind();
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

use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, tokens::Keyword, SyntaxNode};
use super::AccessModifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Debug for SyntaxNode<'_, FunctionParameterSpecifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalFunctionSpecifier {
    Import,
    Latent,
}

impl NamedSyntaxNode for GlobalFunctionSpecifier {
    const NODE_NAME: &'static str = "global_func_specifier";
}

impl SyntaxNode<'_, GlobalFunctionSpecifier> {
    pub fn value(&self) -> GlobalFunctionSpecifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Import => return GlobalFunctionSpecifier::Import,
                Keyword::Latent => return GlobalFunctionSpecifier::Latent,
                _ => {}
            }
        }

        panic!("Unknown global function specifier: {}", s)
    }
}

impl Debug for SyntaxNode<'_, GlobalFunctionSpecifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalFunctionFlavour {
    Exec,
    Quest,
    Storyscene,
}

impl NamedSyntaxNode for GlobalFunctionFlavour {
    const NODE_NAME: &'static str = "global_func_flavour";
}

impl SyntaxNode<'_, GlobalFunctionFlavour> {
    pub fn value(&self) -> GlobalFunctionFlavour {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Exec => return GlobalFunctionFlavour::Exec,
                Keyword::Quest => return GlobalFunctionFlavour::Quest,
                Keyword::Storyscene => return GlobalFunctionFlavour::Storyscene,
                _ => {}
            }
        }

        panic!("Unknown global function flavour: {}", s)
    }
}

impl Debug for SyntaxNode<'_, GlobalFunctionFlavour> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberFunctionSpecifier {
    AccessModifier(AccessModifier),
    Import,
    Final,
    Latent,
}

impl NamedSyntaxNode for MemberFunctionSpecifier {
    const NODE_NAME: &'static str = "member_func_specifier";
}

impl SyntaxNode<'_, MemberFunctionSpecifier> {
    pub fn value(&self) -> MemberFunctionSpecifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Private => return MemberFunctionSpecifier::AccessModifier(AccessModifier::Private),
                Keyword::Protected => return MemberFunctionSpecifier::AccessModifier(AccessModifier::Protected),
                Keyword::Public => return MemberFunctionSpecifier::AccessModifier(AccessModifier::Public),
                Keyword::Import => return MemberFunctionSpecifier::Import,
                Keyword::Final => return MemberFunctionSpecifier::Final,
                Keyword::Latent => return MemberFunctionSpecifier::Latent,
                _ => {}
            }
        }

        panic!("Unknown member function specifier: {}", s)
    }
}

impl Debug for SyntaxNode<'_, MemberFunctionSpecifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberFunctionFlavour {
    Entry,
    Timer
}

impl NamedSyntaxNode for MemberFunctionFlavour {
    const NODE_NAME: &'static str = "member_func_flavour";
}

impl SyntaxNode<'_, MemberFunctionFlavour> {
    pub fn value(&self) -> MemberFunctionFlavour {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Entry => return MemberFunctionFlavour::Entry,
                Keyword::Timer => return MemberFunctionFlavour::Timer,
                _ => {}
            }
        }

        panic!("Unknown member function flavour: {}", s)
    }
}

impl Debug for SyntaxNode<'_, MemberFunctionFlavour> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}
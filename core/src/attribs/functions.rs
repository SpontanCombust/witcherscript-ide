use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, tokens::Keyword, SyntaxNode};
use super::AccessModifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionParameterSpecifier {
    Optional,
    Out
}

pub type FunctionParameterSpecifierNode<'script> = SyntaxNode<'script, FunctionParameterSpecifier>;

impl NamedSyntaxNode for FunctionParameterSpecifierNode<'_> {
    const NODE_KIND: &'static str = "func_param_specifier";
}

impl FunctionParameterSpecifierNode<'_> {
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

impl Debug for FunctionParameterSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalFunctionSpecifier {
    Import,
    Latent,
}

pub type GlobalFunctionSpecifierNode<'script> = SyntaxNode<'script, GlobalFunctionSpecifier>;

impl NamedSyntaxNode for GlobalFunctionSpecifierNode<'_> {
    const NODE_KIND: &'static str = "global_func_specifier";
}

impl GlobalFunctionSpecifierNode<'_> {
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

impl Debug for GlobalFunctionSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalFunctionFlavour {
    Exec,
    Quest,
    Storyscene,
    Reward,
}

pub type GlobalFunctionFlavourNode<'script> = SyntaxNode<'script, GlobalFunctionFlavour>;

impl NamedSyntaxNode for GlobalFunctionFlavourNode<'_> {
    const NODE_KIND: &'static str = "global_func_flavour";
}

impl GlobalFunctionFlavourNode<'_> {
    pub fn value(&self) -> GlobalFunctionFlavour {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Exec => return GlobalFunctionFlavour::Exec,
                Keyword::Quest => return GlobalFunctionFlavour::Quest,
                Keyword::Storyscene => return GlobalFunctionFlavour::Storyscene,
                Keyword::Reward => return GlobalFunctionFlavour::Reward,
                _ => {}
            }
        }

        panic!("Unknown global function flavour: {}", s)
    }
}

impl Debug for GlobalFunctionFlavourNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberFunctionSpecifier {
    AccessModifier(AccessModifier),
    Import,
    Final,
    Latent,
}

pub type MemberFunctionSpecifierNode<'script> = SyntaxNode<'script, MemberFunctionSpecifier>;

impl NamedSyntaxNode for MemberFunctionSpecifierNode<'_> {
    const NODE_KIND: &'static str = "member_func_specifier";
}

impl MemberFunctionSpecifierNode<'_> {
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

impl Debug for MemberFunctionSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberFunctionFlavour {
    Entry,
    Cleanup,
    Timer
}

pub type MemberFunctionFlavourNode<'script> = SyntaxNode<'script, MemberFunctionFlavour>;

impl NamedSyntaxNode for MemberFunctionFlavourNode<'_> {
    const NODE_KIND: &'static str = "member_func_flavour";
}

impl MemberFunctionFlavourNode<'_> {
    pub fn value(&self) -> MemberFunctionFlavour {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Entry => return MemberFunctionFlavour::Entry,
                Keyword::Cleanup => return MemberFunctionFlavour::Cleanup,
                Keyword::Timer => return MemberFunctionFlavour::Timer,
                _ => {}
            }
        }

        panic!("Unknown member function flavour: {}", s)
    }
}

impl Debug for MemberFunctionFlavourNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}
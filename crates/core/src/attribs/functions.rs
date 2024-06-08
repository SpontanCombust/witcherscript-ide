use std::fmt::Debug;
use std::str::FromStr;
use crate::{tokens::Keyword, AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::AccessModifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionParameterSpecifier {
    Optional,
    Out
}

impl From<FunctionParameterSpecifier> for Keyword {
    fn from(value: FunctionParameterSpecifier) -> Self {
        match value {
            FunctionParameterSpecifier::Optional => Keyword::Optional,
            FunctionParameterSpecifier::Out => Keyword::Out,
        }
    }
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

        panic!("Unknown function parameter specifier: {} {}", s, self.range().debug())
    }
}

impl Debug for FunctionParameterSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionParameterSpecifierNode<'script> {
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
pub enum GlobalFunctionSpecifier {
    Import,
    Latent,
}

impl From<GlobalFunctionSpecifier> for Keyword {
    fn from(value: GlobalFunctionSpecifier) -> Self {
        match value {
            GlobalFunctionSpecifier::Import => Keyword::Import,
            GlobalFunctionSpecifier::Latent => Keyword::Latent,
        }
    }
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

        panic!("Unknown global function specifier: {} {}", s, self.range().debug())
    }
}

impl Debug for GlobalFunctionSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for GlobalFunctionSpecifierNode<'script> {
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
pub enum GlobalFunctionFlavour {
    Exec,
    Quest,
    Storyscene,
    Reward,
}

impl From<GlobalFunctionFlavour> for Keyword {
    fn from(value: GlobalFunctionFlavour) -> Self {
        match value {
            GlobalFunctionFlavour::Exec => Keyword::Exec,
            GlobalFunctionFlavour::Quest => Keyword::Quest,
            GlobalFunctionFlavour::Storyscene => Keyword::Storyscene,
            GlobalFunctionFlavour::Reward => Keyword::Reward,
        }
    }
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

        panic!("Unknown global function flavour: {} {}", s, self.range().debug())
    }
}

impl Debug for GlobalFunctionFlavourNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for GlobalFunctionFlavourNode<'script> {
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
pub enum MemberFunctionSpecifier {
    AccessModifier(AccessModifier),
    Import,
    Final,
    Latent,
}

impl From<MemberFunctionSpecifier> for Keyword {
    fn from(value: MemberFunctionSpecifier) -> Self {
        match value {
            MemberFunctionSpecifier::AccessModifier(am) => am.into(),
            MemberFunctionSpecifier::Import => Keyword::Import,
            MemberFunctionSpecifier::Final => Keyword::Final,
            MemberFunctionSpecifier::Latent => Keyword::Latent,
        }
    }
}

impl From<AccessModifier> for MemberFunctionSpecifier {
    fn from(value: AccessModifier) -> Self {
        Self::AccessModifier(value)
    }
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

        panic!("Unknown member function specifier: {} {}", s, self.range().debug())
    }
}

impl Debug for MemberFunctionSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberFunctionSpecifierNode<'script> {
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
pub enum MemberFunctionFlavour {
    Entry,
    Cleanup,
    Timer
}

impl From<MemberFunctionFlavour> for Keyword {
    fn from(value: MemberFunctionFlavour) -> Self {
        match value {
            MemberFunctionFlavour::Entry => Keyword::Entry,
            MemberFunctionFlavour::Cleanup => Keyword::Cleanup,
            MemberFunctionFlavour::Timer => Keyword::Timer,
        }
    }
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

        panic!("Unknown member function flavour: {} {}", s, self.range().debug())
    }
}

impl Debug for MemberFunctionFlavourNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberFunctionFlavourNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}
use std::fmt::Debug;
use std::str::FromStr;
use crate::{tokens::Keyword, AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::{AccessModifier, Specifier};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionFlavour {
    Cleanup,
    Entry,
    Exec,
    Quest,
    Reward,
    Storyscene,
    Timer
}

impl From<FunctionFlavour> for Keyword {
    fn from(value: FunctionFlavour) -> Self {
        match value {
            FunctionFlavour::Cleanup => Keyword::Cleanup,
            FunctionFlavour::Entry => Keyword::Entry,
            FunctionFlavour::Exec => Keyword::Exec,
            FunctionFlavour::Quest => Keyword::Quest,
            FunctionFlavour::Reward => Keyword::Reward,
            FunctionFlavour::Storyscene => Keyword::Storyscene,
            FunctionFlavour::Timer => Keyword::Timer,
        }
    }
}

pub type FunctionFlavourNode<'script> = SyntaxNode<'script, FunctionFlavour>;

impl<'script> NamedSyntaxNode for FunctionFlavourNode<'script> {
    const NODE_KIND: &'static str = "func_flavour";
}

impl FunctionFlavourNode<'_> {
    pub fn value(&self) -> FunctionFlavour {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Cleanup => return FunctionFlavour::Cleanup,
                Keyword::Entry => return FunctionFlavour::Entry,
                Keyword::Exec => return FunctionFlavour::Exec,
                Keyword::Quest => return FunctionFlavour::Quest,
                Keyword::Reward => return FunctionFlavour::Reward,
                Keyword::Storyscene => return FunctionFlavour::Storyscene,
                Keyword::Timer => return FunctionFlavour::Timer,
                _ => {}
            }
        }

        panic!("Unknown function flavour: {} {}", s, self.range().debug())
    }
}

impl std::fmt::Debug for FunctionFlavourNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionFlavourNode<'script> {
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
pub enum FunctionParameterSpecifier {
    Optional,
    Out
}

impl TryFrom<Specifier> for FunctionParameterSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Optional => Ok(Self::Optional),
            Specifier::Out => Ok(Self::Out),
            _ => Err(())
        }
    }
}

impl From<FunctionParameterSpecifier> for Keyword {
    fn from(value: FunctionParameterSpecifier) -> Self {
        match value {
            FunctionParameterSpecifier::Optional => Keyword::Optional,
            FunctionParameterSpecifier::Out => Keyword::Out,
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalFunctionSpecifier {
    Import,
    Latent,
}

impl TryFrom<Specifier> for GlobalFunctionSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Import => Ok(Self::Import),
            Specifier::Latent => Ok(Self::Latent),
            _ => Err(())
        }
    }
}

impl From<GlobalFunctionSpecifier> for Keyword {
    fn from(value: GlobalFunctionSpecifier) -> Self {
        match value {
            GlobalFunctionSpecifier::Import => Keyword::Import,
            GlobalFunctionSpecifier::Latent => Keyword::Latent,
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

impl TryFrom<FunctionFlavour> for GlobalFunctionFlavour {
    type Error = ();

    fn try_from(value: FunctionFlavour) -> Result<Self, Self::Error> {
        match value {
            FunctionFlavour::Exec => Ok(Self::Exec),
            FunctionFlavour::Quest => Ok(Self::Quest),
            FunctionFlavour::Reward => Ok(Self::Reward),
            FunctionFlavour::Storyscene => Ok(Self::Storyscene),
            _ => Err(())
        }
    }
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



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberFunctionSpecifier {
    AccessModifier(AccessModifier),
    Import,
    Final,
    Latent,
}

impl TryFrom<Specifier> for MemberFunctionSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Final => Ok(Self::Final),
            Specifier::Import => Ok(Self::Import),
            Specifier::Latent => Ok(Self::Latent),
            Specifier::Private => Ok(Self::AccessModifier(AccessModifier::Private)),
            Specifier::Protected => Ok(Self::AccessModifier(AccessModifier::Protected)),
            Specifier::Public => Ok(Self::AccessModifier(AccessModifier::Public)),
            _ => Err(())
        }
    }
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



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberFunctionFlavour {
    Entry,
    Cleanup,
    Timer
}

impl TryFrom<FunctionFlavour> for MemberFunctionFlavour {
    type Error = ();

    fn try_from(value: FunctionFlavour) -> Result<Self, Self::Error> {
        match value {
            FunctionFlavour::Cleanup => Ok(Self::Cleanup),
            FunctionFlavour::Entry => Ok(Self::Entry),
            FunctionFlavour::Timer => Ok(Self::Timer),
            _ => Err(())
        }
    }
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

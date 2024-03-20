use std::fmt::Debug;
use std::str::FromStr;
use crate::{tokens::Keyword, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::AccessModifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberVarSpecifier {
    AccessModifier(AccessModifier),
    Const,
    Editable,
    Import,
    Inlined,
    Saved,
}

pub type MemberVarSpecifierNode<'script> = SyntaxNode<'script, MemberVarSpecifier>;

impl NamedSyntaxNode for MemberVarSpecifierNode<'_> {
    const NODE_KIND: &'static str = "member_var_specifier";
}

impl MemberVarSpecifierNode<'_> {
    pub fn value(&self) -> MemberVarSpecifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s)  {
            match k {
                Keyword::Private => return MemberVarSpecifier::AccessModifier(AccessModifier::Private),
                Keyword::Protected => return MemberVarSpecifier::AccessModifier(AccessModifier::Protected),
                Keyword::Public => return MemberVarSpecifier::AccessModifier(AccessModifier::Public),
                Keyword::Const => return MemberVarSpecifier::Const,
                Keyword::Editable => return MemberVarSpecifier::Editable,
                Keyword::Import => return MemberVarSpecifier::Import,
                Keyword::Inlined => return MemberVarSpecifier::Inlined,
                Keyword::Saved => return MemberVarSpecifier::Saved,
                _ => {}
            }
        }

        panic!("Unknown member var specifier: {} {}", s, self.range().debug())
    }
}

impl Debug for MemberVarSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())?;
        write!(f, " {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberVarSpecifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}
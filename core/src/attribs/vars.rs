use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, tokens::Keyword, SyntaxNode};
use super::AccessModifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberVarSpecifier {
    AccessModifier(AccessModifier),
    Const,
    Editable,
    Import,
    Inlined,
    Saved,
}

impl NamedSyntaxNode for MemberVarSpecifier {
    const NODE_NAME: &'static str = "member_var_specifier";
}

impl SyntaxNode<'_, MemberVarSpecifier> {
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

        panic!("Unknown member var specifier: {}", s)
    }
}

impl Debug for SyntaxNode<'_, MemberVarSpecifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}
use std::str::FromStr;
use std::fmt::Debug;
use crate::{tokens::Keyword, SyntaxNode, AnyNode, DebugMaybeAlternate};


#[derive(Debug, Clone)]
pub enum Unnamed {
    Keyword(Keyword),
    Punctuation(&'static str)
}

pub type UnnamedNode<'script> = SyntaxNode<'script, Unnamed>;

impl UnnamedNode<'_> {
    pub fn value(&self) -> Unnamed {
        let kind = self.tree_node.kind();
        if let Ok(kw) = Keyword::from_str(kind) {
            Unnamed::Keyword(kw)
        } else {
            Unnamed::Punctuation(kind)
        }
    }
}

impl Debug for UnnamedNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for UnnamedNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() 
        && !value.tree_node.is_error()
        && !value.tree_node.is_extra() {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}
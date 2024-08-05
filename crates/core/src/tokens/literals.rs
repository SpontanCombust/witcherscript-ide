use std::num::{ParseIntError, ParseFloatError};
use std::fmt::Debug;
use std::str::ParseBoolError;
use shrinkwraprs::Shrinkwrap;
use thiserror::Error;
use crate::MISSING_TEXT;
use crate::{AnyNode, NamedSyntaxNode, SyntaxNode, DebugMaybeAlternate, DebugRange, script_document::ScriptDocument};
use crate::ast::{SyntaxNodeTraversal, SyntaxNodeVisitor, TraversalContextStack};


#[derive(Debug, Clone, Error)]
pub enum LiteralValueError {
    #[error("literal node is marked as missing")]
    NodeMissing,
    #[error("failed to parse integer number: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("failed to parse floating point number: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("failed to parse bool: {0}")]
    ParseBoolError(#[from] ParseBoolError),
    #[error("failed to parse hex: {0}")]
    ParseHexError(ParseIntError),
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiteralInt(i32);

pub type LiteralIntNode<'script> = SyntaxNode<'script, LiteralInt>;

impl NamedSyntaxNode for LiteralIntNode<'_> {
    const NODE_KIND: &'static str = "literal_int";
}

impl LiteralIntNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralInt, LiteralValueError> {
        let s = self.text(doc);
        if s == MISSING_TEXT {
            return Err(LiteralValueError::NodeMissing);
        }

        let i = s.parse::<i32>()?;
        Ok(LiteralInt(i))
    }
}

impl Debug for LiteralIntNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralInt {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralIntNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq, PartialOrd)]
pub struct LiteralFloat(f32);

pub type LiteralFloatNode<'script> = SyntaxNode<'script, LiteralFloat>;

impl NamedSyntaxNode for LiteralFloatNode<'_> {
    const NODE_KIND: &'static str = "literal_float";
}

impl LiteralFloatNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralFloat, LiteralValueError> {
        let s = self.text(doc);
        if s == MISSING_TEXT {
            return Err(LiteralValueError::NodeMissing);
        }

        // trim the optional trailing 'f'
        let s = if s.chars().last().unwrap() == 'f' { 
            &s[..s.len() - 1] 
        } else { 
            &s
        };

        let f = s.parse::<f32>()?;
        Ok(LiteralFloat(f))
    }
}

impl Debug for LiteralFloatNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralFloat {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralFloatNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiteralBool(bool);

pub type LiteralBoolNode<'script> = SyntaxNode<'script, LiteralBool>;

impl NamedSyntaxNode for LiteralBoolNode<'_> {
    const NODE_KIND: &'static str = "literal_bool";
}

impl LiteralBoolNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralBool, LiteralValueError> {
        let s = self.text(doc);
        if s == MISSING_TEXT {
            return Err(LiteralValueError::NodeMissing);
        }

        let b = s.parse::<bool>()?;
        Ok(LiteralBool(b))
    }
}

impl Debug for LiteralBoolNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralBool {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralBoolNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiteralString(String);

pub type LiteralStringNode<'script> = SyntaxNode<'script, LiteralString>;

impl NamedSyntaxNode for LiteralStringNode<'_> {
    const NODE_KIND: &'static str = "literal_string";
}

impl LiteralStringNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralString, LiteralValueError> {
        let s = self.text(doc);
        if s == MISSING_TEXT {
            return Err(LiteralValueError::NodeMissing);
        }

        let s = s[1..s.len()-1] // eliminate surrounding quotes
        .replace(r#"\""#, r#"""#); // escape internal quotes

        Ok(LiteralString(s))
    }
}

impl Debug for LiteralStringNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralString {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralStringNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiteralName(String);

pub type LiteralNameNode<'script> = SyntaxNode<'script, LiteralName>;

impl NamedSyntaxNode for LiteralNameNode<'_> {
    const NODE_KIND: &'static str = "literal_name";
}

impl LiteralNameNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralName, LiteralValueError> {
        let s = self.text(doc);
        if s == MISSING_TEXT {
            return Err(LiteralValueError::NodeMissing);
        }

        let s = s[1..s.len()-1].to_string(); // eliminate surrounding quotes
        // I'm not sure if names can have escaping quotes
        // They are supposed to be rather simple in their form
        // For now I'll asume they can't have any escape sequences

        Ok(LiteralName(s))
    }
}

impl Debug for LiteralNameNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralName {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralNameNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiteralNull;

pub type LiteralNullNode<'script> = SyntaxNode<'script, LiteralNull>;

impl NamedSyntaxNode for LiteralNullNode<'_> {
    const NODE_KIND: &'static str = "literal_null";
}

impl LiteralNullNode<'_> {}

impl Debug for LiteralNullNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralNull {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralNullNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq)]
pub struct LiteralHex(u32);

pub type LiteralHexNode<'script> = SyntaxNode<'script, LiteralHex>;

impl NamedSyntaxNode for LiteralHexNode<'_> {
    const NODE_KIND: &'static str = "literal_hex";
}

impl LiteralHexNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralHex, LiteralValueError> {
        let s = self.text(doc);
        if s == MISSING_TEXT {
            return Err(LiteralValueError::NodeMissing);
        }
        
        let i = u32::from_str_radix(&s[2..], 16).map_err(|err| LiteralValueError::ParseHexError(err))?;
        Ok(LiteralHex(i))
    }
}

impl Debug for LiteralHexNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralHex {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralHexNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


// Represents the unnamed $._literal node
#[derive(Clone, PartialEq)]
pub enum Literal<'script> {
    Int(LiteralIntNode<'script>),
    Hex(LiteralHexNode<'script>),
    Float(LiteralFloatNode<'script>),
    Bool(LiteralBoolNode<'script>),
    String(LiteralStringNode<'script>),
    Name(LiteralNameNode<'script>),
    Null(LiteralNullNode<'script>)
}

impl Debug for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => f.debug_maybe_alternate(n),
            Self::Hex(n) => f.debug_maybe_alternate(n),
            Self::Float(n) => f.debug_maybe_alternate(n),
            Self::Bool(n) => f.debug_maybe_alternate(n),
            Self::String(n) => f.debug_maybe_alternate(n),
            Self::Name(n) => f.debug_maybe_alternate(n),
            Self::Null(n) => f.debug_maybe_alternate(n),
        }
    }
}

pub type LiteralNode<'script> = SyntaxNode<'script, Literal<'script>>;

impl<'script> LiteralNode<'script> {
    pub fn value(self) -> Literal<'script> {
        match self.tree_node.kind() {
            LiteralIntNode::NODE_KIND => Literal::Int(self.into()),
            LiteralHexNode::NODE_KIND => Literal::Hex(self.into()),
            LiteralFloatNode::NODE_KIND => Literal::Float(self.into()),
            LiteralBoolNode::NODE_KIND => Literal::Bool(self.into()),
            LiteralStringNode::NODE_KIND => Literal::String(self.into()),
            LiteralNameNode::NODE_KIND => Literal::Name(self.into()),
            LiteralNullNode::NODE_KIND => Literal::Null(self.into()),
            _ => panic!("Unknown literal type: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for LiteralNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }

        match value.tree_node.kind() {
            LiteralIntNode::NODE_KIND       |
            LiteralHexNode::NODE_KIND       |
            LiteralFloatNode::NODE_KIND     |
            LiteralBoolNode::NODE_KIND      |
            LiteralStringNode::NODE_KIND    |
            LiteralNameNode::NODE_KIND      |
            LiteralNullNode::NODE_KIND      => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for LiteralNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        visitor.visit_literal_expr(self, ctx);
    }
}
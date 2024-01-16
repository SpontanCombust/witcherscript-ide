use std::num::{ParseIntError, ParseFloatError};
use std::fmt::Debug;
use std::str::ParseBoolError;
use shrinkwraprs::Shrinkwrap;
use thiserror::Error;
use crate::script_document::ScriptDocument;
use crate::{AnyNode, NamedSyntaxNode, SyntaxNode, ast::{ExpressionTraversal, ExpressionVisitor}};


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
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralInt(i32);

pub type LiteralIntNode<'script> = SyntaxNode<'script, LiteralInt>;

impl NamedSyntaxNode for LiteralIntNode<'_> {
    const NODE_KIND: &'static str = "literal_int";
}

impl LiteralIntNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralInt, LiteralValueError> {
        let s = self.text(doc).ok_or(LiteralValueError::NodeMissing)?;
        let i = s.parse::<i32>()?;
        Ok(LiteralInt(i))
    }
}

impl Debug for LiteralIntNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralInt")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralIntNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralFloat(f32);

pub type LiteralFloatNode<'script> = SyntaxNode<'script, LiteralFloat>;

impl NamedSyntaxNode for LiteralFloatNode<'_> {
    const NODE_KIND: &'static str = "literal_float";
}

impl LiteralFloatNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralFloat, LiteralValueError> {
        let s = self.text(doc).ok_or(LiteralValueError::NodeMissing)?;

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
        write!(f, "LiteralFloat")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralFloatNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralBool(bool);

pub type LiteralBoolNode<'script> = SyntaxNode<'script, LiteralBool>;

impl NamedSyntaxNode for LiteralBoolNode<'_> {
    const NODE_KIND: &'static str = "literal_bool";
}

impl LiteralBoolNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralBool, LiteralValueError> {
        let s = self.text(doc).ok_or(LiteralValueError::NodeMissing)?;
        let b = s.parse::<bool>()?;
        Ok(LiteralBool(b))
    }
}

impl Debug for LiteralBoolNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralBool")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralBoolNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralString(String);

pub type LiteralStringNode<'script> = SyntaxNode<'script, LiteralString>;

impl NamedSyntaxNode for LiteralStringNode<'_> {
    const NODE_KIND: &'static str = "literal_string";
}

impl LiteralStringNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralString, LiteralValueError> {
        let s = self.text(doc).ok_or(LiteralValueError::NodeMissing)?;

        let s = s[1..s.len()-1] // eliminate surrounding quotes
        .replace(r#"\""#, r#"""#); // escape internal quotes

        Ok(LiteralString(s))
    }
}

impl Debug for LiteralStringNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralString")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralStringNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralName(String);

pub type LiteralNameNode<'script> = SyntaxNode<'script, LiteralName>;

impl NamedSyntaxNode for LiteralNameNode<'_> {
    const NODE_KIND: &'static str = "literal_name";
}

impl LiteralNameNode<'_> {
    pub fn value(&self, doc: &ScriptDocument) -> Result<LiteralName, LiteralValueError> {
        let s = self.text(doc).ok_or(LiteralValueError::NodeMissing)?;

        let s = s[1..s.len()-1].to_string(); // eliminate surrounding quotes
        // I'm not sure if names can have escaping quotes
        // They are supposed to be rather simple in their form
        // For now I'll asume they can't have any escape sequences

        Ok(LiteralName(s))
    }
}

impl Debug for LiteralNameNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralName")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralNameNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNull;

pub type LiteralNullNode<'script> = SyntaxNode<'script, LiteralNull>;

impl NamedSyntaxNode for LiteralNullNode<'_> {
    const NODE_KIND: &'static str = "literal_null";
}

impl LiteralNullNode<'_> {}

impl Debug for LiteralNullNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralNull")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralNullNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'script> {
    Int(LiteralIntNode<'script>),
    Float(LiteralFloatNode<'script>),
    Bool(LiteralBoolNode<'script>),
    String(LiteralStringNode<'script>),
    Name(LiteralNameNode<'script>),
    Null(LiteralNullNode<'script>)
}

pub type LiteralNode<'script> = SyntaxNode<'script, Literal<'script>>;

impl NamedSyntaxNode for LiteralNode<'_> {
    const NODE_KIND: &'static str = "literal";
}

impl LiteralNode<'_> {
    pub fn value(&self) -> Literal<'_> {
        let child = self.first_child(true).unwrap();
        match child.tree_node.kind() {
            LiteralIntNode::NODE_KIND => Literal::Int(child.into()),
            LiteralFloatNode::NODE_KIND => Literal::Float(child.into()),
            LiteralBoolNode::NODE_KIND => Literal::Bool(child.into()),
            LiteralStringNode::NODE_KIND => Literal::String(child.into()),
            LiteralNameNode::NODE_KIND => Literal::Name(child.into()),
            LiteralNullNode::NODE_KIND => Literal::Null(child.into()),
            _ => panic!("Unknown literal type: {}", child.tree_node.kind())
        }
    }
}

impl Debug for LiteralNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LiteralNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for LiteralNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_literal_expr(self);
    }
}
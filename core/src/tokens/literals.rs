use std::fmt::Debug;
use std::error::Error;
use ropey::Rope;
use shrinkwraprs::Shrinkwrap;
use crate::{NamedSyntaxNode, SyntaxNode, ast::{ExpressionTraversal, ExpressionVisitor}};


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralInt(i32);

impl NamedSyntaxNode for LiteralInt {
    const NODE_NAME: &'static str = "literal_int";
}

impl SyntaxNode<'_, LiteralInt> {
    pub fn value(&self, rope: &Rope) -> Result<LiteralInt, impl Error> {
        self.text(rope).parse::<i32>().map(|i| LiteralInt(i))
    }
}

impl Debug for SyntaxNode<'_, LiteralInt> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralInt")
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralFloat(f32);

impl NamedSyntaxNode for LiteralFloat {
    const NODE_NAME: &'static str = "literal_float";
}

impl SyntaxNode<'_, LiteralFloat> {
    pub fn value(&self, rope: &Rope) -> Result<LiteralFloat, impl Error> {
        let s = self.text(rope);

        // trim the optional trailing 'f'
        let s = if s.chars().last().unwrap() == 'f' { 
            &s[..s.len() - 1] 
        } else { 
            &s
        };

        s.parse::<f32>().map(|f| LiteralFloat(f))
    }
}

impl Debug for SyntaxNode<'_, LiteralFloat> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralFloat")
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralBool(bool);

impl NamedSyntaxNode for LiteralBool {
    const NODE_NAME: &'static str = "literal_bool";
}

impl SyntaxNode<'_, LiteralBool> {
    pub fn value(&self, rope: &Rope) -> Result<LiteralBool, impl Error> {
        self.text(rope).parse::<bool>().map(|b| LiteralBool(b))
    }
}

impl Debug for SyntaxNode<'_, LiteralBool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralBool")
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralString(String);

impl NamedSyntaxNode for LiteralString {
    const NODE_NAME: &'static str = "literal_string";
}

impl SyntaxNode<'_, LiteralString> {
    pub fn value(&self, rope: &Rope) -> LiteralString {
        let s = self.text(rope);

        let s = s[1..s.len()-1] // eliminate surrounding quotes
        .replace(r#"\""#, r#"""#); // escape internal quotes

        LiteralString(s)
    }
}

impl Debug for SyntaxNode<'_, LiteralString> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralString")
    }
}


#[derive(Shrinkwrap, Debug, Clone, PartialEq)]
pub struct LiteralName(String);

impl NamedSyntaxNode for LiteralName {
    const NODE_NAME: &'static str = "literal_name";
}

impl SyntaxNode<'_, LiteralName> {
    pub fn value(&self, rope: &Rope) -> LiteralName {
        let s = self.text(rope);

        let s = s[1..s.len()-1].to_string(); // eliminate surrounding quotes
        // I'm not sure if names can have escaping quotes
        // They are supposed to be rather simple in their form
        // For now I'll asume they can't have any escape sequences

        LiteralName(s)
    }
}

impl Debug for SyntaxNode<'_, LiteralName> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralName")
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNull;

impl NamedSyntaxNode for LiteralNull {
    const NODE_NAME: &'static str = "literal_null";
}

impl SyntaxNode<'_, LiteralNull> {}

impl Debug for SyntaxNode<'_, LiteralNull> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteralNull")
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'script> {
    Int(SyntaxNode<'script, LiteralInt>),
    Float(SyntaxNode<'script, LiteralFloat>),
    Bool(SyntaxNode<'script, LiteralBool>),
    String(SyntaxNode<'script, LiteralString>),
    Name(SyntaxNode<'script, LiteralName>),
    Null(SyntaxNode<'script, LiteralNull>)
}

impl NamedSyntaxNode for Literal<'_> {
    const NODE_NAME: &'static str = "literal";
}

impl SyntaxNode<'_, Literal<'_>> {
    pub fn value(&self) -> Literal<'_> {
        let child = self.first_child(true).unwrap();
        match child.tree_node.kind() {
            LiteralInt::NODE_NAME => Literal::Int(child.into()),
            LiteralFloat::NODE_NAME => Literal::Float(child.into()),
            LiteralBool::NODE_NAME => Literal::Bool(child.into()),
            LiteralString::NODE_NAME => Literal::String(child.into()),
            LiteralName::NODE_NAME => Literal::Name(child.into()),
            LiteralNull::NODE_NAME => Literal::Null(child.into()),
            _ => panic!("Unknown literal type: {}", child.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, Literal<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}

impl ExpressionTraversal for SyntaxNode<'_, Literal<'_>> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_literal_expr(self);
    }
}
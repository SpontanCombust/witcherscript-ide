use ropey::Rope;
use tree_sitter::{Node, Range};
use std::marker::PhantomData;

/// Represents a WitcherScript syntax tree node
/// 
/// It is a backbone of the convenience layer for AST that instead of being represented by structs with data is represented by 
/// functions through which you can traverse said tree.
/// This way parsed data is retrieved only on demand and not stored anywhere else than in tree-sitter. 
/// 
/// It works as an adapter for tree-sitter's nodes. Generic parameter T denotes the type of node, e.g. `Identifier`. 
/// It can also be just a marker type. What is important is to have a distinct type for a given node type in the parsed tree.
/// Traits can be blanket-implemented for SyntaxNode by accessing the marker type.
/// 
/// ## Arguments
/// * T - marker for the concrete type of the node; () means it can be any node type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxNode<'script, T = ()> {
    pub(crate) tree_node: Node<'script>,
    pub(crate) rope: Rope, // ropes can be cloned cheaply
    pub(crate) phantom : PhantomData<T>
}

impl<'script, T> SyntaxNode<'script, T> where T: Clone {
    /// Constructs a completely new node from a tree-sitter node and a rope 
    pub(crate) fn new(tree_node: Node<'script>, rope: Rope) -> Self {
        Self {
            tree_node,
            rope,
            phantom: PhantomData,
        }
    }

    /// Interpret this node into a node with a different underlying type
    /// Gives no guarantees as to whether that target type is actually valid
    pub(crate) fn into<U>(self) -> SyntaxNode<'script, U> {
        SyntaxNode::<'_, U> {
            tree_node: self.tree_node,
            rope: self.rope,
            phantom: PhantomData
        }
    }

    /// Consume self, replace its tree-sitter node and return 'any' node
    pub(crate) fn replace_node(self, node: Node<'script>) -> SyntaxNode<'_, ()> {
        SyntaxNode::<'_, ()> {
            tree_node: node,
            rope: self.rope,
            phantom: PhantomData
        }
    }


    /// Returns an iterator over non-error children of this node as 'any' nodes
    pub(crate) fn children(&self, named: Option<bool>) -> impl Iterator<Item = SyntaxNode<'_, ()>> {
        let mut cursor = self.tree_node.walk();
        let name_nodes = self.tree_node
            .children(&mut cursor)
            .filter(|n| !n.is_error())
            .filter(|n| if let Some(named) = named { n.is_named() == named } else { true })
            .collect::<Vec<_>>();

        name_nodes.into_iter()
            .map(|n| self.clone().replace_node(n))
    }

    /// Returns the first non-error child of this node as an 'any' node
    pub(crate) fn first_child(&self, named: Option<bool>) -> Option<SyntaxNode<'_, ()>> {
        self.children(named).next()
    }

    /// Returns the first non-error child of this node with a given field name as an 'any' node
    pub(crate) fn field_child(&self, field: &'static str) -> Option<SyntaxNode<'_, ()>> {
        self.field_children(field).next()
    }

    /// Returns an iterator over non-error children of this node with a fiven field name
    pub(crate) fn field_children(&self, field: &'static str) -> impl Iterator<Item = SyntaxNode<'_, ()>> {
        let mut cursor = self.tree_node.walk();
        let name_nodes = self.tree_node
            .children_by_field_name(field, &mut cursor)
            .filter(|n| !n.is_error())
            .collect::<Vec<_>>();

        name_nodes.into_iter()
            .map(|n| self.clone().replace_node(n))
    }


    /// Returns the span at which this node is located in the text document
    pub fn span(&self) -> Range {
        self.tree_node.range()
    }

    /// Returns text that this node spans in the text document
    pub fn text(&self) -> String {
        let pos_span = self.tree_node.start_position() .. self.tree_node.end_position();
        let byte_span = self.rope.line_to_char(pos_span.start.row) + pos_span.start.column .. self.rope.line_to_char(pos_span.end.row) + pos_span.end.column;
        let slice = self.rope.slice(byte_span);
        slice.to_string()
    }
}


pub trait NamedSyntaxNode {
    const NODE_NAME: &'static str;
}

impl<T> NamedSyntaxNode for SyntaxNode<'_, T> where T: NamedSyntaxNode {
    const NODE_NAME: &'static str = T::NODE_NAME;
}
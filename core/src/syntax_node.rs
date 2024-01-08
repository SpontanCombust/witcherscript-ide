use lsp_types::{Range, Position};
use ropey::Rope;
use tree_sitter::Node;
use std::{marker::PhantomData, fmt::Debug};
use crate::ast::{ErrorNode, UnnamedNode};


/// Represents a WitcherScript syntax tree node
/// 
/// It is a backbone of the strong typed layer for AST that instead of being represented by structs with data is represented by 
/// functions through which you can traverse said tree.
/// This way parsed data is retrieved only on demand and not stored anywhere else than in tree-sitter. 
/// 
/// It works as an adapter for tree-sitter's nodes. Generic parameter T denotes the type of node, e.g. `Identifier`. 
/// It can be just a marker type. What is important is to have a distinct type for a given node type in the parsed tree.
/// 
/// ## Arguments
/// * T - marker for the concrete type of the node; () means it can be any node type
#[derive(Clone, PartialEq, Eq)]
pub struct SyntaxNode<'script, T = ()> {
    pub(crate) tree_node: Node<'script>,
    pub(crate) phantom : PhantomData<T>
}

impl<'script, T> SyntaxNode<'script, T> where T: Clone {
    /// Constructs a completely new node from a tree-sitter node and a rope 
    pub(crate) fn new(tree_node: Node<'script>) -> Self {
        Self {
            tree_node,
            phantom: PhantomData,
        }
    }

    /// Interpret this node into a node with a different underlying type.
    /// Gives no guarantees as to whether that target type is actually valid
    pub(crate) fn into<U>(self) -> SyntaxNode<'script, U> {
        SyntaxNode::<'_, U> {
            tree_node: self.tree_node,
            phantom: PhantomData
        }
    }

    //TODO make public
    /// Returns an iterator over non-error children of this node as 'any' nodes
    pub(crate) fn children(&self, must_be_named: bool) -> impl Iterator<Item = SyntaxNode<'_, ()>> {
        let mut cursor = self.tree_node.walk();
        let name_nodes = self.tree_node
            .children(&mut cursor)
            .filter(|n| !n.is_error() && !n.is_extra())
            .filter(|n| if must_be_named { n.is_named() } else { true })
            .collect::<Vec<_>>();

        name_nodes.into_iter()
            .map(|n| SyntaxNode::new(n))
    }

    /// Returns the first non-error child of this node as an 'any' node
    pub(crate) fn first_child(&self, must_be_named: bool) -> Option<SyntaxNode<'_, ()>> {
        self.children(must_be_named).next()
    }

    /// Returns the first non-error child of this node with a given field name as an 'any' node
    pub(crate) fn field_child(&self, field: &'static str) -> Option<SyntaxNode<'_, ()>> {
        self.field_children(field).next()
    }

    /// Returns an iterator over named, non-error children of this node with a given field name
    pub(crate) fn field_children(&self, field: &'static str) -> impl Iterator<Item = SyntaxNode<'_, ()>> {
        let mut cursor = self.tree_node.walk();
        let name_nodes = self.tree_node
            .children_by_field_name(field, &mut cursor)
            .filter(|n| !n.is_error() && n.is_named())
            .collect::<Vec<_>>();

        name_nodes.into_iter()
            .map(|n| SyntaxNode::new(n))
    }


    pub fn error_children(&self) -> impl Iterator<Item = ErrorNode> {
        let mut cursor = self.tree_node.walk();

        let error_nodes = self.tree_node
            .children(&mut cursor)
            .filter(|n| n.is_error())
            .collect::<Vec<_>>();

        error_nodes.into_iter().map(|n| ErrorNode {
            tree_node: n,
            phantom: PhantomData
        })
    }

    pub fn unnamed_children(&self) -> impl Iterator<Item = UnnamedNode> {
        let mut cursor = self.tree_node.walk();

        let unnamed_nodes = self.tree_node
            .children(&mut cursor)
            .filter(|n| !n.is_named() && !n.is_extra())
            .collect::<Vec<_>>();

        unnamed_nodes.into_iter().map(|n| UnnamedNode {
            tree_node: n,
            phantom: PhantomData
        })
    }


    /// Returns the span at which this node is located in the text document
    pub fn span(&self) -> Range {
        let r = self.tree_node.range();
        Range::new(
            Position::new(r.start_point.row as u32, r.start_point.column as u32),
            Position::new(r.end_point.row as u32, r.end_point.column as u32)
        )
    }

    pub fn is_missing(&self) -> bool {
        self.tree_node.is_missing()
    }

    /// Returns text that this node spans in the text document
    /// If the node is missing returns None
    pub fn text(&self, rope: &Rope) -> Option<String> {
        if self.is_missing() {
            None
        } else {
            let pos_span = self.tree_node.start_position() .. self.tree_node.end_position();
            let char_span = rope.line_to_char(pos_span.start.row) + pos_span.start.column .. rope.line_to_char(pos_span.end.row) + pos_span.end.column;
            let slice = rope.slice(char_span);
            Some(slice.to_string())
        }
    }
}


impl Debug for SyntaxNode<'_, ()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("tree_node", &self.tree_node)
            .finish()
    }
}


/// Describes the name, by which a node is identified in tree-sitter's grammar
pub trait NamedSyntaxNode {
    const NODE_KIND: &'static str;
}

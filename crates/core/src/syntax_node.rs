use std::{borrow::Cow, cell::Cell, fmt::Debug, marker::PhantomData};
use lsp_types as lsp;
use tree_sitter as ts;
use crate::{SyntaxError, script_document::ScriptDocument};


/// Represents a WitcherScript syntax tree node
/// 
/// It is a backbone of the strong typed layer for AST that instead of being represented by structs with data is represented by 
/// functions through which you can traverse said tree.
/// This way parsed data is retrieved only on demand and not stored anywhere else than in tree-sitter. 
/// 
/// It works as an adapter for tree-sitter's nodes. Generic parameter T denotes the type of node, e.g. `Identifier`. 
/// It can be just a marker type. What is important is to have a distinct type for a given node type in the parsed tree.
pub struct SyntaxNode<'script, T> {
    pub(crate) tree_node: ts::Node<'script>,
    phantom : PhantomData<T>,
    cursor: Cell<Option<ts::TreeCursor<'script>>>
}

impl<'script, T> SyntaxNode<'script, T> {
    /// Constructs a completely new node from a tree-sitter node
    pub(crate) fn new(tree_node: ts::Node<'script>) -> Self {
        Self {
            cursor: Cell::new(None),
            tree_node,
            phantom: PhantomData,
        }
    }

    /// Interpret this node into a node with a different underlying type.
    /// Gives no guarantees as to whether that target type is actually valid, so it's not exposed by the crate
    pub(crate) fn into<U>(self) -> SyntaxNode<'script, U> {
        SyntaxNode::new(self.tree_node)
    }

    pub fn into_any(self) -> AnyNode<'script> {
        AnyNode::new(self.tree_node)
    }

    /// Returns an iterator over non-error children of this node as AnyNodes
    pub fn children(&self) -> SyntaxNodeChildren<'script> {
        SyntaxNodeChildren::new(&self.tree_node, None, false)
    }

    /// Returns an iterator over non-error named children of this node as AnyNodes
    pub(crate) fn named_children(&self) -> SyntaxNodeChildren<'script> {
        SyntaxNodeChildren::new(&self.tree_node, None, true)
    }

    /// Returns the first non-error child of this node as an AnyNodes
    pub(crate) fn first_child(&self, must_be_named: bool) -> Option<AnyNode<'script>> {
        self.use_cursor(move |cursor| {
            let mut it = SyntaxNodeChildren::new(&self.tree_node, Some(cursor), must_be_named);
            let child = it.next();
            (it.cursor, child)
        })
    }

    /// Returns the first non-error child of this node with a given field name as an AnyNodes
    pub(crate) fn field_child(&self, field: &'static str) -> Option<AnyNode<'script>> {
        self.use_cursor(move |cursor| {
            let mut it = SyntaxNodeFieldChildren::new(&self.tree_node, Some(cursor), field);
            let child = it.next();
            (it.cursor, child)
        })
    }

    /// Returns an iterator over named, non-error children of this node with a given field name
    pub(crate) fn field_children(&self, field: &'static str) -> SyntaxNodeFieldChildren<'script> {
        SyntaxNodeFieldChildren::new(&self.tree_node, None, field)
    }

    /// Invoke a function using cursor stored in self. The invoked function should return back the cursor it got in the parameter.
    /// Cursor is created only for the first and only time on the first call of [`Self::use_cursor`] on self.
    /// Thanks to this method a new cursor doesn't need to be unnecesaily allocated 
    /// when the return value doesn't borrow self in any way, like when getting a single child node.
    pub(crate) fn use_cursor<F, R>(&self, f: F) -> R
    where F: Fn(ts::TreeCursor<'script>) -> (ts::TreeCursor<'script>, R) {
        // Extract the cursor from self and once the result is fetched get it back and put it back into self.
        // Cell is needed to be able to take the value from self non-mutably.
        let mut cursor = self.cursor.replace(None).unwrap_or(self.tree_node.walk());

        let ret;
        (cursor, ret) = f(cursor);

        cursor.reset(self.tree_node);
        self.cursor.replace(Some(cursor));

        ret
    }


    /// Whether any nodes descending from this node are errors
    pub fn has_errors(&self) -> bool {
        self.use_cursor(|mut cursor| {
            let any_errors = self.tree_node
                .children(&mut cursor)
                .any(|child| child.has_error());
    
            (cursor, any_errors)
        })
    }

    /// Returns an iterator over ERROR or missing children nodes
    pub fn errors(&self) -> Vec<SyntaxError> {
        self.use_cursor(|mut cursor| {
            let mut errors = Vec::new();
            for n in self.tree_node.children(&mut cursor) {
                if n.is_error() {
                    errors.push(SyntaxError::Invalid(AnyNode::new(n)));
                } else if n.is_missing() {
                    errors.push(SyntaxError::Missing(AnyNode::new(n)));
                }
            }
    
            (cursor, errors)
        })
    }

    //TODO unnamed children


    /// Returns the range at which this node is located in the text document
    #[inline]
    pub fn range(&self) -> lsp::Range {
        let r = self.tree_node.range();
        lsp::Range::new(
            lsp::Position::new(r.start_point.row as u32, r.start_point.column as u32),
            lsp::Position::new(r.end_point.row as u32, r.end_point.column as u32)
        )
    }

    #[inline]
    pub fn spans_position(&self, position: lsp::Position) -> bool {
        let r = self.range();
        if r.start.line < r.end.line {
            r.start.line <= position.line && position.line <= r.end.line
        } else if r.start.line == position.line {
            r.start.character <= position.character && position.character <= r.end.character
        } else {
            false
        }
    }

    #[inline]
    pub fn is_missing(&self) -> bool {
        // More reliable way than tree_sitter::Node::is_missing().
        // That's because in the node tree only leaves can be marked as missing.
        // TS is also a bit annoying when it comes to those leave nodes.
        // Named nodes can never be leaves, they always contain an unnamed node inside them,
        // even if this node corresponds to a single token.
        let range = self.range();
        range.start == range.end
    }

    /// Returns text that this node spans in the text document
    /// If the node is missing returns None
    #[inline]
    pub fn text<'d>(&self, doc: &'d ScriptDocument) -> Option<Cow<'d, str>> {
        if self.is_missing() {
            None
        } else {
            Some(doc.text_at(self.range()))
        }
    }


    /// Returns tree-sitter's node structure in a form of XML.
    /// Use for debugging purposes.
    pub fn debug_ts_tree(&self, doc: &ScriptDocument) -> String {
        self.use_cursor(|mut cursor| {
            let mut buf = String::new();
    
            let mut needs_newline = false;
            let mut indent_level = 0;
            let mut did_visit_children = false;
            let mut tags: Vec<&str> = Vec::new();
    
            loop {
                let node = cursor.node();
                let is_named = node.is_named();
                if did_visit_children {
                    if is_named {
                        let tag = tags.pop();
                        buf += &format!("</{}>\n", tag.expect("there is a tag"));
                        needs_newline = true;
                    }
                    if cursor.goto_next_sibling() {
                        did_visit_children = false;
                    } else if cursor.goto_parent() {
                        did_visit_children = true;
                        indent_level -= 1;
                    } else {
                        break;
                    }
                } else {
                    if is_named {
                        if needs_newline {
                            buf += &format!("\n");
                        }
                        for _ in 0..indent_level {
                            buf += &format!("  ");
                        }
                        buf += &format!("<{}", node.kind());
                        if let Some(field_name) = cursor.field_name() {
                            buf += &format!(" type=\"{}\"", field_name);
                        }
                        buf += &format!(">");
                        tags.push(node.kind());
                        needs_newline = true;
                    }
                    if cursor.goto_first_child() {
                        did_visit_children = false;
                        indent_level += 1;
                    } else {
                        let node_range = node.range();
                        let lsp_range = lsp::Range::new(
                            lsp::Position::new(node_range.start_point.row as u32, node_range.start_point.column as u32),
                            lsp::Position::new(node_range.end_point.row as u32, node_range.end_point.column as u32)
                        );
    
                        buf += &doc.text_at(lsp_range);
                        did_visit_children = true;
                    }
                    if node.is_missing() {
                        buf += &format!("MISSING");
                    }
                }
            }
    
            (cursor, buf)
        })
    }
}

impl<T> Clone for SyntaxNode<'_, T> {
    fn clone(&self) -> Self {
        Self {
            tree_node: self.tree_node.clone(),
            phantom: PhantomData,
            cursor: Cell::new(None)
        }
    }
}

impl<T> PartialEq for SyntaxNode<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.tree_node == other.tree_node
    }
}

impl<T> Eq for SyntaxNode<'_, T> {}



/// Default opaque node type not possessing any additional capabilities.
pub type AnyNode<'script> = SyntaxNode<'script, ()>;


impl Debug for AnyNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("tree_node", &self.tree_node)
            .finish()
    }
}



pub struct SyntaxNodeChildren<'script> {
    cursor: ts::TreeCursor<'script>,
    any_children_left: bool,
    must_be_named: bool
}

impl<'script> SyntaxNodeChildren<'script> {
    fn new(tree_node: &ts::Node<'script>, cursor: Option<ts::TreeCursor<'script>>, must_be_named: bool) -> Self {
        let mut cursor = cursor.unwrap_or(tree_node.walk());
        let any_children_left = cursor.goto_first_child(); 

        Self {
            cursor,
            any_children_left,
            must_be_named
        }       
    }
}

impl<'script> Iterator for SyntaxNodeChildren<'script> {
    type Item = AnyNode<'script>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.any_children_left {
            let mut n = self.cursor.node();
            while n.is_error() || n.is_extra() || (self.must_be_named && !n.is_named()) {
                if self.cursor.goto_next_sibling() {
                    n = self.cursor.node();
                } else {
                    return None;
                }
            }

            self.any_children_left = self.cursor.goto_next_sibling();
            Some(AnyNode::new(n))
        } else {
            None
        }
    }
}


pub struct SyntaxNodeFieldChildren<'script> {
    cursor: ts::TreeCursor<'script>,
    any_children_left: bool,
    field_id: u16
}

impl<'script> SyntaxNodeFieldChildren<'script> {
    fn new(tree_node: &ts::Node<'script>, cursor: Option<ts::TreeCursor<'script>>, field_name: &str) -> Self {
        let mut cursor = cursor.unwrap_or(tree_node.walk());
        let any_children_left = cursor.goto_first_child();

        let field_id = tree_sitter_witcherscript::language()
            .field_id_for_name(field_name)
            .expect(&format!("Unknown field name {}", field_name));
    
        Self {
            cursor,
            any_children_left,
            field_id
        }       
    }
}

impl<'script> Iterator for SyntaxNodeFieldChildren<'script> {
    type Item = AnyNode<'script>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.any_children_left {
            let mut n = self.cursor.node();
            while self.cursor.field_id() != Some(self.field_id) || n.is_error() || n.is_extra() || !n.is_named() {
                if self.cursor.goto_next_sibling() {
                    n = self.cursor.node();
                } else {
                    return None;
                }
            }

            self.any_children_left = self.cursor.goto_next_sibling();
            Some(AnyNode::new(n))
        } else {
            None
        }
    }
}



/// Describes the name, by which a node is identified in tree-sitter's grammar
pub trait NamedSyntaxNode {
    const NODE_KIND: &'static str;
}

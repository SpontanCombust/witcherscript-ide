//! Common types for communication between the server and client

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tower_lsp::lsp_types as lsp;
use witcherscript_project::{content_graph::GraphNode, content::*};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentInfo {
    pub content_uri: lsp::Url,
    pub content_kind: ContentKind,
    pub content_name: String,
    pub scripts_root_uri: lsp::Url,
    pub is_in_workspace: bool,
    pub is_in_repository: bool,
    pub is_native: bool
}

impl From<&GraphNode> for ContentInfo {
    fn from(n: &GraphNode) -> Self {
        Self {
            content_uri: n.content.path().to_uri(),
            content_kind: n.content.as_ref().into(),
            content_name: n.content.content_name().to_string(),
            scripts_root_uri: n.content.source_tree_root().to_uri(),
            is_in_workspace: n.in_workspace,
            is_in_repository: n.in_repository,
            is_native: n.is_native
        }
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ContentKind {
    Raw = 0,
    WideProject = 1,
    RedkitProject = 2
}

impl From<&dyn Content> for ContentKind {
    fn from(value: &dyn Content) -> Self {
        let any = value.as_any();

        if any.is::<ProjectDirectory>() {
            ContentKind::WideProject
        } 
        else if any.is::<RedkitProjectDirectory>() {
            ContentKind::RedkitProject
        }
        else {
            ContentKind::Raw
        }
    }
}
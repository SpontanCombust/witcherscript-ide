//! Common types for communication between the server and client

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types as lsp;
use witcherscript_project::content_graph::GraphNode;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentInfo {
    pub content_uri: lsp::Url,
    pub scripts_root_uri: lsp::Url,
    pub content_name: String,
    pub is_in_workspace: bool,
    pub is_in_repository: bool
}

impl From<&GraphNode> for ContentInfo {
    fn from(n: &GraphNode) -> Self {
        Self {
            content_uri: n.content.path().to_uri(),
            scripts_root_uri: n.content.source_tree_root().to_uri(),
            content_name: n.content.content_name().to_string(),
            is_in_workspace: n.in_workspace,
            is_in_repository: n.in_repository
        }
    }
}
//! Common types for communication between the server and client

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types as lsp;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentInfo {
    pub content_uri: lsp::Url,
    pub scripts_root_uri: lsp::Url,
    pub content_name: String,
    pub is_in_workspace: bool,
    pub is_in_repository: bool
}

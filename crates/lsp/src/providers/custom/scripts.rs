use abs_path::AbsPath;
use tower_lsp::jsonrpc::{self, Result};
use crate::{requests::{self, ContentInfo}, Backend};


pub trait LangaugeServerCustomScripts {
    async fn parent_content(&self, params: requests::scripts::parent_content::Parameters) -> Result<requests::scripts::parent_content::Response>;
}


impl LangaugeServerCustomScripts for Backend {
    async fn parent_content(&self, params: requests::scripts::parent_content::Parameters) -> Result<requests::scripts::parent_content::Response> {
        let script_path: AbsPath;
        if let Ok(path) = AbsPath::try_from(params.script_uri) {
            script_path = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("script_uri parameter is not a valid file URI"));
        }

        let script_state;
        if let Some(ss) = self.scripts.get(&script_path) {
            script_state = ss;
        } else {
            return Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1020),
                message: "This script file is uknown to the langauge server".into(),
                data: None
            })
        }

        let parent_content_path;
        if let Some(content_info) = &script_state.content_info {
            parent_content_path = content_info.content_path.clone();
        } else {
            return Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1021),
                message: "Script does not belong to any content in the content graph".into(),
                data: None
            })
        }

        if let Some(n) = self.content_graph.read().await.get_node_by_path(&parent_content_path) {
            Ok(requests::scripts::parent_content::Response {
                parent_content_info: ContentInfo { 
                    content_uri: parent_content_path.into(), 
                    scripts_root_uri: n.content.source_tree_root().to_uri(), 
                    content_name: n.content.content_name().into(),
                    is_in_workspace: n.in_workspace,
                    is_in_repository: n.in_repository
                }
            })
        } else {
            Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1022),
                message: "Could not find content in content graph".into(),
                data: None
            })
        }
    }
}
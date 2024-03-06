use serde::Deserialize;
use serde::Serialize;
use tower_lsp::lsp_types as lsp;


pub mod create_project {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Parameters {
        pub directory_uri: lsp::Url
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub manifest_uri: lsp::Url,
        pub manifest_content_name_range: lsp::Range
    }

    pub const METHOD: &'static str = "witcherscript-ide/workspace/createProject";
}

pub mod script_ast {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Parameters {
        pub script_uri: lsp::Url
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub ast: String
    }

    pub const METHOD: &'static str = "witcherscript-ide/debug/scriptAst";
}


//TODO project name for script request
/* 
pub struct ProjectNameForScript;

#[derive(Serialize, Deserialize)]
pub struct ProjectNameForScriptParams {
    script_url: lsp::Url
}

#[derive(Serialize, Deserialize)]
pub struct ProjectNameForScriptResponse {
    project_name: Option<String>
}

impl lsp::request::Request for ProjectNameForScript {
    type Params = ProjectNameForScriptParams;
    type Result = ProjectNameForScriptResponse;

    const METHOD: &'static str = "custom/projects/projectNameForScript";
}

impl Backend {
    // custom/projects/projectNameForScript
    pub async fn request_project_name_for_script(&self, params: ProjectNameForScriptParams) -> jsonrpc::Result<ProjectNameForScriptResponse> {
        if let Ok(path) = params.script_url.to_file_path() {
            let graph = self.content_graph.read().await;
            let project_name = graph.get_workspace_projects().iter()
                .find(|proj| path.strip_prefix(proj.path()).is_ok())
                .map(|proj| proj.content_name().to_string());

            Ok(ProjectNameForScriptResponse { 
                project_name
            })
        } else {
            Err(jsonrpc::Error::invalid_params("script_url is not file path"))
        }
    }
}
*/
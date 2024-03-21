use std::io::Write;
use std::path::PathBuf;
use tower_lsp::{jsonrpc, lsp_types as lsp};
use tower_lsp::jsonrpc::Result;
use witcherscript_project::Manifest;
use crate::Backend;
use super::requests::{self, ContentInfo};


impl Backend {
    pub async fn handle_projects_create_request(&self, params: requests::projects::create::Parameters) -> Result<requests::projects::create::Response> {
        let project_dir: PathBuf;
        if let Ok(path) = params.directory_uri.to_file_path() {
            project_dir = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("directory_uri parameter is not a file URI"));
        }

        if !project_dir.exists() {
            return Err(jsonrpc::Error { 
                // probably any code outside of protocol's reserve range should be ok
                // https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#errorCodes
                code: jsonrpc::ErrorCode::ServerError(-1000), 
                message: "Directory does not exist".into(), 
                data: None 
            })
        }

        let manifest_path = project_dir.join(Manifest::FILE_NAME);
        if manifest_path.exists() {
            return Err(jsonrpc::Error { 
                code: jsonrpc::ErrorCode::ServerError(-1001), 
                message: "Script project already exists in the chosen directory".into(), 
                data: None
            })
        }


        let scripts_path = project_dir.join("scripts");
        if !scripts_path.exists() {
            if let Err(err) = std::fs::create_dir(scripts_path) {
                return Err(jsonrpc::Error { 
                    code: jsonrpc::ErrorCode::ServerError(-1002), 
                    message: format!("File system error: {err}").into(), 
                    data: None
                })
            }
        }

        let mut manifest_file;
        match std::fs::File::create(&manifest_path) {
            Ok(file) => {
                manifest_file = file;
            },
            Err(err) => {
                return Err(jsonrpc::Error { 
                    code: jsonrpc::ErrorCode::ServerError(-1003), 
                    message: format!("File system error: {err}").into(), 
                    data: None
                })
            },
        };

        let project_name = project_dir
            .file_name().unwrap()
            .to_string_lossy()
            .to_string()
            .replace(" ", ""); // remove spaces

        let (template, manifest_content_name_range) = manifest_template(&project_name);

        if let Err(err) = manifest_file.write_all(template.as_bytes()) {
            return Err(jsonrpc::Error { 
                code: jsonrpc::ErrorCode::ServerError(-1004), 
                message: format!("File system error: {err}").into(), 
                data: None
            })
        }

        let manifest_uri = lsp::Url::from_file_path(manifest_path).unwrap();
        Ok(requests::projects::create::Response { 
            manifest_uri,
            manifest_content_name_range
        })
    }

    pub async fn handle_script_ast_request(&self, params: requests::debug::script_ast::Parameters) -> Result<requests::debug::script_ast::Response> {
        let path = params.script_uri.to_file_path().map_err(|_| jsonrpc::Error::invalid_params("script_uri parameter is not a file URI"))?;
        let script = self.scripts.get(&path).ok_or(jsonrpc::Error {
            code: jsonrpc::ErrorCode::ServerError(-1010),
            message: "Script file not found".into(),
            data: None
        })?;

        let ast = format!("{:#?}", script.root_node());

        Ok(requests::debug::script_ast::Response { 
            ast
        })
    }

    pub async fn handle_scripts_parent_content_request(&self, params: requests::scripts::parent_content::Parameters) -> Result<requests::scripts::parent_content::Response>  {
        let script_path: PathBuf;
        if let Ok(path) = params.script_uri.to_file_path() {
            script_path = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("script_uri parameter is not a valid file URI"));
        }

        let mut parent_content_path = None;
        for it in self.source_trees.iter() {
            let source_tree = it.value();
            if source_tree.contains(&script_path) {
                parent_content_path = Some(it.key().to_owned());
                break;
            }
        }

        if parent_content_path.is_none() {
            return Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1020),
                message: "Script does not belong to any content in the content graph".into(),
                data: None
            })
        }
        let parent_content_path = parent_content_path.unwrap();
        
        if let Some(n) = self.content_graph.read().await.get_node_by_path(&parent_content_path) {
            Ok(requests::scripts::parent_content::Response {
                parent_content_info: ContentInfo { 
                    content_uri: lsp::Url::from_file_path(&parent_content_path).unwrap(), 
                    scripts_root_uri: lsp::Url::from_file_path(n.content.source_tree_root()).unwrap(), 
                    content_name: n.content.content_name().into(),
                    is_in_workspace: n.in_workspace,
                    is_in_repository: n.in_repository
                }
            })
        } else {
            Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1021),
                message: "Could not find content in content graph".into(),
                data: None
            })
        }
    }

    pub async fn handle_projects_vanilla_dependency_content_request(&self, params: requests::projects::vanilla_dependency_content::Parameters) -> Result<requests::projects::vanilla_dependency_content::Response> {
        let project_path: PathBuf;
        if let Ok(path) = params.project_uri.to_file_path() {
            project_path = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("project_uri parameter is not a valid file URI"));
        }

        let graph = self.content_graph.read().await;
        if graph.get_node_by_path(&project_path).is_none() {
            return Err(jsonrpc::Error { 
                code: jsonrpc::ErrorCode::ServerError(-1030), 
                message: "The project is absent from the content graph".into(), 
                data: None
            })
        }

        let mut content0_info = None;
        for n in graph.walk_dependencies(&project_path) {
            if n.content.content_name() == "content0" {
                content0_info = Some(ContentInfo {
                    content_uri: lsp::Url::from_file_path(n.content.path()).unwrap(),
                    scripts_root_uri: lsp::Url::from_file_path(n.content.source_tree_root()).unwrap(),
                    content_name: n.content.content_name().to_owned(),
                    is_in_workspace: n.in_workspace,
                    is_in_repository: n.in_repository
                });
            }
        }

        if let Some(content0_info) = content0_info {
            Ok(requests::projects::vanilla_dependency_content::Response {
                content0_info,
            })
        } else {
            Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1031),
                message: "Project does not depend on content0".into(),
                data: None
            })
        }
    }

    pub async fn handle_projects_list_request(&self, params: requests::projects::list::Parameters) -> Result<requests::projects::list::Response> {
        let only_from_workspace = params.only_from_workspace.unwrap_or(true);

        let mut project_infos = Vec::new();

        let graph = self.content_graph.read().await;
        for n in graph.nodes() {
            if only_from_workspace && !n.in_workspace {
                continue;
            }

            project_infos.push(ContentInfo { 
                content_uri: lsp::Url::from_file_path(n.content.path()).unwrap(), 
                scripts_root_uri: lsp::Url::from_file_path(n.content.source_tree_root()).unwrap(), 
                content_name: n.content.content_name().into(), 
                is_in_workspace: n.in_workspace, 
                is_in_repository: n.in_repository 
            })
        }

        Ok(requests::projects::list::Response {
            project_infos
        })
    }
}


fn manifest_template(project_name: &str) -> (String, lsp::Range) {
    // Serialization would've been better if not for the fact that the default behaviour for inline tables
    // is to instead create a new table with a dotted key. So it would require extra effort to make something
    // small look better.
    let text = format!(
r#"# Basic information about this project
[content]
name = "{project_name}"
version = "1.0.0"
authors = []
game_version = "4.04"

# Any dependencies that this project might need
[dependencies]
content0 = true

# For details check the manual
# https://spontancombust.github.io/witcherscript-ide/user-manual/project-manifest/
"#
    );

    // if text above is changed in any way before {project_name} the range has to be updated
    let content_name_range = lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 8 + project_name.len() as u32));

    (text, content_name_range)
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use witcherscript_project::Manifest;
    use super::manifest_template;


    #[test]
    fn test_manifest_template() {
        let (template, _) = manifest_template("modFoo_Bar");
        let manifest = Manifest::from_str(&template);
        assert!(manifest.is_ok());
    }
}
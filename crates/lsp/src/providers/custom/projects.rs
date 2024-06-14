use std::io::Write;
use abs_path::AbsPath;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::{self, Result};
use witcherscript_project::{content::VANILLA_CONTENT_NAME, Manifest};
use crate::{notifications, requests::{self, ContentInfo}, Backend};


pub trait LangaugeServerCustomProjects {
    async fn create_project(&self, params: requests::projects::create::Parameters) -> Result<requests::projects::create::Response>;

    async fn vanilla_dependency_content(&self, params: requests::projects::vanilla_dependency_content::Parameters) -> Result<requests::projects::vanilla_dependency_content::Response>;

    async fn project_list(&self, params: requests::projects::list::Parameters) -> Result<requests::projects::list::Response>;

    async fn did_import_scripts(&self, params: notifications::projects::did_import_scripts::Parameters);
}


impl LangaugeServerCustomProjects for Backend {
    async fn create_project(&self, params: requests::projects::create::Parameters) -> Result<requests::projects::create::Response> {
        let project_dir: AbsPath;
        if let Ok(path) = AbsPath::try_from(params.directory_uri) {
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

        let manifest_path = project_dir.join(Manifest::FILE_NAME).unwrap();
        if manifest_path.exists() {
            return Err(jsonrpc::Error { 
                code: jsonrpc::ErrorCode::ServerError(-1001), 
                message: "Script project already exists in the chosen directory".into(), 
                data: None
            })
        }


        let scripts_root_candidates_rel = [
            "scripts",
            "content/scripts",
            "workspace/scripts",
        ];
        
        let scripts_root_candidates_abs = 
            scripts_root_candidates_rel.iter()
            .map(|rel| project_dir.join(rel).unwrap())
            .collect::<Vec<_>>();

        let candidate_idx =
            scripts_root_candidates_abs.iter()
            .enumerate()
            .find(|(_, abs)| abs.exists())
            .map(|(i, _)| i)
            .unwrap_or(0);

        let scripts_root_rel = scripts_root_candidates_rel[candidate_idx];
        let scripts_root_abs = &scripts_root_candidates_abs[candidate_idx];

        if !scripts_root_abs.exists() {
            if let Err(err) = std::fs::create_dir(scripts_root_abs) {
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

        if !Manifest::validate_content_name(&params.project_name) {
            return Err(jsonrpc::Error { 
                code: jsonrpc::ErrorCode::ServerError(-1004), 
                message: format!("Name of the project is invalid. Please refer to the user manual to find out about proper project name.").into(), 
                data: None
            })
        }

        let template = manifest_template(&params.project_name, scripts_root_rel);

        if let Err(err) = manifest_file.write_all(template.as_bytes()) {
            return Err(jsonrpc::Error { 
                code: jsonrpc::ErrorCode::ServerError(-1005), 
                message: format!("File system error: {err}").into(), 
                data: None
            })
        }

        //TODO reload content graph if manifest was created in workspace

        Ok(requests::projects::create::Response { 
            manifest_uri: manifest_path.into()
        })
    }

    async fn vanilla_dependency_content(&self, params: requests::projects::vanilla_dependency_content::Parameters) -> Result<requests::projects::vanilla_dependency_content::Response> {
        let project_path: AbsPath;
        if let Ok(abs_path) = AbsPath::try_from(params.project_uri) {
            project_path = abs_path;
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
            if n.content.content_name() == VANILLA_CONTENT_NAME {
                content0_info = Some(ContentInfo {
                    content_uri: n.content.path().to_uri(),
                    scripts_root_uri: n.content.source_tree_root().to_uri(),
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

    async fn project_list(&self, params: requests::projects::list::Parameters) -> Result<requests::projects::list::Response> {
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

    async fn did_import_scripts(&self, params: notifications::projects::did_import_scripts::Parameters) {
        let paths: Vec<AbsPath> = 
            params.imported_scripts_uris.into_iter()
            .filter_map(|uri| AbsPath::try_from(uri).ok())
            .collect();

        if paths.is_empty() {
            return;
        }

        if let Some(content_path) = self.content_graph.read().await.strip_content_path_prefix(&paths[0]) {
            self.scan_source_tree(&content_path).await;
            self.reporter.commit_all_diagnostics().await;
        } else {
            self.reporter.log_error("Imported files do no belong to a known content!").await;
        }
    }
}


fn manifest_template(project_name: &str, scripts_root: &str) -> String {
    // Serialization would've been better if not for the fact that the default behaviour for inline tables
    // is to instead create a new table with a dotted key. 
    // So it would require extra effort to make something small look better when you can just write the template by hand.
    let text = format!(
r#"# Basic information about this project
[content]
name = "{project_name}"
description = ""
version = "1.0.0"
authors = []
game_version = "4.04"
scripts_root = "{scripts_root}"

# Any dependencies that this project might need
[dependencies]
content0 = true

# For details check the manual
# https://spontancombust.github.io/witcherscript-ide/user-manual/project-system/#manifest-format
"#
    );

    text
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use witcherscript_project::Manifest;
    use super::manifest_template;


    #[test]
    fn test_manifest_template() {
        let template = manifest_template("modFoo_Bar", "scripts");
        let manifest = Manifest::from_str(&template);
        assert!(manifest.is_ok());
    }
}
use std::io::Write;

use tower_lsp::{jsonrpc, lsp_types as lsp};
use tower_lsp::jsonrpc::Result;
use witcherscript_project::Manifest;
use crate::Backend;
use super::requests::{create_project, script_ast};


impl Backend {
    pub async fn handle_create_project_request(&self, params: create_project::Parameters) -> Result<create_project::Response> {
        let project_dir;
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
        Ok(create_project::Response { 
            manifest_uri,
            manifest_content_name_range
        })
    }

    pub async fn handle_script_ast_request(&self, params: script_ast::Parameters) -> Result<script_ast::Response> {
        let path = params.script_uri.to_file_path().map_err(|_| jsonrpc::Error::invalid_params("script_uri parameter is not a file URI"))?;
        let script = self.scripts.get(&path).ok_or(jsonrpc::Error {
            code: jsonrpc::ErrorCode::ServerError(-1100),
            message: "Script file not found".into(),
            data: None
        })?;

        let ast = format!("{:#?}", script.root_node());

        Ok(script_ast::Response { 
            ast
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
# The allowed formats for now are:
# modTest1 = true   # get the dependency from a repository
# modTest2 = {{ path = "../path/to/modTest2" }}     # get the dependency from a specific path
[dependencies]
content0 = true
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
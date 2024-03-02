use std::io::Write;

use tower_lsp::{jsonrpc, lsp_types as lsp};
use tower_lsp::jsonrpc::Result;
use witcherscript_project::Manifest;
use crate::Backend;
use super::requests::{CreateProjectParams, CreateProjectResponse};


impl Backend {
    pub async fn handle_create_project_request(&self, params: CreateProjectParams) -> Result<CreateProjectResponse> {
        let project_dir;
        if let Ok(path) = params.directory_uri.to_file_path() {
            project_dir = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("directory_uri parameter is not a file path"));
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

        let template = manifest_template(&project_name);

        if let Err(err) = manifest_file.write_all(template.as_bytes()) {
            return Err(jsonrpc::Error { 
                code: jsonrpc::ErrorCode::ServerError(-1004), 
                message: format!("File system error: {err}").into(), 
                data: None
            })
        }

        let manifest_uri = lsp::Url::from_file_path(manifest_path).unwrap();
        Ok(CreateProjectResponse { 
            manifest_uri
        })
    }
}

fn manifest_template(project_name: &str) -> String {
    // Serialization would've been better if not for the fact that the default behaviour for inline tables
    // is to instead create a new table with a dotted key. So it would require extra effort to make something
    // small look better.
    format!(
r#"# Basic information about this project
[content]
name = "{project_name}"
version = "1.0.0"
authors = []
game_version = "4.04"

# Any dependencies that this project might need
# The allowed formats are:
# modTest1 = true   # get the dependency from a repository
# modTest2 = {{ path = "../path/to/modTest2" }}     # get the dependency from a specific path
[dependencies]
"#
    )
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use witcherscript_project::Manifest;
    use super::manifest_template;


    #[test]
    fn test_manifest_template() {
        let template = manifest_template("modFoo_Bar");
        let manifest = Manifest::from_str(&template);
        assert!(manifest.is_ok());
    }
}
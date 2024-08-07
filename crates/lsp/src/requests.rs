use serde::Deserialize;
use serde::Serialize;
use tower_lsp::lsp_types as lsp;
use super::model;


pub mod projects {
    use super::*;

    /// Creates a script project in the specified directory.
    /// Returns path to the newly created manifest file.
    pub mod create {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub directory_uri: lsp::Url,
            pub project_name: String
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub manifest_uri: lsp::Url
        }

        pub const METHOD: &'static str = "witcherscript-ide/projects/create";
    }

    /// Lists information about currently managed projects 
    pub mod list {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            /// If is None, defaults to true
            pub only_from_workspace: Option<bool>
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub project_infos: Vec<model::ContentInfo>
        }

        pub const METHOD: &'static str = "witcherscript-ide/projects/list";
    }

    /// Returns information about content0 dependency of a given project, if it depends on it
    pub mod vanilla_dependency_content {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub project_uri: lsp::Url
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub content0_info: model::ContentInfo
        }

        pub const METHOD: &'static str = "witcherscript-ide/projects/vanillaDependencyContent";
    }

    /// Returns information about content0 content, doesn't need any source content to check as opposed to [`vanilla_dependency_content`].
    /// Returns None if doesn't find content0
    pub mod vanilla_content {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub content0_info: Option<model::ContentInfo>
        }

        pub const METHOD: &'static str = "witcherscript-ide/projects/vanillaContent";
    }

    /// Returns a list of source file paths associated with a given content sorted in alphabetical order
    pub mod source_tree {
        use std::path::PathBuf;
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub content_uri: lsp::Url
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub scripts_root_path: PathBuf,
            pub local_script_paths: Vec<PathBuf>
        }

        pub const METHOD: &'static str = "witcherscript-ide/projects/sourceTree";
    }
}

pub mod scripts {
    use super::*;

    /// Returns information about content containing the given script
    pub mod parent_content {
        use super::*;
    
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub script_uri: lsp::Url
        }
    
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub parent_content_info: model::ContentInfo
        }
    
        pub const METHOD: &'static str = "witcherscript-ide/scripts/parentContent";
    }    
}

pub mod debug {
    use super::*;

    /// Returns script file's AST representation
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

    /// Returns script file's CST (Concrete Syntax Tree) representation
    pub mod script_cst {
        use super::*;
    
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub script_uri: lsp::Url
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub cst: String
        }
    
        pub const METHOD: &'static str = "witcherscript-ide/debug/scriptCst";
    }

    /// Returns the content graph visualization in graphviz .dot format
    pub mod content_graph_dot {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub dot_graph: String
        }
    
        pub const METHOD: &'static str = "witcherscript-ide/debug/contentGraphDot";
    }

    /// Returns a human-readable report containing all code symbols in a given script file
    pub mod script_symbols {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub script_uri: lsp::Url
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub symbols: String
        }
    
        pub const METHOD: &'static str = "witcherscript-ide/debug/scriptSymbols";
    }
}

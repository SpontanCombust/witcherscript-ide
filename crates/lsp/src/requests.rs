use serde::Deserialize;
use serde::Serialize;
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
            pub project_infos: Vec<ContentInfo>
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
            pub content0_info: ContentInfo
        }

        pub const METHOD: &'static str = "witcherscript-ide/projects/vanillaDependencyContent";
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
            pub parent_content_info: ContentInfo
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

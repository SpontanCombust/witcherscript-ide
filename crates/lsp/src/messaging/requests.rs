use serde::Deserialize;
use serde::Serialize;
use tower_lsp::lsp_types as lsp;


pub mod projects {
    use super::*;

    pub mod create {
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

        pub const METHOD: &'static str = "witcherscript-ide/projects/create";
    }
}

pub mod debug {
    use super::*;

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
}

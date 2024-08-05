use serde::Deserialize;
use serde::Serialize;
use tower_lsp::lsp_types as lsp;


pub mod client {
    use super::*;

    pub mod show_foreign_script_warning {
        use super::*;
        
        pub const METHOD: &'static str = "witcherscript-ide/client/showForeignScriptWarning";

        pub struct Type;
        impl lsp::notification::Notification for Type {
            type Params = ();
        
            const METHOD: &'static str = METHOD;
        }
    }
}

pub mod projects {
    use super::*;

    /// Notification sent to the server to tell it when vanilla files have been imported to the project.
    /// All scripts are expected to have been imported into a single content.
    pub mod did_import_scripts {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub imported_scripts_uris: Vec<lsp::Url>,
        }

        pub const METHOD: &'static str = "witcherscript-ide/projects/didImportScripts";
    }

    // Sent after making changes to the content graph and performing tasks associated with that
    pub mod did_change_content_graph {
        use super::*;

        pub const METHOD: &'static str = "witcherscript-ide/projects/didChangeContentGraph";

        pub struct Type;
        impl lsp::notification::Notification for Type {
            type Params = ();
        
            const METHOD: &'static str = METHOD;
        }
    }
}

pub mod scripts {
    use super::*;

    pub mod did_finish_initial_indexing {
        use super::*;

        pub const METHOD: &'static str = "witcherscript-ide/scripts/didFinishInitialIndexing";

        pub struct Type;
        impl lsp::notification::Notification for Type {
            type Params = ();
        
            const METHOD: &'static str = METHOD;
        }
    }
    
    pub mod did_start_script_parsing {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub content_name: String,
        }

        pub const METHOD: &'static str = "witcherscript-ide/scripts/didStartScriptParsing";

        pub struct Type;
        impl lsp::notification::Notification for Type {
            type Params = Parameters;
        
            const METHOD: &'static str = METHOD;
        }
    }

    pub mod did_finish_script_parsing {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub content_name: String,
        }

        pub const METHOD: &'static str = "witcherscript-ide/scripts/didFinishScriptParsing";

        pub struct Type;
        impl lsp::notification::Notification for Type {
            type Params = Parameters;
        
            const METHOD: &'static str = METHOD;
        }
    }
}
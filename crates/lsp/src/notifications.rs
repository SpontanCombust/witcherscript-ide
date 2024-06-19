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
}

pub mod scripts {
    use super::*;

    pub mod script_parsing_started {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub content_name: String,
        }

        pub const METHOD: &'static str = "witcherscript-ide/scripts/scriptParsingStarted";

        pub struct Type;
        impl lsp::notification::Notification for Type {
            type Params = Parameters;
        
            const METHOD: &'static str = METHOD;
        }
    }

    pub mod script_parsing_finished {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Parameters {
            pub content_name: String,
        }

        pub const METHOD: &'static str = "witcherscript-ide/scripts/scriptParsingFinished";

        pub struct Type;
        impl lsp::notification::Notification for Type {
            type Params = Parameters;
        
            const METHOD: &'static str = METHOD;
        }
    }
}
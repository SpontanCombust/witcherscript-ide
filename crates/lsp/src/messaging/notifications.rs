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
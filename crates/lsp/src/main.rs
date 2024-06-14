use messaging::{notifications, requests};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types as lsp;
use tower_lsp::{LanguageServer, LspService, Server};


mod backend;
pub use backend::*;

mod providers;
mod config;
mod reporting;
mod tasks;
mod messaging;


#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
        self.initialize_impl(params).await
    }

    async fn initialized(&self, params: lsp::InitializedParams) {
        self.initialized_impl(params).await
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }


    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        self.did_open_impl(params).await
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        self.did_change_impl(params).await
    }

    async fn did_save(&self, params: lsp::DidSaveTextDocumentParams) {
        self.did_save_impl(params).await
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        self.did_close_impl(params).await
    }

    async fn did_create_files(&self, params: lsp::CreateFilesParams) {
        self.did_create_files_impl(params).await
    }

    async fn did_delete_files(&self, params: lsp::DeleteFilesParams) {
        self.did_delete_files_impl(params).await
    }

    async fn did_rename_files(&self, params: lsp::RenameFilesParams) {
        self.did_rename_files_impl(params).await
    }


    async fn did_change_configuration(&self, params: lsp::DidChangeConfigurationParams) {
        self.did_change_configuration_impl(params).await
    }

    async fn did_change_workspace_folders(&self, params: lsp::DidChangeWorkspaceFoldersParams) {
        self.did_change_workspace_folders_impl(params).await
    }


    async fn selection_range(&self, params: lsp::SelectionRangeParams) -> Result<Option<Vec<lsp::SelectionRange>>> {
        self.selection_range_impl(params).await
    }

    async fn document_symbol(&self, params: lsp::DocumentSymbolParams) -> Result<Option<lsp::DocumentSymbolResponse>> {
        self.document_symbol_impl(params).await
    }


    async fn goto_definition(&self, params: lsp::GotoDefinitionParams) -> Result<Option<lsp::GotoDefinitionResponse>> {
        self.goto_definition_impl(params).await
    }

    async fn goto_declaration(&self, params: lsp::request::GotoDeclarationParams) -> Result<Option<lsp::request::GotoDeclarationResponse>> {
        self.goto_declaration_impl(params).await
    }

    async fn goto_type_definition(&self, params: lsp::request::GotoTypeDefinitionParams) -> Result<Option<lsp::request::GotoTypeDefinitionResponse>> {
        self.goto_type_definition_impl(params).await
    }


    async fn hover(&self, params: lsp::HoverParams) -> Result<Option<lsp::Hover>> {
        self.hover_impl(params).await
    }
}


/// The server communicates only with 1 client, so the protocol handling part itself does not need more resources than maybe 2 threads.
/// This way the rest of threads will be free to do some heavy lifting. 
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::build(|client| Backend::new(client))
        .custom_method(requests::projects::create::METHOD, Backend::handle_projects_create_request)
        .custom_method(requests::projects::list::METHOD, Backend::handle_projects_list_request)
        .custom_method(requests::projects::vanilla_dependency_content::METHOD, Backend::handle_projects_vanilla_dependency_content_request)
        .custom_method(requests::scripts::parent_content::METHOD, Backend::handle_scripts_parent_content_request)
        .custom_method(requests::debug::script_ast::METHOD, Backend::handle_debug_script_ast_request)
        .custom_method(requests::debug::script_cst::METHOD, Backend::handle_debug_script_cst_request)
        .custom_method(requests::debug::content_graph_dot::METHOD, Backend::handle_debug_content_graph_dot_request)
        .custom_method(requests::debug::script_symbols::METHOD, Backend::handle_debug_script_symbols_request)
        .custom_method(notifications::projects::did_import_scripts::METHOD, Backend::handle_projects_did_import_scripts_notification)
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

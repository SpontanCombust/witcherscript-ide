use std::path::PathBuf;
use dashmap::DashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types as lsp;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use witcherscript::Script;
use witcherscript::script_document::ScriptDocument;
use witcherscript_project::{ContentGraph, SourceTree};
use crate::config::Config;
use crate::messaging::requests;

mod providers;
mod config;
mod reporting;
mod source_management;
mod messaging;


#[derive(Debug)]
pub struct Backend {
    client: Client,
    config: RwLock<Config>,
    workspace_roots: RwLock<Vec<PathBuf>>,

    content_graph: RwLock<ContentGraph>,
    
    // key is path to content directory
    source_trees: DashMap<PathBuf, SourceTree>,

    // key is path to the file
    doc_buffers: DashMap<PathBuf, ScriptDocument>,
    scripts: DashMap<PathBuf, Script>
}

impl Backend {
    pub const LANGUAGE_ID: &str = "witcherscript";
    pub const SERVER_NAME: &str = "witcherscript-ide";

    fn new(client: Client) -> Self {
        Self {
            client,
            config: RwLock::new(Config::default()),
            workspace_roots: RwLock::new(Vec::new()),

            content_graph: RwLock::new(ContentGraph::new()),
            
            source_trees: DashMap::new(),

            doc_buffers: DashMap::new(),
            scripts: DashMap::new()
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
        providers::initialization::initialize(self, params).await
    }

    async fn initialized(&self, params: lsp::InitializedParams) {
        providers::initialization::initialized(self, params).await
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        providers::document_ops::did_open(self, params).await
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        providers::document_ops::did_change(self, params).await
    }

    async fn did_save(&self, params: lsp::DidSaveTextDocumentParams) {
        providers::document_ops::did_save(self, params).await
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        providers::document_ops::did_close(self, params).await
    }

    async fn did_change_configuration(&self, params: lsp::DidChangeConfigurationParams) {
        providers::configuration::did_change_configuration(self, params).await
    }

    async fn did_change_workspace_folders(&self, params: lsp::DidChangeWorkspaceFoldersParams) {
        providers::workspace::did_change_workspace_folders(self, params).await
    }
}


/// The server communicates only with 1 client, so the protocol handling part itself does not need more resources than maybe 2 threads.
/// This way the rest of threads will be free to do some heavy lifting. 
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::build(|client| Backend::new(client))
        .custom_method(requests::create_project::METHOD, Backend::handle_create_project_request)
        .custom_method(requests::script_ast::METHOD, Backend::handle_script_ast_request)
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

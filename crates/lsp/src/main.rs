use std::path::PathBuf;
use dashmap::DashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types as lsp;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use witcherscript::Script;
use witcherscript::script_document::ScriptDocument;
use witcherscript_project::ContentGraph;
use crate::config::Config;

mod providers;
mod config;
mod reporting;
mod workspace;


#[derive(Debug)]
pub struct Backend {
    client: Client,
    config: RwLock<Config>,
    workspace_roots: RwLock<Vec<PathBuf>>,

    content_graph: RwLock<ContentGraph>,
    // source_trees: DashMap<PathBuf, SourceTree>,

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
            // source_trees: DashMap::new(),
            //TODO a collection tracking errors for/in files, so error diagnostics don't dangle when these files get forgotten about

            doc_buffers: DashMap::new(),
            scripts: DashMap::new()
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
        if let Some(workspace_folders) = params.workspace_folders {
            let mut workspace_roots = self.workspace_roots.write().await;
            *workspace_roots = workspace_folders.into_iter()
                .map(|f| f.uri.to_file_path().unwrap())
                .collect();
        }

        Ok(lsp::InitializeResult {
            server_info: Some(lsp::ServerInfo {
                name: Backend::SERVER_NAME.into(),
                version: Some(env!("CARGO_PKG_VERSION").into())
            }),
            capabilities: lsp::ServerCapabilities {
                text_document_sync: Some(lsp::TextDocumentSyncCapability::Options(lsp::TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(lsp::TextDocumentSyncKind::INCREMENTAL),
                    will_save: Some(false),
                    will_save_wait_until: Some(false),
                    save: Some(lsp::TextDocumentSyncSaveOptions::SaveOptions(lsp::SaveOptions {
                        include_text: Some(false)
                    }))
                })),
                workspace: Some(lsp::WorkspaceServerCapabilities {
                    workspace_folders: Some(lsp::WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(lsp::OneOf::Left(true)),
                    }),
                    //TODO file operations notifiction config to know if content graph changed in some way
                    file_operations: None
                }),
                ..lsp::ServerCapabilities::default()
            }
        })
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        self.log_info("Server initialized!").await;

        self.client.register_capability(vec![
            lsp::Registration { 
                id: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
                method: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
                register_options: None 
            }
        ]).await.unwrap();

        self.fetch_config().await;

        self.scan_content_repositories().await;
        self.scan_workspace_projects().await;
        self.build_content_graph().await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        providers::document_ops::did_open(self, params).await;
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        providers::document_ops::did_change(self, params).await;
    }

    async fn did_save(&self, params: lsp::DidSaveTextDocumentParams) {
        providers::document_ops::did_save(self, params).await;
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        providers::document_ops::did_close(self, params).await;
    }

    async fn did_change_configuration(&self, _: lsp::DidChangeConfigurationParams) {
        self.fetch_config().await;
        self.scan_content_repositories().await;
        self.build_content_graph().await;
    }

    async fn did_change_workspace_folders(&self, params: lsp::DidChangeWorkspaceFoldersParams) {
        let added: Vec<_> = params.event.added.into_iter()
            .map(|f| f.uri.to_file_path().unwrap())
            .collect();

        let removed: Vec<_> = params.event.removed.into_iter()
            .map(|f| f.uri.to_file_path().unwrap())
            .collect();

        let mut workspace_roots = self.workspace_roots.write().await;
        workspace_roots.retain(|p| !removed.contains(p));
        workspace_roots.extend(added);

        self.scan_workspace_projects().await;
        self.build_content_graph().await;
    }
}


#[tokio::main]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

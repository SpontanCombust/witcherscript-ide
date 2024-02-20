use dashmap::DashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types as lsp;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use witcherscript::Script;
use witcherscript::script_document::ScriptDocument;
use crate::config::Config;

mod providers;
mod config;


#[derive(Debug)]
pub struct Backend {
    client: Client,
    config: RwLock<Option<Config>>,
    doc_buffers: DashMap<lsp::Url, ScriptDocument>,
    scripts: DashMap<lsp::Url, Script>
}

impl Backend {
    pub const LANGUAGE_ID: &str = "witcherscript";
    pub const SERVER_NAME: &str = "witcherscript-ide";

    fn new(client: Client) -> Self {
        Self {
            client,
            config: RwLock::new(None),
            doc_buffers: DashMap::new(),
            scripts: DashMap::new()
        }
    }

    async fn fetch_config(&self) {
        match Config::fetch(&self.client).await {
            Ok(config) => {
                let mut lock = self.config.write().await;
                *lock = Some(config);
            },
            Err(err) => {
                self.client.log_message(lsp::MessageType::ERROR, format!("Client configuration fetch error: {}", err)).await;
            },
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
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
                ..lsp::ServerCapabilities::default()
            }
        })
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        self.client.log_message(lsp::MessageType::INFO, "Server initialized!").await;

        self.client.register_capability(vec![
            lsp::Registration { 
                id: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
                method: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
                register_options: None 
            }
        ]).await.unwrap();

        self.fetch_config().await;
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

    // Not needed for now
    // async fn did_save(&self, params: DidSaveTextDocumentParams) {
    //     providers::document_sync::did_save(self, params).await;
    // }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        providers::document_ops::did_close(self, params).await;
    }

    async fn did_change_configuration(&self, _: lsp::DidChangeConfigurationParams) {
        self.fetch_config().await;
    }
}

#[tokio::main]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use witcherscript::Script;
use witcherscript::script_document::ScriptDocument;

mod providers;


#[derive(Debug)]
pub struct Backend {
    client: Client,
    doc_buffers: DashMap<Url, ScriptDocument>,
    scripts: DashMap<Url, Script> // temporary solution, use types provided from witcherscript_project later
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            doc_buffers: DashMap::new(),
            scripts: DashMap::new() 
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "witcherscript-ide".into(),
                version: Some(env!("CARGO_PKG_VERSION").into())
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::INCREMENTAL),
                    will_save: Some(false),
                    will_save_wait_until: Some(false),
                    save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                        include_text: Some(false)
                    }))
                })),
                ..ServerCapabilities::default()
            }
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        providers::document_sync::did_open(self, params).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        providers::document_sync::did_change(self, params).await;
    }

    // Not needed for now
    // async fn did_save(&self, params: DidSaveTextDocumentParams) {
    //     providers::document_sync::did_save(self, params).await;
    // }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        providers::document_sync::did_close(self, params).await;
    }
}

#[tokio::main]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

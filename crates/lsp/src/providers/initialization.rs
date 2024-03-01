use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use crate::Backend;


pub async fn initialize(backend: &Backend, params: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
    if let Some(workspace_folders) = params.workspace_folders {
        let mut workspace_roots = backend.workspace_roots.write().await;
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
                file_operations: None
            }),
            ..lsp::ServerCapabilities::default()
        }
    })
}

pub async fn initialized(backend: &Backend, _: lsp::InitializedParams) {
    backend.log_info("Server initialized!").await;

    backend.client.register_capability(vec![
        lsp::Registration { 
            id: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
            method: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
            register_options: None 
        }
    ]).await.unwrap();

    backend.fetch_config().await;

    backend.scan_content_repositories().await;
    backend.scan_workspace_projects().await;
    backend.build_content_graph().await;
}
use abs_path::AbsPath;
use serde::Deserialize;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use crate::config::Config;
use crate::Backend;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializationOptions {
    config: Config
}

pub async fn initialize(backend: &Backend, params: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
    if let Some(workspace_folders) = params.workspace_folders {
        let mut workspace_roots = backend.workspace_roots.write().await;
        *workspace_roots = workspace_folders.into_iter()
            .map(|f| AbsPath::try_from(f.uri).unwrap())
            .collect();
    }

    if let Some(init_opts) = params.initialization_options {
        match serde_json::from_value::<InitializationOptions>(init_opts) {
            Ok(val) => {
                let mut config = backend.config.write().await;
                *config = val.config;
            },
            Err(err) => {
                backend.reporter.log_error(format!("initializationOptions deserialization fail: {}", err)).await;
            },
        }
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
                    include_text: Some(true)
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
    backend.reporter.log_info("Server initialized!").await;

    backend.client.register_capability(vec![
        lsp::Registration { 
            id: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
            method: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
            register_options: None 
        }
    ]).await.unwrap();

    backend.setup_workspace_content_scanners().await;
    backend.setup_repository_content_scanners().await;
    backend.build_content_graph().await;

    backend.reporter.commit_all_diagnostics().await;
}
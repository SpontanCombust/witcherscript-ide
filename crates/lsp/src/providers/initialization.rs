use std::path::PathBuf;
use abs_path::AbsPath;
use serde::Deserialize;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use crate::Backend;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializationOptions {
    native_content_uri: lsp::Url,
    game_directory: PathBuf,
    content_repositories: Vec<PathBuf>
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
                match AbsPath::try_from(val.native_content_uri) {
                    Ok(native_content_path) => {
                        let mut graph = backend.content_graph.write().await;
                        graph.set_native_content_path(&native_content_path);
                    },
                    Err(_) => {
                        backend.reporter.log_error("Invalid native_content_path URI").await;
                    }
                }

                let mut config = backend.config.write().await;
                config.game_directory = val.game_directory;
                config.content_repositories = val.content_repositories;
            },
            Err(err) => {
                backend.reporter.log_error(format!("InitializationOptions deserialization fail: {}", err)).await;
            },
        }
    } else {
        backend.reporter.log_error("Initialization options missing!").await;
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
            selection_range_provider: Some(lsp::SelectionRangeProviderCapability::Simple(true)),
            document_symbol_provider: Some(lsp::OneOf::Left(true)),
            definition_provider: Some(lsp::OneOf::Left(true)),
            declaration_provider: Some(lsp::DeclarationCapability::Simple(true)),
            type_definition_provider: Some(lsp::TypeDefinitionProviderCapability::Simple(true)),
            implementation_provider: None,
            hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
            ..lsp::ServerCapabilities::default()
        }
    })
}

pub async fn initialized(backend: &Backend, _: lsp::InitializedParams) {
    // make sure that content graph's lock is always acquired first here
    let mut content_graph = backend.content_graph.try_write().unwrap();

    backend.reporter.log_info("Server initialized!").await;

    backend.client.register_capability(vec![
        lsp::Registration { 
            id: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
            method: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
            register_options: None 
        }
    ]).await.unwrap();

    backend.setup_workspace_content_scanners(&mut content_graph).await;
    backend.setup_repository_content_scanners(&mut content_graph).await;
    backend.build_content_graph(&mut content_graph).await;
    drop(content_graph);

    backend.reporter.commit_all_diagnostics().await;
}
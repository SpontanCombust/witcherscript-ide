use std::path::PathBuf;
use abs_path::AbsPath;
use serde::Deserialize;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use witcherscript_project::redkit::RedkitManifest;
use witcherscript_project::Manifest;
use crate::{notifications, Backend};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializationOptions {
    rayon_threads: i32,
    native_content_path: PathBuf,
    game_directory: PathBuf,
    content_repositories: Vec<PathBuf>,
    enable_syntax_analysis: bool
}

impl Backend {
    pub async fn initialize_impl(&self, params: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
        if let Some(init_opts) = params.initialization_options {
            match serde_json::from_value::<InitializationOptions>(init_opts) {
                Ok(val) => {
                    self.reporter.log_info(format!("Initializing server with: {:#?}", val)).await;


                    let rayon_threads = if val.rayon_threads > 0 {
                        val.rayon_threads as usize
                    } else {
                        std::thread::available_parallelism()
                            .expect("Cannot access the number of available threads")
                            .get()
                            .checked_sub(2) // make sure rayon doesn't take up all resources
                            .unwrap_or(1)
                    };

                    rayon::ThreadPoolBuilder::new()
                        .num_threads(rayon_threads) 
                        .build_global()
                        .expect("rayon::ThreadPoolBuilder panik");

                    self.reporter.log_info(format!("Configured tokio worker threads: {}", tokio::runtime::Handle::current().metrics().num_workers())).await;
                    self.reporter.log_info(format!("Configured rayon threads: {}", rayon_threads)).await;

    
                    match AbsPath::try_from(val.native_content_path) {
                        Ok(native_content_path) => {
                            let mut graph = self.content_graph.write().await;
                            graph.set_native_content_path(&native_content_path);
                        },
                        Err(_) => {
                            self.reporter.log_error("Invalid native_content_path").await;
                        }
                    }
    
                    let mut config = self.config.write().await;
                    config.game_directory = val.game_directory;
                    config.content_repositories = val.content_repositories;
                    config.enable_syntax_analysis = val.enable_syntax_analysis;
                },
                Err(err) => {
                    self.reporter.log_error(format!("InitializationOptions deserialization fail: {}", err)).await;
                },
            }
        } else {
            self.reporter.log_error("Initialization options missing!").await;
        }

        if let Some(workspace_folders) = params.workspace_folders {
            let mut workspace_roots = self.workspace_roots.write().await;
            *workspace_roots = workspace_folders.into_iter()
                .map(|f| AbsPath::try_from(f.uri).unwrap())
                .collect();
        }
    
    
        let file_ops_filter_ws = lsp::FileOperationFilter {
            scheme: Some("file".into()),
            pattern: lsp::FileOperationPattern {
                glob: "**/*.ws".into(),
                matches: Some(lsp::FileOperationPatternKind::File),
                ..Default::default()
            }
        };
    
        let file_ops_filter_manifest = lsp::FileOperationFilter {
            scheme: Some("file".into()),
            pattern: lsp::FileOperationPattern {
                glob: format!("**/{}", Manifest::FILE_NAME),
                matches: Some(lsp::FileOperationPatternKind::File),
                ..Default::default()
            }
        };
    
        let file_ops_filter_redkit_manifest = lsp::FileOperationFilter {
            scheme: Some("file".into()),
            pattern: lsp::FileOperationPattern {
                glob: format!("**/*.{}", RedkitManifest::EXTENSION),
                matches: Some(lsp::FileOperationPatternKind::File),
                ..Default::default()
            }
        };
    
        let file_ops_reg_opts = Some(lsp::FileOperationRegistrationOptions {
            filters: vec![
                file_ops_filter_ws,
                file_ops_filter_manifest,
                file_ops_filter_redkit_manifest
            ]
        });
    
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
                    file_operations: Some(lsp::WorkspaceFileOperationsServerCapabilities {
                        did_create: file_ops_reg_opts.clone(),
                        did_delete: file_ops_reg_opts.clone(),
                        did_rename: file_ops_reg_opts,
                        ..Default::default()
                    })
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
    
    pub async fn initialized_impl(&self, _: lsp::InitializedParams) {
        self.reporter.log_info("Server initialized!").await;
    
        self.client.register_capability(vec![
            lsp::Registration { 
                id: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
                method: lsp::notification::DidChangeConfiguration::METHOD.to_string(), 
                register_options: None 
            }
        ]).await.unwrap();
    
        self.setup_workspace_content_scanners().await;
        self.setup_repository_content_scanners().await;
        self.build_content_graph(true).await;

        self.client.send_notification::<notifications::scripts::did_finish_initial_indexing::Type>(()).await;

        self.reporter.commit_all_diagnostics().await;
    }
}
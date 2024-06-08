use tower_lsp::lsp_types as lsp;
use crate::Backend;


pub async fn did_change_configuration(backend: &Backend, _: lsp::DidChangeConfigurationParams) {
    let diff = backend.fetch_config().await;

    if diff.game_directory_changed || diff.content_repositories_changed {
        backend.setup_repository_content_scanners().await;
        backend.build_content_graph(true).await;
        
        backend.reporter.commit_all_diagnostics().await;
    }
}
use tower_lsp::lsp_types as lsp;
use crate::Backend;


pub async fn did_change_configuration(backend: &Backend, _: lsp::DidChangeConfigurationParams) {
    if backend.fetch_config().await {
        let mut content_graph = backend.content_graph.write().await;

        backend.setup_repository_content_scanners(&mut content_graph).await;
        backend.build_content_graph(&mut content_graph).await;
        
        backend.reporter.commit_all_diagnostics().await;
    }
}
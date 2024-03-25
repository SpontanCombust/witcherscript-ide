use tower_lsp::lsp_types as lsp;
use crate::Backend;


pub async fn did_change_configuration(backend: &Backend, _: lsp::DidChangeConfigurationParams) {
    if backend.fetch_config().await {
        backend.setup_repository_content_scanners().await;
        backend.build_content_graph().await;
    }
}
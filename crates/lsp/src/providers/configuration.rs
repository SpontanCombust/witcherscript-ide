use tower_lsp::lsp_types as lsp;
use crate::Backend;


impl Backend {
    pub async fn did_change_configuration_impl(&self, _: lsp::DidChangeConfigurationParams) {
        let diff = self.fetch_config().await;
    
        if diff.game_directory_changed || diff.content_repositories_changed {
            self.setup_repository_content_scanners().await;
            self.build_content_graph(true).await;
            
            self.reporter.commit_all_diagnostics().await;
        }
    }
}
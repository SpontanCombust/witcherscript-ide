use std::path::Path;
use tokio::time::Instant;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::{diagnostics::Diagnostic, jobs::syntax_analysis};
use witcherscript_project::source_tree::{SourceFilePath, SourceTreeDifference};
use crate::{reporting::IntoLspDiagnostic, Backend};


impl Backend {
    pub async fn scan_source_tree(&self, content_path: &Path) {
        let mut diff: Option<SourceTreeDifference> = None;
        if let Some(mut source_tree) = self.source_trees.get_mut(content_path) {
            self.log_info(format!("Scanning source tree of content {} for changes...", content_path.display())).await;

            diff = Some(source_tree.scan());

            if !source_tree.errors.is_empty() {
                for err in &source_tree.errors {
                    self.log_warning(format!("Source tree scanning issue for {}: {}", err.path.display(), err.error)).await
                }
            }

            // handling of `diff` outside of the if to let go of reference to self.source_trees
        }
        
        if let Some(diff) = diff {
            if !diff.is_empty() {
                self.on_source_tree_changed(content_path, diff).await;
            } else {
                self.log_info("Found no source tree changes.").await;
            }
        }
    }

    pub async fn on_source_tree_changed(&self, content_path: &Path, diff: SourceTreeDifference) {
        let start = Instant::now();

        let (diff_added, diff_removed) = (diff.added, diff.removed);
        self.on_source_tree_paths_removed(diff_removed).await;
        self.on_source_tree_paths_added(diff_added).await;

        let duration = Instant::now() - start;
        self.log_info(format!("Handled source tree related changes to {} in {:.3}s", content_path.display(), duration.as_secs_f32())).await;
    }

    async fn on_source_tree_paths_added(&self, added_paths: Vec<SourceFilePath>) {
        // No multi-threading for now!
        
        for added_path in added_paths {
            self.log_info(format!("Discovered script: {}", added_path)).await;

            let doc = ScriptDocument::from_file(added_path.absolute()).unwrap();
            let script = Script::new(&doc).unwrap();

            let mut diagnostics: Vec<Diagnostic> = Vec::new();
            syntax_analysis::syntax_analysis(script.root_node(), &mut diagnostics);
            let lsp_diags = diagnostics.into_iter().map(|diag| diag.into_lsp_diagnostic());

            self.scripts.insert(added_path.absolute().to_owned(), script);
            self.publish_diagnostics(added_path.absolute().to_owned(), lsp_diags).await;
        }
    }

    async fn on_source_tree_paths_removed(&self, removed_paths: Vec<SourceFilePath>) {
        for removed_path in removed_paths {
            self.log_info(format!("Deprecated script: {}", removed_path)).await;
            self.scripts.remove(removed_path.absolute());
        }
    }
}

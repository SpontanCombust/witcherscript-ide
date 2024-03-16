use std::path::Path;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::{diagnostics::Diagnostic, jobs::syntax_analysis};
use witcherscript_project::source_tree::{SourceFilePath, SourceTreeDifference};
use crate::{reporting::IntoLspDiagnostic, Backend};


impl Backend {
    pub async fn scan_source_tree(&self, content_path: &Path) {
        if let Some(mut source_tree) = self.source_trees.get_mut(content_path) {
            self.log_info(format!("Scanning source tree of content {} for changes...", content_path.display())).await;

            let diff = source_tree.scan();

            if !source_tree.errors.is_empty() {
                for err in &source_tree.errors {
                    self.log_warning(format!("Source tree scanning issue for {}: {}", err.path.display(), err.error)).await
                }
            }

            if !diff.is_empty() {
                let (count_added, count_removed) = (diff.added.len(), diff.removed.len());
                self.on_source_tree_changed(content_path, diff).await;
                self.log_info(format!("Changes made to source tree: added {}, delisted {}", count_added, count_removed)).await;
            } else {
                self.log_info("Found no changes.").await;
            }

            //TODO add time elapsed logs
        }
    }

    pub async fn on_source_tree_changed(&self, content_path: &Path, diff: SourceTreeDifference) {
        let _ = content_path; // param unused for now
        let (diff_added, diff_removed) = (diff.added, diff.removed);
        self.on_source_tree_paths_removed(diff_removed).await;
        self.on_source_tree_paths_added(diff_added).await;
    }

    async fn on_source_tree_paths_added(&self, added_paths: Vec<SourceFilePath>) {
        // No multi-threading for now!

        let mut script_parse_tasks = Vec::with_capacity(added_paths.len());
        for added_path in added_paths {
            let doc = ScriptDocument::from_file(added_path.absolute()).unwrap();
            let script = Script::new(&doc).unwrap();

            let mut diagnostics: Vec<Diagnostic> = Vec::new();
            syntax_analysis::syntax_analysis(script.root_node(), &mut diagnostics);
            let lsp_diags = diagnostics.into_iter().map(|diag| diag.into_lsp_diagnostic());

            script_parse_tasks.push((script, lsp_diags, added_path));
        }

        for (script_path, lsp_diags, path) in script_parse_tasks {
            self.scripts.insert(path.absolute().to_owned(), script_path);

            self.log_info(format!("Found new script: {}", path)).await;
            self.publish_diagnostics(path.absolute().to_owned(), lsp_diags).await;
        }
    }

    async fn on_source_tree_paths_removed(&self, removed_paths: Vec<SourceFilePath>) {
        for removed_path in removed_paths {
            self.scripts.remove(removed_path.absolute());
            self.log_info(format!("Delisted script: {}", removed_path)).await;
        }
    }
}

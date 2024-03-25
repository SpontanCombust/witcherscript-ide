use rayon::iter::{ParallelBridge, ParallelIterator};
use tokio::{sync::mpsc, time::Instant};
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::{diagnostics::Diagnostic, jobs::syntax_analysis};
use witcherscript_project::source_tree::{SourceFilePath, SourceTreeDifference};
use crate::{reporting::IntoLspDiagnostic, Backend};


impl Backend {
    pub async fn scan_source_tree(&self, content_path: &AbsPath) {
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

    pub async fn on_source_tree_changed(&self, content_path: &AbsPath, diff: SourceTreeDifference) {
        let (added_count, removed_count) = (diff.added.len(), diff.removed.len());
        self.log_info(format!("Changes to source tree in {}: {} script(s) discovered, {} to be deprecated", content_path.display(), added_count, removed_count)).await;

        let start = Instant::now();

        let (diff_added, diff_removed) = (diff.added, diff.removed);
        self.on_source_tree_paths_removed(diff_removed).await;
        self.on_source_tree_paths_added(diff_added).await;

        let duration = Instant::now() - start;
        self.log_info(format!("Handled source tree related changes to {} in {:.3}s", content_path.display(), duration.as_secs_f32())).await;
    }

    async fn on_source_tree_paths_added(&self, added_paths: Vec<SourceFilePath>) {
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        rayon::spawn(move || {
            added_paths.into_iter()
                .par_bridge()
                .map(|p| p.absolute().to_owned())
                .map(|p| {
                    let doc = ScriptDocument::from_file(&p).unwrap();
                    (p, doc)
                })
                .map(|(p, doc)| {
                    let script = Script::new(&doc).unwrap();
                    (p, script)
                })
                .map(|(p, script)| {
                    let mut diagnostics: Vec<Diagnostic> = Vec::new();
                    syntax_analysis::syntax_analysis(script.root_node(), &mut diagnostics);
                    let lsp_diags: Vec<_> = diagnostics.into_iter().map(|diag| diag.into_lsp_diagnostic()).collect();
                    (p, script, lsp_diags)
                })
                .for_each(|result| send.blocking_send(result).expect("mpsc send fail"));
        });

        while let Some((script_path, script, diags)) = recv.recv().await {
            // Doing to many logs at once puts a strain on the connection, better to do this through a Progress or something...
            // self.log_info(format!("Discovered script: {}", script_path.display())).await;
            self.scripts.insert(script_path.clone(), script);
            self.publish_diagnostics(script_path, diags).await;
        }
    }

    async fn on_source_tree_paths_removed(&self, removed_paths: Vec<SourceFilePath>) {
        for removed_path in removed_paths {
            // self.log_info(format!("Deprecated script: {}", removed_path)).await;
            self.scripts.remove(removed_path.absolute());
            // clearing diagnostics is done in will_delete_files
        }
    }
}

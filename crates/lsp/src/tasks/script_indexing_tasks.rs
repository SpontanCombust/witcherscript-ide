use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::{sync::mpsc, time::Instant};
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_project::source_tree::{SourceFilePath, SourceTreeDifference};
use crate::{Backend, ScriptState};
use super::ScriptAnalysisKind;


impl Backend {
    pub async fn scan_source_tree(&self, content_path: &AbsPath) {
        let mut diff: Option<SourceTreeDifference> = None;
        if let Some(mut source_tree) = self.source_trees.get_mut(content_path) {
            self.reporter.log_info(format!("Scanning source tree of content {} for changes...", content_path.display())).await;

            diff = Some(source_tree.scan());

            if !source_tree.errors.is_empty() {
                for err in &source_tree.errors {
                    self.reporter.log_warning(format!("Source tree scanning issue for {}: {}", err.path.display(), err.error)).await
                }
            }

            // handling of `diff` outside of the if to let go of reference to self.source_trees
        }
        
        if let Some(diff) = diff {
            if !diff.is_empty() {
                self.on_source_tree_changed(content_path, diff, true).await;
            } else {
                self.reporter.log_info("Found no source tree changes.").await;
            }
        }
    }

    pub async fn on_source_tree_changed(&self, content_path: &AbsPath, diff: SourceTreeDifference, run_diagnostics_for_added: bool) {
        let (added_count, removed_count) = (diff.added.len(), diff.removed.len());
        self.reporter.log_info(format!("Changes to source tree in {}: {} script(s) discovered, {} to be deprecated", content_path.display(), added_count, removed_count)).await;

        let start = Instant::now();

        let (diff_added, diff_removed) = (diff.added, diff.removed);
        self.on_source_tree_paths_removed(diff_removed).await;
        self.on_source_tree_paths_added(diff_added, run_diagnostics_for_added).await;

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Handled source tree related changes to {} in {:.3}s", content_path.display(), duration.as_secs_f32())).await;
    }

    async fn on_source_tree_paths_added(&self, added_paths: Vec<SourceFilePath>, run_diagnostics: bool) {
        let script_paths: Vec<_> = added_paths.into_iter().map(|p| p.absolute().to_owned()).collect();
        
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        let script_paths_cloned = script_paths.clone();
        rayon::spawn(move || {
            script_paths_cloned.into_par_iter()
                .map(|p| {
                    let doc = ScriptDocument::from_file(&p).unwrap();
                    let script = Script::new(&doc).unwrap();
                    (p, script)
                })
                .for_each(|result| send.blocking_send(result).expect("on_source_tree_paths_added mpsc::send fail"));
        });

        while let Some((script_path, script)) = recv.recv().await {
            // Doing to many logs at once puts a strain on the connection, better to do this through a Progress or something...
            // self.log_info(format!("Discovered script: {}", script_path.display())).await;
            self.scripts.insert(script_path, ScriptState { 
                script, 
                buffer: None,
                is_foreign: false
            });
        }

        if run_diagnostics {
            self.run_script_analysis(script_paths, ScriptAnalysisKind::all()).await;
        }
    }

    async fn on_source_tree_paths_removed(&self, removed_paths: Vec<SourceFilePath>) {
        for removed_path in removed_paths {
            // self.log_info(format!("Deprecated script: {}", removed_path)).await;
            self.scripts.remove(removed_path.absolute());
            self.reporter.purge_diagnostics(removed_path.absolute());
        }
    }
}

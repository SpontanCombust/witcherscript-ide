use tokio::{sync::mpsc, time::Instant};
use rayon::prelude::*;
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_project::source_tree::{SourceTreeDifference, SourceTreeFile};
use crate::{Backend, ScriptState};


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

    pub async fn on_source_tree_changed(&self, content_path: &AbsPath, diff: SourceTreeDifference, run_diagnostics: bool) {
        let (added_count, removed_count, modified_count) = (diff.added.len(), diff.removed.len(), diff.modified.len());
        self.reporter.log_info(format!(
            "Changes to source tree in {}: {} script(s) discovered, {} to be deprecated, {} modified", 
            content_path.display(), added_count, removed_count, modified_count
        )).await;

        let start = Instant::now();

        let (diff_added, diff_removed, diff_modified) = (diff.added, diff.removed, diff.modified);
        if !diff_removed.is_empty() {
            self.on_source_tree_files_removed(diff_removed).await;
        }
        if !diff_added.is_empty() {
            self.on_source_tree_files_added(diff_added.clone()).await;
        }
        if !diff_modified.is_empty() {
            self.on_source_tree_files_modified(diff_modified.clone()).await;
        }

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Handled source tree related changes to {} in {:.3}s", content_path.display(), duration.as_secs_f32())).await;

        let added_or_modified: Vec<_> = 
            diff_added.into_iter()
            .chain(diff_modified.into_iter())
            .collect();

        if !added_or_modified.is_empty() {
            {
                let mut symtabs = self.symtabs.write().await;
                if let Some(mut content_symtab) = symtabs.get_mut(content_path) {
                    let paths = added_or_modified.iter()
                        .map(|f| f.path.clone())
                        .collect();

                    self.scan_symbols(&mut content_symtab, content_path, paths).await;
                }
            }

            if run_diagnostics {
                let paths = added_or_modified.into_iter()
                    .map(|f| f.path.into_absolute())
                    .par_bridge();

                self.run_script_analysis(paths).await;
            }
        }
    }

    async fn on_source_tree_files_added(&self, added_files: Vec<SourceTreeFile>) {
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        rayon::spawn(move || {
            added_files.into_par_iter()
                .map(|f| {
                    let path = f.path;
                    let doc = ScriptDocument::from_file(path.absolute()).unwrap();
                    let script = Script::new(&doc).unwrap();
                    (path, doc, script, f.modified_timestamp)
                })
                .for_each(|result| send.blocking_send(result).expect("on_source_tree_paths_added mpsc::send fail"));
        });

        let mut results = Vec::new();
        while let Some(res) = recv.recv().await {
            results.push(res);
        }

        for (source_tree_path, buffer, script, modified_timestamp) in results {
            self.scripts.insert(source_tree_path.absolute().to_owned(), ScriptState { 
                script, 
                buffer,
                modified_timestamp,
                source_tree_path: Some(source_tree_path)
            });
        }
    }

    async fn on_source_tree_files_removed(&self, removed_files: Vec<SourceTreeFile>) {
        for removed_file in removed_files {
            // self.log_info(format!("Deprecated script: {}", removed_path)).await;
            self.scripts.remove(removed_file.path.absolute());
            self.reporter.purge_diagnostics(removed_file.path.absolute()).await;
        }
    }
    
    async fn on_source_tree_files_modified(&self, modified_files: Vec<SourceTreeFile>) {
        for modified_file in &modified_files {
            if let Some(mut script_state) = self.scripts.get_mut(modified_file.path.absolute()) {
                // for cases when files have been updated outside of of LSP client's knowledge
                if modified_file.modified_timestamp > script_state.modified_timestamp {
                    let doc = ScriptDocument::from_file(modified_file.path.absolute()).unwrap();
                    script_state.script.refresh(&doc).unwrap();
                    script_state.buffer = doc;
                    script_state.modified_timestamp = modified_file.modified_timestamp;
                }
            }
        }
    }
}


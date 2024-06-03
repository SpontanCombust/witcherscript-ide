use tokio::{sync::oneshot, time::Instant};
use rayon::prelude::*;
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_project::source_tree::{SourceTreeDifference, SourceTreeFile};
use crate::{Backend, ScriptState, ScriptStateContentInfo};


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
                let paths_for_analysis: Vec<_> = 
                    diff.added.iter().map(|f| f.path.absolute().to_owned())
                    .chain(diff.modified.iter().map(|f| f.path.absolute().to_owned()))
                    .collect();

                self.on_source_tree_changed(content_path, diff).await;
                self.run_script_analysis(paths_for_analysis, true).await;
            } else {
                self.reporter.log_info("Found no source tree changes.").await;
            }
        }
    }

    pub async fn on_source_tree_changed(&self, content_path: &AbsPath, diff: SourceTreeDifference) {
        let (added_count, removed_count, modified_count) = (diff.added.len(), diff.removed.len(), diff.modified.len());
        self.reporter.log_info(format!(
            "Changes to source tree in {}: {} script(s) discovered, {} to be deprecated, {} modified", 
            content_path.display(), added_count, removed_count, modified_count
        )).await;

        let start = Instant::now();

        let (diff_added, diff_removed, diff_modified) = (diff.added, diff.removed, diff.modified);
        let diff_added_or_modified = diff_added.iter()
            .chain(diff_modified.iter())
            .map(|f| f.to_owned())
            .collect::<Vec<_>>();

        if !diff_removed.is_empty() {
            self.on_source_tree_files_removed(diff_removed).await;
        }
        if !diff_added.is_empty() {
            self.on_source_tree_files_added(diff_added, content_path).await;
        }
        if !diff_modified.is_empty() {
            self.on_source_tree_files_modified(diff_modified).await;
        }
        if !diff_added_or_modified.is_empty() {
            self.on_source_tree_files_added_or_modified(diff_added_or_modified, content_path).await;
        }

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Handled source tree related changes to {} in {:.3}s", content_path.display(), duration.as_secs_f32())).await;
    }

    async fn on_source_tree_files_added(&self, added_files: Vec<SourceTreeFile>, content_path: &AbsPath) {
        let start = Instant::now();

        let (send, recv) = oneshot::channel();

        rayon::spawn(move || {
            let results: Vec<_> = 
                added_files.into_par_iter()
                .map(|f| {
                    let path = f.path;
                    let doc = ScriptDocument::from_file(path.absolute()).unwrap();
                    let script = Script::new(&doc).unwrap();
                    (path, doc, script, f.modified_timestamp)
                })
                .collect();

            send.send(results).expect("on_source_tree_paths_added oneshot::send fail")
        });

        let results = recv.await.expect("on_source_tree_paths_added oneshot::recv fail");

        for (source_tree_path, buffer, script, modified_timestamp) in results {
            self.scripts.insert(source_tree_path.absolute().to_owned(), ScriptState { 
                script, 
                buffer,
                modified_timestamp,
                content_info: Some(ScriptStateContentInfo {
                    content_path: content_path.to_owned(), 
                    source_tree_path
                })
            });
        }

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Parsed discovered scripts in {:.3}s", duration.as_secs_f32())).await;
    }

    async fn on_source_tree_files_removed(&self, removed_files: Vec<SourceTreeFile>) {
        for removed_file in removed_files {
            // self.log_info(format!("Deprecated script: {}", removed_path)).await;
            self.scripts.remove(removed_file.path.absolute());
            self.reporter.purge_diagnostics(removed_file.path.absolute());
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

    async fn on_source_tree_files_added_or_modified(&self, added_or_modified_files: Vec<SourceTreeFile>, content_path: &AbsPath) {
        let paths = added_or_modified_files.into_iter()
                .map(|f| f.path.clone())
                .collect();

        self.scan_symbols(content_path, paths, true).await;
    }
}


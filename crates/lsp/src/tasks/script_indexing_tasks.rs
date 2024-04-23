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

        let modified_script_paths: Vec<_> = 
            diff_added.into_iter()
            .chain(diff_modified.into_iter())
            .map(|f| f.into_absolute_path())
            .collect();

        if !modified_script_paths.is_empty() {
            self.on_scripts_modified(modified_script_paths, Some(content_path), true, run_diagnostics).await;
        }

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Handled source tree related changes to {} in {:.3}s", content_path.display(), duration.as_secs_f32())).await;
    }

    async fn on_source_tree_files_added(&self, added_files: Vec<SourceTreeFile>) {
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        let added_files_cloned = added_files.clone();
        rayon::spawn(move || {
            added_files_cloned.into_par_iter()
                .map(|f| {
                    let path = f.absolute_path().to_owned();
                    let doc = ScriptDocument::from_file(&path).unwrap();
                    let script = Script::new(&doc).unwrap();
                    (path, doc, script, f.modified_timestamp())
                })
                .for_each(|result| send.blocking_send(result).expect("on_source_tree_paths_added mpsc::send fail"));
        });

        while let Some((script_path, buffer, script, modified_timestamp)) = recv.recv().await {
            // Doing to many logs at once puts a strain on the connection, better to do this through a Progress or something...
            // self.log_info(format!("Discovered script: {}", script_path.display())).await;
            self.scripts.insert(script_path, ScriptState { 
                script, 
                buffer,
                modified_timestamp,
                is_foreign: false
            });
        }
    }

    async fn on_source_tree_files_removed(&self, removed_files: Vec<SourceTreeFile>) {
        for removed_file in removed_files {
            // self.log_info(format!("Deprecated script: {}", removed_path)).await;
            self.scripts.remove(removed_file.absolute_path());
            self.reporter.purge_diagnostics(removed_file.absolute_path());
        }
    }
    
    async fn on_source_tree_files_modified(&self, modified_files: Vec<SourceTreeFile>) {
        for modified_file in &modified_files {
            if let Some(mut script_state) = self.scripts.get_mut(modified_file.absolute_path()) {
                // for cases when files have been updated outside of of LSP client's knowledge
                if modified_file.modified_timestamp() > script_state.modified_timestamp {
                    let doc = ScriptDocument::from_file(modified_file.absolute_path()).unwrap();
                    script_state.script.refresh(&doc).unwrap();
                    script_state.buffer = doc;
                    script_state.modified_timestamp = modified_file.modified_timestamp();
                }
            }
        }
    }

    
    pub async fn on_scripts_modified(&self, modified_script_paths: Vec<AbsPath>, content_path: Option<&AbsPath>, scan_symbols: bool, run_diagnostics: bool) {
        if scan_symbols {
            if let Ok(mut symtabs) = self.symtabs.try_write() {
                if let Some(mut main_symtab) = content_path.and_then(|content_path| symtabs.get_mut(content_path)) {
                    self.scan_symbols(&mut main_symtab, &modified_script_paths).await;
                }
            }
        }

        if run_diagnostics {
            self.run_script_analysis(modified_script_paths).await;
        }
    }
}


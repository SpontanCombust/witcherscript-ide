use std::{collections::HashMap, sync::{Arc, Mutex}};
use rayon::prelude::*;
use tokio::{sync::{mpsc, oneshot}, time::Instant};
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::{diagnostics::Diagnostic, jobs, model::collections::SymbolTable};
use witcherscript_project::source_tree::{SourceTreeFile, SourceTreeDifference};
use crate::{reporting::{DiagnosticGroup, IntoLspDiagnostic}, Backend, ScriptState, ScriptStates};


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
            self.on_source_tree_files_added(diff_added, content_path, run_diagnostics).await;
        }
        if !diff_modified.is_empty() {
            self.on_source_tree_files_modified(diff_modified, content_path, run_diagnostics).await;
        }

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Handled source tree related changes to {} in {:.3}s", content_path.display(), duration.as_secs_f32())).await;
    }

    async fn on_source_tree_files_added(&self, added_files: Vec<SourceTreeFile>, content_path: &AbsPath, run_diagnostics: bool) {
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

        let script_paths = added_files.into_iter()
            .map(|f| f.into_absolute_path())
            .collect();
        
        self.on_scripts_modified(script_paths, Some(content_path), run_diagnostics).await;
    }

    async fn on_source_tree_files_removed(&self, removed_files: Vec<SourceTreeFile>) {
        for removed_file in removed_files {
            // self.log_info(format!("Deprecated script: {}", removed_path)).await;
            self.scripts.remove(removed_file.absolute_path());
            self.reporter.purge_diagnostics(removed_file.absolute_path());
        }
    }
    
    async fn on_source_tree_files_modified(&self, modified_files: Vec<SourceTreeFile>, content_path: &AbsPath, run_diagnostics: bool) {
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

        let modified_script_paths = modified_files.into_iter()
            .map(|f| f.into_absolute_path())
            .collect();

        self.on_scripts_modified(modified_script_paths, Some(content_path), run_diagnostics).await;
    }

    
    pub async fn on_scripts_modified(&self, modified_script_paths: Vec<AbsPath>, content_path: Option<&AbsPath>, run_diagnostics: bool) {
        if let Ok(mut symtabs) = self.symtabs.try_write() {
            if let Some(mut main_symtab) = content_path.and_then(|content_path| symtabs.get_mut(content_path)) {
                self.alter_symbol_table(&mut main_symtab, &modified_script_paths).await;
            }
        }

        if run_diagnostics {
            self.run_script_analysis(modified_script_paths).await;
        }
    }

    async fn alter_symbol_table(&self, symtab: &mut SymbolTable, modified_script_paths: &Vec<AbsPath>) {
        let job_provider = SymbolScanJobProvider {
            scripts: self.scripts.clone(),
            script_paths: Arc::new(Mutex::new(modified_script_paths.clone()))
        };

        let worker_count = std::cmp::min(rayon::current_num_threads(), modified_script_paths.len());

        let (send, recv) = oneshot::channel();
        rayon::spawn(move || {
            let mut workers = Vec::with_capacity(worker_count);

            for _ in 0..worker_count {
                workers.push(SymbolScanWorker::new(job_provider.clone()));
            }

            workers.par_iter_mut()
                .for_each(|w| w.work());

            let (mut merged_symtab, mut merged_diagnostics) = workers.pop().unwrap().finish();
            while let Some(worker) = workers.pop() {
                let (symtab, diagnostics) = worker.finish();
                merged_diagnostics.extend(diagnostics);
                jobs::merge_symbol_tables(&mut merged_symtab, symtab, &mut merged_diagnostics);
            }

            send.send((merged_symtab, merged_diagnostics)).expect("on_scripts_modified symbol scan send fail");
        });

        for p in modified_script_paths {
            symtab.remove_for_file(p);
        }

        let (scanning_symtab, mut scanning_diagnostis) = recv.await.expect("on_scripts_modified symbol scan recv fail");

        jobs::merge_symbol_tables(symtab, scanning_symtab, &mut scanning_diagnostis);

        for (file_path, diagnostics) in scanning_diagnostis {
            self.reporter.clear_diagnostics(&file_path, DiagnosticGroup::SymbolScan);
            self.reporter.push_diagnostics(&file_path, diagnostics.into_iter().map(|diag| diag.into_lsp_diagnostic()),  DiagnosticGroup::SymbolScan);
        }
    }
}

struct SymbolScanWorker {
    symtab: SymbolTable,
    diagnostics: HashMap<AbsPath, Vec<Diagnostic>>,
    job_provider: SymbolScanJobProvider
}

impl SymbolScanWorker {
    fn new(job_provider: SymbolScanJobProvider) -> Self {
        Self {
            symtab: SymbolTable::new(),
            diagnostics: HashMap::new(),
            job_provider
        }
    }

    fn work(&mut self) {
        while let Some(job) = self.job_provider.poll() {
            let script_state = job.scripts.get(&job.script_path).unwrap();
            let diagnostics = jobs::scan_symbols(
                &script_state.script, 
                &script_state.buffer, 
                &job.script_path, 
                &mut self.symtab
            );

            self.diagnostics.insert(job.script_path.to_owned(), diagnostics);
        }
    }

    fn finish(self) -> (SymbolTable, HashMap<AbsPath, Vec<Diagnostic>>) {
        (self.symtab, self.diagnostics)
    }
}

struct SymbolScanJob {
    script_path: AbsPath,
    scripts: Arc<ScriptStates>
}

#[derive(Clone)]
struct SymbolScanJobProvider {
    script_paths: Arc<Mutex<Vec<AbsPath>>>,
    scripts: Arc<ScriptStates>
}

impl SymbolScanJobProvider {
    fn poll(&self) -> Option<SymbolScanJob> {
        let mut paths = self.script_paths.lock().unwrap();
        paths.pop().map(|p| {
            SymbolScanJob { script_path: p, scripts: self.scripts.clone() }
        })
    }
}
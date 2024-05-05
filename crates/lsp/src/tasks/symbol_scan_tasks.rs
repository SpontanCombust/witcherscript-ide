use std::{collections::HashMap, sync::{Arc, Mutex}};
use abs_path::AbsPath;
use rayon::prelude::*;
use tokio::{sync::oneshot, time::Instant};
use witcherscript_analysis::{diagnostics::AnalysisDiagnostic, jobs, model::collections::SymbolTable};
use witcherscript_project::SourceTreePath;
use crate::{reporting::{DiagnosticGroup, IntoLspDiagnostic}, Backend, ScriptStates};


impl Backend {
    /// modified_script_paths must belong to the same content as the symtab
    pub async fn scan_symbols(&self, symtab: &mut SymbolTable, content_path: &AbsPath, modified_source_paths: Vec<SourceTreePath>) {
        if modified_source_paths.is_empty() {
            return;
        }

        let start = Instant::now();

        for p in modified_source_paths.iter() {
            symtab.remove_for_source(p.local());
        }

        let worker_count = std::cmp::min(rayon::current_num_threads(), modified_source_paths.len());
        let scripts = Arc::clone(&self.scripts);
        let scripts_root = modified_source_paths.first().unwrap().script_root().to_owned();
        let scripts_root_clone = scripts_root.clone();

        let job_provider = SymbolScanJobProvider {
            script_paths: Arc::new(Mutex::new(modified_source_paths))
        };

        let (send, recv) = oneshot::channel();
        rayon::spawn(move || {
            let mut workers = Vec::with_capacity(worker_count);

            for _ in 0..worker_count {
                workers.push(SymbolScanWorker::new(job_provider.clone(), Arc::clone(&scripts)));
            }

            workers.par_iter_mut()
                .for_each(|w| w.work());

            let (mut merged_symtab, mut merged_diagnostics) = workers.pop().unwrap().finish();
            while let Some(worker) = workers.pop() {
                let (symtab, diagnostics) = worker.finish();
                merged_diagnostics.extend(diagnostics);
                jobs::merge_symbol_tables(&mut merged_symtab, symtab, &scripts_root, &mut merged_diagnostics);
            }

            send.send((merged_symtab, merged_diagnostics)).expect("on_scripts_modified symbol scan send fail");
        });

        let (scanning_symtab, mut scanning_diagnostis) = recv.await.expect("on_scripts_modified symbol scan recv fail");

        jobs::merge_symbol_tables(symtab, scanning_symtab, &scripts_root_clone,  &mut scanning_diagnostis);

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Updated symbol table for content {} in {:.3}s", content_path, duration.as_secs_f32())).await;

        for (file_path, diagnostics) in scanning_diagnostis {
            self.reporter.clear_diagnostics(&file_path, DiagnosticGroup::SymbolScan).await;
            self.reporter.push_diagnostics(&file_path, diagnostics.into_iter().map(|diag| diag.into_lsp_diagnostic()),  DiagnosticGroup::SymbolScan).await;
        }
    }
}

struct SymbolScanWorker {
    symtab: SymbolTable,
    diagnostics: HashMap<AbsPath, Vec<AnalysisDiagnostic>>,
    job_provider: SymbolScanJobProvider,
    scripts: Arc<ScriptStates>
}

impl SymbolScanWorker {
    fn new(job_provider: SymbolScanJobProvider, scripts: Arc<ScriptStates>) -> Self {
        Self {
            symtab: SymbolTable::new(),
            diagnostics: HashMap::new(),
            job_provider,
            scripts
        }
    }

    fn work(&mut self) {
        while let Some(job) = self.job_provider.poll() {
            let script_state = self.scripts.get(job.source_path.absolute()).unwrap();
            let diagnostics = jobs::scan_symbols(
                &script_state.script, 
                &script_state.buffer, 
                &job.source_path.local(),
                &job.source_path.script_root(),
                &mut self.symtab
            );

            self.diagnostics.insert(job.source_path.into_absolute(), diagnostics);
        }
    }

    fn finish(self) -> (SymbolTable, HashMap<AbsPath, Vec<AnalysisDiagnostic>>) {
        (self.symtab, self.diagnostics)
    }
}

struct SymbolScanJob {
    source_path: SourceTreePath
}

#[derive(Clone)]
struct SymbolScanJobProvider {
    script_paths: Arc<Mutex<Vec<SourceTreePath>>>
}

impl SymbolScanJobProvider {
    fn poll(&self) -> Option<SymbolScanJob> {
        let mut paths = self.script_paths.lock().unwrap();
        paths.pop().map(|source_path| {
            SymbolScanJob { source_path }
        })
    }
}
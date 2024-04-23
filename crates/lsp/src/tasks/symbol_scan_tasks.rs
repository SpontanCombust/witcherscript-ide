use std::{collections::HashMap, sync::{Arc, Mutex}};
use abs_path::AbsPath;
use rayon::prelude::*;
use tokio::sync::oneshot;
use witcherscript_analysis::{diagnostics::Diagnostic, jobs, model::collections::SymbolTable};
use crate::{reporting::{DiagnosticGroup, IntoLspDiagnostic}, Backend, ScriptStates};


impl Backend {
    /// modified_script_paths must belong to the same content as the symtab
    pub async fn scan_symbols(&self, symtab: &mut SymbolTable, modified_script_paths: &Vec<AbsPath>) {
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
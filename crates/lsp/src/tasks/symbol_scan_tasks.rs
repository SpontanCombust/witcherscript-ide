use std::sync::{Arc, Mutex};
use abs_path::AbsPath;
use rayon::prelude::*;
use tokio::{sync::oneshot, time::Instant};
use witcherscript_diagnostics::*;
use witcherscript_analysis::{jobs, symbol_analysis::symbol_table::SymbolTable};
use witcherscript_project::SourceTreePath;
use crate::{Backend, ScriptStates, SymbolTables};


impl Backend {
    pub async fn scan_symbols(&self, symtabs: &mut SymbolTables, content_path: &AbsPath, modified_source_paths: Vec<SourceTreePath>) {
        if modified_source_paths.is_empty() {
            return;
        }

        let symtab;
        if let Some(val) = symtabs.get_mut(content_path) {
            symtab = val;
        } else {
            return;
        }
 

        let worker_count = std::cmp::min(rayon::current_num_threads(), modified_source_paths.len());
        let scripts = Arc::clone(&self.scripts);
        let scripts_root = modified_source_paths.first().unwrap().script_root_arc();
        
        let job_provider = SymbolScanJobProvider {
            script_paths: Arc::new(Mutex::new(modified_source_paths.clone()))
        };


        let start = Instant::now();

        let (send, recv) = oneshot::channel();
        rayon::spawn(move || {
            let mut workers = Vec::with_capacity(worker_count);

            for _ in 0..worker_count {
                workers.push(SymbolScanWorker::new(job_provider.clone(), Arc::clone(&scripts), scripts_root.clone()));
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

        // while symbols are getting collected, remove old ones for files we're interested in
        for p in modified_source_paths.iter() {
            symtab.remove_symbols_for_source(p.local());
            self.reporter.clear_diagnostics(p.absolute(), DiagnosticDomain::SymbolAnalysis);
        }

        let (scanning_symtab, mut scanning_diagnostis) = recv.await.expect("on_scripts_modified symbol scan recv fail");
        jobs::merge_symbol_tables(symtab, scanning_symtab, &mut scanning_diagnostis);

        symtab.dispose_unreferenced_array_symbols();

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Updated symbol table for content {} in {:.3}s", content_path, duration.as_secs_f32())).await;


        for loc_diag in scanning_diagnostis {
            self.reporter.push_diagnostic(&loc_diag.path, loc_diag.diagnostic);
        }
    }
}

struct SymbolScanWorker {
    symtab: SymbolTable,
    diagnostics: Vec<LocatedDiagnostic>,
    job_provider: SymbolScanJobProvider,
    scripts: Arc<ScriptStates>
}

impl SymbolScanWorker {
    fn new(job_provider: SymbolScanJobProvider, scripts: Arc<ScriptStates>, scripts_root: Arc<AbsPath>) -> Self {
        Self {
            symtab: SymbolTable::new(scripts_root),
            diagnostics: Vec::new(),
            job_provider,
            scripts
        }
    }

    fn work(&mut self) {
        while let Some(job) = self.job_provider.poll() {
            let script_state = self.scripts.get(job.source_path.absolute()).unwrap();
            jobs::scan_symbols(
                &script_state.script, 
                &script_state.buffer, 
                &job.source_path.local(),
                &mut self.symtab,
                &mut self.diagnostics
            );
        }
    }

    fn finish(self) -> (SymbolTable, Vec<LocatedDiagnostic>) {
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
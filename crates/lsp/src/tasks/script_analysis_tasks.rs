use std::sync::Arc;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::{sync::oneshot, time::Instant};
use abs_path::AbsPath;
use witcherscript_analysis::jobs;
use witcherscript_diagnostics::*;
use crate::Backend;


impl Backend {
    pub async fn run_script_analysis(&self, script_paths: Vec<AbsPath>, full: bool) {
        let start = Instant::now();

        // first go analytics that can run on each keystroke
        self.syntax_analysis(script_paths.clone()).await;
        
        if full {
            // here should go more expensive analytics that should be done only when the file is explicitly saved
            
        }

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Analysis finished in {}s", duration.as_secs_f32())).await;
    }

    
    async fn syntax_analysis(&self, script_paths: Vec<AbsPath>) {
        for path in &script_paths {
            self.reporter.clear_diagnostics(path, DiagnosticDomain::SyntaxAnalysis);
        }

        let scripts = Arc::clone(&self.scripts);
        let (send, recv) = oneshot::channel();

        rayon::spawn(move || {
            let loc_diagnostics: Vec<_> = 
                script_paths.into_par_iter()
                .filter_map(move |script_path| {
                    if let Some(kv) = scripts.get(&script_path) {
                        let script_state = kv.value();
                        let script = &script_state.script;
                        let mut diags = Vec::new();
                        jobs::syntax_analysis(script, &mut diags);
                        drop(kv);

                        Some((script_path, diags))
                    } else {
                        None
                    }
                })
                .collect();

            send.send(loc_diagnostics).expect("syntax_analysis oneshot::send fail")
        });
        
        let results = recv.await.expect("syntax_analysis oneshot::recv fail");

        for (script_path, diags) in results {
            self.reporter.push_diagnostics(&script_path, diags);
        }
    }   
}
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::{sync::oneshot, time::Instant};
use abs_path::AbsPath;
use witcherscript_analysis::jobs;
use witcherscript_diagnostics::*;
use crate::Backend;


impl Backend {
    pub async fn run_script_analysis(&self, script_paths: Vec<AbsPath>, full: bool) {
        let config = self.config.read().await;
        let do_syntax_analysis = config.enable_syntax_analysis;
        drop(config);

        let start = Instant::now();

        // first go analytics that can run on each keystroke
        if do_syntax_analysis {
            self.syntax_analysis(script_paths.clone()).await;
        }
        
        if full {
            // here should go more expensive analytics that should be done only when the file is explicitly saved
            self.workspace_symbol_analysis(script_paths.clone()).await;
        }

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Analysis finished in {:.3}s", duration.as_secs_f32())).await;
    }

    
    async fn syntax_analysis(&self, script_paths: Vec<AbsPath>) {
        for path in &script_paths {
            self.reporter.clear_diagnostics(path, DiagnosticDomain::SyntaxAnalysis);
            self.reporter.clear_diagnostics(path, DiagnosticDomain::ContextualSyntaxAnalysis);
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
                        let doc = &script_state.buffer;
                        let mut diags = Vec::new();
                        jobs::syntax_analysis(script, &mut diags);
                        jobs::contextual_syntax_analysis(script, doc, &mut diags);
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

    async fn workspace_symbol_analysis(&self, script_paths: Vec<AbsPath>) {
        let mut grouped_by_content: HashMap<AbsPath, Vec<PathBuf>> = HashMap::new();

        for path in script_paths {
            self.reporter.clear_diagnostics(&path, DiagnosticDomain::WorkspaceSymbolAnalysis);

            if let Some(script_state) = self.scripts.get(&path) {
                if let Some(content_info) = &script_state.content_info {
                    grouped_by_content.entry(content_info.content_path.clone())
                        .or_default()
                        .push(content_info.source_tree_path.local().to_owned());
                }
            }
        }

        let symtabs = self.symtabs.read().await;

        let mut diagnostics = Vec::new();
        for (content_path, local_source_paths) in grouped_by_content {
            if let Some(content_symtab) = symtabs.get(&content_path) {
                let marcher = self.march_symbol_tables(&symtabs, &content_path).await;

                jobs::workspace_symbol_analysis(content_symtab, marcher, local_source_paths, &mut diagnostics);
            }
        }

        drop(symtabs);

        if !diagnostics.is_empty() {
            diagnostics.sort_by(|ld1, ld2| ld1.path.cmp(&ld2.path));

            let mut current_path = diagnostics[0].path.clone();
            for ld in diagnostics {
                if ld.path != current_path {
                    current_path = ld.path;
                    self.reporter.clear_diagnostics(&current_path, DiagnosticDomain::WorkspaceSymbolAnalysis);
                }

                self.reporter.push_diagnostic(&current_path, ld.diagnostic);
            }
        }
    } 
}
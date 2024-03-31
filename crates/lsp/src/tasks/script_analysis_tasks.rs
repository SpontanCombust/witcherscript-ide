use std::sync::Arc;
use bitmask_enum::bitmask;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use tokio::sync::mpsc;
use abs_path::AbsPath;
use witcherscript::Script;
use witcherscript_analysis::diagnostics::Diagnostic;
use crate::{reporting::IntoLspDiagnostic, Backend};


#[bitmask(u8)]
pub enum ScriptAnalysisKind {
    SyntaxAnalysis
}

impl Backend {
    pub async fn run_script_analysis<It>(&self, script_paths: It, analysis_kinds: ScriptAnalysisKind) 
    where It: IntoParallelIterator<Item = AbsPath> + Send + 'static {
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        let scripts = Arc::clone(&self.scripts);
        rayon::spawn(move || {
            script_paths.into_par_iter()
                .for_each(move |script_path| {
                    if let Some(kv) = scripts.get(&script_path) {
                        let script = &kv.value().script;
                        let diagnostics = diagnose_script(script, analysis_kinds)
                            .into_iter()
                            .map(|d| d.into_lsp_diagnostic());

                        send.blocking_send((script_path, diagnostics)).expect("run_script_analysis mpsc::send fail");
                    }    
                });
        });

        while let Some((script_path, diags)) = recv.recv().await {
            self.reporter.clear_diagnostics(script_path.as_ref());
            self.reporter.push_diagnostics(script_path.as_ref(), diags);
        }
    }

    pub async fn run_script_analysis_for_single(&self, script_path: &AbsPath, analysis_kinds: ScriptAnalysisKind) {
        if let Some(kv) = self.scripts.get(script_path) {
            let script = &kv.value().script;

            let diagnostics = diagnose_script(script, analysis_kinds)
                .into_iter()
                .map(|d| d.into_lsp_diagnostic());
            
            self.reporter.clear_diagnostics(script_path);
            self.reporter.push_diagnostics(script_path, diagnostics);
        }
    }

    pub async fn run_script_analysis_for_content(&self, content_path: &AbsPath, analysis_kinds: ScriptAnalysisKind) {
        if let Some(kv) = self.source_trees.get(content_path) {
            let tree = kv.value();
            let script_paths: Vec<_> = tree.iter().map(|p| p.absolute().to_owned()).collect();
            self.run_script_analysis(script_paths, analysis_kinds).await;
        }
    }

    pub async fn run_script_analysis_for_all(&self, analysis_kinds: ScriptAnalysisKind) {
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        let scripts = Arc::clone(&self.scripts);
        rayon::spawn(move || {
            scripts.iter().par_bridge()
                .for_each(move |kv| {
                    let script_path = kv.key().to_owned();
                    let script = &kv.value().script;
                    let diagnostics = diagnose_script(script, analysis_kinds)
                        .into_iter()
                        .map(|d| d.into_lsp_diagnostic());

                    send.blocking_send((script_path, diagnostics)).expect("run_script_analysis mpsc::send fail");
                });
        });

        while let Some((script_path, diags)) = recv.recv().await {
            self.reporter.clear_diagnostics(script_path.as_ref());
            self.reporter.push_diagnostics(script_path.as_ref(), diags);
        }
    }
}

fn diagnose_script(script: &Script, analysis_kinds: ScriptAnalysisKind) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    
    if analysis_kinds.contains(ScriptAnalysisKind::SyntaxAnalysis) {
        witcherscript_analysis::jobs::syntax_analysis(script.root_node(), &mut diagnostics);
    }

    diagnostics
}
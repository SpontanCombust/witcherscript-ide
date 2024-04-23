use std::sync::Arc;
use bitmask_enum::bitmask;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use tokio::sync::mpsc;
use abs_path::AbsPath;
use witcherscript::Script;
use witcherscript_analysis::diagnostics::AnalysisDiagnostic;
use crate::{reporting::{DiagnosticGroup, IntoLspDiagnostic}, Backend, ScriptState};


#[bitmask(u8)]
pub enum ScriptAnalysisKind {
    SyntaxAnalysis
}

impl ScriptAnalysisKind {
    pub fn suggested_for_script(script_state: &ScriptState) -> Self {
        if script_state.is_foreign {
            ScriptAnalysisKind::SyntaxAnalysis
        } else {
            ScriptAnalysisKind::all()
        }
    }
}

impl Backend {
    pub async fn run_script_analysis<It>(&self, script_paths: It) 
    where It: IntoParallelIterator<Item = AbsPath> + Send + 'static {
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        let scripts = Arc::clone(&self.scripts);
        rayon::spawn(move || {
            script_paths.into_par_iter()
                .for_each(move |script_path| {
                    if let Some(kv) = scripts.get(&script_path) {
                        let script_state = kv.value();
                        let script = &script_state.script;
                        let analysis_kinds = ScriptAnalysisKind::suggested_for_script(script_state);
                        let diagnostics = diagnose_script(script, analysis_kinds)
                            .into_iter()
                            .map(|d| d.into_lsp_diagnostic());

                        send.blocking_send((script_path, diagnostics)).expect("run_script_analysis mpsc::send fail");
                    }    
                });
        });

        while let Some((script_path, diags)) = recv.recv().await {
            self.reporter.clear_diagnostics(script_path.as_ref(), DiagnosticGroup::Analysis);
            self.reporter.push_diagnostics(script_path.as_ref(), diags,  DiagnosticGroup::Analysis);
        }
    }

    pub async fn run_script_analysis_for_all(&self) {
        let (send, mut recv) = mpsc::channel(rayon::current_num_threads());

        let scripts = Arc::clone(&self.scripts);
        rayon::spawn(move || {
            scripts.iter().par_bridge()
                .for_each(move |kv| {
                    let script_path = kv.key().to_owned();
                    let script_state = kv.value();
                    let script = &script_state.script;
                    let analysis_kinds = ScriptAnalysisKind::suggested_for_script(script_state);
                    let diagnostics = diagnose_script(script, analysis_kinds)
                        .into_iter()
                        .map(|d| d.into_lsp_diagnostic());

                    send.blocking_send((script_path, diagnostics)).expect("run_script_analysis mpsc::send fail");
                });
        });

        while let Some((script_path, diags)) = recv.recv().await {
            self.reporter.clear_diagnostics(script_path.as_ref(), DiagnosticGroup::Analysis);
            self.reporter.push_diagnostics(script_path.as_ref(), diags, DiagnosticGroup::Analysis);
        }
    }
}

fn diagnose_script(script: &Script, analysis_kinds: ScriptAnalysisKind) -> Vec<AnalysisDiagnostic> {
    let mut diagnostics: Vec<AnalysisDiagnostic> = Vec::new();
    
    if analysis_kinds.contains(ScriptAnalysisKind::SyntaxAnalysis) {
        witcherscript_analysis::jobs::syntax_analysis(script.root_node(), &mut diagnostics);
    }

    diagnostics
}
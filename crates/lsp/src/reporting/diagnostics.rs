use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript_analysis::diagnostics::{AnalysisDiagnostic, AnalysisDiagnosticBody};
use witcherscript_project::{manifest, redkit, FileError};
use crate::Backend;
use super::Reporter;


pub trait IntoLspDiagnostic {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic;
}

impl IntoLspDiagnostic for AnalysisDiagnostic {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic {
        lsp::Diagnostic {
            range: self.range,
            severity: Some(match self.body {
                AnalysisDiagnosticBody::Error(_) => lsp::DiagnosticSeverity::ERROR,
                AnalysisDiagnosticBody::Warning(_) => lsp::DiagnosticSeverity::WARNING,
                AnalysisDiagnosticBody::Info(_) => lsp::DiagnosticSeverity::INFORMATION,
            }),
            source: Some(Backend::SERVER_NAME.to_string()),
            message: self.body.to_string(),
            ..Default::default()
        }
    }
}

impl IntoLspDiagnostic for FileError<std::io::Error> {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic {
        lsp::Diagnostic {
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 1)),
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            source: Some(Backend::SERVER_NAME.to_string()),
            message: self.error.to_string(),
            ..Default::default()
        }
    }
}

impl IntoLspDiagnostic for FileError<manifest::Error> {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic {
        let error = self.error.as_ref();
        
        let range = match error {
            manifest::Error::Io(_) => lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 1)),
            manifest::Error::Toml { range, msg: _ } => range.clone(),
            manifest::Error::InvalidNameField { range } => range.clone(),
        };

        let message = error.to_string();

        lsp::Diagnostic {
            range,
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            source: Some(Backend::SERVER_NAME.to_string()),
            message,
            ..Default::default()
        }
    }
}

impl IntoLspDiagnostic for FileError<redkit::manifest::Error> {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic {
        let error = self.error.as_ref();

        let range = match error {
            redkit::manifest::Error::Io(_) => lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 1)),
            redkit::manifest::Error::Json { position, msg: _ } => lsp::Range::new(position.to_owned(), lsp::Position::new(position.line, position.character + 1)),
        };

        let message = error.to_string();

        lsp::Diagnostic {
            range,
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            source: Some(Backend::SERVER_NAME.to_string()),
            message,
            ..Default::default()
        }
    }
}

impl IntoLspDiagnostic for (String, lsp::Range) {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic {
        lsp::Diagnostic {
            range: self.1,
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            source: Some(Backend::SERVER_NAME.to_string()),
            message: self.0,
            ..Default::default()
        }
    }
}


#[derive(Debug)]
pub struct BufferedDiagnostics {
    diags: Vec<BufferedDiagnostic>,
    changed: bool,
    should_purge: bool
}

#[derive(Debug)]
struct BufferedDiagnostic {
    diag: lsp::Diagnostic,
    group: DiagnosticGroup
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticGroup {
    ContentScan,
    SymbolScan,
    Analysis
}

impl Reporter {
    pub fn push_diagnostic(&self, path: &AbsPath, diag: lsp::Diagnostic, group: DiagnosticGroup) {
        let bd = BufferedDiagnostic { diag, group };
        if let Some(mut kv) = self.buffered_diagnostics.get_mut(path) {
            let v = kv.value_mut();
            v.diags.push(bd);
            v.changed = true;
        } else {
            self.buffered_diagnostics.insert(path.clone(), BufferedDiagnostics {
                diags: vec![bd],
                changed: true,
                should_purge: false
            });
        }
    }

    pub fn push_diagnostics(&self, path: &AbsPath, diags: impl IntoIterator<Item = lsp::Diagnostic>, group: DiagnosticGroup) {
        let bds = diags.into_iter().map(|diag| BufferedDiagnostic { diag, group });
        if let Some(mut kv) = self.buffered_diagnostics.get_mut(path) {
            let v = kv.value_mut();
            v.diags.extend(bds);
            v.changed = true;
        } else {
            self.buffered_diagnostics.insert(path.clone(), BufferedDiagnostics {
                diags: bds.collect(),
                changed: true,
                should_purge: false
            });
        }
    }

    pub fn clear_diagnostics(&self, path: &AbsPath, source: DiagnosticGroup) {
        self.buffered_diagnostics
            .alter(path, |_, mut v|  {
                v.diags.retain(|d| d.group != source);
                v.changed = true;
                v
            });
    }

    /// Clears all diagnostics for a given file and additionally that file gets forgotten about by the diagnostic system
    pub fn purge_diagnostics(&self, path: &AbsPath) {
        self.buffered_diagnostics
            .alter(path, |_, mut v|  {
                v.diags.clear();
                v.changed = true;
                v.should_purge = true;
                v
            });
    }

    pub fn clear_all_diagnostics(&self) {
        self.buffered_diagnostics
            .alter_all(|_, mut v| {
                v.diags.clear();
                v.changed = true;
                v
            });
    }


    pub async fn commit_diagnostics(&self, path: &AbsPath) {
        if let Some(mut kv) = self.buffered_diagnostics.get_mut(path) {
            if kv.value().changed {
                let uri = kv.key().to_uri();
    
                let v = kv.value_mut();
                let diags = v.diags.iter().map(|d| d.diag.clone()).collect();
                v.changed = false;
    
                self.client.publish_diagnostics(uri, diags, None).await;
            }
        }

        self.buffered_diagnostics.remove_if(path, |_, v| v.should_purge);
    }

    pub async fn commit_all_diagnostics(&self) {
        for mut kv in self.buffered_diagnostics.iter_mut().filter(|kv| kv.value().changed) {
            let uri = kv.key().to_uri();

            let v = kv.value_mut();
            let diags = v.diags.iter().map(|d| d.diag.clone()).collect();
            v.changed = false;

            self.client.publish_diagnostics(uri, diags, None).await;
        }

        self.buffered_diagnostics.retain(|_, v| !v.should_purge);
    }
}

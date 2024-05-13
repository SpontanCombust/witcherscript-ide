use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript_project::{manifest, redkit, FileError};
use crate::Backend;
use super::Reporter;


pub trait IntoLspDiagnostic {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic;
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
    pub async fn push_diagnostic(&self, path: &AbsPath, diag: lsp::Diagnostic, group: DiagnosticGroup) {
        let bd = BufferedDiagnostic { diag, group };
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.push(bd);
            v.changed = true;
        } else {
            diags.insert(path.clone(), BufferedDiagnostics {
                diags: vec![bd],
                changed: true,
                should_purge: false
            });
        }
    }

    pub async fn push_diagnostics(&self, path: &AbsPath, diags: impl IntoIterator<Item = lsp::Diagnostic>, group: DiagnosticGroup) {
        let bds = diags.into_iter().map(|diag| BufferedDiagnostic { diag, group });
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.extend(bds);
            v.changed = true;
        } else {
            diags.insert(path.clone(), BufferedDiagnostics {
                diags: bds.collect(),
                changed: true,
                should_purge: false
            });
        }
    }

    pub async fn clear_diagnostics(&self, path: &AbsPath, source: DiagnosticGroup) {
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.retain(|d| d.group != source);
            v.changed = true;
        }
    }

    /// Clears all diagnostics for a given file and additionally that file gets forgotten about by the diagnostic system
    pub async fn purge_diagnostics(&self, path: &AbsPath) {
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.clear();
            v.changed = true;
            v.should_purge = true;
        }
    }

    pub async fn clear_all_diagnostics(&self) {
        let mut diags = self.buffered_diagnostics.lock().await;
        for (_, v) in diags.iter_mut() {
            v.diags.clear();
            v.changed = true;
        }
    }


    pub async fn commit_diagnostics(&self, path: &AbsPath) {
        let mut diags = self.buffered_diagnostics.lock().await;
        let mut should_purge = false;
        if let Some(v) = diags.get_mut(path) {
            if v.changed {
                let uri = path.to_uri();
    
                let to_publish = v.diags.iter().map(|d| d.diag.clone()).collect();
                v.changed = false;
        
                self.client.publish_diagnostics(uri, to_publish, None).await;
            }

            should_purge = v.should_purge;            
        }

        if should_purge {
            diags.remove(path);
        }
    }

    pub async fn commit_all_diagnostics(&self) {
        let mut diags = self.buffered_diagnostics.lock().await;
        for (k, v) in diags.iter_mut().filter(|(_, v)| v.changed) {
            let uri = k.to_uri();

            let to_publish = v.diags.iter().map(|d| d.diag.clone()).collect();
            v.changed = false;

            self.client.publish_diagnostics(uri, to_publish, None).await;
        }

        diags.retain(|_, v| !v.should_purge);
    }
}

use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript_analysis::diagnostics::{Diagnostic, DiagnosticBody};
use witcherscript_project::{manifest::ManifestParseError, redkit, FileError};
use crate::Backend;
use super::Reporter;


pub trait IntoLspDiagnostic {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic;
}

impl IntoLspDiagnostic for Diagnostic {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic {
        lsp::Diagnostic {
            range: self.range,
            severity: Some(match self.body {
                DiagnosticBody::Error(_) => lsp::DiagnosticSeverity::ERROR,
                DiagnosticBody::Warning(_) => lsp::DiagnosticSeverity::WARNING,
                DiagnosticBody::Info(_) => lsp::DiagnosticSeverity::INFORMATION,
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

impl IntoLspDiagnostic for FileError<ManifestParseError> {
    fn into_lsp_diagnostic(self) -> lsp::Diagnostic {
        let error = self.error.as_ref();
        
        let range = match error {
            ManifestParseError::Io(_) => lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 1)),
            ManifestParseError::Toml { range, msg: _ } => range.clone(),
            ManifestParseError::InvalidNameField { range } => range.clone(),
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
    pub diags: Vec<lsp::Diagnostic>,
    pub changed: bool,
    pub should_purge: bool
}

impl Reporter {
    pub fn push_diagnostic(&self, path: &AbsPath, diag: lsp::Diagnostic) {
        if let Some(mut kv) = self.buffered_diagnostics.get_mut(path) {
            let v = kv.value_mut();
            v.diags.push(diag);
            v.changed = true;
        } else {
            self.buffered_diagnostics.insert(path.clone(), BufferedDiagnostics {
                diags: vec![diag],
                changed: true,
                should_purge: false
            });
        }
    }

    pub fn push_diagnostics(&self, path: &AbsPath, diags: impl IntoIterator<Item = lsp::Diagnostic>) {
        if let Some(mut kv) = self.buffered_diagnostics.get_mut(path) {
            let v = kv.value_mut();
            v.diags.extend(diags.into_iter());
            v.changed = true;
        } else {
            self.buffered_diagnostics.insert(path.clone(), BufferedDiagnostics {
                diags: diags.into_iter().collect(),
                changed: true,
                should_purge: false
            });
        }
    }

    pub fn clear_diagnostics(&self, path: &AbsPath) {
        self.buffered_diagnostics
            .alter(path, |_, mut v|  {
                v.diags.clear();
                v.changed = true;
                v
            });
    }

    /// In addition to clearing diagnostics for a given file, said file will be forgotten about
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
                let diags = v.diags.drain(..).collect();
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
            let diags = v.diags.drain(..).collect();
            v.changed = false;

            self.client.publish_diagnostics(uri, diags, None).await;
        }

        self.buffered_diagnostics.retain(|_, v| !v.should_purge);
    }
}

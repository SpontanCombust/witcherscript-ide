use std::path::{Path, PathBuf};
use tower_lsp::lsp_types as lsp;
use witcherscript_analysis::diagnostics::{Diagnostic, DiagnosticBody};
use witcherscript_project::{manifest::ManifestParseError, FileError};
use crate::Backend;


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


pub trait TryIntoUrl {
    fn try_into_url(self) -> Result<lsp::Url, ()>;
}

impl TryIntoUrl for lsp::Url {
    fn try_into_url(self) -> Result<lsp::Url, ()> {
        Ok(self)
    }
}

impl TryIntoUrl for PathBuf {
    fn try_into_url(self) -> Result<lsp::Url, ()> {
        let path = self.canonicalize().map_err(|_| ())?;
        lsp::Url::from_file_path(path)
    }
}

impl TryIntoUrl for &Path {
    fn try_into_url(self) -> Result<lsp::Url, ()> {
        let path = self.canonicalize().map_err(|_| ())?;
        lsp::Url::from_file_path(path)
    }
}

impl Backend {
    pub async fn publish_diagnostics<P: TryIntoUrl>(&self, path: P, diags: impl IntoIterator<Item = lsp::Diagnostic>) {
        if let Ok(url) = path.try_into_url() {
            self.client.publish_diagnostics(url, diags.into_iter().collect(), None).await;
        }
    }

    pub async fn clear_diagnostics<P: TryIntoUrl>(&self, path: P) {
        if let Ok(url) = path.try_into_url() {
            self.client.publish_diagnostics(url, Vec::new(), None).await;
        }
    }

    pub async fn clear_all_diagnostics(&self) {
        let _ = self.client.workspace_diagnostic_refresh().await;
    }
}
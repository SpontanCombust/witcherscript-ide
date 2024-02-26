use tower_lsp::lsp_types::{self as lsp, Position};
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
            range: lsp::Range::new(Position::new(0, 0), Position::new(0, 1)),
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
            ManifestParseError::Io(_) => lsp::Range::new(Position::new(0, 0), Position::new(0, 1)),
            ManifestParseError::Toml { range, msg: _ } => range.clone(),
        };

        let message = match error {
            ManifestParseError::Io(err) => err.to_string(),
            ManifestParseError::Toml { range: _, msg } => msg.clone(),
        };

        lsp::Diagnostic {
            range,
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            source: Some(Backend::SERVER_NAME.to_string()),
            message,
            ..Default::default()
        }
    }
}
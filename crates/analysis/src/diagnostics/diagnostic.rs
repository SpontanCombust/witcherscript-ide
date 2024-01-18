use std::fmt::Display;
use lsp_types::Range;
use super::{ErrorDiagnostic, WarningDiagnostic, InfoDiagnostic};


#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: Range,
    pub body: DiagnosticBody
}

#[derive(Debug, Clone)]
pub enum DiagnosticBody {
    Error(ErrorDiagnostic),
    Warning(WarningDiagnostic),
    Info(InfoDiagnostic)
}

impl From<ErrorDiagnostic> for DiagnosticBody {
    fn from(value: ErrorDiagnostic) -> Self {
        Self::Error(value)
    }
}

impl From<WarningDiagnostic> for DiagnosticBody {
    fn from(value: WarningDiagnostic) -> Self {
        Self::Warning(value)
    }
}

impl From<InfoDiagnostic> for DiagnosticBody {
    fn from(value: InfoDiagnostic) -> Self {
        Self::Info(value)
    }
}

impl Display for DiagnosticBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticBody::Error(err) => write!(f, "{}", err),
            DiagnosticBody::Warning(warn) => write!(f, "{}", warn),
            DiagnosticBody::Info(info) => write!(f, "{}", info),
        }
    }
}
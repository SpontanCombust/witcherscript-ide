use witcherscript::DocSpan;
use super::{ErrorDiagnostic, WarningDiagnostic, InfoDiagnostic};


#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub span: DocSpan,
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

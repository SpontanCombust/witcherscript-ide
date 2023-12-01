use witcherscript::DocSpan;

use crate::model::symbols::SymbolType;


#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub span: DocSpan,
    pub severity: DiagnosticSeverity,
    pub body: DiagnosticBody
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info
}

#[derive(Debug, Clone)]
pub enum DiagnosticBody {
    MissingElement {
        what: String
    },
    SymbolNameTaken {
        name: String,
        this_type: SymbolType,
        precursor_type: SymbolType,
        // precursor_span: DocSpan, //TODO symbols storing their spans
    },
    TypeNotFound,
}
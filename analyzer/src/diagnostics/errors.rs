use crate::model::symbols::SymbolType;


#[derive(Debug, Clone)]
pub enum ErrorDiagnostic {
    Syntax(SyntaxErrorDiagnostic),
    SymbolNameTaken {
        name: String,
        this_type: SymbolType,
        precursor_type: SymbolType,
        // precursor_span: DocSpan, //TODO symbols storing their spans
    },
    TypeNotFound,
    MissingTypeArg,
    UnnecessaryTypeArg,
    RepeatedSpecifier,
    MultipleAccessModifiers
}

#[derive(Debug, Clone)]
pub enum SyntaxErrorDiagnostic {
    MissingElement(String),
    UnexpectedElement(String),
    Other
}
use crate::model::symbols::SymbolType;


#[derive(Debug, Clone)]
pub enum ErrorDiagnostic {
    Syntax(SyntaxErrorDiagnostic),
    SymbolNameTaken {
        name: String,
        this_type: SymbolType,
        precursor_type: SymbolType,
        // precursor_range: Range, //TODO symbols storing their ranges
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
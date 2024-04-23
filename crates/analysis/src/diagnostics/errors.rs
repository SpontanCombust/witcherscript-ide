use std::fmt::Display;
use lsp_types as lsp;
use abs_path::AbsPath;
use crate::model::symbols::SymbolType;


#[derive(Debug, Clone)]
pub enum AnalysisError {
    Syntax(SyntaxErrorDiagnostic),
    SymbolNameTaken {
        name: String,
        this_type: SymbolType,
        precursor_type: SymbolType,
        precursor_file_path: Option<AbsPath>,
        precursor_range: Option<lsp::Range>
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


impl Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::Syntax(err) => write!(f, "Syntax error: {}", err),
            AnalysisError::SymbolNameTaken { name, .. } => write!(f, "The name {} is defined multiple times", name),
            AnalysisError::TypeNotFound => write!(f, "Type could not be found"),
            AnalysisError::MissingTypeArg => write!(f, "Missing type argument"),
            AnalysisError::UnnecessaryTypeArg => write!(f, "This type does not take any type arguments"),
            AnalysisError::RepeatedSpecifier => write!(f, "Repeated specifier"),
            AnalysisError::MultipleAccessModifiers => write!(f, "Only one access modifier is allowed"),
        }
    }
}

impl Display for SyntaxErrorDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxErrorDiagnostic::MissingElement(s) => write!(f, "expected {}", s),
            SyntaxErrorDiagnostic::UnexpectedElement(s) => write!(f, "unexpected {}", s),
            SyntaxErrorDiagnostic::Other => write!(f, "invalid syntax"),
        }
    }
}
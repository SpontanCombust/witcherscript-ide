use std::fmt::Display;
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


impl Display for ErrorDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorDiagnostic::Syntax(err) => write!(f, "Syntax error: {}", err),
            ErrorDiagnostic::SymbolNameTaken { name, this_type: _, precursor_type: _ } => write!(f, "Name {} is already taken", name),
            ErrorDiagnostic::TypeNotFound => write!(f, "Type could not be found"),
            ErrorDiagnostic::MissingTypeArg => write!(f, "Missing type argument"),
            ErrorDiagnostic::UnnecessaryTypeArg => write!(f, "This type does not take any type arguments"),
            ErrorDiagnostic::RepeatedSpecifier => write!(f, "Repeated specifier"),
            ErrorDiagnostic::MultipleAccessModifiers => write!(f, "Only one access modifier is allowed"),
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
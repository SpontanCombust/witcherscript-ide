use crate::model::symbols::SymbolType;


#[derive(Debug, Clone)]
pub enum ErrorDiagnostic {
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
    MissingGenericArg,
    RepeatedSpecifier,
    MultipleAccessModifiers
}
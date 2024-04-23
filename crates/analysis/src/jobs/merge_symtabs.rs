use std::collections::HashMap;
use abs_path::AbsPath;
use crate::{diagnostics::{AnalysisDiagnostic, AnalysisError}, model::collections::SymbolTable};


pub fn merge_symbol_tables(target_symtab: &mut SymbolTable, source_symtab: SymbolTable, diagnostics: &mut HashMap<AbsPath, Vec<AnalysisDiagnostic>>) {
    for (file_path, errors) in target_symtab.merge(source_symtab) {
        let errors_as_diags = errors.into_iter()
            .map(|err| AnalysisDiagnostic { 
                range: err.incoming_location.range, 
                body: AnalysisError::SymbolNameTaken { 
                    name: err.occupied_path.components().last().unwrap().name.to_string(), 
                    this_type: err.incoming_type, 
                    precursor_type: err.occupied_type,
                    precursor_range: err.occupied_location.as_ref().map(|loc| loc.range),
                    precursor_file_path: err.occupied_location.as_ref().map(|loc| loc.file_path.clone())
                }.into()
            });

        diagnostics
            .entry(file_path)
            .or_default()
            .extend(errors_as_diags);
    }
}
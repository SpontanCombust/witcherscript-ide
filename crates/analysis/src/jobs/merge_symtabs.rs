use std::collections::HashMap;
use abs_path::AbsPath;
use crate::{diagnostics::{AnalysisDiagnostic, AnalysisError}, model::collections::SymbolTable};


/// Key in the `diagnostics` map is a local path of the source file in the source tree
pub fn merge_symbol_tables(
    target_symtab: &mut SymbolTable, 
    source_symtab: SymbolTable,
    scripts_root: &AbsPath,
    diagnostics: &mut HashMap<AbsPath, Vec<AnalysisDiagnostic>>
) {
    for (local_source_path, errors) in target_symtab.merge(source_symtab) {
        let abs_source_path = scripts_root.join(local_source_path).unwrap();

        let errors_as_diags = errors.into_iter()
            .map(|err| AnalysisDiagnostic { 
                range: err.incoming_location.label_range, 
                body: AnalysisError::SymbolNameTaken { 
                    name: err.occupied_path.components().last().unwrap().name.to_string(),
                    precursor_range: err.occupied_location.as_ref().map(|loc| loc.label_range),
                    precursor_file_path: err.occupied_location.as_ref().map(|loc| scripts_root.join(&loc.local_source_path).unwrap())
                }.into()
            });

        diagnostics
            .entry(abs_source_path)
            .or_default()
            .extend(errors_as_diags);
    }
}
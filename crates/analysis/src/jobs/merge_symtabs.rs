use std::collections::HashMap;
use abs_path::AbsPath;
use witcherscript_diagnostics::*;
use crate::model::collections::SymbolTable;


/// Key in the `diagnostics` map is a local path of the source file in the source tree
pub fn merge_symbol_tables(
    target_symtab: &mut SymbolTable, 
    source_symtab: SymbolTable,
    diagnostics: &mut HashMap<AbsPath, Vec<Diagnostic>>
) {
    for (local_source_path, errors) in target_symtab.merge(source_symtab) {
        let abs_source_path = target_symtab.script_root().join(local_source_path).unwrap();

        let errors_as_diags = errors.into_iter()
            .map(|err| Diagnostic { 
                range: err.incoming_location.label_range, 
                kind: DiagnosticKind::SymbolNameTaken { 
                    name: err.occupied_path.components().last().unwrap().name.to_string(),
                    precursor_range: err.occupied_location.as_ref().map(|loc| loc.label_range),
                    precursor_file_path: err.occupied_location.as_ref().map(|loc| target_symtab.script_root().join(&loc.local_source_path).unwrap())
                }.into()
            });

        diagnostics
            .entry(abs_source_path)
            .or_default()
            .extend(errors_as_diags);
    }
}
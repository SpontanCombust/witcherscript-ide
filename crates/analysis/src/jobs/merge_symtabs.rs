use witcherscript_diagnostics::*;
use crate::symbol_analysis::symbol_table::SymbolTable;


pub fn merge_symbol_tables(
    target_symtab: &mut SymbolTable, 
    source_symtab: SymbolTable,
    diagnostics: &mut Vec<LocatedDiagnostic>
) {
    diagnostics.extend(
        target_symtab
        .merge(source_symtab)
        .into_iter()
        .map(|err| {
            LocatedDiagnostic {
                path: err.incoming_location.abs_source_path(),
                diagnostic: Diagnostic { 
                    range: err.incoming_location.label_range, 
                    kind: DiagnosticKind::SymbolNameTaken { 
                        name: err.occupied_path.components().last().unwrap().name.to_string(),
                        precursor_range: err.occupied_location.as_ref().map(|loc| loc.label_range),
                        precursor_file_path: err.occupied_location.as_ref().map(|loc| loc.abs_source_path())
                    }
                }
            }
        })
    );
}
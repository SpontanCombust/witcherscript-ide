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
            let (occupied_file_path, occupied_range) = err.occupied_location
                .map(|loc| (Some(loc.abs_source_path()), Some(loc.label_range)))
                .unwrap_or((None, None));
            
            LocatedDiagnostic {
                path: err.incoming_location.abs_source_path(),
                diagnostic: Diagnostic { 
                    range: err.incoming_location.label_range, 
                    kind: DiagnosticKind::SymbolNameTaken { 
                        name: err.occupied_path.components().last().unwrap().name.to_string(),
                        precursor_file_path: occupied_file_path,
                        precursor_range: occupied_range
                    }
                }
            }
        })
    );
}
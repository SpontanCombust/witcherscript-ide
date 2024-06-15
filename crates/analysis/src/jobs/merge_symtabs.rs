use witcherscript_diagnostics::*;
use crate::symbol_analysis::{symbol_table::SymbolTable, symbols::SymbolType};


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

            match (err.occupied_typ, err.incoming_typ) {
                (SymbolType::MemberFunction, SymbolType::MemberFunctionReplacer) |
                (SymbolType::MemberFunction, SymbolType::MemberFunctionWrapper)  |
                (SymbolType::GlobalFunction, SymbolType::GlobalFunctionReplacer) => {
                    LocatedDiagnostic { 
                        path: err.incoming_location.abs_source_path(), 
                        diagnostic: Diagnostic { 
                            range: err.incoming_location.label_range, 
                            kind: DiagnosticKind::SameContentAnnotation { 
                                original_file_path: occupied_file_path,
                                original_range: occupied_range
                            }
                        }
                    }
                },
                (SymbolType::MemberFunctionReplacer, SymbolType::MemberFunction) |
                (SymbolType::MemberFunctionWrapper, SymbolType::MemberFunction)  |
                (SymbolType::GlobalFunctionReplacer, SymbolType::GlobalFunction) => {
                    LocatedDiagnostic {
                        path: occupied_file_path.expect("Annotation symbol without location"), 
                        diagnostic: Diagnostic { 
                            range: occupied_range.unwrap_or_default(), 
                            kind: DiagnosticKind::SameContentAnnotation { 
                                original_file_path: Some(err.incoming_location.abs_source_path()),
                                original_range: Some(err.incoming_location.range)
                            }
                        }
                    }
                },
                _ => {
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
                }
            }
        })
    );
}
use std::path::PathBuf;
use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};
use witcherscript_diagnostics::{Diagnostic, DiagnosticKind, LocatedDiagnostic};
use crate::symbol_analysis::{symbol_table::{marcher::SymbolTableMarcher, SymbolTable}, symbols::SymbolType};


pub fn workspace_symbol_analysis(target_symtab: &SymbolTable, marcher: SymbolTableMarcher, local_source_paths: Vec<PathBuf>, diagnostics: &mut Vec<LocatedDiagnostic>) {
    let marcher = marcher.skip_first_step(true);

    let diags_iter = local_source_paths.into_par_iter()
        .map(|p| target_symtab.get_primary_symbols_for_source(&p))
        .flatten_iter()
        // ignore conflicts with annotation symbols
        // they are expected to have the same symbol paths
        .filter(|p| !matches!(p.typ(), SymbolType::MemberFunctionWrapper | SymbolType::MemberFunctionReplacer | SymbolType::GlobalFunctionReplacer))
        .filter_map(|primary| {
            let primary_loc = primary.location().unwrap(); // primary symbols always have location
            if let Err(err) = marcher.test_contains_symbol(primary.path()) {
                Some(LocatedDiagnostic {
                    path: primary_loc.abs_source_path(),
                    diagnostic: Diagnostic {
                        kind: DiagnosticKind::SymbolNameTakenInDependency { 
                            name: primary.name().to_string(), 
                            precursor_file_path: err.occupied_location.as_ref().map(|loc| loc.abs_source_path()), 
                            precursor_range: err.occupied_location.as_ref().map(|loc| loc.label_range)
                        },
                        range: primary_loc.label_range
                    },
                })
            } else {
                None
            }
        });

    diagnostics.par_extend(diags_iter);
}

use lsp_types::Range;
use ropey::Rope;
use uuid::Uuid;
use witcherscript::ast::TypeAnnotationNode;
use witcherscript::tokens::IdentifierNode;
use crate::model::collections::*;
use crate::model::symbols::{SymbolType, ArrayTypeSymbol, SymbolCategory, ERROR_SYMBOL_ID};
use crate::jobs::inject_native_symbols::inject_array_type;
use crate::diagnostics::*;


pub(super) trait SymbolCollectorCommons {
    fn symtab(&mut self) -> &mut SymbolTable;
    fn ctx(&mut self) -> &mut SymbolContext;
    fn symtab_and_ctx(&mut self) -> (&mut SymbolTable, &mut SymbolContext);
    fn diagnostics(&mut self) -> &mut Vec<Diagnostic>;
    fn rope(&self) -> &Rope;


    fn check_duplicate(&mut self, sym_name: String, sym_typ: SymbolType, span: Range) -> Option<String> {
        if let Err(err) = self.ctx().can_insert(&sym_name, sym_typ) {
            let precursor_type = match err {
                SymbolContextError::GlobalVarAlreadyExists(_, v) => v.typ,
                SymbolContextError::TypeAlreadyExists(_, v) => v.typ,
                SymbolContextError::DataAlreadyExists(_, v) => v.typ,
                SymbolContextError::CallableAlreadyExists(_, v) => v.typ,
                SymbolContextError::TypeDoesntExist(_) => panic!(),
            };
            
            self.diagnostics().push(Diagnostic { 
                span, 
                body: ErrorDiagnostic::SymbolNameTaken { 
                    name: sym_name, 
                    this_type: sym_typ, 
                    precursor_type
                }.into()
            });

            None
        } else {
            Some(sym_name)
        }
    }

    /// Returns type id and type name, if it's invalid returns ERROR_SYMBOL_ID and empty string
    fn check_type_from_identifier(&mut self, n: IdentifierNode) -> (Uuid, String) {
        if let Some(type_name) = n.value(&self.rope()) {
            if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                self.diagnostics().push(Diagnostic { 
                    span: Range::new(n.span().end, n.span().end), 
                    body: ErrorDiagnostic::MissingTypeArg.into()
                });
            } else {
                if let Some(SymbolPointer { id, .. }) = self.ctx().get(&type_name, SymbolCategory::Type) {
                    return (*id, type_name.into());
                } else {
                    self.diagnostics().push(Diagnostic { 
                        span: n.span(), 
                        body: ErrorDiagnostic::TypeNotFound.into()
                    });
                }
            }
        }

        (ERROR_SYMBOL_ID, String::new())
    }

    /// Returns type id and full type name, if it's invalid returns ERROR_SYMBOL_ID and empty string
    fn check_type_from_type_annot(&mut self, n: TypeAnnotationNode) -> (Uuid, String) {
        if let Some(type_arg_node) = n.type_arg() {
            if let Some(type_name) = n.type_name().value(&self.rope()) {
                if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                    let (type_arg_id, type_arg_name) = self.check_type_from_type_annot(type_arg_node);
                    if type_arg_id != ERROR_SYMBOL_ID {
                        let final_type_name = ArrayTypeSymbol::final_type_name(&type_arg_name);
                        // if the type has already been created before, retrieve its ID
                        // otherwise inject the new type into symtab and get the new ID
                        let type_id = self.ctx().get(&final_type_name, SymbolCategory::Type).map(|p| p.id).unwrap_or_else(|| {
                            let (symtab, ctx) = self.symtab_and_ctx();
                            inject_array_type(symtab, ctx, type_arg_id, &type_arg_name)
                        });

                        return (type_id, final_type_name);
                    }   
                } else {
                    // since only array type takes type argument, all other uses of type arg are invalid
                    self.diagnostics().push(Diagnostic { 
                        span: n.type_arg().unwrap().span(), 
                        body: ErrorDiagnostic::UnnecessaryTypeArg.into()
                    });

                    return self.check_type_from_identifier(n.type_name());
                }
            }

            (ERROR_SYMBOL_ID, String::new())
        } else {
            self.check_type_from_identifier(n.type_name())
        }   
    }
}

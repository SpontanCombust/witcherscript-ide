use ropey::Rope;
use uuid::Uuid;
use witcherscript::{DocSpan, ast::TypeAnnotationNode};
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


    fn check_duplicate(&mut self, sym_name: String, sym_typ: SymbolType, span: DocSpan) -> Option<String> {
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

    fn check_array_type(&mut self, generic_arg: Option<&str>, span: DocSpan) -> Option<Uuid> {
        if let Some(t) = generic_arg {
            if let Some(t_id) = self.check_type(t, None, span) {
                let final_typ = ArrayTypeSymbol::final_type_name(t);
                if let Some(SymbolPointer { id, .. }) = self.ctx().get(&final_typ, SymbolCategory::Type) {
                    Some(*id)
                } else {
                    let (symtab, ctx) = self.symtab_and_ctx();
                    Some(inject_array_type(symtab, ctx, t_id, t))
                }
            } else {
                None
            }

        } else {
            self.diagnostics().push(Diagnostic { 
                span, 
                body: ErrorDiagnostic::MissingGenericArg.into()
            });
    
            None
        }
    }

    fn check_type(&mut self, typ: &str, generic_arg: Option<&str>, span: DocSpan) -> Option<Uuid> {
        if typ == ArrayTypeSymbol::TYPE_NAME {
            self.check_array_type(generic_arg, span)
        } else {
            if let Some(SymbolPointer { id, .. }) = self.ctx().get(typ, SymbolCategory::Type) {
                Some(*id)
            } else {
                self.diagnostics().push(Diagnostic { 
                    span, 
                    body: ErrorDiagnostic::TypeNotFound.into()
                });
                None
            }
        }
    }


    fn get_type_from_node(&mut self, n: TypeAnnotationNode) -> Uuid {
        let mut type_id: Uuid = ERROR_SYMBOL_ID;
        if let Some(primary_type) = n.type_name().value(self.rope()) {
            let generic_arg = n.generic_arg().and_then(|g| g.value(self.rope()));
            let generic_arg_ref = generic_arg.as_ref().map(|s| s.as_str());

            if let Some(id) = self.check_type(&primary_type, generic_arg_ref, n.span()) {
                type_id = id;
            }
        }

        type_id
    }
}

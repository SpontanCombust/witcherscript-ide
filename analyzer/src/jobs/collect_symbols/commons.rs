use lsp_types::Range;
use ropey::Rope;
use witcherscript::ast::TypeAnnotationNode;
use witcherscript::tokens::IdentifierNode;
use crate::model::collections::symbol_table::SymbolTable;
use crate::model::symbol_variant::SymbolVariant;
use crate::model::symbols::*;
use crate::diagnostics::*;


pub(super) trait SymbolCollectorCommons {
    fn symtab(&mut self) -> &mut SymbolTable;
    fn diagnostics(&mut self) -> &mut Vec<Diagnostic>;
    fn rope(&self) -> &Rope;


    /// Inserts the symbol into symbol table, but only if it is not a duplicate.
    /// Returns true if symbol was inserted successfully, false otherwise.
    fn try_insert_with_duplicate_check<S>(&mut self, sym: S, span: Range) -> bool 
    where S: Symbol + Into<SymbolVariant> {
        let sym_typ = sym.typ();
        if let Err(err) = self.symtab().insert(sym) {
            self.diagnostics().push(Diagnostic { 
                span, 
                body: ErrorDiagnostic::SymbolNameTaken { 
                    name: err.occupied_path.components().last().unwrap().name.to_string(), 
                    this_type: sym_typ, 
                    precursor_type: err.occupyed_type
                }.into()
            });
            
            false
        } else {
            true
        }
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_identifier(&mut self, n: IdentifierNode) -> TypeSymbolPath {
        if let Some(type_name) = n.value(&self.rope()) {
            if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                self.diagnostics().push(Diagnostic { 
                    span: Range::new(n.span().end, n.span().end), 
                    body: ErrorDiagnostic::MissingTypeArg.into()
                });
            } else {
                let path = TypeSymbolPath::Basic(BasicTypeSymbolPath::new(&type_name));
                return path;
            }
        }

        TypeSymbolPath::empty()
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_type_annot(&mut self, n: TypeAnnotationNode) -> TypeSymbolPath {
        if let Some(type_arg_node) = n.type_arg() {
            if let Some(type_name) = n.type_name().value(&self.rope()) {
                if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                    let type_arg_path = self.check_type_from_type_annot(type_arg_node);
                    if !type_arg_path.is_empty() {
                        let path = TypeSymbolPath::Array(ArrayTypeSymbolPath::new(type_arg_path));
                        //TODO remember later about injecting array type
                        return path;
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

            TypeSymbolPath::empty()
        } else {
            self.check_type_from_identifier(n.type_name())
        }   
    }
}

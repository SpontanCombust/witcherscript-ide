use std::collections::HashSet;
use abs_path::AbsPath;
use lsp_types::Range;
use witcherscript::attribs::AutobindSpecifier;
use witcherscript::attribs::MemberVarSpecifier;
use witcherscript::script_document::ScriptDocument;
use witcherscript::tokens::*;
use witcherscript::ast::*;
use witcherscript::Script;
use crate::model::collections::symbol_table::SymbolTable;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};
use crate::model::symbols::*;
use crate::diagnostics::*;


pub fn scan_symbols(script: &Script, doc: &ScriptDocument, doc_path: &AbsPath, symtab: &mut SymbolTable) -> Vec<Diagnostic> {
    let mut visitor = SymbolScannerVisitor {
        symtab,
        doc,
        doc_path,
        diagnostics: Vec::new(),
        current_path: SymbolPathBuf::empty(),
        current_ordinal: 0
    };

    script.root_node().accept(&mut visitor);
    visitor.diagnostics
}


struct SymbolScannerVisitor<'a> {
    symtab: &'a mut SymbolTable,
    doc: &'a ScriptDocument,
    doc_path: &'a AbsPath,
    diagnostics: Vec<Diagnostic>,

    current_path: SymbolPathBuf,
    current_ordinal: usize // used for struct members and function parameters
}

impl SymbolScannerVisitor<'_> {
    // Returns whether the symbol is not a duplicate
    fn check_contains(&mut self, path: &SymbolPath, sym_typ: SymbolType, range: Range) -> bool {
        if let Err(err) = self.symtab.contains(path) {
            let (precursor_file_path, precursor_range) = err.occupied_location
                .map(|loc| (Some(loc.file_path), Some(loc.range)))
                .unwrap_or((None, None));

            self.diagnostics.push(Diagnostic { 
                range, 
                body: ErrorDiagnostic::SymbolNameTaken { 
                    name: err.occupied_path.components().last().unwrap().name.to_string(), 
                    this_type: sym_typ, 
                    precursor_type: err.occupied_type,
                    precursor_file_path,
                    precursor_range
                }.into()
            });
            
            false
        } else {
            true
        }
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_identifier(&mut self, n: IdentifierNode) -> BasicTypeSymbolPath {
        if let Some(type_name) = n.value(&self.doc) {
            if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                self.diagnostics.push(Diagnostic { 
                    range: Range::new(n.range().end, n.range().end), 
                    body: ErrorDiagnostic::MissingTypeArg.into()
                });
            } else {
                return BasicTypeSymbolPath::new(&type_name);
            }
        }

        BasicTypeSymbolPath::empty()
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_type_annot(&mut self, n: TypeAnnotationNode) -> TypeSymbolPath {
        if let Some(type_arg_node) = n.type_arg() {
            if let Some(type_name) = n.type_name().value(&self.doc) {
                if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                    let type_arg_path = self.check_type_from_type_annot(type_arg_node);
                    if !type_arg_path.is_empty() {
                        return TypeSymbolPath::Array(ArrayTypeSymbolPath::new(type_arg_path));
                    }   
                } else {
                    // since only array type takes type argument, all other uses of type arg are invalid
                    self.diagnostics.push(Diagnostic { 
                        range: n.type_arg().unwrap().range(), 
                        body: ErrorDiagnostic::UnnecessaryTypeArg.into()
                    });

                    return self.check_type_from_identifier(n.type_name()).into();
                }
            }

            TypeSymbolPath::empty()
        } else {
            self.check_type_from_identifier(n.type_name()).into()
        }   
    }
}



impl DeclarationVisitor for SymbolScannerVisitor<'_> {
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let name_node = n.name();
        if let Some(class_name) = name_node.value(&self.doc) {
            let path = BasicTypeSymbolPath::new(&class_name);
            if self.check_contains(&path, SymbolType::Class, name_node.range()) {
                let mut sym = ClassSymbol::new(path.clone(), self.doc_path.clone(), name_node.range());
                
                for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                }
    
                sym.base_path = n.base().map(|base| self.check_type_from_identifier(base));


                let this_path = SpecialVarSymbolPath::new(&path, SpecialVarSymbolKind::This);
                let this_sym = SpecialVarSymbol::new(this_path, path.clone());
                self.symtab.insert(this_sym);

                if let Some(base_path) = &sym.base_path {
                    let super_path = SpecialVarSymbolPath::new(&path, SpecialVarSymbolKind::Super);
                    let super_sym = SpecialVarSymbol::new(super_path, base_path.clone());
                    self.symtab.insert(super_sym);
                }


                path.as_ref().clone_into(&mut self.current_path);
                self.symtab.insert_primary(sym);
                
                traverse_definition = true;
            }
        }

        ClassDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
            self.current_ordinal = 0;
        }
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let state_name_node = n.name();
        let state_name = state_name_node.value(&self.doc);
        let parent_name = n.parent().value(&self.doc);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let path = StateSymbolPath::new(&state_name, BasicTypeSymbolPath::new(&parent_name));
            if self.check_contains(&path, SymbolType::State, state_name_node.range()) {
                let mut sym = StateSymbol::new(path.clone(), self.doc_path.clone(), state_name_node.range());
    
                for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                }
    
                sym.base_state_name = n.base().and_then(|base| base.value(&self.doc)).map(|ident| ident.into());


                let this_path = SpecialVarSymbolPath::new(&path, SpecialVarSymbolKind::This);
                let this_sym = SpecialVarSymbol::new(this_path, path.clone().into());
                self.symtab.insert(this_sym);

                //TODO super_path can only be known after all states of all base classes are known

                let parent_path = SpecialVarSymbolPath::new(&path, SpecialVarSymbolKind::Parent);
                let parent_sym = SpecialVarSymbol::new(parent_path, path.parent_class_path.clone());
                self.symtab.insert(parent_sym);

                let virtual_parent_path = SpecialVarSymbolPath::new(&path, SpecialVarSymbolKind::VirtualParent);
                let virtual_parent_sym = SpecialVarSymbol::new(virtual_parent_path, path.parent_class_path.clone());
                self.symtab.insert(virtual_parent_sym);
    
    
                path.as_ref().clone_into(&mut self.current_path);
                self.symtab.insert_primary(sym);

                traverse_definition = true;
            }
        }

        StateDeclarationTraversalPolicy { 
            traverse_definition 
        }       
    }

    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
            self.current_ordinal = 0;
        }
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let name_node = n.name();
        if let Some(struct_name) = name_node.value(&self.doc) {
            let path = BasicTypeSymbolPath::new(&struct_name);
            if self.check_contains(&path, SymbolType::Struct, name_node.range()) {
                let mut sym = StructSymbol::new(path, self.doc_path.clone(), name_node.range());
    
                for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                }
    
                sym.path().clone_into(&mut self.current_path);
                self.symtab.insert_primary(sym);

                traverse_definition = true;
            }
        }

        StructDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
            self.current_ordinal = 0;
        }
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let name_node = n.name();
        if let Some(enum_name) = name_node.value(&self.doc) {
            let path = BasicTypeSymbolPath::new(&enum_name);
            if self.check_contains(&path, SymbolType::Enum, name_node.range()) {
                let sym = EnumSymbol::new(path, self.doc_path.clone(), name_node.range());
    
                sym.path().clone_into(&mut self.current_path);
                self.symtab.insert_primary(sym);

                traverse_definition = true;
            }
        }

        EnumDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        let name_node = n.name();
        if let Some(enum_variant_name) = name_node.value(&self.doc) {
            let path = DataSymbolPath::new(&self.current_path, &enum_variant_name);
            if self.check_contains(&path, SymbolType::EnumVariant, name_node.range()) {
                let sym = EnumVariantSymbol::new(path, name_node.range());
    
                self.symtab.insert(sym);
            }
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        let mut traverse_params = false;

        let name_node = n.name();
        if let Some(func_name) = name_node.value(&self.doc) {
            let path = GlobalCallableSymbolPath::new(&func_name);
            if self.check_contains(&path, SymbolType::GlobalFunction, name_node.range()) {
                let mut sym = GlobalFunctionSymbol::new(path, self.doc_path.clone(), name_node.range());
    
                for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                }
    
                sym.flavour = n.flavour().map(|flavn| flavn.value());
    
                sym.return_type_path = if let Some(ret_typn) = n.return_type() {
                    self.check_type_from_type_annot(ret_typn)
                } else {
                    TypeSymbolPath::BasicOrState(BasicTypeSymbolPath::new("void"))
                };
    
                sym.path().clone_into(&mut self.current_path);
                self.symtab.insert_primary(sym);

                traverse_params = true;
            }
        }

        GlobalFunctionDeclarationTraversalPolicy { 
            traverse_params
        }
    }

    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            // n.definition().accept(self);
            self.current_path.pop();
            self.current_ordinal = 0;
        }
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> MemberFunctionDeclarationTraversalPolicy {
        let mut traverse_params = false;

        let name_node = n.name();
        if let Some(func_name) = name_node.value(&self.doc) {
            let path = MemberCallableSymbolPath::new(&self.current_path, &func_name);
            if self.check_contains(&path, SymbolType::MemberFunction, name_node.range()) {
                let mut sym = MemberFunctionSymbol::new(path, name_node.range());
    
                for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                }
    
                sym.flavour = n.flavour().map(|flavn| flavn.value());
    
                sym.return_type_path = if let Some(ret_typn) = n.return_type() {
                    self.check_type_from_type_annot(ret_typn)
                } else {
                    TypeSymbolPath::BasicOrState(BasicTypeSymbolPath::new("void"))
                };
    
                sym.path().clone_into(&mut self.current_path);
                self.symtab.insert(sym);

                traverse_params = true;
            }
        }

        MemberFunctionDeclarationTraversalPolicy {
            traverse_params
        }
    }

    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) {
        // pop only if visit managed to create the symbol
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            // n.definition().accept(self);
            self.current_path.pop();
            self.current_ordinal = 0;
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> EventDeclarationTraversalPolicy {
        let mut traverse_params = false;

        let name_node = n.name();
        if let Some(event_name) = name_node.value(&self.doc) {
            let path = MemberCallableSymbolPath::new(&self.current_path, &event_name);
            if self.check_contains(&path, SymbolType::Event, name_node.range()) {
                let sym = EventSymbol::new(path, name_node.range());
    
                sym.path().clone_into(&mut self.current_path);
                self.symtab.insert(sym);

                traverse_params = true;
            }
        }

        EventDeclarationTraversalPolicy { 
            traverse_params
        }
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            // n.definition().accept(self);
            self.current_path.pop();
            self.current_ordinal = 0;
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {
        let mut specifiers = HashSet::new();
        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if !specifiers.insert(spec) {
                self.diagnostics.push(Diagnostic { 
                    range, 
                    body: ErrorDiagnostic::RepeatedSpecifier.into()
                });
            }
        }

        let type_path = self.check_type_from_type_annot(n.param_type());


        for name_node in n.names() {
            if let Some(param_name) = name_node.value(&self.doc) {
                let path = DataSymbolPath::new(&self.current_path, &param_name);
                if self.check_contains(&path, SymbolType::Parameter, name_node.range()) {
                    let mut sym = FunctionParameterSymbol::new(path, name_node.range());
                    sym.specifiers = specifiers.clone();
                    sym.type_path = type_path.clone();
                    sym.ordinal = self.current_ordinal;

                    self.symtab.insert(sym);
                }
            }
            self.current_ordinal += 1;
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        let mut specifiers = HashSet::new();
        let mut found_access_modif_before = false;
        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if matches!(spec, MemberVarSpecifier::AccessModifier(_)) {
                if found_access_modif_before {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::MultipleAccessModifiers.into()
                    })
                }
                found_access_modif_before = true;
            }

            if !specifiers.insert(spec) {
                self.diagnostics.push(Diagnostic { 
                    range, 
                    body: ErrorDiagnostic::RepeatedSpecifier.into()
                });
            }
        }

        let type_path = self.check_type_from_type_annot(n.var_type());

        
        for name_node in n.names() {
            if let Some(var_name) = name_node.value(&self.doc) {
                let path = DataSymbolPath::new(&self.current_path, &var_name);
                if self.check_contains(&path, SymbolType::MemberVar, name_node.range()) {
                    let mut sym = MemberVarSymbol::new(path, name_node.range());
                    sym.specifiers = specifiers.clone();
                    sym.type_path = type_path.clone();
                    sym.ordinal = self.current_ordinal;

                    self.symtab.insert(sym);
                }
            }
            self.current_ordinal += 1;
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {
        let name_node = n.name();
        if let Some(autobind_name) = name_node.value(&self.doc) {
            let path = DataSymbolPath::new(&self.current_path, &autobind_name);
            if self.check_contains(&path, SymbolType::Autobind, name_node.range()) {
                let mut sym = AutobindSymbol::new(path, name_node.range());
    
                let mut found_access_modif_before = false;
                for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                    if matches!(spec, AutobindSpecifier::AccessModifier(_)) {
                        if found_access_modif_before {
                            self.diagnostics.push(Diagnostic { 
                                range, 
                                body: ErrorDiagnostic::MultipleAccessModifiers.into()
                            })
                        }
                        found_access_modif_before = true;
                    }
    
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                }
    
                sym.type_path = self.check_type_from_type_annot(n.autobind_type());
    
                self.symtab.insert(sym);
            }
        }
    }
}

// local variable symbols will be checked dynamically when analysing functions
// impl StatementVisitor for SymbolScannerVisitor<'_> {
//     fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode) {
//         let type_path = self.check_type_from_type_annot(n.var_type());
    
//         for name_node in n.names() {
//             if let Some(var_name) = name_node.value(&self.doc) {
//                 let path = DataSymbolPath::new(&self.current_path, &var_name);
//                 let mut sym = LocalVarSymbol::new(path);
//                 sym.type_path = type_path.clone();
//                 self.try_insert_with_duplicate_check(sym, name_node.range());
//             }
//         }
//     }
// }
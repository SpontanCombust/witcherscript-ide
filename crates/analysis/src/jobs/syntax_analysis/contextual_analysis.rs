use smallvec::SmallVec;
use witcherscript::{ast::*, attribs::*, tokens::Keyword, Script};
use witcherscript_diagnostics::{Diagnostic, DiagnosticKind};


/// For situations where a valid syntax tree is produced by tree-sitter, 
/// but we can still deduce some based only of the AST.
pub fn contextual_syntax_analysis(script: &Script, diagnostics: &mut Vec<Diagnostic>) {
    let mut visitor = ContextualSyntaxAnalysis {
        diagnostics
    };

    script.visit_nodes(&mut visitor);
}

struct ContextualSyntaxAnalysis<'a> {
    diagnostics: &'a mut Vec<Diagnostic>
}

impl SyntaxNodeVisitor for ContextualSyntaxAnalysis<'_> {
    fn traversal_policy_default(&self) -> bool {
        false
    }


    fn visit_root(&mut self, _: &RootNode) -> RootTraversalPolicy {
        TraversalPolicy::default_to(true)
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut specifiers = SmallVec::<[ClassSpecifier; 3]>::new();

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(class_spec) = ClassSpecifier::try_from(spec) {
                if !specifiers.contains(&class_spec) {
                    specifiers.push(class_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a class".into() }
                })
            } 
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut specifiers = SmallVec::<[StructSpecifier; 2]>::new();

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(struct_spec) = StructSpecifier::try_from(spec) {
                if !specifiers.contains(&struct_spec) {
                    specifiers.push(struct_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a struct".into() }
                })
            } 
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut specifiers = SmallVec::<[StateSpecifier; 2]>::new();

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(state_spec) = StateSpecifier::try_from(spec) {
                if !specifiers.contains(&state_spec) {
                    specifiers.push(state_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a state".into() }
                })
            } 
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        let mut specifiers = SmallVec::<[GlobalFunctionSpecifier; 2]>::new();

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(func_spec) = GlobalFunctionSpecifier::try_from(spec) {
                if !specifiers.contains(&func_spec) {
                    specifiers.push(func_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a global function".into() }
                })
            } 
        }

        if let Some((flavour, range)) = n.flavour().map(|f| (f.value(), f.range())) {
            if GlobalFunctionFlavour::try_from(flavour).is_err() {
                let flavour_name = Keyword::from(flavour).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleFunctionFlavour { flavour_name, sym_name: "a global function".into() }
                });
            }
        };

        FunctionDeclarationTraversalPolicy { 
            traverse_params: true,
            traverse_definition: false
        }   
    }

    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        let mut specifiers = SmallVec::<[MemberVarSpecifier; 6]>::new();
        let mut found_access_modif_before = false;

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(var_spec) = MemberVarSpecifier::try_from(spec) {
                if matches!(var_spec, MemberVarSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            kind: DiagnosticKind::MultipleAccessModifiers
                        });
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.contains(&var_spec) {
                    specifiers.push(var_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a class field".into() }
                })
            }
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: DeclarationTraversalContext) {
        let mut specifiers = SmallVec::<[MemberVarSpecifier; 6]>::new();
        let mut found_access_modif_before = false;

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(var_spec) = MemberVarSpecifier::try_from(spec) {
                if matches!(var_spec, MemberVarSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            kind: DiagnosticKind::MultipleAccessModifiers
                        });
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.contains(&var_spec) {
                    specifiers.push(var_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a field".into() }
                })
            }
        }
    }

    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, _: DeclarationTraversalContext) -> FunctionDeclarationTraversalPolicy {
        let mut specifiers = SmallVec::<[MemberFunctionSpecifier; 4]>::new();
        let mut found_access_modif_before = false;

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(func_spec) = MemberFunctionSpecifier::try_from(spec) {
                if matches!(func_spec, MemberFunctionSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            kind: DiagnosticKind::MultipleAccessModifiers
                        });
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.contains(&func_spec) {
                    specifiers.push(func_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a method".into() }
                })
            } 
        }

        if let Some((flavour, range)) = n.flavour().map(|f| (f.value(), f.range())) {
            if MemberFunctionFlavour::try_from(flavour).is_err() {
                let flavour_name = Keyword::from(flavour).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleFunctionFlavour { flavour_name, sym_name: "a method".into() }
                });
            }
        };

        FunctionDeclarationTraversalPolicy {
            traverse_params: true,
            traverse_definition: false
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: DeclarationTraversalContext) {
        let mut specifiers = SmallVec::<[AutobindSpecifier; 2]>::new();
        let mut found_access_modif_before = false;

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(autobind_spec) = AutobindSpecifier::try_from(spec) {
                if matches!(autobind_spec, AutobindSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            kind: DiagnosticKind::MultipleAccessModifiers
                        });
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.contains(&autobind_spec) {
                    specifiers.push(autobind_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "an autobind".into() }
                })
            } 
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: FunctionTraversalContext) {
        let mut specifiers = SmallVec::<[FunctionParameterSpecifier; 2]>::new();

        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if let Ok(param_spec) = FunctionParameterSpecifier::try_from(spec) {
                if !specifiers.contains(&param_spec) {
                    specifiers.push(param_spec);
                } else {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    });
                }
            } else {
                let spec_name = Keyword::from(spec).to_string();
                self.diagnostics.push(Diagnostic {
                    range,
                    kind: DiagnosticKind::IncompatibleSpecifier { spec_name, sym_name: "a function parameter".into() }
                })
            } 
        }
    }
}
use lsp_types::Range;
use witcherscript::{SyntaxNode, SyntaxError};
use witcherscript::tokens::*;
use witcherscript::ast::*;
use crate::diagnostics::{Diagnostic, ErrorDiagnostic, SyntaxErrorDiagnostic};


pub fn syntax_analysis(script_root: RootNode, diagnostics: &mut Vec<Diagnostic>) {
    let mut visitor = SyntaxErrorVisitor {
        diagnostics
    };

    script_root.accept(&mut visitor);
}


struct SyntaxErrorVisitor<'a> {
    diagnostics: &'a mut Vec<Diagnostic>   
}

impl SyntaxErrorVisitor<'_> {
    fn missing_element(&mut self, range: Range, expected: String) {
        self.diagnostics.push(Diagnostic { 
            range, 
            body: ErrorDiagnostic::Syntax(SyntaxErrorDiagnostic::MissingElement(expected)).into()
        })
    }

    /// Returns true if the node is present, false otherwise
    fn check_missing<T>(&mut self, n: &SyntaxNode<'_, T>, expected: &str) -> bool {
        if n.is_missing() {
            self.missing_element(n.range(), expected.to_string());
            false
        } else {
            true
        }
    }

    /// Returns true if the identifier is present, false otherwise
    fn check_identifier(&mut self, n: &IdentifierNode) -> bool {
        self.check_missing(n, "identifier")
    }

    fn check_type_annot(&mut self, n: &TypeAnnotationNode) {
        self.check_missing(&n.type_name(), "type");

        if let Some(type_argn) = n.type_arg() {
            self.check_type_annot(&type_argn);
        }

        self.check_errors(&n);
    }

    fn check_expression(&mut self, n: &ExpressionNode) {
        if self.check_missing(n, "expression") {
            if n.has_errors() {
                n.accept(self);
            }
        }
    }

    /// Returns true if the statement is present and contains no errors, false otherwise
    fn check_function_stmt(&mut self, n: &FunctionStatementNode) -> bool {
        if self.check_missing(n, "statement") {
            if n.has_errors() {
                return false;
            }

            true
        } else {
            false
        }
    }

    /// Returns whether the definition contains no errors
    fn check_function_def(&mut self, n: &FunctionDefinitionNode) -> bool {
        if self.check_missing(n, "{ or ;") {
            if let FunctionDefinition::Some(block) = n.clone().value() {
                if block.has_errors() {
                    self.check_errors(&block);
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    fn check_errors<T>(&mut self, n: &SyntaxNode<'_, T>) {
        let errors = n.errors();
        if errors.is_empty() {
            return;
        }

        for err in n.errors() {
            match err {
                SyntaxError::Missing(missing) => {
                    // named nodes are handled seperately
                    if let Ok(missing) = UnnamedNode::try_from(missing.clone()) {
                        match missing.value() {
                            Unnamed::Keyword(kw) => {
                                self.missing_element(missing.range(), format!("keyword {}", kw.as_ref()));
                            },
                            Unnamed::Punctuation(punct) => {
                                self.missing_element(missing.range(), punct.to_string());
                            },
                        }
                    }
                },
                SyntaxError::Invalid(errn) => {
                    self.diagnostics.push(Diagnostic { 
                        range: errn.range(), 
                        // for now just create a generic syntax error on the range to know that this thing works
                        body: ErrorDiagnostic::Syntax(SyntaxErrorDiagnostic::Other).into()
                    })       
                }
            }
        }
    }
}

impl StatementVisitor for SyntaxErrorVisitor<'_> {
    fn visit_root(&mut self, n: &RootNode) -> bool {
        if n.has_errors() {
            self.check_errors(n);
            true
        } else {
            false
        }
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            if let Some(base) = n.base() {
                self.check_identifier(&base);
            }
    
    
            self.check_errors(n);
    
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                return true;
            }
        }

        false
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            self.check_identifier(&n.parent());
    
            if let Some(base) = n.base() {
                self.check_identifier(&base);
            }
    
            self.check_errors(n);
    
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                return true;
            }
        }

        false
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            self.check_errors(n);
            
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                return true;
            }
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            self.check_errors(n);
            
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                return true;
            }
        }

        false
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            n.value().map(|v| match v {
                EnumVariantValue::Int(n) => self.check_missing(&n, "variant integer value"),
                EnumVariantValue::Hex(n) => self.check_missing(&n, "variant integer value"),
            });
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.var_type());
    
            self.check_errors(n);
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode) {
        if n.has_errors() {
            self.check_identifier(&n.member());
            self.check_expression(&n.value());
    
            self.check_errors(n);
        }
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode) {
        if n.has_errors() {
            self.check_identifier(&n.member());
            self.check_missing(&n.value(), "hint string");
    
            self.check_errors(n);
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {
        if n.has_errors() {
            self.check_identifier(&n.name());
            self.check_type_annot(&n.autobind_type());
    
            self.check_errors(n);
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.param_type());
    
            self.check_errors(n);
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> (bool, bool) {
        if n.has_errors() {
            self.check_identifier(&n.name());
            n.return_type().map(|n| self.check_type_annot(&n));
    
            self.check_errors(n);
        
            let params = n.params();
            let errors_in_params = params.has_errors();
            if errors_in_params {
                self.check_errors(&params);
            }

            let errors_in_def = !self.check_function_def(&n.definition());

            return (errors_in_params, errors_in_def);
        }

        (false, false)
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> (bool ,bool) {
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            if let Some(ret) = n.return_type() {
                self.check_type_annot(&ret);
            }
    
            self.check_errors(n);
    
            let params = n.params();
            let errors_in_params = params.has_errors();
            if errors_in_params {
                self.check_errors(&params);
            }

            let errors_in_def = !self.check_function_def(&n.definition());

            return (errors_in_params, errors_in_def);
        }
        
        (false, false)
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> (bool ,bool) {
        if n.has_errors() {
            self.check_identifier(&n.name());

            if let Some(ret) = n.return_type() {
                self.check_type_annot(&ret);
            }
    
            self.check_errors(n);

            let params = n.params();
            let errors_in_params = params.has_errors();
            if errors_in_params {
                self.check_errors(&params);
            }

            let errors_in_def = !self.check_function_def(&n.definition());

            return (errors_in_params, errors_in_def);
        }
        
        (false, false)
    }

    fn visit_block_stmt(&mut self, n: &FunctionBlockNode) -> bool {
        if n.has_errors() {
            self.check_errors(n);
            
            return true;
        }

        false
    }

    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.var_type());
    
            self.check_errors(n);
        }
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode) {
        if n.has_errors() {
            self.check_expression(&n.expr());
    
            self.check_errors(n);
        }
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode) {
        if n.has_errors() {
            n.value().map(|n| self.check_expression(&n));
    
            self.check_errors(n);
        }
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode) {
        if n.has_errors() {
            self.check_expression(&n.value());
    
            self.check_errors(n);
        }
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode) -> bool {
        if n.has_errors() {
            n.init().map(|n| self.check_expression(&n));
            n.cond().map(|n| self.check_expression(&n));
            n.iter().map(|n| self.check_expression(&n));

            self.check_errors(n);
    
            return !self.check_function_stmt(&n.body());
        }

        false
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode) -> bool {
        if n.has_errors() {
            self.check_expression(&n.cond());
    
            self.check_errors(n);
    
            return !self.check_function_stmt(&n.body());
        }

        false
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode) -> bool {
        if n.has_errors() {
            self.check_expression(&n.cond());
    
            self.check_errors(n);
    
            return !self.check_function_stmt(&n.body());
        }

        false
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode) -> bool {
        if n.has_errors() {
            self.check_expression(&n.cond());
    
            self.check_errors(n);
            
    
            return !self.check_function_stmt(&n.body())
                || n.else_body().map(|n| !self.check_function_stmt(&n)).unwrap_or(false)
        }

        false
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode) -> bool {
        if n.has_errors() {
            self.check_expression(&n.cond());
    
            self.check_errors(n);
    
            let body = n.body();
            if body.has_errors() {
                self.check_errors(&body);
                return true;
            }
        }

        false
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode) {
        if n.has_errors() {
            self.check_expression(&n.value());
    
            self.check_errors(n);
        }
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }
}

impl ExpressionVisitor for SyntaxErrorVisitor<'_> {
    fn visit_array_expr(&mut self, n: &ArrayExpressionNode) {
        self.check_expression(&n.accessor());
        self.check_expression(&n.index());

        self.check_errors(n);
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode) {
        self.check_expression(&n.left());
        self.check_expression(&n.right());

        self.check_errors(n);
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode) {
        self.check_expression(&n.left());
        self.check_expression(&n.right());

        self.check_errors(n);
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode) {
        self.check_expression(&n.right());

        self.check_errors(n);
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode) {
        self.check_missing(&n.func(), "function");

        if let Some(args) = n.args() {
            self.check_errors(&args);
        }

        self.check_errors(n);
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode) {
        self.check_identifier(&n.class());
        n.lifetime_obj().map(|n| self.check_expression(&n));

        self.check_errors(n);
    }

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode) {
        self.check_expression(&n.accessor());
        self.check_identifier(&n.member());

        self.check_errors(n);
    }

    fn visit_nested_expr(&mut self, n: &NestedExpressionNode) {
        self.check_expression(&n.value());

        self.check_errors(n);
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode) {
        self.check_expression(&n.cond());
        self.check_expression(&n.conseq());
        self.check_expression(&n.alt());

        self.check_errors(n);
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode) {
        self.check_identifier(&n.target_type());
        self.check_expression(&n.value());

        self.check_errors(n);
    }
    
    // No point in checking single token expressions
}
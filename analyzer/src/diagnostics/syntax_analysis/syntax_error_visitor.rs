use lsp_types::Range;
use witcherscript::{SyntaxNode, SyntaxError};
use witcherscript::tokens::*;
use witcherscript::ast::*;
use crate::diagnostics::{Diagnostic, ErrorDiagnostic, SyntaxErrorDiagnostic, InfoDiagnostic};


pub fn syntax_analisys(script: ScriptNode, diagnostics: &mut Vec<Diagnostic>) {
    let mut visitor = SyntaxErrorVisitor {
        diagnostics
    };

    script.accept(&mut visitor);
}


struct SyntaxErrorVisitor<'a> {
    diagnostics: &'a mut Vec<Diagnostic>   
}

impl SyntaxErrorVisitor<'_> {
    fn missing_element(&mut self, span: Range, expected: String) {
        self.diagnostics.push(Diagnostic { 
            span, 
            body: ErrorDiagnostic::Syntax(SyntaxErrorDiagnostic::MissingElement(expected)).into()
        })
    }

    fn check_missing<T>(&mut self, n: SyntaxNode<'_, T>, expected: &str) -> bool {
        if n.is_missing() {
            self.missing_element(n.span(), expected.to_string());
            false
        } else {
            true
        }
    }

    fn check_identifier(&mut self, n: IdentifierNode) -> bool {
        self.check_missing(n, "identifier")
    }

    fn check_type_annot(&mut self, n: TypeAnnotationNode) {
        self.check_missing(n.type_name(), "type identifier");

        if let Some(type_argn) = n.type_arg() {
            self.check_type_annot(type_argn);
        }

        self.check_errors(&n);
    }

    fn check_literal_int(&mut self, n: LiteralIntNode) -> bool {
        self.check_missing(n, "integer number")
    }

    fn check_literal_string(&mut self, n: LiteralStringNode) -> bool{
        self.check_missing(n, "string")
    }

    fn check_expression(&mut self, n: ExpressionNode) {
        if n.is_missing() {
            self.missing_element(n.span(), "expression".to_string());
        } else {
            if n.has_errors() {
                n.accept(self);
            }
        }
    }

    fn check_errors<T>(&mut self, n: &SyntaxNode<'_, T>) -> bool {
        let errors = n.errors();
        if errors.is_empty() {
            return false;
        }

        for err in n.errors() {
            match err {
                SyntaxError::Missing(missing) => {
                    // named nodes are handled seperately
                    if let Ok(missing) = UnnamedNode::try_from(missing.clone()) {
                        match missing.value() {
                            Unnamed::Keyword(kw) => {
                                self.missing_element(missing.span(), format!("keyword {}", kw.as_ref()));
                            },
                            Unnamed::Punctuation(punct) => {
                                self.missing_element(missing.span(), punct.to_string());
                            },
                        }
                    }
                },
                SyntaxError::Invalid(errn) => {
                    self.diagnostics.push(Diagnostic { 
                        span: errn.span(), 
                        // for now just create a generic syntax error on the span to know that this thing works
                        body: ErrorDiagnostic::Syntax(SyntaxErrorDiagnostic::Other).into()
                    })       
                }
            }
        }

        true
    }
}

impl StatementVisitor for SyntaxErrorVisitor<'_> {
    fn visit_script(&mut self, n: &ScriptNode) -> bool {
        if n.has_errors() {
            self.check_errors(n);
            true
        } else {
            false
        }
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(n.name());
    
            if let Some(base) = n.base() {
                self.check_identifier(base);
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
            self.check_identifier(n.name());
    
            self.check_identifier(n.parent());
    
            if let Some(base) = n.base() {
                self.check_identifier(base);
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
            self.check_identifier(n.name());
    
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
            self.check_identifier(n.name());
    
            self.check_errors(n);
            
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                return true;
            }
        }

        false
    }

    fn visit_enum_member_decl(&mut self, n: &EnumMemberDeclarationNode) {
        if n.has_errors() {
            self.check_identifier(n.name());
    
            n.value().map(|n| self.check_literal_int(n));
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(name); } );
            self.check_type_annot(n.var_type());
    
            self.check_errors(n);
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode) {
        if n.has_errors() {
            self.check_identifier(n.member());
            self.check_expression(n.value());
    
            self.check_errors(n);
        }
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode) {
        if n.has_errors() {
            self.check_identifier(n.member());
            self.check_literal_string(n.value());
    
            self.check_errors(n);
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {
        if n.has_errors() {
            self.check_identifier(n.name());
            self.check_type_annot(n.autobind_type());
    
            self.check_errors(n);
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(name); } );
            self.check_type_annot(n.param_type());
    
            self.check_errors(n);
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(n.name());
            n.return_type().map(|n| self.check_type_annot(n));
    
            self.check_errors(n);
    
            if let Some(def) = n.definition() {
                if def.has_errors() {
                    self.check_errors(&def);
                    return true;
                }
            }
        }

        false
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(n.name());
    
            if let Some(ret) = n.return_type() {
                self.check_type_annot(ret);
            }
    
            self.check_errors(n);
    
            if let Some(def) = n.definition() {
                if def.has_errors() {
                    self.check_errors(&def);
                    return true;
                }
            }
        }

        false
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> bool {
        if n.has_errors() {
            self.check_identifier(n.name());
    
            self.check_errors(n);
    
            if let Some(def) = n.definition() {
                if def.has_errors() {
                    self.check_errors(&def);
                    return true;
                }
            }
        }

        false
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
            n.names().for_each(|name| { self.check_identifier(name); } );
            self.check_type_annot(n.var_type());
    
            self.check_errors(n);
        }
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode) {
        if n.has_errors() {
            self.check_expression(n.expr());
    
            self.check_errors(n);
        }
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode) {
        if n.has_errors() {
            n.value().map(|n| self.check_expression(n));
    
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
            self.check_expression(n.value());
    
            self.check_errors(n);
        }
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode) -> bool {
        if n.has_errors() {
            n.init().map(|n| self.check_expression(n));
            n.cond().map(|n| self.check_expression(n));
            n.iter().map(|n| self.check_expression(n));
    
            if n.body().has_errors() {
                return true;
            }
        }

        false
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode) -> bool {
        if n.has_errors() {
            self.check_expression(n.cond());
    
            self.check_errors(n);
    
            if n.body().has_errors() {
                return true;
            }
        }

        false
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode) -> bool {
        if n.has_errors() {
            self.check_expression(n.cond());
    
            self.check_errors(n);
    
    
            return n.body().has_errors();
        }

        false
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode) -> bool {
        if n.has_errors() {
            self.check_expression(n.cond());
    
            self.check_errors(n);
            
    
            return n.body().has_errors() 
                || n.else_body().map(|n| n.has_errors()).unwrap_or(false)
        }

        false
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode) -> bool {
        if n.has_errors() {
            self.check_expression(n.matched_expr());
    
            self.check_errors(n);
    
    
            return n.has_errors();
        }

        false
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseNode) {
        if n.has_errors() {
            self.check_expression(n.value());
    
            self.check_errors(n);
        }
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_nop_stmt(&mut self, n: &NopNode) {
        self.diagnostics.push(Diagnostic { 
            span: n.span(), 
            body: InfoDiagnostic::TrailingSemicolon.into()
        })
    }
}

impl ExpressionVisitor for SyntaxErrorVisitor<'_> {
    fn visit_array_expr(&mut self, n: &ArrayExpressionNode) {
        self.check_expression(n.accessor());
        self.check_expression(n.index());

        self.check_errors(n);
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode) {
        self.check_expression(n.left());
        self.check_expression(n.right());

        self.check_errors(n);
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode) {
        self.check_expression(n.left());
        self.check_expression(n.right());

        self.check_errors(n);
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode) {
        self.check_expression(n.right());

        self.check_errors(n);
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode) {
        self.check_identifier(n.func());

        self.check_errors(n);
    }

    fn visit_instantiation_expr(&mut self, n: &InstantiationExpressionNode) {
        self.check_identifier(n.class());
        self.check_expression(n.lifetime_obj());

        self.check_errors(n);
    }

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode) {
        self.check_expression(n.accessor());
        self.check_identifier(n.member());

        self.check_errors(n);
    }

    fn visit_method_call_expr(&mut self, n: &MethodCallExpressionNode) {
        self.check_expression(n.accessor());
        self.check_identifier(n.func());

        self.check_errors(n);
    }

    fn visit_nested_expr(&mut self, n: &NestedExpressionNode) {
        self.check_expression(n.value());

        self.check_errors(n);
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode) {
        self.check_expression(n.cond());
        self.check_expression(n.conseq());
        self.check_expression(n.alt());

        self.check_errors(n);
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode) {
        self.check_identifier(n.target_type());
        self.check_expression(n.value());

        self.check_errors(n);
    }

    // No point in checking single token expressions
}
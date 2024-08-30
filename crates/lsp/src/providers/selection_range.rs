use std::{cell::RefCell, rc::Rc};
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::{ast::*, tokens::*, ErrorNode};
use witcherscript_analysis::utils::{PositionFilter, PositionFilterEndpoint, PositionFilterPayload};
use crate::Backend;


impl Backend {
    pub async fn selection_range_impl(&self, params: lsp::SelectionRangeParams) -> Result<Option<Vec<lsp::SelectionRange>>> {
        let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    
        if doc_path.extension().unwrap_or_default() != "ws" {
            return Ok(None);
        }
        
        if let Some(script_state) = self.scripts.get(&doc_path) {
            let mut found_ranges = Vec::with_capacity(params.positions.len());
    
            let (pos_filter, payload) = PositionFilter::new_rc(lsp::Position::default());
            let resolver = SelectionRangeResolver::new_rc(payload.clone());
    
            for pos in params.positions {
                resolver.borrow_mut().reset(pos);
                pos_filter.borrow_mut().reset(pos);
    
                let mut chain = SyntaxNodeVisitorChain::new()
                    .link_rc(pos_filter.clone())
                    .link_rc(resolver.clone());
    
                script_state.script.visit_nodes(&mut chain);
    
                let resolver_ref = resolver.borrow();
                if !resolver_ref.range_stack.is_empty() {
                    let mut sr = lsp::SelectionRange {
                        range: resolver_ref.range_stack[0],
                        parent: None
                    };
    
                    for range in resolver_ref.range_stack.iter().skip(1) {
                        sr = lsp::SelectionRange {
                            range: range.clone(),
                            parent: Some(Box::new(sr))
                        };
                    }
    
                    found_ranges.push(sr);
                } 
                else {
                    found_ranges.push(lsp::SelectionRange {
                        range: lsp::Range::default(),
                        parent: None
                    })
                }
            }
    
            Ok(Some(found_ranges))
        } else {
            Ok(None)
        }
    }
}


struct SelectionRangeResolver {
    pos: lsp::Position,
    range_stack: Vec<lsp::Range>,
    payload: Rc<RefCell<PositionFilterPayload>>
}

impl SelectionRangeResolver {
    fn new_rc(pos_filter_payload: Rc<RefCell<PositionFilterPayload>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos: lsp::Position::default(),
            range_stack: Vec::new(),
            payload: pos_filter_payload
        }))
    }

    fn reset(&mut self, pos: lsp::Position) {
        self.pos = pos;
        self.range_stack.clear();
    }
}

impl SyntaxNodeVisitor for SelectionRangeResolver {
    fn traversal_policy_default(&self) -> bool {
        true        
    }


    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::ClassName => {
                    self.range_stack.push(n.name().range());
                },
                PositionFilterEndpoint::ClassBase => {
                    self.range_stack.push(n.base().map(|b| b.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::ClassSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                _ => {}
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::StateName => {
                    self.range_stack.push(n.name().range());   
                },
                PositionFilterEndpoint::StateParent => {
                    self.range_stack.push(n.parent().range());
                },
                PositionFilterEndpoint::StateBase => {
                    self.range_stack.push(n.base().map(|b| b.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::StateSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                _ => {}
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::StructName => {
                    self.range_stack.push(n.name().range());
                },
                PositionFilterEndpoint::StructSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                _ => {}
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::EnumName => {
                    self.range_stack.push(n.name().range());
                },
                _ => {}
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) -> EnumVariantDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::EnumVariantName => {
                    self.range_stack.push(n.name().range());
                },
                PositionFilterEndpoint::EnumVariantValue => {
                    match n.value() {
                        Some(EnumVariantValue::Int(int)) => {
                            self.range_stack.push(int.range());
                        },
                        Some(EnumVariantValue::Hex(hex)) => {
                            self.range_stack.push(hex.range());
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(false)
    }

    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::FunctionSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::FunctionFlavour => {
                    self.range_stack.push(n.flavour().map(|f| f.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::FunctionName => {
                    self.range_stack.push(n.name().range());                    
                },
                _ => {}
            }
        }
        else if n.params().spans_position(self.pos) {
            self.range_stack.push(n.params().range());
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) -> MemberVarDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::VarSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::VarName { nth } => {
                    self.range_stack.push(n.names().nth(nth).map(|n| n.range()).unwrap_or_default());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(false)
    }




    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, _: &TraversalContextStack) -> FunctionDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::FunctionSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::FunctionFlavour => {
                    self.range_stack.push(n.flavour().map(|f| f.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::FunctionName => {
                    self.range_stack.push(n.name().range());                    
                },
                _ => {}
            }
        }
        else if n.params().spans_position(self.pos) {
            self.range_stack.push(n.params().range());
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: &TraversalContextStack) -> EventDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::EventName => {
                    self.range_stack.push(n.name().range());
                },
                _ => {}
            }
        }
        else if n.params().spans_position(self.pos) {
            self.range_stack.push(n.params().range());
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: &TraversalContextStack) -> FunctionParameterGroupTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::FunctionParameterName { nth } => {
                    self.range_stack.push(n.names().nth(nth).map(|n| n.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::FunctionParameterSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(false)
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: &TraversalContextStack) -> MemberVarDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::VarSpecifier { nth } => {
                    self.range_stack.push(n.specifiers().nth(nth).map(|s| s.range()).unwrap_or_default());
                },
                PositionFilterEndpoint::VarName { nth } => {
                    self.range_stack.push(n.names().nth(nth).map(|n| n.range()).unwrap_or_default());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(false)
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: &TraversalContextStack) -> AutobindDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if n.name().spans_position(self.pos) {
            self.range_stack.push(n.name().range());
        }
        else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
            self.range_stack.push(spec.range());
        }
        else {
            match n.value() {
                AutobindValue::Single(single) => {
                    if single.spans_position(self.pos) {
                        self.range_stack.push(single.range());
                    }
                },
                AutobindValue::Concrete(concrete) => {
                    if concrete.spans_position(self.pos) {
                        self.range_stack.push(concrete.range());
                    }
                }
            }
        }

        TraversalPolicy::default_to(false)
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::MemberDefaultValueMember => {
                    self.range_stack.push(n.member().range());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, _: &TraversalContextStack) -> MemberDefaultsBlockTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, _: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::MemberDefaultValueMember => {
                    self.range_stack.push(n.member().range());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: &TraversalContextStack) -> MemberHintTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::MemberHintMember => {
                    self.range_stack.push(n.member().range());
                },
                PositionFilterEndpoint::MemberHintValue => {
                    self.range_stack.push(n.value().range());
                }
                _ => {}
            }
        }

        TraversalPolicy::default_to(false)
    }




    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, _: &TraversalContextStack) -> VarDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::VarName { nth } => {
                    self.range_stack.push(n.names().nth(nth).map(|n| n.range()).unwrap_or_default());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode, _: &TraversalContextStack) -> IfConditionalTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode, _: &TraversalContextStack) -> SwitchConditionalTraversalPolicy {
        self.range_stack.push(n.range());

        if n.body().spans_position(self.pos) {
            self.range_stack.push(n.body().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode, _: &TraversalContextStack) -> SwitchConditionalCaseLabelTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode, _: &TraversalContextStack) -> SwitchConditionalDefaultLabelTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(false)
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode, _: &TraversalContextStack) -> ForLoopTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode, _: &TraversalContextStack) -> WhileLoopTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode, _: &TraversalContextStack) -> DoWhileLoopTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_compound_stmt(&mut self, n: &CompoundStatementNode, _: &TraversalContextStack) -> CompoundStatementTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode, _: &TraversalContextStack) -> ExpressionStatementTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode, _: &TraversalContextStack) -> ReturnStatementTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode, _: &TraversalContextStack) -> DeleteStatementTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode, _: &TraversalContextStack) -> BreakStatementTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(false)
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, _: &TraversalContextStack) -> ContinueStatementTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(false)
    }

    fn visit_nop_stmt(&mut self, n: &NopNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }


    

    fn visit_nested_expr(&mut self, n: &NestedExpressionNode, _: &TraversalContextStack) -> NestedExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }  

    fn visit_array_expr(&mut self, n: &ArrayExpressionNode, _: &TraversalContextStack) -> ArrayExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, _: &TraversalContextStack) -> AssignmentOperationExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, _: &TraversalContextStack) -> BinaryOperationExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, _: &TraversalContextStack) -> UnaryOperationExpressionTraversalPolicy {
        self.range_stack.push(n.range());
        
        TraversalPolicy::default_to(true)
    }

    fn visit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, _: &TraversalContextStack) -> MemberFieldExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::MemberAccessExpressionMember => {
                    self.range_stack.push(n.member().range());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: &TraversalContextStack) -> NewExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::NewExpressionClass => {
                    self.range_stack.push(n.class().range());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: &TraversalContextStack) -> TypeCastExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(endpoint) = self.payload.borrow().leaf_endpoint {
            match endpoint {
                PositionFilterEndpoint::TypeCastExpressionTargetType => {
                    self.range_stack.push(n.target_type().range());
                },
                _ => {}
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, _: &TraversalContextStack) -> TernaryConditionalExpressionTraversalPolicy {
        self.range_stack.push(n.range());
        
        TraversalPolicy::default_to(true)
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, _: &TraversalContextStack) -> FunctionCallExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        if let Some(args) = n.args() {
            if args.spans_position(self.pos) {
                self.range_stack.push(args.range());
            }
        }
        
        TraversalPolicy::default_to(true)
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }

    fn visit_literal_expr(&mut self, n: &LiteralNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }
    
    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }

    fn visit_array_initializer_expr(&mut self, n: &ArrayInitializerExpressionNode, _: &TraversalContextStack) -> ArrayInitializerExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }


    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode, _: &TraversalContextStack) -> TypeAnnotationTraversalPolicy {
        self.range_stack.push(n.range());

        if n.type_name().spans_position(self.pos) {
            self.range_stack.push(n.type_name().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_annotation(&mut self, n: &AnnotationNode, _: &TraversalContextStack) -> AnnotationTraversalPolicy {
        self.range_stack.push(n.range());

        if n.name().spans_position(self.pos) {
            self.range_stack.push(n.name().range());
        }
        else if let Some(arg) = n.arg() {
            if arg.spans_position(self.pos) {
                self.range_stack.push(arg.range());
            }
        }

        AnnotationTraversalPolicy {
            traverse_unnamed: false,
            traverse_errors: false
        }
    }


    fn visit_error(&mut self, n: &ErrorNode, _: &TraversalContextStack) -> ErrorTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(false)
    }
}

impl SyntaxNodeVisitorChainLink for SelectionRangeResolver {}
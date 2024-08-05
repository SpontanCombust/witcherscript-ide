use std::{cell::RefCell, rc::Rc};
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::{ast::*, tokens::*};
use witcherscript_analysis::utils::{PositionFilter, PositionFilterPayload};
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


    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode) {
        self.range_stack.push(n.range());

        if n.type_name().spans_position(self.pos) {
            self.range_stack.push(n.type_name().range());
        } 
        else if let Some(type_arg) = n.type_arg() {
            if type_arg.spans_position(self.pos) {
                self.visit_type_annotation(&type_arg);
            }
        }
    }

    fn visit_annotation(&mut self, n: &AnnotationNode) {
        self.range_stack.push(n.range());

        if n.name().spans_position(self.pos) {
            self.range_stack.push(n.name().range());
        }
        else if let Some(arg) = n.arg() {
            if arg.spans_position(self.pos) {
                self.range_stack.push(arg.range());
            }
        }
    }
}

impl SyntaxNodeVisitor for SelectionRangeResolver {
    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy {
        self.range_stack.push(n.range());

        TraversalPolicy::default_to(true)
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.range_stack.push(n.name().range());
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.range_stack.push(base.range());
            }
            else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
                self.range_stack.push(spec.range());
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.range_stack.push(n.name().range());
            }
            else if n.parent().spans_position(self.pos) {
                self.range_stack.push(n.parent().range());
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.range_stack.push(base.range());
            }
            else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
                self.range_stack.push(spec.range());
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.range_stack.push(n.name().range());
            }
            else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
                self.range_stack.push(spec.range());
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.range_stack.push(n.name().range());
            }
        }
        else if n.definition().spans_position(self.pos) {
            self.range_stack.push(n.definition().range());
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        self.range_stack.push(n.range());

        if n.name().spans_position(self.pos) {
            self.range_stack.push(n.name().range());
        }
        if let Some(value) = n.value() {
            match value {
                EnumVariantValue::Int(int) => {
                    if int.spans_position(self.pos) {
                        self.range_stack.push(int.range());
                    }
                },
                EnumVariantValue::Hex(hex) => {
                    if hex.spans_position(self.pos) {
                        self.range_stack.push(hex.range());
                    }
                }
            }
        }
    }

    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if let Some(annot) = n.annotation().filter(|annot| annot.spans_position(self.pos)) {
                self.visit_annotation(&annot);
            }
            else if n.name().spans_position(self.pos) {
                self.range_stack.push(n.name().range());
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
            else if let Some(flavour) = n.flavour().filter(|f| f.spans_position(self.pos)) {
                self.range_stack.push(flavour.range());
            }
            else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
                self.range_stack.push(spec.range());
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

    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        self.range_stack.push(n.range());

        if let Some(annot) = n.annotation().filter(|annot| annot.spans_position(self.pos)) {
            self.visit_annotation(&annot);
        }
        else if n.var_type().spans_position(self.pos) {
            self.visit_type_annotation(&n.var_type());
        }
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.range_stack.push(name.range());
        }
        else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
            self.range_stack.push(spec.range());
        }
    }




    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, _: &TraversalContextStack) -> FunctionDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if let Some(annot) = n.annotation().filter(|annot| annot.spans_position(self.pos)) {
                self.visit_annotation(&annot);
            }
            else if n.name().spans_position(self.pos) {
                self.range_stack.push(n.name().range());
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
            else if let Some(flavour) = n.flavour().filter(|f| f.spans_position(self.pos)) {
                self.range_stack.push(flavour.range());
            }
            else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
                self.range_stack.push(spec.range());
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

        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.range_stack.push(n.name().range());
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
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

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());

        if n.param_type().spans_position(self.pos) {
            self.visit_type_annotation(&n.param_type());
        } 
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.range_stack.push(name.range());
        }
        else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
            self.range_stack.push(spec.range());
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());

        if let Some(annot) = n.annotation().filter(|annot| annot.spans_position(self.pos)) {
            self.visit_annotation(&annot);
        }
        else if n.var_type().spans_position(self.pos) {
            self.visit_type_annotation(&n.var_type());
        }
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.range_stack.push(name.range());
        }
        else if let Some(spec) = n.specifiers().find(|spec| spec.spans_position(self.pos)) {
            self.range_stack.push(spec.range());
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());

        if n.name().spans_position(self.pos) {
            self.range_stack.push(n.name().range());
        }
        else if n.autobind_type().spans_position(self.pos) {
            self.visit_type_annotation(&n.autobind_type());
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
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.range_stack.push(n.member().range());
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

        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.range_stack.push(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());

        if n.member().spans_position(self.pos) {
            self.range_stack.push(n.member().range());
        }
        else if n.value().spans_position(self.pos) {
            self.range_stack.push(n.value().range());
        }
    }




    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, _: &TraversalContextStack) -> VarDeclarationTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.var_type().spans_position(self.pos) {
                self.visit_type_annotation(&n.var_type());
            } 
            else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
                self.range_stack.push(name.range());
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

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
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

    fn visit_break_stmt(&mut self, n: &BreakStatementNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, _: &TraversalContextStack) {
        self.range_stack.push(n.range());
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

        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.range_stack.push(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: &TraversalContextStack) -> NewExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.class().spans_position(self.pos) {
                self.range_stack.push(n.class().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: &TraversalContextStack) -> TypeCastExpressionTraversalPolicy {
        self.range_stack.push(n.range());

        if self.payload.borrow().done {
            if n.target_type().spans_position(self.pos) {
                self.range_stack.push(n.target_type().range());
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
}

impl SyntaxNodeVisitorChainLink for SelectionRangeResolver {}
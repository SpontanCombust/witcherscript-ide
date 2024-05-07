use std::{cell::RefCell, rc::Rc};
use tower_lsp::lsp_types as lsp;
use witcherscript::{ast::*, tokens::*};
use witcherscript_analysis::utils::PositionSeekerPayload;


/// A node visitor that can resolve a code identifier/symbol if a specified position points to such.
/// Expects to work after PositionSeeker in visitor chain.
pub(super) struct TextDocumentPositionResolver {
    pos: lsp::Position,
    payload: Rc<RefCell<PositionSeekerPayload>>,
    pub found_target: Option<PositionTarget>
}

pub(super) struct PositionTarget {
    pub range: lsp::Range
}

impl TextDocumentPositionResolver {
    pub fn new_rc(pos: lsp::Position, point_seeker_payload: Rc<RefCell<PositionSeekerPayload>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos,
            payload: point_seeker_payload,
            found_target: None
        }))
    }


    fn target_found(&mut self, range: lsp::Range) {
        self.found_target = Some(PositionTarget { 
            range
        });
    }

    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode) {
        if n.type_name().spans_position(self.pos) {
            self.target_found(n.type_name().range());
        } 
        else if let Some(type_arg) = n.type_arg() {
            if type_arg.spans_position(self.pos) {
                self.visit_type_annotation(&type_arg);
            }
        }
    }
}


impl SyntaxNodeVisitor for TextDocumentPositionResolver {
    fn traversal_policy_default(&self) -> bool {
        true
    }


    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.target_found(base.range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
            else if n.parent().spans_position(self.pos) {
                self.target_found(n.parent().range());
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.target_found(base.range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    
    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: PropertyTraversalContext) {
        if n.var_type().spans_position(self.pos) {
            self.visit_type_annotation(&n.var_type());
        }
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.target_found(name.range());
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: PropertyTraversalContext) {
        if n.name().spans_position(self.pos) {
            self.target_found(n.name().range());
        }
        else if n.autobind_type().spans_position(self.pos) {
            self.visit_type_annotation(&n.autobind_type());
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: PropertyTraversalContext) -> MemberDefaultValueTraversalPolicy {
        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.target_found(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) -> MemberDefaultValueTraversalPolicy {
        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.target_found(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: PropertyTraversalContext) {
        if n.member().spans_position(self.pos) {
            self.target_found(n.member().range());
        }
    }


    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) -> MemberFunctionDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.target_found(n.name().range());
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: FunctionTraversalContext) {
        if n.param_type().spans_position(self.pos) {
            self.visit_type_annotation(&n.param_type());
        } 
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.target_found(name.range());
        }
    }


    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.var_type().spans_position(self.pos) {
                self.visit_type_annotation(&n.var_type());
            } 
            else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
                self.target_found(name.range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    
    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range());
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range());
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range());
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range());
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, _: ExpressionTraversalContext) {
        self.target_found(n.range());
    }

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode, _: ExpressionTraversalContext) -> MemberFieldExpressionTraversalPolicy {
        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.target_found(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: ExpressionTraversalContext) -> NewExpressionTraversalPolicy {
        if self.payload.borrow().done {
            if n.class().spans_position(self.pos) {
                self.target_found(n.class().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: ExpressionTraversalContext) -> TypeCastExpressionTraversalPolicy {
        if self.payload.borrow().done {
            if n.target_type().spans_position(self.pos) {
                self.target_found(n.target_type().range());
            }
        }

        TraversalPolicy::default_to(true)
    }
}

impl SyntaxNodeVisitorChainLink for TextDocumentPositionResolver {}
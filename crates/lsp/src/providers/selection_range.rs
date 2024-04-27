use std::{cell::RefCell, rc::Rc};
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::{ast::*, tokens::*};
use witcherscript_analysis::utils::{PositionSeeker, PositionSeekerPayload};
use crate::Backend;


pub async fn selection_range(backend: &Backend, params: lsp::SelectionRangeParams) -> Result<Option<Vec<lsp::SelectionRange>>> {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if let Some(script_state) = backend.scripts.get(&doc_path) {
        let mut found_ranges = Vec::with_capacity(params.positions.len());

        let (pos_seeker, payload) = PositionSeeker::new_rc(lsp::Position::default());
        let resolver = SelectionRangeResolver::new_rc(payload.clone());

        for pos in params.positions {
            resolver.borrow_mut().reset(pos);
            pos_seeker.borrow_mut().reset(pos);

            let mut chain = SyntaxNodeVisitorChain::new()
                .link_rc(pos_seeker.clone())
                .link_rc(resolver.clone());

            script_state.script.visit_nodes(&mut chain);

            let range = resolver.borrow()
                .found_range.clone()
                .unwrap_or(lsp::Range::default());

            found_ranges.push(lsp::SelectionRange {
                range,
                parent: None
            });
        }

        Ok(Some(found_ranges))
    } else {
        Ok(None)
    }
}


struct SelectionRangeResolver {
    pos: lsp::Position,
    found_range: Option<lsp::Range>,
    payload: Rc<RefCell<PositionSeekerPayload>>
}

impl SelectionRangeResolver {
    fn new_rc(point_seeker_payload: Rc<RefCell<PositionSeekerPayload>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos: lsp::Position::default(),
            found_range: None,
            payload: point_seeker_payload
        }))
    }

    fn reset(&mut self, pos: lsp::Position) {
        self.pos = pos;
        self.found_range = None;
    }


    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode) -> bool {
        if n.type_name().spans_position(self.pos) {
            self.found_range = Some(n.type_name().range());
            return true;
        } 
        if let Some(type_arg) = n.type_arg() {
            return self.visit_type_annotation(&type_arg);
        }

        false
    }
}

impl SyntaxNodeVisitor for SelectionRangeResolver {
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
            else if let Some(base) = n.base() {
                if base.spans_position(self.pos) {
                    self.found_range = Some(base.range());
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
            else if n.parent().spans_position(self.pos) {
                self.found_range = Some(n.parent().range());
            }
            else if let Some(base) = n.base() {
                if base.spans_position(self.pos) {
                    self.found_range = Some(base.range());
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
            else if let Some(return_type) = n.return_type() {
                if return_type.spans_position(self.pos) {
                    self.visit_type_annotation(&return_type);
                }
            }
        }

        TraversalPolicy::default_to(true)
    }




    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) -> MemberFunctionDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
            else if let Some(return_type) = n.return_type() {
                if return_type.spans_position(self.pos) {
                    self.visit_type_annotation(&return_type);
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
            else if let Some(return_type) = n.return_type() {
                if return_type.spans_position(self.pos) {
                    self.visit_type_annotation(&return_type);
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: FunctionTraversalContext) {
        if self.payload.borrow().done {
            if n.param_type().spans_position(self.pos) {
                self.visit_type_annotation(&n.param_type());
            } 
            else {
                for name in n.names() {
                    if name.spans_position(self.pos) {
                        self.found_range = Some(name.range());
                        break;
                    }
                }
            }
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: PropertyTraversalContext) {
        if self.payload.borrow().done {
            if n.var_type().spans_position(self.pos) {
                self.visit_type_annotation(&n.var_type());
            } 
            else {
                for name in n.names() {
                    if name.spans_position(self.pos) {
                        self.found_range = Some(name.range());
                        break;
                    }
                }
            }
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: PropertyTraversalContext) {
        if self.payload.borrow().done {
            if n.autobind_type().spans_position(self.pos) {
                self.visit_type_annotation(&n.autobind_type());
            }
            else if n.name().spans_position(self.pos) {
                self.found_range = Some(n.name().range());
            }
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: PropertyTraversalContext) -> MemberDefaultValueTraversalPolicy {
        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.found_range = Some(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) -> MemberDefaultValueTraversalPolicy {
        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.found_range = Some(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: PropertyTraversalContext) {
        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.found_range = Some(n.member().range());
            }
        }
    }




    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            if n.var_type().spans_position(self.pos) {
                self.visit_type_annotation(&n.var_type());
            } 
            else {
                for name in n.names() {
                    if name.spans_position(self.pos) {
                        self.found_range = Some(name.range());
                        break;
                    }
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, _: ExpressionTraversalContext) {
        if self.payload.borrow().done {
            self.found_range = Some(n.range());
        }
    }

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode, _: ExpressionTraversalContext) -> MemberFieldExpressionTraversalPolicy {
        if self.payload.borrow().done {
            if n.member().spans_position(self.pos) {
                self.found_range = Some(n.member().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: ExpressionTraversalContext) -> NewExpressionTraversalPolicy {
        if self.payload.borrow().done {
            if n.class().spans_position(self.pos) {
                self.found_range = Some(n.class().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: ExpressionTraversalContext) -> TypeCastExpressionTraversalPolicy {
        if self.payload.borrow().done {
            if n.target_type().spans_position(self.pos) {
                self.found_range = Some(n.target_type().range());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: ExpressionTraversalContext) {
        if self.payload.borrow().done {
            self.found_range = Some(n.range());
        }
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: ExpressionTraversalContext) {
        if self.payload.borrow().done {
            self.found_range = Some(n.range());
        }
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: ExpressionTraversalContext) {
        if self.payload.borrow().done {
            self.found_range = Some(n.range());
        }
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: ExpressionTraversalContext) {
        if self.payload.borrow().done {
            self.found_range = Some(n.range());
        }
    }
}

impl SyntaxNodeVisitorChainLink for SelectionRangeResolver {}
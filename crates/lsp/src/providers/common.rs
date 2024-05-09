use std::{cell::RefCell, rc::Rc};
use tower_lsp::lsp_types as lsp;
use witcherscript::{ast::*, script_document::ScriptDocument, tokens::*};
use witcherscript_analysis::utils::PositionSeekerPayload;


/// A node visitor that can resolve a code identifier/symbol if a specified position points to such.
/// Expects to work after PositionSeeker in visitor chain.
pub(super) struct TextDocumentPositionResolver<'a> {
    pos: lsp::Position,
    doc: &'a ScriptDocument,
    payload: Rc<RefCell<PositionSeekerPayload>>,
    pub found_target: Option<PositionTarget>
}

#[derive(Debug, Clone)]
pub(super) struct PositionTarget {
    pub range: lsp::Range,
    pub kind: PositionTargetKind
}

#[derive(Debug, Clone)]
pub(super) enum PositionTargetKind {
    TypeIdentifier(String),
    StateBaseIdentifier(String),

    DataIdentifier(String),
    CallableIdentifier(String),

    ThisKeyword,
    SuperKeyword,
    ParentKeyword,
    VirtualParentKeyword
}

impl<'a> TextDocumentPositionResolver<'a> {
    pub fn new_rc(pos: lsp::Position, doc: &'a ScriptDocument, point_seeker_payload: Rc<RefCell<PositionSeekerPayload>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos,
            doc,
            payload: point_seeker_payload,
            found_target: None
        }))
    }


    fn target_found(&mut self, range: lsp::Range, kind: PositionTargetKind) {
        self.found_target = Some(PositionTarget { 
            range,
            kind
        });
    }

    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode) {
        let type_name = n.type_name();
        if type_name.spans_position(self.pos) {
            self.target_found(type_name.range(), PositionTargetKind::TypeIdentifier(type_name.value(self.doc).to_string()));
        } 
        else if let Some(type_arg) = n.type_arg() {
            if type_arg.spans_position(self.pos) {
                self.visit_type_annotation(&type_arg);
            }
        }
    }
}


impl SyntaxNodeVisitor for TextDocumentPositionResolver<'_> {
    fn traversal_policy_default(&self) -> bool {
        true
    }


    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::TypeIdentifier(name.value(self.doc).to_string()));
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.target_found(base.range(), PositionTargetKind::TypeIdentifier(base.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let name = n.name();
            let parent = n.parent();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::TypeIdentifier(name.value(self.doc).to_string()));
            }
            else if parent.spans_position(self.pos) {
                self.target_found(parent.range(), PositionTargetKind::TypeIdentifier(parent.value(self.doc).to_string()));
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.target_found(base.range(), PositionTargetKind::StateBaseIdentifier(base.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::TypeIdentifier(name.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::TypeIdentifier(name.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    
    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        if self.payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::DataIdentifier(name.value(self.doc).to_string()));
            }
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: PropertyTraversalContext) {
        let var_type = n.var_type();
        
        if var_type.spans_position(self.pos) {
            self.visit_type_annotation(&var_type);
        }
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.target_found(name.range(), PositionTargetKind::DataIdentifier(name.value(self.doc).to_string()));
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: PropertyTraversalContext) {
        let name = n.name();
        let autobind_type = n.autobind_type();

        if name.spans_position(self.pos) {
            self.target_found(name.range(), PositionTargetKind::DataIdentifier(name.value(self.doc).to_string()));
        }
        else if autobind_type.spans_position(self.pos) {
            self.visit_type_annotation(&autobind_type);
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: PropertyTraversalContext) -> MemberDefaultValueTraversalPolicy {
        if self.payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                self.target_found(member.range(), PositionTargetKind::DataIdentifier(member.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) -> MemberDefaultValueTraversalPolicy {
        if self.payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                self.target_found(member.range(), PositionTargetKind::DataIdentifier(member.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: PropertyTraversalContext) {
        let member = n.member();

        if member.spans_position(self.pos) {
            self.target_found(member.range(), PositionTargetKind::DataIdentifier(member.value(self.doc).to_string()));
        }
    }


    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::CallableIdentifier(name.value(self.doc).to_string()));
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) -> MemberFunctionDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::CallableIdentifier(name.value(self.doc).to_string()));
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.target_found(name.range(), PositionTargetKind::CallableIdentifier(name.value(self.doc).to_string()));
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: FunctionTraversalContext) {
        let param_type = n.param_type();

        if param_type.spans_position(self.pos) {
            self.visit_type_annotation(&param_type);
        } 
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.target_found(name.range(), PositionTargetKind::DataIdentifier(name.value(self.doc).to_string()));
        }
    }


    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        if self.payload.borrow().done {
            let var_type = n.var_type();

            if var_type.spans_position(self.pos) {
                self.visit_type_annotation(&var_type);
            } 
            else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
                self.target_found(name.range(), PositionTargetKind::DataIdentifier(name.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    
    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range(), PositionTargetKind::ThisKeyword);
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range(), PositionTargetKind::SuperKeyword);
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range(), PositionTargetKind::ParentKeyword);
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: ExpressionTraversalContext) {
        self.target_found(n.range(), PositionTargetKind::VirtualParentKeyword);
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, cx: ExpressionTraversalContext) {
        let kind = if cx == ExpressionTraversalContext::FunctionCallExpressionFunc {
            PositionTargetKind::CallableIdentifier(n.value(self.doc).to_string())
        } else {
            PositionTargetKind::DataIdentifier(n.value(self.doc).to_string())
        };
        self.target_found(n.range(), kind);
    }

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode, cx: ExpressionTraversalContext) -> MemberFieldExpressionTraversalPolicy {
        if self.payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                let kind = if cx == ExpressionTraversalContext::FunctionCallExpressionFunc {
                    PositionTargetKind::CallableIdentifier(member.value(self.doc).to_string())
                } else {
                    PositionTargetKind::DataIdentifier(member.value(self.doc).to_string())
                };
                self.target_found(member.range(), kind);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: ExpressionTraversalContext) -> NewExpressionTraversalPolicy {
        if self.payload.borrow().done {
            let class = n.class();

            if class.spans_position(self.pos) {
                self.target_found(class.range(), PositionTargetKind::TypeIdentifier(class.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: ExpressionTraversalContext) -> TypeCastExpressionTraversalPolicy {
        if self.payload.borrow().done {
            let target_type = n.target_type();

            if n.target_type().spans_position(self.pos) {
                self.target_found(target_type.range(), PositionTargetKind::TypeIdentifier(target_type.value(self.doc).to_string()));
            }
        }

        TraversalPolicy::default_to(true)
    }
}

impl SyntaxNodeVisitorChainLink for TextDocumentPositionResolver<'_> {}
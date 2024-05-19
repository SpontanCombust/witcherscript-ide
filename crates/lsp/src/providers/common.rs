use std::{cell::RefCell, rc::Rc};
use tower_lsp::lsp_types as lsp;
use witcherscript::{ast::*, script_document::ScriptDocument, tokens::*};
use witcherscript_analysis::symbol_analysis::{symbol_path::SymbolPathBuf, unqualified_name_lookup::UnqualifiedNameLookup};
use witcherscript_analysis::utils::{PositionFilterPayload, SymbolPathBuilderPayload};


/// A node visitor that can resolve a code identifier/symbol if a specified position points to such.
/// Expects to work after PositionSeeker in visitor chain.
pub(super) struct TextDocumentPositionResolver<'a> {
    pos: lsp::Position,
    doc: &'a ScriptDocument,
    pos_filter_payload: Rc<RefCell<PositionFilterPayload>>,
    sympath_builder_payload: Rc<RefCell<SymbolPathBuilderPayload>>,
    unl_builder_payload: Rc<RefCell<UnqualifiedNameLookup>>,
    pub found_target: Option<PositionTarget>
}

#[derive(Debug, Clone)]
pub(super) struct PositionTarget {
    pub range: lsp::Range,
    pub kind: PositionTargetKind,
    pub sympath_ctx: SymbolPathBuf,
    pub unl_ctx: UnqualifiedNameLookup
}

#[derive(Debug, Clone)]
pub(super) enum PositionTargetKind {
    TypeIdentifier(String),
    StateDeclarationNameIdentifier, // more info can be fetched using sympath_ctx 
    StateDeclarationBaseIdentifier, // more info can be fetched using sympath_ctx 

    DataDeclarationNameIdentifier(String),
    CallableDeclarationNameIdentifier, // more info can be fetched using sympath_ctx 

    // unqualified - a freely present identifier in the code
    UnqualifiedDataIdentifier(String),
    UnqualifiedCallableIdentifier(String),

    // qualified - an identifier with an additional context of an accessor from `MemberFieldExpression`
    QualifiedDataIdentifier(String),
    QualifiedCallableIdentifier(String),

    ThisKeyword,
    SuperKeyword,
    ParentKeyword,
    VirtualParentKeyword
}

impl<'a> TextDocumentPositionResolver<'a> {
    pub fn new_rc(
        pos: lsp::Position, 
        doc: &'a ScriptDocument, 
        pos_filter_payload: Rc<RefCell<PositionFilterPayload>>,
        sympath_builder_payload: Rc<RefCell<SymbolPathBuilderPayload>>,
        unl_builder_payload: Rc<RefCell<UnqualifiedNameLookup>>
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos,
            doc,
            pos_filter_payload,
            sympath_builder_payload,
            unl_builder_payload,
            found_target: None
        }))
    }


    fn found_type_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::TypeIdentifier(n.value(self.doc).to_string()),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_state_ident(&mut self, name: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: name.range(),
            kind: PositionTargetKind::StateDeclarationNameIdentifier,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_state_base_ident(&mut self, base: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: base.range(),
            kind: PositionTargetKind::StateDeclarationBaseIdentifier,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_data_decl_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::DataDeclarationNameIdentifier(n.value(self.doc).to_string()),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_callable_decl_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::CallableDeclarationNameIdentifier,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_unqualified_data_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::UnqualifiedDataIdentifier(n.value(self.doc).to_string()),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_unqualified_callable_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::UnqualifiedCallableIdentifier(n.value(self.doc).to_string()),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }
    
    fn found_qualified_data_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::QualifiedDataIdentifier(n.value(self.doc).to_string()),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_qualified_callable_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::QualifiedCallableIdentifier(n.value(self.doc).to_string()),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_this_kw(&mut self, n: &ThisExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::ThisKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_super_kw(&mut self, n: &SuperExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::SuperKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_parent_kw(&mut self, n: &ParentExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::ParentKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }

    fn found_virtual_parent_kw(&mut self, n: &VirtualParentExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::VirtualParentKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
            unl_ctx: self.unl_builder_payload.borrow().clone()
        });
    }


    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode) {
        let type_name = n.type_name();
        if type_name.spans_position(self.pos) {
            self.found_type_ident(&type_name);
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
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_type_ident(&name);
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.found_type_ident(&base);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();
            let parent = n.parent();

            if name.spans_position(self.pos) {
                self.found_state_ident(&name);
            }
            else if parent.spans_position(self.pos) {
                self.found_type_ident(&parent);
            }
            else if let Some(base) = n.base().filter(|base| base.spans_position(self.pos)) {
                self.found_state_base_ident(&base);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_type_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_type_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    
    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_data_decl_ident(&name);
            }
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: PropertyTraversalContext) {
        let var_type = n.var_type();
        
        if var_type.spans_position(self.pos) {
            self.visit_type_annotation(&var_type);
        }
        else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.found_data_decl_ident(&name);
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: PropertyTraversalContext) {
        let name = n.name();
        let autobind_type = n.autobind_type();

        if name.spans_position(self.pos) {
            self.found_data_decl_ident(&name);
        }
        else if autobind_type.spans_position(self.pos) {
            self.visit_type_annotation(&autobind_type);
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: PropertyTraversalContext) -> MemberDefaultValueTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                self.found_unqualified_data_ident(&member);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) -> MemberDefaultValueTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                self.found_unqualified_data_ident(&member);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: PropertyTraversalContext) {
        let member = n.member();

        if member.spans_position(self.pos) {
            self.found_unqualified_data_ident(&member);
        }
    }


    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_callable_decl_ident(&name);
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) -> MemberFunctionDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_callable_decl_ident(&name);
            }
            else if let Some(rt) = n.return_type().filter(|rt| rt.spans_position(self.pos)) {
                self.visit_type_annotation(&rt);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_callable_decl_ident(&name);
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
            self.found_data_decl_ident(&name);
        }
    }


    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let var_type = n.var_type();

            if var_type.spans_position(self.pos) {
                self.visit_type_annotation(&var_type);
            } 
            else if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
                self.found_data_decl_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    
    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: ExpressionTraversalContext) {
        self.found_this_kw(n);
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: ExpressionTraversalContext) {
        self.found_super_kw(n);
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: ExpressionTraversalContext) {
        self.found_parent_kw(n);
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: ExpressionTraversalContext) {
        self.found_virtual_parent_kw(n);
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, cx: ExpressionTraversalContext) {
        if cx == ExpressionTraversalContext::FunctionCallExpressionFunc {
            self.found_unqualified_callable_ident(n);
        } else {
            self.found_unqualified_data_ident(n);
        };
    }

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode, cx: ExpressionTraversalContext) -> MemberFieldExpressionTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                if cx == ExpressionTraversalContext::FunctionCallExpressionFunc {
                    self.found_qualified_callable_ident(&member);
                } else {
                    self.found_qualified_data_ident(&member);
                };
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: ExpressionTraversalContext) -> NewExpressionTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let class = n.class();

            if class.spans_position(self.pos) {
                self.found_type_ident(&class);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: ExpressionTraversalContext) -> TypeCastExpressionTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let target_type = n.target_type();

            if n.target_type().spans_position(self.pos) {
                self.found_type_ident(&target_type);
            }
        }

        TraversalPolicy::default_to(true)
    }
}

impl SyntaxNodeVisitorChainLink for TextDocumentPositionResolver<'_> {}
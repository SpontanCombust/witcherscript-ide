use std::{cell::RefCell, rc::Rc};
use tower_lsp::lsp_types as lsp;
use witcherscript::ErrorNode;
use witcherscript::{ast::*, script_document::ScriptDocument, tokens::*};
use witcherscript_analysis::symbol_analysis::symbol_table::marcher::SymbolTableMarcher;
use witcherscript_analysis::symbol_analysis::symbols::*;
use witcherscript_analysis::symbol_analysis::unqualified_name_lookup::*;
use witcherscript_analysis::symbol_analysis::symbol_path::SymbolPathBuf;
use witcherscript_analysis::utils::*;
use crate::ScriptState;


#[derive(Debug, Clone)]
pub struct PositionTarget {
    pub range: lsp::Range,
    pub kind: PositionTargetKind,
    pub sympath_ctx: SymbolPathBuf
}

#[derive(Debug, Clone)]
pub enum PositionTargetKind {
    ArrayTypeIdentifier,
    TypeIdentifier(String),
    StateDeclarationNameIdentifier, // more info can be fetched using sympath_ctx 
    StateDeclarationBaseIdentifier, // more info can be fetched using sympath_ctx 

    DataDeclarationNameIdentifier(String),
    CallableDeclarationNameIdentifier, // more info can be fetched using sympath_ctx 

    ExpressionIdentifier(SymbolPathBuf),

    ThisKeyword,
    SuperKeyword,
    ParentKeyword,
    VirtualParentKeyword
}

impl PositionTarget {
    pub fn target_symbol_path<'a>(&self, symtab_marcher: &SymbolTableMarcher<'a>) -> Option<SymbolPathBuf> {
        match &self.kind {
            PositionTargetKind::ArrayTypeIdentifier => {
                None
            },
            PositionTargetKind::TypeIdentifier(type_name) => {
                Some(BasicTypeSymbolPath::new(&type_name).into())
            },
            PositionTargetKind::StateDeclarationNameIdentifier => {
                Some(self.sympath_ctx.clone())
            },
            PositionTargetKind::StateDeclarationBaseIdentifier => {
                if let Some(target_state_sym) = symtab_marcher.get_symbol(&self.sympath_ctx).and_then(|v| v.try_as_state_ref()) {
                    let base_state_name = target_state_sym.base_state_name.as_ref().map(|s| s.as_str()).unwrap_or_default();

                    let mut base_state_path = None;
                    for state in symtab_marcher.state_hierarchy(target_state_sym.path()) {
                        if state.state_name() == base_state_name {
                            base_state_path = Some(state.path_ref().to_owned());
                            break;
                        }
                    }
                    
                    base_state_path
                } else {
                    None
                }
            },
            PositionTargetKind::DataDeclarationNameIdentifier(name) => {
                if let Some(ctx_sym) = symtab_marcher.get_symbol(&self.sympath_ctx) {
                    if ctx_sym.is_enum() {
                        Some(GlobalDataSymbolPath::new(&name).into())
                    } else {
                        Some(MemberDataSymbolPath::new(&self.sympath_ctx, &name).into())
                    }
                } else {
                    None
                }
            },
            PositionTargetKind::CallableDeclarationNameIdentifier => {
                Some(self.sympath_ctx.clone())
            },
            PositionTargetKind::ThisKeyword => {
                Some(ThisVarSymbolPath::new(self.sympath_ctx.root().unwrap_or_default()).into())
            },
            PositionTargetKind::SuperKeyword => {
                Some(SuperVarSymbolPath::new(self.sympath_ctx.root().unwrap_or_default()).into())
            },
            PositionTargetKind::ParentKeyword => {
                Some(ParentVarSymbolPath::new(self.sympath_ctx.root().unwrap_or_default()).into())
            },
            PositionTargetKind::VirtualParentKeyword => {
                Some(VirtualParentVarSymbolPath::new(self.sympath_ctx.root().unwrap_or_default()).into())
            },
            PositionTargetKind::ExpressionIdentifier(expr_type_path) => {
                Some(expr_type_path.to_owned())
            }
        }
    }
}


/// A node visitor that can resolve a code identifier/symbol if a specified position points to such.
/// Expects to work after PositionSeeker in visitor chain.
pub struct TextDocumentPositionResolver<'a> {
    pos: lsp::Position,
    doc: &'a ScriptDocument,
    pos_filter_payload: Rc<RefCell<PositionFilterPayload>>,
    symtab_marcher: SymbolTableMarcher<'a>,
    sympath_builder_payload: Rc<RefCell<SymbolPathBuilderPayload>>,
    unl_builder_payload: Rc<RefCell<UnqualifiedNameLookup>>,
    pub found_target: Option<PositionTarget>
}

impl<'a> TextDocumentPositionResolver<'a> {
    pub fn new_rc(
        pos: lsp::Position, 
        doc: &'a ScriptDocument, 
        pos_filter_payload: Rc<RefCell<PositionFilterPayload>>,
        symtab_marcher: SymbolTableMarcher<'a>,
        sympath_builder_payload: Rc<RefCell<SymbolPathBuilderPayload>>,
        unl_builder_payload: Rc<RefCell<UnqualifiedNameLookup>>
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos,
            doc,
            pos_filter_payload,
            symtab_marcher,
            sympath_builder_payload,
            unl_builder_payload,
            found_target: None
        }))
    }


    fn found_type_ident(&mut self, n: &IdentifierNode) {
        let name = n.value(self.doc);
        let kind = if name == ArrayTypeSymbol::TYPE_NAME {
            PositionTargetKind::ArrayTypeIdentifier
        } else {
            PositionTargetKind::TypeIdentifier(name.to_string())
        };

        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_state_ident(&mut self, name: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: name.range(),
            kind: PositionTargetKind::StateDeclarationNameIdentifier,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_state_base_ident(&mut self, base: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: base.range(),
            kind: PositionTargetKind::StateDeclarationBaseIdentifier,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_data_decl_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::DataDeclarationNameIdentifier(n.value(self.doc).to_string()),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_callable_decl_ident(&mut self, n: &IdentifierNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::CallableDeclarationNameIdentifier,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_expression_ident(&mut self, n: &IdentifierNode, expr: ExpressionNode, ctx: TraversalContext) {
        let expr_typ = evaluate_expression(
            expr, ctx,
            self.doc, 
            self.symtab_marcher.clone(), 
            self.sympath_builder_payload.clone(), 
            self.unl_builder_payload.clone()
        );

        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::ExpressionIdentifier(expr_typ),
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_this_kw(&mut self, n: &ThisExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::ThisKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_super_kw(&mut self, n: &SuperExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::SuperKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_parent_kw(&mut self, n: &ParentExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::ParentKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
    }

    fn found_virtual_parent_kw(&mut self, n: &VirtualParentExpressionNode) {
        self.found_target = Some(PositionTarget { 
            range: n.range(),
            kind: PositionTargetKind::VirtualParentKeyword,
            sympath_ctx: self.sympath_builder_payload.borrow().current_sympath.clone(),
        });
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

    
    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) -> EnumVariantDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_data_decl_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) -> MemberVarDeclarationTraversalPolicy {
        if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            let class_path = n.annotation()
                .and_then(|annot| annot.arg())
                .map(|arg| arg.value(self.doc))
                .map(|class_name| SymbolPathBuf::new(&class_name, SymbolCategory::Type))
                .unwrap_or_default();

            self.found_target = Some(PositionTarget { 
                range: n.range(),
                kind: PositionTargetKind::DataDeclarationNameIdentifier(name.value(self.doc).to_string()),
                sympath_ctx: class_path,
            });
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: &TraversalContextStack) -> MemberVarDeclarationTraversalPolicy {
        // not checking the annotation, because it'll be erroneous anyways
        if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.found_data_decl_ident(&name);
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: &TraversalContextStack) -> AutobindDeclarationTraversalPolicy {
        let name = n.name();

        if name.spans_position(self.pos) {
            self.found_data_decl_ident(&name);
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, ctx: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                self.found_expression_ident(&member, member.clone().into(), ctx.top());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, ctx: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                self.found_expression_ident(&member, member.clone().into(), ctx.top());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, ctx: &TraversalContextStack) -> MemberHintTraversalPolicy {
        let member = n.member();

        if member.spans_position(self.pos) {
            self.found_expression_ident(&member, member.clone().into(), ctx.top());
        }

        TraversalPolicy::default_to(true)
    }


    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_callable_decl_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, _: &TraversalContextStack) -> FunctionDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            // not checking the annotation, because it'll be erroneous anyways
            if name.spans_position(self.pos) {
                self.found_callable_decl_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: &TraversalContextStack) -> EventDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let name = n.name();

            if name.spans_position(self.pos) {
                self.found_callable_decl_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: &TraversalContextStack) -> FunctionParameterGroupTraversalPolicy {
        if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
            self.found_data_decl_ident(&name);
        }

        TraversalPolicy::default_to(true)
    }


    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, _: &TraversalContextStack) -> VarDeclarationTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            if let Some(name) = n.names().find(|name| name.spans_position(self.pos)) {
                self.found_data_decl_ident(&name);
            }
        }

        TraversalPolicy::default_to(true)
    }

    
    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: &TraversalContextStack) {
        self.found_this_kw(n);
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: &TraversalContextStack) {
        self.found_super_kw(n);
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: &TraversalContextStack) {
        self.found_parent_kw(n);
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: &TraversalContextStack) {
        self.found_virtual_parent_kw(n);
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, ctx: &TraversalContextStack) {
        self.found_expression_ident(n, n.clone().into(), ctx.top());
    }

    fn visit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, ctx: &TraversalContextStack) -> MemberFieldExpressionTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let member = n.member();

            if member.spans_position(self.pos) {
                self.found_expression_ident(&member, n.clone().into(), ctx.top());
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: &TraversalContextStack) -> NewExpressionTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let class = n.class();

            if class.spans_position(self.pos) {
                self.found_type_ident(&class);
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: &TraversalContextStack) -> TypeCastExpressionTraversalPolicy {
        if self.pos_filter_payload.borrow().done {
            let target_type = n.target_type();

            if n.target_type().spans_position(self.pos) {
                self.found_type_ident(&target_type);
            }
        }

        TraversalPolicy::default_to(true)
    }


    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode, _: &TraversalContextStack) -> TypeAnnotationTraversalPolicy {
        let type_name = n.type_name();
        if type_name.spans_position(self.pos) {
            self.found_type_ident(&type_name);
        }

        TraversalPolicy::default_to(true)
    }

    fn visit_annotation(&mut self, n: &AnnotationNode, _: &TraversalContextStack) -> AnnotationTraversalPolicy {
        if let Some(arg) = n.arg().filter(|arg| arg.spans_position(self.pos)) {
            self.found_type_ident(&arg);
        }

        TraversalPolicy::default_to(true)
    }


    fn visit_error(&mut self, _n: &ErrorNode, _ctx: &TraversalContextStack) -> ErrorTraversalPolicy {
        TraversalPolicy::default_to(false)
    }
}

impl SyntaxNodeVisitorChainLink for TextDocumentPositionResolver<'_> {}



pub fn resolve_text_document_position<'a>(position: lsp::Position, script_state: &'a ScriptState, symtab_marcher: SymbolTableMarcher<'a>) -> Option<PositionTarget> {
    let (mut main_pos_filter, _) = PositionFilter::new(position);
    main_pos_filter.filter_statements = false;

    let (mut detail_pos_filter, detail_pos_filter_payload) = PositionFilter::new(position);
    detail_pos_filter.filter_statements = true;

    let (sympath_builder, sympath_builder_payload) = SymbolPathBuilder::new(&script_state.buffer);
    let (unl_builder, unl_payload) = UnqualifiedNameLookupBuilder::new(&script_state.buffer, sympath_builder_payload.clone(), symtab_marcher.clone());
    let resolver = TextDocumentPositionResolver::new_rc(
        position, 
        &script_state.buffer, 
        detail_pos_filter_payload.clone(),
        symtab_marcher,
        sympath_builder_payload.clone(),
        unl_payload.clone(),
    );

    let mut chain = SyntaxNodeVisitorChain::new()
        .link(main_pos_filter)
        .link(sympath_builder)
        .link(unl_builder)
        .link(detail_pos_filter)
        .link_rc(resolver.clone());

    script_state.script.visit_nodes(&mut chain);

    let resolver_ref = resolver.borrow();
    resolver_ref.found_target.clone()
}
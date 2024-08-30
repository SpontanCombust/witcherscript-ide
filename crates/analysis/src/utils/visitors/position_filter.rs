use std::{cell::RefCell, rc::Rc};
use lsp_types as lsp;
use witcherscript::{ast::*, tokens::*, AnyNode};


/// Utility node visitor travels only through nodes that span a specified position
/// and it keeps traversing until it stumbles onto a node with no traversable children.
/// Then it sets the `done` flag in the payload it can share with other objects.
/// It is not guaranteed that the `done` flag will be eventually set.
/// This visitor can be used in a visitor chain.
#[derive(Debug, Clone)]
pub struct PositionFilter {
    pos: lsp::Position,
    currently_in_range: bool,
    payload: Rc<RefCell<PositionFilterPayload>>,

    /// Set whether statements should be checked against the position.
    /// 
    /// If false will set and exception for statements inside callables 
    /// and always allow the next link in the chain to see the node even if it doesn't span the position.
    /// 
    /// True by default.
    pub filter_statements: bool,
    currently_in_callable_range: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PositionFilterPayload {
    /// Signals that the given node directly contains a child leaf node, 
    /// which spans the specified position.
    pub leaf_endpoint: Option<PositionFilterEndpoint>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionFilterEndpoint {
    Current,


    ClassSpecifier { nth: usize },
    ClassName,
    ClassBase,

    StateSpecifier { nth: usize },
    StateName,
    StateParent,
    StateBase,

    StructSpecifier { nth: usize },
    StructName,

    EnumName,
    EnumVariantName,
    EnumVariantValue,


    FunctionSpecifier { nth: usize },
    FunctionFlavour,
    FunctionName,
    EventName,

    FunctionParameterSpecifier { nth: usize },
    FunctionParameterName { nth: usize },

    MemberDefaultValueMember,
    MemberHintMember,
    MemberHintValue,

    VarSpecifier { nth: usize },
    VarName { nth: usize },

    AutobindSpecifier { nth: usize },
    AutobindName,
    AutobindValue,


    MemberAccessExpressionMember,
    NewExpressionClass,
    TypeCastExpressionTargetType,
    UnaryOperationExpressionOp,
    BinaryOperationExpressionOp,
    AssignmentOperationExpressionOp,


    TypeAnnotationTypeName,


    AnnotationName,
    AnnotationArg
}


impl PositionFilter {
    pub fn new(position: lsp::Position) -> (Self, Rc<RefCell<PositionFilterPayload>>) {
        let payload = Rc::new(RefCell::new(PositionFilterPayload {
            leaf_endpoint: None
        }));

        let self_ = Self {
            pos: position,
            currently_in_range: false,
            payload: payload.clone(),

            filter_statements: true,
            currently_in_callable_range: false
        };

        (self_, payload)
    }

    pub fn new_rc(position: lsp::Position) -> (Rc<RefCell<Self>>, Rc<RefCell<PositionFilterPayload>>) {
        let (self_, payload) = Self::new(position);
        (Rc::new(RefCell::new(self_)), payload)
    }

    pub fn reset(&mut self, position: lsp::Position) {
        self.pos = position;
        self.currently_in_range = false;
        self.payload.borrow_mut().leaf_endpoint.take();
    }
}

impl SyntaxNodeVisitor for PositionFilter {
    fn traversal_policy_default(&self) -> bool {
        false
    }
    

    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy {
        self.currently_in_range = n.spans_position(self.pos);

        RootTraversalPolicy::default_to(self.currently_in_range)
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut tp = ClassDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::ClassSpecifier { nth: speci });
            }
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::ClassName);
            }
            else if n.base().map(|b| b.spans_position(self.pos)).unwrap_or(false) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::ClassBase);
            } 
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut tp = StateDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            } 
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::StateSpecifier { nth: speci });
            }
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::StateName);
            }
            else if n.parent().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::StateParent);
            }
            else if n.base().map(|b| b.spans_position(self.pos)).unwrap_or(false) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::StateBase);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut tp = StructDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            } 
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::StructSpecifier { nth: speci });
            }
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::StructName);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        let mut tp = EnumDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::EnumName);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) -> EnumVariantDeclarationTraversalPolicy {
        let mut tp = EnumVariantDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::EnumVariantName);
            }
            else if n.value().map(|v| match v {
                EnumVariantValue::Int(n) => n.spans_position(self.pos),
                EnumVariantValue::Hex(n) => n.spans_position(self.pos),
            }).unwrap_or(false) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::EnumVariantValue);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        let mut tp = FunctionDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.currently_in_callable_range = true;

            if n.annotation().map(|annot| annot.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_annotation = true;
            }
            else if n.params().spans_position(self.pos) {
                tp.traverse_params = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else if n.return_type().map(|rt| rt.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_return_type = true;
            }
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionSpecifier { nth: speci });
            }
            else if n.flavour().map(|f| f.spans_position(self.pos)).unwrap_or(false) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionFlavour);
            }
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionName);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn exit_global_func_decl(&mut self, _: &FunctionDeclarationNode) {
        self.currently_in_callable_range = false;
    }

    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) -> MemberVarDeclarationTraversalPolicy {
        let mut tp = MemberVarDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.annotation().map(|annot| annot.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_annotation = true;
            }
            else if n.var_type().spans_position(self.pos) {
                tp.traverse_type = true;
            } 
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::VarSpecifier { nth: speci });
            }
            else if let Some(namei) = n.names().enumerate().find(|(_, n)| n.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::VarName { nth: namei });
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }




    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, _: &TraversalContextStack) -> FunctionDeclarationTraversalPolicy {
        let mut tp = FunctionDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.currently_in_callable_range = true;

            if n.annotation().map(|annot| annot.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_annotation = true;
            }
            else if n.params().spans_position(self.pos) {
                tp.traverse_params = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else if n.return_type().map(|rt| rt.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_return_type = true;
            }
            else if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionSpecifier { nth: speci });
            }
            else if n.flavour().map(|f| f.spans_position(self.pos)).unwrap_or(false) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionFlavour);
            }
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionName);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn exit_member_func_decl(&mut self, _: &FunctionDeclarationNode, _: &TraversalContextStack) {
        self.currently_in_callable_range = false;
    }
    
    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: &TraversalContextStack) -> EventDeclarationTraversalPolicy {
        let mut tp = EventDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.currently_in_callable_range = true;
            if n.params().spans_position(self.pos) {
                tp.traverse_params = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else if n.return_type().map(|rt| rt.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_return_type = true;
            }
            else if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            } 
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::EventName);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode, _: &TraversalContextStack) {
        self.currently_in_callable_range = false;
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: &TraversalContextStack) -> FunctionParameterGroupTraversalPolicy {
        let mut tp = FunctionParameterGroupTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.param_type().spans_position(self.pos) {
                tp.traverse_type = true;
            } 
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionParameterSpecifier { nth: speci });
            }
            else if let Some(namei) = n.names().enumerate().find(|(_, n)| n.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::FunctionParameterName { nth: namei });
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: &TraversalContextStack) -> MemberVarDeclarationTraversalPolicy {
        let mut tp = MemberVarDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            // not checking annotation node as it's erroniuous in this context

            if n.var_type().spans_position(self.pos) {
                tp.traverse_type = true;
            }
            else if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::VarSpecifier { nth: speci });
            }
            else if let Some(namei) = n.names().enumerate().find(|(_, n)| n.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::VarName { nth: namei });
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: &TraversalContextStack) -> AutobindDeclarationTraversalPolicy {
        let mut tp = AutobindDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if let Some(speci) = n.specifiers().enumerate().find(|(_, s)| s.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::AutobindSpecifier { nth: speci });
            }
            else if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::AutobindName);
            }
            else if n.autobind_type().spans_position(self.pos) {
                tp.traverse_type = true;
            }
            else if match n.value() {
                AutobindValue::Single(n) => n.spans_position(self.pos),
                AutobindValue::Concrete(n) => n.spans_position(self.pos),
            } {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::AutobindValue);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: &TraversalContextStack) -> MemberHintTraversalPolicy {
        let mut tp = MemberHintTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.member().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberHintMember);
            }
            else if n.value().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberHintValue);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        let mut tp = MemberDefaultValueTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else if n.member().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberDefaultValueMember);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, _: &TraversalContextStack) -> MemberDefaultsBlockTraversalPolicy {
        let mut tp = MemberDefaultsBlockTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse_assignments = true;
            tp.traverse_unnamed = true;
            tp.traverse_errors = true;
        }

        tp
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, _: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        let mut tp = MemberDefaultValueTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else if n.member().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberDefaultValueMember);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }




    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, _: &TraversalContextStack) -> VarDeclarationTraversalPolicy {
        let mut tp = VarDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.init_value().map(|init_value| init_value.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_init_value = true;
            }
            else if n.var_type().spans_position(self.pos) {
                tp.traverse_type = true;
            }
            else if let Some(namei) = n.names().enumerate().find(|(_, n)| n.spans_position(self.pos)).map(|(i, _)| i) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::VarName { nth: namei });
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_compound_stmt(&mut self, n: &CompoundStatementNode, _: &TraversalContextStack) -> CompoundStatementTraversalPolicy {
        let mut tp = CompoundStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse_statements = true;
            tp.traverse_unnamed = true;
            tp.traverse_errors = true;
        }
      
        tp
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode, _: &TraversalContextStack) -> ForLoopTraversalPolicy {
        let mut tp = ForLoopTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.init().map(|init| init.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_init = true;
            }
            else if n.cond().map(|cond| cond.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_cond = true;
            }
            else if n.iter().map(|iter| iter.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_iter = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode, _: &TraversalContextStack) -> WhileLoopTraversalPolicy {
        let mut tp = WhileLoopTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode, _: &TraversalContextStack) -> DoWhileLoopTraversalPolicy {
        let mut tp = DoWhileLoopTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode, _: &TraversalContextStack) -> IfConditionalTraversalPolicy {
        let mut tp = IfConditionalTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else if n.else_body().map(|else_body| else_body.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_else_body = true;
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode, _: &TraversalContextStack) -> SwitchConditionalTraversalPolicy {
        let mut tp = SwitchConditionalTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode, _: &TraversalContextStack) -> SwitchConditionalCaseLabelTraversalPolicy {
        let mut tp = SwitchConditionalCaseLabelTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode, _: &TraversalContextStack) -> SwitchConditionalDefaultLabelTraversalPolicy {
        let mut tp = SwitchConditionalDefaultLabelTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse_unnamed = true;
            tp.traverse_errors = true;
        }

        tp
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode, _: &TraversalContextStack) -> BreakStatementTraversalPolicy {
        let mut tp = BreakStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse_unnamed = true;
            tp.traverse_errors = true;
        }

        tp
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, _: &TraversalContextStack) -> ContinueStatementTraversalPolicy {
        let mut tp = ContinueStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse_unnamed = true;
            tp.traverse_errors = true;
        }

        tp
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode, _: &TraversalContextStack) -> DeleteStatementTraversalPolicy {
        let mut tp = DeleteStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode, _: &TraversalContextStack) -> ReturnStatementTraversalPolicy {
        let mut tp = ReturnStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().map(|value| value.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_value = true;
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode, _: &TraversalContextStack) -> ExpressionStatementTraversalPolicy {
        let mut tp = ExpressionStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.expr().spans_position(self.pos) {
                tp.traverse_expr = true;
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_nop_stmt(&mut self, n: &NopNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::Current);
        }
    }




    fn visit_nested_expr(&mut self, n: &NestedExpressionNode, _: &TraversalContextStack) -> NestedExpressionTraversalPolicy {
        let mut tp = NestedExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.inner().spans_position(self.pos) {
                tp.traverse_inner = true;
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, _: &TraversalContextStack) -> AssignmentOperationExpressionTraversalPolicy {
        let mut tp = AssignmentOperationExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.left().spans_position(self.pos) {
                tp.traverse_left = true;
            }
            else if n.right().spans_position(self.pos) {
                tp.traverse_right = true;
            }
            else if n.op().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::AssignmentOperationExpressionOp);
            }
            else { 
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, _: &TraversalContextStack) -> BinaryOperationExpressionTraversalPolicy {
        let mut tp = BinaryOperationExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.left().spans_position(self.pos) {
                tp.traverse_left = true;
            }
            else if n.right().spans_position(self.pos) {
                tp.traverse_right = true;
            }
            else if n.op().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::BinaryOperationExpressionOp);
            }
            else { 
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, _: &TraversalContextStack) -> UnaryOperationExpressionTraversalPolicy {
        let mut tp = UnaryOperationExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.right().spans_position(self.pos) {
                tp.traverse_right = true;
            }
            else if n.op().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::UnaryOperationExpressionOp);
            }
            else { 
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: &TraversalContextStack) -> NewExpressionTraversalPolicy {
        let mut tp = NewExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.lifetime_obj().map(|lifetime_obj| lifetime_obj.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_lifetime_obj = true;
            }
            else if n.class().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::NewExpressionClass);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: &TraversalContextStack) -> TypeCastExpressionTraversalPolicy {
        let mut tp = TypeCastExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else if n.target_type().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::TypeCastExpressionTargetType);
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, _: &TraversalContextStack) -> TernaryConditionalExpressionTraversalPolicy {
        let mut tp = TernaryConditionalExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.conseq().spans_position(self.pos) {
                tp.traverse_conseq = true;
            }
            else if n.alt().spans_position(self.pos) {
                tp.traverse_alt = true;
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, _: &TraversalContextStack) -> MemberFieldExpressionTraversalPolicy {
        let mut tp = MemberFieldExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.accessor().spans_position(self.pos) {
                tp.traverse_accessor = true;
            }
            else if n.member().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberAccessExpressionMember);
            }
            else {
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_array_expr(&mut self, n: &ArrayExpressionNode, _: &TraversalContextStack) -> ArrayExpressionTraversalPolicy {
        let mut tp = ArrayExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.accessor().spans_position(self.pos) {
                tp.traverse_accessor = true;
            }
            else if n.index().spans_position(self.pos) {
                tp.traverse_index = true;
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, _: &TraversalContextStack) -> FunctionCallExpressionTraversalPolicy {
        let mut tp = FunctionCallExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.func().spans_position(self.pos) {
                tp.traverse_func = true;
            }
            else if n.args().map(|args| args.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_args = true;
            }
            else { 
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }
      
        tp
    }

    fn visit_func_call_arg(&mut self, n: &FunctionCallArgument, _: &TraversalContextStack) -> FunctionCallArgumentTraversalPolicy {
        let mut tp = FunctionCallArgumentTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            match n {
                FunctionCallArgument::Some(_) => {
                    tp.traverse_expr = true;
                }
                FunctionCallArgument::Omitted(_) => {
                    self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::Current);
                }
            }
        }
        
        tp
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberAccessExpressionMember);
        }
    }

    fn visit_literal_expr(&mut self, n: &LiteralNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberAccessExpressionMember);
        }
    }

    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberAccessExpressionMember);
        }
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberAccessExpressionMember);
        }
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberAccessExpressionMember);
        }
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::MemberAccessExpressionMember);
        }
    }

    fn visit_array_initializer_expr(&mut self, n: &ArrayInitializerExpressionNode, _: &TraversalContextStack) -> ArrayInitializerExpressionTraversalPolicy {
        let mut tp = ArrayInitializerExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse_items = true;
            tp.traverse_unnamed = true;
            tp.traverse_errors = true;
        }
      
        tp
    }


    
    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode, _: &TraversalContextStack) -> TypeAnnotationTraversalPolicy {
        let mut tp = TypeAnnotationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.type_name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::TypeAnnotationTypeName);
            }
            else if n.type_arg().map(|type_arg| type_arg.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_type_arg = true;
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }

    fn visit_annotation(&mut self, n: &AnnotationNode, _: &TraversalContextStack) -> AnnotationTraversalPolicy {
        let mut tp = AnnotationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.name().spans_position(self.pos) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::AnnotationName);
            }
            else if n.arg().map(|arg| arg.spans_position(self.pos)).unwrap_or(false) {
                self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::AnnotationArg);
            }
            else {
                tp.traverse_unnamed = true;
                tp.traverse_errors = true;
            }
        }

        tp
    }


    fn visit_unnamed(&mut self, n: &UnnamedNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().leaf_endpoint = Some(PositionFilterEndpoint::Current);
        }
    }


    fn visit_error(&mut self, n: &witcherscript::ErrorNode, _: &TraversalContextStack) -> ErrorTraversalPolicy {
        let mut tp = ErrorTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse = true;
        }

        tp
    }

    fn visit_error_child(&mut self, n: &AnyNode, _: &TraversalContextStack) {
        self.currently_in_range = n.spans_position(self.pos);
    }
}


impl SyntaxNodeVisitorChainLink for PositionFilter {
    fn pass_onto_next_link(&self) -> bool { 
        self.currently_in_range || (self.currently_in_callable_range && !self.filter_statements)
    }
}
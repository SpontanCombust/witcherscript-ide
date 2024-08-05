use std::{cell::RefCell, rc::Rc};
use witcherscript::{ast::*, tokens::*, script_document::ScriptDocument};
use crate::symbol_analysis::{symbol_path::{SymbolPath, SymbolPathBuf}, symbol_table::marcher::SymbolTableMarcher, symbols::*, unqualified_name_lookup::UnqualifiedNameLookup};
use super::SymbolPathBuilderPayload;


/// Compute a symbol that the given node represents.
/// Requires valid for the given context `SymbolPathBuilderPayload` and `UnqualifiedNameLookup` to work.
pub fn evaluate_expression<'a>(
    n: ExpressionNode,
    ctx: TraversalContext,
    doc: &'a ScriptDocument, 
    symtab_marcher: SymbolTableMarcher<'a>, 
    sympath_payload: Rc<RefCell<SymbolPathBuilderPayload>>,
    unl_payload: Rc<RefCell<UnqualifiedNameLookup>>
) -> SymbolPathBuf {
    let mut evaluator = ExpressionEvaluator::new(doc, symtab_marcher, sympath_payload, unl_payload);

    // we don't need information about the exact whereabouts of the expression
    let mut ctx_stack = TraversalContextStack::new();
    ctx_stack.push(ctx);

    n.accept(&mut evaluator, &mut ctx_stack); 
    evaluator.finish()
}


/// A node visitor that can compute the symbol that the given node represents.
/// Requires valid for the given context `SymbolPathBuilderPayload` and `UnqualifiedNameLookup` to work.
#[derive(Clone)]
pub struct ExpressionEvaluator<'a> {
    doc: &'a ScriptDocument,
    symtab_marcher: SymbolTableMarcher<'a>,
    sympath_payload: Rc<RefCell<SymbolPathBuilderPayload>>,
    unl_payload: Rc<RefCell<UnqualifiedNameLookup>>,

    type_stack: Vec<TypeStackElement>
}

#[derive(Clone)]
struct TypeStackElement {
    path: SymbolPathBuf,
    ctx: TraversalContext
}

impl<'a> ExpressionEvaluator<'a> {
    pub fn new(
        doc: &'a ScriptDocument,
        symtab_marcher: SymbolTableMarcher<'a>,
        sympath_payload: Rc<RefCell<SymbolPathBuilderPayload>>,
        unl_payload: Rc<RefCell<UnqualifiedNameLookup>>,
    ) -> Self {
        Self {
            doc,
            symtab_marcher,
            sympath_payload,
            unl_payload,

            type_stack: Vec::new()
        }
    }

    pub fn finish(mut self) -> SymbolPathBuf {
        self.type_stack
            .pop()
            .map(|e| e.path)
            .unwrap_or(SymbolPathBuf::unknown(SymbolCategory::Type))
    }


    #[inline]
    fn push(&mut self, path: SymbolPathBuf, ctx: TraversalContext) {
        self.type_stack.push(TypeStackElement { path, ctx })
    }

    #[inline]
    fn top(&self) -> Option<&TypeStackElement> {
        self.type_stack.last()
    }

    #[inline]
    fn top_mut(&mut self) -> Option<&mut TypeStackElement> {
        self.type_stack.last_mut()
    }

    #[inline]
    fn pop(&mut self) -> Option<TypeStackElement> {
        self.type_stack.pop()
    }

    fn produce_type(&self, path: &SymbolPath) -> SymbolPathBuf {
        if let Some(symvar) = self.symtab_marcher.get_symbol(path) {
            match symvar {
                SymbolVariant::Class(s) => s.path().to_owned(),
                SymbolVariant::State(s) => s.path().to_owned(),
                SymbolVariant::Struct(s) => s.path().to_owned(),
                SymbolVariant::Enum(s) => s.path().to_owned(),
                SymbolVariant::Array(s) => s.path().to_owned(),
                SymbolVariant::ArrayFunc(s) => s.return_type_path.clone().into(),
                SymbolVariant::ArrayFuncParam(s) => s.type_path.clone().into(),
                SymbolVariant::GlobalFunc(s) => s.return_type_path.clone().into(),
                SymbolVariant::MemberFunc(s) => s.return_type_path.clone().into(),
                SymbolVariant::Event(_) => BasicTypeSymbolPath::new("void").into(), // I guess??
                SymbolVariant::Constructor(s) => s.parent_type_path.clone().into(),
                SymbolVariant::Primitive(s) => s.path().to_owned(),
                SymbolVariant::EnumVariant(s) => s.parent_enum_path.clone().into(),
                SymbolVariant::FuncParam(s) => s.type_path.clone().into(),
                SymbolVariant::GlobalVar(s) => s.type_path().to_owned().into(),
                SymbolVariant::MemberVar(s) => s.type_path.clone().into(),
                SymbolVariant::Autobind(s) => s.type_path.clone().into(),
                SymbolVariant::LocalVar(s) => s.type_path.clone().into(),
                SymbolVariant::ThisVar(s) => s.type_path().to_owned(),
                SymbolVariant::SuperVar(s) => s.type_path().to_owned(),
                SymbolVariant::StateSuperVar(s) => {
                    if s.base_state_name().is_some() {
                        let state_path = path.root().unwrap_or_default();
                        self.symtab_marcher
                            .state_hierarchy(state_path)
                            .skip(1).next()
                            .map(|sym| sym.path().to_owned())
                            .unwrap_or(SymbolPathBuf::unknown(SymbolCategory::Type))
                    } else {
                        BasicTypeSymbolPath::new(StateSymbol::DEFAULT_STATE_BASE_NAME).into()
                    }
                },
                SymbolVariant::ParentVar(s) => s.type_path().to_owned(),
                SymbolVariant::VirtualParentVar(s) => s.type_path().to_owned(),
                SymbolVariant::MemberFuncInjector(s) => s.return_type_path.clone().into(),
                SymbolVariant::MemberFuncReplacer(s) => s.return_type_path.clone().into(),
                SymbolVariant::GlobalFuncReplacer(s) => s.return_type_path.clone().into(),
                SymbolVariant::MemberFuncWrapper(s) => s.return_type_path.clone().into(),
                SymbolVariant::MemberVarInjector(s) => s.type_path.clone().into(),
                SymbolVariant::WrappedMethod(s) => {
                    self.symtab_marcher
                        .get_symbol(s.wrapped_path())
                        .and_then(|v| v.try_as_member_func_wrapper_ref())
                        .map(|wrapper| wrapper.return_type_path.clone().into())
                        .unwrap_or(SymbolPathBuf::unknown(SymbolCategory::Type))
                }
            }
        } else {
            SymbolPathBuf::unknown(SymbolCategory::Type)
        }
    }
}

impl SyntaxNodeVisitor for ExpressionEvaluator<'_> {
    fn exit_nested_expr(&mut self, _: &NestedExpressionNode, ctx: &TraversalContextStack) {
        self.top_mut().map(|e| e.ctx = ctx.top() );
    }

    fn visit_literal_expr(&mut self, n: &LiteralNode, ctx: &TraversalContextStack) {
        match n.clone().value() {
            Literal::Int(_) => self.push(BasicTypeSymbolPath::new("int").into(), ctx.top()),
            Literal::Hex(_) => self.push(BasicTypeSymbolPath::new("int").into(), ctx.top()),
            Literal::Float(_) => self.push(BasicTypeSymbolPath::new("float").into(), ctx.top()),
            Literal::Bool(_) => self.push(BasicTypeSymbolPath::new("bool").into(), ctx.top()),
            Literal::String(_) => self.push(BasicTypeSymbolPath::new("string").into(), ctx.top()),
            Literal::Name(_) => self.push(BasicTypeSymbolPath::new("name").into(), ctx.top()),
            Literal::Null(_) => self.push(BasicTypeSymbolPath::new("NULL").into(), ctx.top()),
        }
    }

    fn visit_this_expr(&mut self, _: &ThisExpressionNode, ctx: &TraversalContextStack) {
        let sympath_payload = self.sympath_payload.borrow();
        let type_path = sympath_payload.current_sympath.root().unwrap_or_default();
        let this_path = ThisVarSymbolPath::new(type_path);
        drop(sympath_payload);

        self.push(this_path.into(), ctx.top());
    }

    fn visit_super_expr(&mut self, _: &SuperExpressionNode, ctx: &TraversalContextStack) {
        let sympath_payload = self.sympath_payload.borrow();
        let type_path = sympath_payload.current_sympath.root().unwrap_or_default();
        let super_path = SuperVarSymbolPath::new(type_path);
        drop(sympath_payload);
        
        self.push(super_path.into(), ctx.top());
    }

    fn visit_parent_expr(&mut self, _: &ParentExpressionNode, ctx: &TraversalContextStack) {
        let sympath_payload = self.sympath_payload.borrow();
        let type_path = sympath_payload.current_sympath.root().unwrap_or_default();
        let parent_path = ParentVarSymbolPath::new(type_path);
        drop(sympath_payload);
        
        self.push(parent_path.into(), ctx.top());
    }

    fn visit_virtual_parent_expr(&mut self, _: &VirtualParentExpressionNode, ctx: &TraversalContextStack) {
        let sympath_payload = self.sympath_payload.borrow();
        let type_path = sympath_payload.current_sympath.root().unwrap_or_default();
        let virtual_parent_path = VirtualParentVarSymbolPath::new(type_path);
        drop(sympath_payload);
        
        self.push(virtual_parent_path.into(), ctx.top());
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, ctx: &TraversalContextStack) {
        let name = n.value(self.doc);
        let ident_category = if ctx.top() == TraversalContext::FunctionCallExpressionFunc {
            SymbolCategory::Callable
        } else {
            SymbolCategory::Data
        };

        let ident_path = self.unl_payload.borrow()
            .get(&name, ident_category)
            .map(|p| p.to_owned())
            .unwrap_or(SymbolPathBuf::new(&name, ident_category));

        self.push(ident_path, ctx.top());
    }

    fn exit_func_call_expr(&mut self, _: &FunctionCallExpressionNode, ctx: &TraversalContextStack) {
        // this is a completely explicit language in terms of typeing and there is no function overloading
        // so we don't need argument types to get the return type of a function 
        while self.top().map(|e| e.ctx == TraversalContext::FunctionCallArg).unwrap_or(false) {
            self.pop();
        }

        if self.top().map(|e| e.ctx == TraversalContext::FunctionCallExpressionFunc).unwrap_or(false) {
            let func_path = self.pop().unwrap().path;
            self.push(self.produce_type(&func_path), ctx.top());
        } else {
            self.push(SymbolPathBuf::unknown(SymbolCategory::Type), ctx.top());
        }
    }

    fn exit_array_expr(&mut self, _: &ArrayExpressionNode, ctx: &TraversalContextStack) {
        // type of the index doesn't matter in this case
        if self.top().map(|e| e.ctx == TraversalContext::ArrayExpressionIndex).unwrap_or(false) {
            self.pop();
        }

        if self.top().map(|e| e.ctx == TraversalContext::ArrayExpressionAccessor).unwrap_or(false) {
            let accessor_path = self.pop().unwrap().path;
            let accessor_type = self.produce_type(&accessor_path);
            let op_path = MemberCallableSymbolPath::new(&accessor_type, ArrayTypeSymbol::INDEX_OPERATOR_NAME);
            self.push(self.produce_type(&op_path), ctx.top());
        } else {
            self.push(SymbolPathBuf::unknown(SymbolCategory::Type), ctx.top());
        }
    }

    fn exit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, ctx: &TraversalContextStack) {
        if self.top().map(|e| e.ctx == TraversalContext::MemberAccessExpressionAccessor).unwrap_or(false) {
            let accessor_path = self.pop().unwrap().path;
  
            let member = n.member().value(self.doc);
            let member_category = if ctx.top() == TraversalContext::FunctionCallExpressionFunc {
                SymbolCategory::Callable
            } else {
                SymbolCategory::Data
            };
        
            let accessor_type = self.produce_type(&accessor_path);
            if let Some(accessor_symvar) = self.symtab_marcher.get_symbol(&accessor_type) {
                let member_path = match accessor_symvar {
                    SymbolVariant::Class(s) => {
                        let mut member_path = SymbolPathBuf::unknown(member_category);

                        for class in self.symtab_marcher.class_hierarchy(s.path()) {
                            let path = class.path().join(&SymbolPathBuf::new(&member, member_category));

                            if self.symtab_marcher.contains_symbol(&path) {
                                member_path = path;
                                break;
                            }
                        }

                        member_path
                    },
                    SymbolVariant::State(s) => {
                        let mut member_path = SymbolPathBuf::unknown(member_category);

                        for state in self.symtab_marcher.state_hierarchy(s.path()) {
                            let path = state.path().join(&SymbolPathBuf::new(&member, member_category));

                            if self.symtab_marcher.contains_symbol(&path) {
                                member_path = path;
                                break;
                            }
                        }

                        if member_path.has_unknown() {
                            let mut path = SymbolPathBuf::new(StateSymbol::DEFAULT_STATE_BASE_NAME, SymbolCategory::Type);
                            path.push(&member, member_category);

                            if self.symtab_marcher.contains_symbol(&path) {
                                member_path = path;
                            }
                        }

                        member_path
                    },
                    SymbolVariant::Struct(s) => {
                        s.path().join(&SymbolPathBuf::new(&member, member_category))
                    },
                    SymbolVariant::Array(s) => {
                        s.path().join(&SymbolPathBuf::new(&member, member_category))
                    },
                    _ => SymbolPathBuf::unknown(SymbolCategory::Type)
                };

                self.push(member_path, ctx.top());
            } else {
                self.push(SymbolPathBuf::unknown(SymbolCategory::Type), ctx.top());    
            }
        } else {
            self.push(SymbolPathBuf::unknown(SymbolCategory::Type), ctx.top());
        }
    }

    fn exit_new_expr(&mut self, n: &NewExpressionNode, ctx: &TraversalContextStack) {
        // lifetime object has no bearing on the type of the expression
        if self.top().map(|e| e.ctx == TraversalContext::NewExpressionLifetimeObj).unwrap_or(false) {
            self.pop();
        }

        self.push(BasicTypeSymbolPath::new(&n.class().value(self.doc)).into(), ctx.top());
    }

    fn exit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, ctx: &TraversalContextStack) {
        if self.top().map(|e| e.ctx == TraversalContext::TypeCastExpressionValue).unwrap_or(false) {
            self.pop();
        }

        let target_type = n.target_type().value(self.doc);
        self.push(BasicTypeSymbolPath::new(&target_type).into(), ctx.top());
    }

    fn exit_unary_op_expr(&mut self, _: &UnaryOperationExpressionNode, ctx: &TraversalContextStack) {
        if self.top().map(|e| e.ctx == TraversalContext::UnaryOperationExpressionRight).unwrap_or(false) {
            // there is no operator overloading as far as I'm aware, so this propagation is probably ok
            self.top_mut().map(|e| e.ctx = ctx.top());
        } else {
            self.push(SymbolPathBuf::unknown(SymbolCategory::Type), ctx.top());
        }
    }

    fn exit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, ctx: &TraversalContextStack) {
        if self.top().map(|e| e.ctx == TraversalContext::BinaryOperationExpressionRight).unwrap_or(false) {
            self.pop();
        }

        let left_path = if self.top().map(|e| e.ctx == TraversalContext::BinaryOperationExpressionLeft).unwrap_or(false) {
            self.pop().unwrap().path
        } else {
            SymbolPathBuf::unknown(SymbolCategory::Type)
        };

        let op_path = match n.op().value() {
            // for now just gonna get the type from the left-hand-side
            BinaryOperator::Mult
            | BinaryOperator::Div
            | BinaryOperator::Mod
            | BinaryOperator::Sum
            | BinaryOperator::Diff
            | BinaryOperator::BitAnd
            | BinaryOperator::BitOr
            | BinaryOperator::BitXor => left_path,
            BinaryOperator::And
            | BinaryOperator::Or
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Lesser
            | BinaryOperator::LesserOrEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterOrEqual => BasicTypeSymbolPath::new("bool").into(),
        };

        self.push(op_path, ctx.top());
    }

    fn exit_assign_op_expr(&mut self, _: &AssignmentOperationExpressionNode, ctx: &TraversalContextStack) {
        if self.top().map(|e| e.ctx == TraversalContext::AssignmentOperationExpressionRight).unwrap_or(false) {
            self.pop();
        }

        if self.top().map(|e| e.ctx == TraversalContext::AssignmentOperationExpressionLeft).unwrap_or(false) {
            self.top_mut().map(|e| e.ctx = ctx.top() );
        } else {
            self.push(SymbolPathBuf::unknown(SymbolCategory::Type), ctx.top());
        }
    }

    fn exit_ternary_cond_expr(&mut self, _: &TernaryConditionalExpressionNode, ctx: &TraversalContextStack) {
        if self.top().map(|e| e.ctx == TraversalContext::TernaryConditionalExpressionAlt).unwrap_or(false) {
            self.pop();
        }

        let conseq_path = if self.top().map(|e| e.ctx == TraversalContext::TernaryConditionalExpressionConseq).unwrap_or(false) {
            self.pop().unwrap().path
        } else {
            SymbolPathBuf::unknown(SymbolCategory::Type)
        };

        if self.top().map(|e| e.ctx == TraversalContext::TernaryConditionalExpressionCond).unwrap_or(false) {
            self.pop();
        }

        self.push(conseq_path, ctx.top());
    }
}

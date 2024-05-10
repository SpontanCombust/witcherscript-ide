use std::{cell::RefCell, rc::Rc};
use witcherscript::{ast::*, script_document::ScriptDocument};
use crate::model::{symbol_path::SymbolPathBuf, symbols::*};


/// A visitor that builds a symbol path as it traverses the syntax tree.
/// 
/// Possible built symbol paths are:
/// - class declarations, 
/// - state declarations, 
/// - struct declarations,
/// - enum declarations, 
/// - global function declarations
/// - member function declaration
/// - event declaration
/// 
/// Symbol paths for "leaf" symbols are not built. So a member var declaration for example is not taken into account. 
#[derive(Debug, Clone)]
pub struct SymbolPathBuilder<'a> {
    doc: &'a ScriptDocument,
    payload: Rc<RefCell<SymbolPathBuilderPayload>>,
}

#[derive(Debug, Clone)]
pub struct SymbolPathBuilderPayload {
    pub current_sympath: SymbolPathBuf
}

impl<'a> SymbolPathBuilder<'a> {
    pub fn new(doc: &'a ScriptDocument) -> (Self, Rc<RefCell<SymbolPathBuilderPayload>>) {
        let payload = Rc::new(RefCell::new(SymbolPathBuilderPayload {
            current_sympath: SymbolPathBuf::empty()
        }));

        let self_ = Self {
            doc,
            payload: payload.clone()
        };

        (self_, payload)
    }

    pub fn new_rc(doc: &'a ScriptDocument) -> (Rc<RefCell<Self>>, Rc<RefCell<SymbolPathBuilderPayload>>) {
        let (self_, payload) = Self::new(doc);
        (Rc::new(RefCell::new(self_)), payload)
    }
}

impl SyntaxNodeVisitor for SymbolPathBuilder<'_> {
    fn traversal_policy_default(&self) -> bool {
        true
    }


    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let name = n.name().value(self.doc);
        self.payload.borrow_mut().current_sympath = BasicTypeSymbolPath::new(&name).into();
        TraversalPolicy::default_to(true)
    }

    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {
        self.payload.borrow_mut().current_sympath.pop();
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let name = n.name().value(self.doc);
        let parent = n.parent().value(self.doc);
        self.payload.borrow_mut().current_sympath = StateSymbolPath::new(&name, &parent).into();
        TraversalPolicy::default_to(true)
    }

    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {
        self.payload.borrow_mut().current_sympath.pop();
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let name = n.name().value(self.doc);
        self.payload.borrow_mut().current_sympath = BasicTypeSymbolPath::new(&name).into();
        TraversalPolicy::default_to(true)
    }

    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {
        self.payload.borrow_mut().current_sympath.pop();
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        let name = n.name().value(self.doc);
        self.payload.borrow_mut().current_sympath = BasicTypeSymbolPath::new(&name).into();
        TraversalPolicy::default_to(true)
    }

    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {
        self.payload.borrow_mut().current_sympath.pop();
    }



    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        let name = n.name().value(self.doc);
        self.payload.borrow_mut().current_sympath = GlobalCallableSymbolPath::new(&name).into();
        TraversalPolicy::default_to(true)
    }

    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {
        self.payload.borrow_mut().current_sympath.pop();
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) -> MemberFunctionDeclarationTraversalPolicy {
        let name = n.name().value(self.doc);
        self.payload.borrow_mut().current_sympath.push(&name, SymbolCategory::Callable);
        TraversalPolicy::default_to(true)
    }

    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) {
        self.payload.borrow_mut().current_sympath.pop();
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        let name = n.name().value(self.doc);
        self.payload.borrow_mut().current_sympath.push(&name, SymbolCategory::Callable);
        TraversalPolicy::default_to(true)
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode, _: PropertyTraversalContext) {
        self.payload.borrow_mut().current_sympath.pop();
    }
}

impl SyntaxNodeVisitorChainLink for SymbolPathBuilder<'_> {}
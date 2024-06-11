use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};
use witcherscript::{ast::*, attribs::*, script_document::ScriptDocument};
use crate::utils::SymbolPathBuilderPayload;
use super::symbol_path::{SymbolPath, SymbolPathBuf};
use super::symbols::{BasicTypeSymbolPath, MemberDataSymbolPath, StateSymbol, Symbol, SymbolCategory};
use super::symbol_table::{iter::*, marcher::SymbolTableMarcher};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    name: String,
    category: SymbolCategory
}

impl Key {
    fn new(name: String, category: SymbolCategory) -> Self {
        Self {
            name,
            category
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BorrowedKey<'s> {
    name: &'s str,
    category: SymbolCategory
}

impl<'s> BorrowedKey<'s> {
    fn new(name: &'s str, category: SymbolCategory) -> Self {
        Self {
            name, category
        }
    }
}

trait AsBorrowedKey {
    fn borrow(&self) -> BorrowedKey;
}

impl AsBorrowedKey for Key {
    fn borrow(&self) -> BorrowedKey {
        BorrowedKey { 
            name: &self.name, 
            category: self.category 
        }
    }
}

impl AsBorrowedKey for BorrowedKey<'_> {
    fn borrow(&self) -> BorrowedKey {
        *self
    }
}

impl<'s> std::borrow::Borrow<dyn AsBorrowedKey + 's> for Key {
    fn borrow(&self) -> &(dyn AsBorrowedKey + 's) {
        self
    }
}

impl PartialEq for (dyn AsBorrowedKey + '_) {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().eq(&other.borrow())
    }
}

impl Eq for (dyn AsBorrowedKey + '_) {}

impl Hash for (dyn AsBorrowedKey + '_) {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.borrow().hash(state)
    }
}


type Scope = HashMap<Key, SymbolPathBuf>;


/// Keeps track of all unqualified symbol identifiers that are valid and accessible in the current context.  
/// "Unqualified" means that the name/identifier appears freely in the code and is ambiguous without a greater context,
/// e.g. a member var can be used without `this` keyword and thus becomes ambiguous
/// without the context of inherited properties, local vars and global constants (enum variants).  
/// Names on each deeper scope layer can overshadow the same name from higher layers.
#[derive(Debug, Clone)]
pub struct UnqualifiedNameLookup {
    stack: Vec<Scope>
}

impl UnqualifiedNameLookup {
    fn new() -> Self {
        Self {
            stack: vec![Scope::new()]
        }
    }

    fn push_scope(&mut self) {
        self.stack.push(Scope::new())
    }

    fn pop_scope(&mut self) {
        // always keep at least one scope level
        if self.stack.len() > 1 {
            self.stack.pop();
        } else {
            // if there is one scope left, only clear its contents
            self.stack.last_mut().unwrap().clear();
        }
    }
    
    pub fn contains(&self, name: &str, category: SymbolCategory) -> bool {
        let k = BorrowedKey::new(name, category);
        for level in self.stack.iter().rev() {
            if level.contains_key(&k as &dyn AsBorrowedKey) {
                return true;
            }
        }

        false
    }

    pub fn get(&self, name: &str, category: SymbolCategory) -> Option<&SymbolPath> {
        let k = BorrowedKey::new(name, category);
        for level in self.stack.iter().rev() {
            if let Some(path) = level.get(&k as &dyn AsBorrowedKey) {
                return Some(path);
            }
        }

        None
    }

    /// Inserts a symbol name mapping into the stack.
    fn insert(&mut self, path: SymbolPathBuf) {
        let comp = path.components().last().unwrap();

        self.stack.last_mut().unwrap().insert(
            Key::new(comp.name.to_string(), comp.category),
            path.to_owned()
        );
    }
}




#[derive(Clone)]
pub struct UnqualifiedNameLookupBuilder<'a> {
    doc: &'a ScriptDocument,
    payload: Rc<RefCell<UnqualifiedNameLookup>>,
    sympath_ctx: Rc<RefCell<SymbolPathBuilderPayload>>,
    symtab_marcher: SymbolTableMarcher<'a>
}

impl<'a> UnqualifiedNameLookupBuilder<'a> {
    /// The first symbol table in `symtab_marcher` should be the one corresponding to the currently visited document
    pub fn new(
        doc: &'a ScriptDocument, 
        sympath_ctx: Rc<RefCell<SymbolPathBuilderPayload>>,
        symtab_marcher: SymbolTableMarcher<'a>
    ) -> (Self, Rc<RefCell<UnqualifiedNameLookup>>) {
        let payload = Rc::new(RefCell::new(UnqualifiedNameLookup::new()));

        let self_ = Self {
            doc,
            payload: payload.clone(),
            sympath_ctx,
            symtab_marcher
        };

        (self_, payload)
    }

    /// The first symbol table in `symtab_marcher` should be the one corresponding to the currently visited document
    pub fn new_rc(
        doc: &'a ScriptDocument, 
        sympath_ctx: Rc<RefCell<SymbolPathBuilderPayload>>,
        symtab_marcher: SymbolTableMarcher<'a>
    ) -> (Rc<RefCell<Self>>, Rc<RefCell<UnqualifiedNameLookup>>) {
        let (self_, payload) = Self::new(doc, sympath_ctx, symtab_marcher);
        (Rc::new(RefCell::new(self_)), payload)
    }
}

impl SyntaxNodeVisitor for UnqualifiedNameLookupBuilder<'_> {
    fn traversal_policy_default(&self) -> bool {
        true
    }


    fn visit_class_decl(&mut self, _: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut unl = self.payload.borrow_mut();
        let sympath_ctx = self.sympath_ctx.borrow();

        unl.push_scope();

        let mut inherit_chain = self.symtab_marcher
            .class_hierarchy(&sympath_ctx.current_sympath)
            .collect::<Vec<_>>();

        // we want to iterate classes starting from the most base class
        inherit_chain.reverse();

        // with each class in the inheritance chain we add properties to the UNL that `this` inherits from
        // if any functions are overriden in child classes, the record gets overwritten
        for class in inherit_chain {
            if let Some(class_symtab) = self.symtab_marcher.find_table_containing_symbol(class.path()) {
                for ch in class_symtab.get_symbol_children_filtered(class) {
                    match ch {
                        ClassSymbolChild::Var(s) => {
                            if class.path() == &sympath_ctx.current_sympath || !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        ClassSymbolChild::Autobind(s) => {
                            if class.path() == &sympath_ctx.current_sympath || !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        ClassSymbolChild::Method(s) => {
                            if class.path() == &sympath_ctx.current_sympath || !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        ClassSymbolChild::Event(s) => {
                            unl.insert(s.path().to_owned());
                        },
                    }
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {
        self.payload.borrow_mut().pop_scope();
    }

    fn visit_state_decl(&mut self, _: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut unl = self.payload.borrow_mut();
        let sympath_ctx = self.sympath_ctx.borrow();

        unl.push_scope(); 

        // all state types inherit properties from this one class
        let base_type_path = BasicTypeSymbolPath::new(StateSymbol::DEFAULT_STATE_BASE_NAME);
        if let Some((base_type_symtab, base_type_symvar)) = self.symtab_marcher.get_symbol_with_containing_table(&base_type_path) {
            if let Some(class) = base_type_symvar.try_as_class_ref() {
                for ch in base_type_symtab.get_symbol_children_filtered(class) {
                    match ch {
                        ClassSymbolChild::Var(s) => {
                            if !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        ClassSymbolChild::Autobind(s) => {
                            if !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        ClassSymbolChild::Method(s) => {
                            if !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        ClassSymbolChild::Event(s) => {
                            unl.insert(s.path().to_owned());
                        },
                    }
                }
            }
        }

        let mut inherit_chain = self.symtab_marcher
            .state_hierarchy(&sympath_ctx.current_sympath)
            .collect::<Vec<_>>();

        inherit_chain.reverse();

        for state in inherit_chain {
            if let Some(state_symtab) = self.symtab_marcher.find_table_containing_symbol(state.path()) {
                for ch in state_symtab.get_symbol_children_filtered(state) {
                    match ch {
                        StateSymbolChild::Var(s) => {
                            if state.path() == &sympath_ctx.current_sympath || !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        StateSymbolChild::Autobind(s) => {
                            if state.path() == &sympath_ctx.current_sympath || !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        StateSymbolChild::Method(s) => {
                            if state.path() == &sympath_ctx.current_sympath || !s.specifiers.contains(AccessModifier::Private.into()) {
                                unl.insert(s.path().to_owned());
                            }
                        },
                        StateSymbolChild::Event(s) => {
                            unl.insert(s.path().to_owned());
                        },
                    }
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {
        self.payload.borrow_mut().pop_scope();
    }

    fn visit_struct_decl(&mut self, _: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut unl = self.payload.borrow_mut();
        let sympath_ctx = self.sympath_ctx.borrow();

        unl.push_scope();

        if let Some((struct_symtab, struct_symvar)) = self.symtab_marcher.get_symbol_with_containing_table(&sympath_ctx.current_sympath) {
            if let Some(struct_sym) = struct_symvar.try_as_struct_ref() {
                for s in struct_symtab.get_symbol_children_filtered(struct_sym) {
                    unl.insert(s.path().to_owned());
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {
        self.payload.borrow_mut().pop_scope();
    }

    fn visit_enum_decl(&mut self, _: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        // no point in adding anything here as enum variants are already exposed in the global scope

        TraversalPolicy::default_to(true)
    }

    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {
        
    }

    fn visit_global_func_decl(&mut self, _: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        let mut unl = self.payload.borrow_mut();
        let sympath_ctx = self.sympath_ctx.borrow();

        unl.push_scope();

        if let Some((func_symtab, func_symvar)) = self.symtab_marcher.get_symbol_with_containing_table(&sympath_ctx.current_sympath) {
            if let Some(func) = func_symvar.try_as_global_func_ref() {
                for ch in func_symtab.get_symbol_children_filtered(func) {
                    if let CallableSymbolChild::Param(s) = ch {
                        unl.insert(s.path().to_owned());
                    }
                    // local vars will be pushed dynamically as a function will go on
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn exit_global_func_decl(&mut self, _: &FunctionDeclarationNode) {
        self.payload.borrow_mut().pop_scope();
    }

    fn visit_member_func_decl(&mut self, _: &FunctionDeclarationNode, _: PropertyTraversalContext) -> FunctionDeclarationTraversalPolicy {
        let mut unl = self.payload.borrow_mut();
        let sympath_ctx = self.sympath_ctx.borrow();

        unl.push_scope();

        if let Some((func_symtab, func_symvar)) = self.symtab_marcher.get_symbol_with_containing_table(&sympath_ctx.current_sympath) {
            if let Some(func) = func_symvar.try_as_member_func_ref() {
                for ch in func_symtab.get_symbol_children_filtered(func) {
                    if let CallableSymbolChild::Param(s) = ch {
                        unl.insert(s.path().to_owned());
                    }
                    // local vars will be pushed dynamically as a function will go on
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn exit_member_func_decl(&mut self, _: &FunctionDeclarationNode, _: PropertyTraversalContext) {
        self.payload.borrow_mut().pop_scope();
    }

    fn visit_event_decl(&mut self, _: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        let mut unl = self.payload.borrow_mut();
        let sympath_ctx = self.sympath_ctx.borrow();

        unl.push_scope();

        if let Some((event_symtab, event_symvar)) = self.symtab_marcher.get_symbol_with_containing_table(&sympath_ctx.current_sympath) {
            if let Some(event) = event_symvar.try_as_event_ref() {
                for ch in event_symtab.get_symbol_children_filtered(event) {
                    if let CallableSymbolChild::Param(s) = ch {
                        unl.insert(s.path().to_owned());
                    }
                    // local vars will be pushed dynamically as a function will go on
                }
            }
        }

        TraversalPolicy::default_to(true)
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode, _: PropertyTraversalContext) {
        self.payload.borrow_mut().pop_scope();
    }

    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        let mut unl = self.payload.borrow_mut();
        let sympath_ctx = self.sympath_ctx.borrow();

        for name in n.names().map(|n| n.value(self.doc)) {
            let path = MemberDataSymbolPath::new(&sympath_ctx.current_sympath, &name);
            unl.insert(path.into());
        }

        TraversalPolicy::default_to(true)
    }
}

impl SyntaxNodeVisitorChainLink for UnqualifiedNameLookupBuilder<'_> {}
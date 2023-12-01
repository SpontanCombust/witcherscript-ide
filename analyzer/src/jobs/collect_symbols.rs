use ropey::Rope;
use thiserror::Error;
use uuid::Uuid;
use witcherscript::tokens::Identifier;
use witcherscript::{SyntaxNode, DocPos};
use witcherscript::ast::*;
use crate::model::{collections::*, symbols::*};


#[derive(Debug, Error)]
pub enum SymbolCollectionError {
    #[error("missing symbol at {0}")]
    NodeMissing(DocPos),
    #[error("duplicate declaration of type {symbol_name} at {duplicate_appearance}")]
    DuplicateDefinition{
        symbol_name: String,
        // first_appearance: DocSpan, //TODO symbols storing their spans
        duplicate_appearance: DocPos 
    },
    #[error("type {0} is missing")]
    TypeMissing(DocPos),
}

use SymbolCollectionError::*;


struct TypeCollectingVisitor {
    db: SymbolDb,
    symtab: SymbolTable,
    script_id: Uuid,
    rope: Rope,
    errors: Vec<SymbolCollectionError>,
}

impl TypeCollectingVisitor {
    /// If identifier node is missing appends an error and returns None. Otherwise returns identifier text.
    fn check_missing_node(&mut self, ident_node: &SyntaxNode<'_, Identifier>) -> Option<String> {
        if let Some(ident) = ident_node.value(&self.rope) { 
            Some(ident.into())
        } else {
            self.errors.push(NodeMissing(ident_node.span().start));
            None
        }
    }

    /// If symbol is a duplicate appends an error and returns false, otherwise returns true.
    fn check_duplicate(&mut self, sym_name: &str, pos: DocPos) -> bool {
        if self.symtab.contains_key(sym_name) {
            self.errors.push(DuplicateDefinition { 
                symbol_name: sym_name.to_owned(), 
                duplicate_appearance: pos
            });

            true
        } else {
            false
        }
    }
}

impl StatementVisitor for TypeCollectingVisitor {
    fn visit_class_decl(&mut self, n: &SyntaxNode<'_, ClassDeclaration>) -> bool {
        if let Some(c) = self.check_missing_node(&n.name()) {
            let pos = n.span().start;
            if self.check_duplicate(c.as_str(), pos) {
                let sym = ClassSymbol::new(self.script_id, c.as_str());
                self.symtab.insert(c, SymbolTableValue::from_symbol(&sym));
                self.db.classes.insert(sym.symbol_id(), sym);
            }
        }

        false
    }

    fn visit_state_decl(&mut self, n: &SyntaxNode<'_, StateDeclaration>) -> bool {
        if let (Some(state_name), Some(parent_name)) = (self.check_missing_node(&n.name()), self.check_missing_node(&n.parent())) {
            let state_class = StateSymbol::class_name(&state_name, &parent_name);
            let pos = n.span().start;
            if self.check_duplicate(state_class.as_str(), pos) {
                let sym = StateSymbol::new(self.script_id, state_class.as_str());
                self.symtab.insert(state_class, SymbolTableValue::from_symbol(&sym));
                self.db.states.insert(sym.symbol_id(), sym);
            }
        }

        false
    }

    fn visit_struct_decl(&mut self, n: &SyntaxNode<'_, StructDeclaration>) -> bool {
        if let Some(s) = self.check_missing_node(&n.name()) {
            let pos = n.span().start;
            if self.check_duplicate(s.as_str(), pos) {
                let sym = StructSymbol::new(self.script_id, s.as_str());
                self.symtab.insert(s, SymbolTableValue::from_symbol(&sym));
                self.db.structs.insert(sym.symbol_id(), sym);
            }
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &SyntaxNode<'_, EnumDeclaration>) -> bool {
        if let Some(e) = self.check_missing_node(&n.name()) {
            let pos = n.span().start;
            if self.check_duplicate(e.as_str(), pos) {
                let sym = EnumSymbol::new(self.script_id, e.as_str());
                self.symtab.insert(e, SymbolTableValue::from_symbol(&sym));
                self.db.enums.insert(sym.symbol_id(), sym);
            }
        }

        false
    }
}
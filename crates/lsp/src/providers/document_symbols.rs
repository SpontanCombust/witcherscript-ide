use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::attribs::MemberVarSpecifier;
use witcherscript_analysis::model::symbol_path::SymbolPathBuf;
use witcherscript_analysis::model::symbol_variant::SymbolVariant;
use witcherscript_analysis::model::symbols::*;
use crate::Backend;


pub async fn document_symbol(backend: &Backend, params: lsp::DocumentSymbolParams) -> Result<Option<lsp::DocumentSymbolResponse>> {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    
    let (content_path, source_file);
    if let Some((path, file)) = backend.source_trees.find_source_file(&doc_path) {
        content_path = path;
        source_file = file;
    } 
    else {
        return Ok(None);
    }

    let symtabs = backend.symtabs.read().await;
    let symtab_ref;
    if let Some(symtab) = symtabs.get(&content_path) {
        symtab_ref = symtab;
    } 
    else {
        backend.reporter.log_error(format!("[document_symbol] Unexpeted: symbol table not found for content {}", content_path)).await;
        return Ok(None);
    }

    
    let mut doc_syms = Vec::new();
    
    type DocSymStack = Vec<(SymbolPathBuf, lsp::DocumentSymbol)>;
    let mut doc_sym_stack: DocSymStack = Vec::with_capacity(4);

    // this works on assumption that stack is not empty and the symbol on top of it can have children
    let reduce_stack = |stack: &mut DocSymStack, doc_syms: &mut Vec<lsp::DocumentSymbol>| {
        let top = stack.pop().unwrap().1;
        if let Some(reduced_top) = stack.last_mut().map(|(_, s)| s) {
            reduced_top.children.as_mut().unwrap().push(top);
        } else {
            doc_syms.push(top);
        }
    };
    
    for sym_variant in symtab_ref.get_for_source(source_file.path.local()) {
        if let Some(doc_sym) = sym_variant.to_doc_sym() {
            let sympath = sym_variant.as_dyn().path();

            // if the symbol is not primary, then we need to reduce the stack to a form
            // in which the top element if the parent symbol of the current
            if let Some(parent_sympath) = sympath.parent() {
                while doc_sym_stack.last().map(|(p, _)| p != parent_sympath).unwrap_or(false) {
                    reduce_stack(&mut doc_sym_stack, &mut doc_syms);
                }
            // ...if it is primary we reduce the stack until it's empty,
            // because the current symbol has no parent symbols
            } else {
                while !doc_sym_stack.is_empty() {
                    reduce_stack(&mut doc_sym_stack, &mut doc_syms);
                }
            }

            // after the stack is reduced to an appropriate ancestor
            // we push the current symbol onto stack
            let can_have_children = doc_sym.children.is_some();
            doc_sym_stack.push((sympath.to_owned(), doc_sym));

            // if this current symbol cannot have children we can immediately reduce the stack
            // otherwise the symbol stays on top and will have other symbols added as its children
            if !can_have_children {
                reduce_stack(&mut doc_sym_stack, &mut doc_syms);
            }
        }
    }

    while !doc_sym_stack.is_empty() {
        reduce_stack(&mut doc_sym_stack, &mut doc_syms);
    }

    Ok(Some(lsp::DocumentSymbolResponse::Nested(doc_syms)))
}


trait ToDocumentSymbol {
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol>;
}

impl ToDocumentSymbol for ClassSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::CLASS,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            // we mark symbols that are **able to** have children with a Some value in `children` field 
            // even if that symbol may not in fact have any children
            children: Some(Vec::new()), 
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for StructSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::STRUCT,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for StateSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: format!("state {} in {}", self.state_name(), self.parent_class_name()),
            kind: lsp::SymbolKind::CLASS,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for EnumSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::ENUM,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for EnumVariantSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::ENUM_MEMBER,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: None,
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for ArrayTypeSymbol {
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for GlobalFunctionSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::FUNCTION,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for MemberFunctionSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::METHOD,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for EventSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::EVENT,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for PrimitiveTypeSymbol {
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for FunctionParameterSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        // we're not going to include parameters in listed symbols
        None
        // Some(lsp::DocumentSymbol {
        //     name: self.name().to_owned(),
        //     kind: lsp::SymbolKind::VARIABLE,
        //     range: self.range(),
        //     selection_range: self.label_range(),
        //     detail: None,
        //     tags: None,
        //     children: None,
        //     deprecated: None
        // })
    }
}

impl ToDocumentSymbol for GlobalVarSymbol {
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for MemberVarSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: if self.specifiers.contains(&MemberVarSpecifier::Const) { 
                lsp::SymbolKind::CONSTANT 
            } else {
                lsp::SymbolKind::FIELD
            },
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: None,
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for AutobindSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::FIELD,
            range: self.range(),
            selection_range: self.label_range(),
            detail: None,
            tags: None,
            children: None,
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for LocalVarSymbol {
    #[allow(deprecated)]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        // we're not going to include local vars in listed symbols
        None
        // Some(lsp::DocumentSymbol {
        //     name: self.name().to_owned(),
        //     kind: lsp::SymbolKind::VARIABLE,
        //     range: self.range(),
        //     selection_range: self.label_range(),
        //     detail: None,
        //     tags: None,
        //     children: None,
        //     deprecated: None
        // })
    }
}

impl ToDocumentSymbol for SpecialVarSymbol {
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for SymbolVariant {
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        match self {
            SymbolVariant::Class(s) => s.to_doc_sym(),
            SymbolVariant::State(s) => s.to_doc_sym(),
            SymbolVariant::Struct(s) => s.to_doc_sym(),
            SymbolVariant::Enum(s) => s.to_doc_sym(),
            SymbolVariant::Array(s) => s.to_doc_sym(),
            SymbolVariant::GlobalFunc(s) => s.to_doc_sym(),
            SymbolVariant::MemberFunc(s) => s.to_doc_sym(),
            SymbolVariant::Event(s) => s.to_doc_sym(),
            SymbolVariant::Primitive(s) => s.to_doc_sym(),
            SymbolVariant::EnumVariant(s) => s.to_doc_sym(),
            SymbolVariant::FuncParam(s) => s.to_doc_sym(),
            SymbolVariant::GlobalVar(s) => s.to_doc_sym(),
            SymbolVariant::MemberVar(s) => s.to_doc_sym(),
            SymbolVariant::Autobind(s) => s.to_doc_sym(),
            SymbolVariant::LocalVar(s) => s.to_doc_sym(),
            SymbolVariant::SpecialVar(s) => s.to_doc_sym(),
        }
    }
}
use std::collections::HashMap;

use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::attribs::MemberVarSpecifier;
use witcherscript_analysis::symbol_analysis::symbol_path::SymbolPathBuf;
use witcherscript_analysis::symbol_analysis::symbols::*;
use crate::Backend;


pub async fn document_symbol(backend: &Backend, params: lsp::DocumentSymbolParams) -> Result<Option<lsp::DocumentSymbolResponse>> {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();

    if doc_path.extension().unwrap_or_default() != "ws" {
        return Ok(None);
    }
    
    let content_info;
    if let Some(ci) = backend.scripts.get(&doc_path).and_then(|ss| ss.content_info.clone()) {
        content_info = ci;
    } 
    else {
        return Ok(None);
    }

    let symtabs = backend.symtabs.read().await;
    let symtab_ref;
    if let Some(symtab) = symtabs.get(&content_info.content_path) {
        symtab_ref = symtab;
    } 
    else {
        backend.reporter.log_error(format!("[document_symbol] Unexpeted: symbol table not found for content {}", content_info.content_path)).await;
        return Ok(None);
    }

    
    let mut doc_syms = Vec::new();
    
    // enums are an exception to the symbol path system
    let mut doc_enums = HashMap::<SymbolPathBuf, lsp::DocumentSymbol>::new();
    let mut doc_enum_variants = Vec::<(SymbolPathBuf, lsp::DocumentSymbol)>::new();

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
    
    for sym_variant in symtab_ref.get_symbols_for_source(content_info.source_tree_path.local()) {
        if let Some(doc_sym) = sym_variant.to_doc_sym() {    
            if let Some(enum_sym) = sym_variant.try_as_enum_ref() {
                doc_enums.insert(enum_sym.path().to_owned(), doc_sym);
            }
            else if let Some(enum_variant_sym) = sym_variant.try_as_enum_variant_ref() {
                doc_enum_variants.push((enum_variant_sym.parent_enum_path.clone().into(), doc_sym));
            }
            else {
                let sympath = sym_variant.path();
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
    }

    drop(symtabs);

    while !doc_sym_stack.is_empty() {
        reduce_stack(&mut doc_sym_stack, &mut doc_syms);
    }

    for (parent_enum_sympath, doc_enum_variant) in doc_enum_variants {
        doc_enums.entry(parent_enum_sympath)
            .and_modify(|doc_enum| {
                doc_enum.children.as_mut()
                    .map(|ch| ch.push(doc_enum_variant));
            });
    }

    doc_syms.extend(doc_enums.into_values());

    Ok(Some(lsp::DocumentSymbolResponse::Nested(doc_syms)))
}


trait ToDocumentSymbol {
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol>;
}

impl ToDocumentSymbol for ClassSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::CLASS,
            range: self.location().range,
            selection_range: self.location().label_range,
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
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::STRUCT,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for StateSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: format!("state {} in {}", self.state_name(), self.parent_class_name()),
            kind: lsp::SymbolKind::CLASS,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for EnumSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::ENUM,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for EnumVariantSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::ENUM_MEMBER,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: None,
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for ArrayTypeSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for ArrayTypeFunctionSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for ArrayTypeFunctionParameterSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for GlobalFunctionSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::FUNCTION,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for MemberFunctionSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::METHOD,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for EventSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::EVENT,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: Some(Vec::new()),
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for PrimitiveTypeSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for FunctionParameterSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        // we're not going to include parameters in listed symbols
        None
        // Some(lsp::DocumentSymbol {
        //     name: self.name().to_owned(),
        //     kind: lsp::SymbolKind::VARIABLE,
        //      range: self.location().range,
        //      selection_range: self.location().label_range,
        //     detail: None,
        //     tags: None,
        //     children: None,
        //     deprecated: None
        // })
    }
}

impl ToDocumentSymbol for GlobalVarSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for MemberVarSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: if self.specifiers.contains(MemberVarSpecifier::Const) { 
                lsp::SymbolKind::CONSTANT 
            } else {
                lsp::SymbolKind::FIELD
            },
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: None,
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for AutobindSymbol {
    #[allow(deprecated)]
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        Some(lsp::DocumentSymbol {
            name: self.name().to_owned(),
            kind: lsp::SymbolKind::FIELD,
            range: self.location().range,
            selection_range: self.location().label_range,
            detail: None,
            tags: None,
            children: None,
            deprecated: None
        })
    }
}

impl ToDocumentSymbol for LocalVarSymbol {
    #[allow(deprecated)]
    #[inline]
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

impl ToDocumentSymbol for ThisVarSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for SuperVarSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for StateSuperVarSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for ParentVarSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for VirtualParentVarSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for ConstructorSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        None
    }
}

impl ToDocumentSymbol for MemberFunctionInjectorSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        self.inner.to_doc_sym()
    }
}

impl ToDocumentSymbol for MemberFunctionReplacerSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        self.inner.to_doc_sym()
    }
}

impl ToDocumentSymbol for GlobalFunctionReplacerSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        self.inner.to_doc_sym()
    }
}

impl ToDocumentSymbol for MemberFunctionWrapperSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        self.inner.to_doc_sym()
    }
}

impl ToDocumentSymbol for MemberVarInjectorSymbol {
    #[inline]
    fn to_doc_sym(&self) -> Option<lsp::DocumentSymbol> {
        self.inner.to_doc_sym()
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
            SymbolVariant::ArrayFunc(s) => s.to_doc_sym(),
            SymbolVariant::ArrayFuncParam(s) => s.to_doc_sym(),
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
            SymbolVariant::ThisVar(s) => s.to_doc_sym(),
            SymbolVariant::SuperVar(s) => s.to_doc_sym(),
            SymbolVariant::StateSuperVar(s) => s.to_doc_sym(),
            SymbolVariant::ParentVar(s) => s.to_doc_sym(),
            SymbolVariant::VirtualParentVar(s) => s.to_doc_sym(),
            SymbolVariant::Constructor(s) => s.to_doc_sym(),
            SymbolVariant::MemberFuncInjector(s) => s.to_doc_sym(),
            SymbolVariant::MemberFuncReplacer(s) => s.to_doc_sym(),
            SymbolVariant::GlobalFuncReplacer(s) => s.to_doc_sym(),
            SymbolVariant::MemberFuncWrapper(s) => s.to_doc_sym(),
            SymbolVariant::MemberVarInjector(s) => s.to_doc_sym(),
        }
    }
}
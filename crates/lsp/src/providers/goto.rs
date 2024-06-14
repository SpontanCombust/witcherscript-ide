use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript_analysis::symbol_analysis::{symbol_path::SymbolPathBuf, symbols::*};
use crate::{Backend, messaging::notifications};
use super::common::resolve_text_document_position;


impl Backend {
    pub async fn goto_definition_impl(&self, params: lsp::GotoDefinitionParams) -> Result<Option<lsp::GotoDefinitionResponse>> {
        let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();
    
        if doc_path.extension().unwrap_or_default() != "ws" {
            return Ok(None);
        }
    
        let content_path;
        if let Some(path) = self.scripts.get(&doc_path).and_then(|ss| ss.content_info.as_ref().map(|ci| ci.content_path.to_owned())) {
            content_path = path;
        } 
        else {
            self.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
            return Ok(None);
        }
    
        if let Some(inspected) = self.inspect_symbol_at_position(&content_path, &doc_path, params.text_document_position_params.position).await {
            let origin_selection_range = Some(inspected.origin_selection_range);
            let mut loc = inspected.loc;
    
            if let Some(symvar) = inspected.symvar {
                if let Some(wrapped_method_sym) = symvar.try_as_wrapped_method_ref() {
                    let symtabs = self.symtabs.read().await;
                    let symtabs_marcher = self
                        .march_symbol_tables(&symtabs, &content_path).await
                        .skip_first_step(true);
    
                    let wrapped_loc = symtabs_marcher
                        .redefinition_chain(&wrapped_method_sym.wrapped_path())
                        .skip(1).next()
                        .and_then(|v| v.location());
    
                    if let Some(wrapped_loc) = wrapped_loc {
                        loc = Some(wrapped_loc.to_owned());
                    }
                }
            }
    
            let target_uri = loc.as_ref()
                .map(|loc| loc.abs_source_path().to_uri())
                .unwrap_or(params.text_document_position_params.text_document.uri.clone());
    
            let target_range = loc.as_ref()
                .map(|loc| loc.range)
                .unwrap_or(inspected.origin_selection_range);
    
            let target_selection_range = loc.as_ref()
                .map(|loc| loc.label_range)
                .unwrap_or(inspected.origin_selection_range);
    
            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range,
                    target_uri,
                    target_range,
                    target_selection_range,
                }
            ])))
        } else {
            Ok(None)
        }
    }

    pub async fn goto_declaration_impl(&self, params: lsp::request::GotoDeclarationParams) -> Result<Option<lsp::request::GotoDeclarationResponse>> {
        let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();
    
        if doc_path.extension().unwrap_or_default() != "ws" {
            return Ok(None);
        }
    
        let content_path;
        if let Some(path) = self.scripts.get(&doc_path).and_then(|ss| ss.content_info.as_ref().map(|ci| ci.content_path.to_owned())) {
            content_path = path;
        } 
        else {
            self.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
            return Ok(None);
        }
    
        if let Some(inspected) = self.inspect_symbol_at_position(&content_path, &doc_path, params.text_document_position_params.position).await {
            let origin_selection_range = Some(inspected.origin_selection_range);
    
            let mut loc = inspected.loc;
            if let Some(symvar) = inspected.symvar {
                // if the inspected symbol is a method or an event
                // attempt to find the very first declaration of the callable
                // because normally you get the location of the last override of the function
                if symvar.is_member_func() || symvar.is_event() {
                    let func_path = symvar.path().to_owned();
                    let mut parent_path = func_path.clone();
                    parent_path.pop();
    
                    let func_name = symvar.name();
    
                    let symtabs = self.symtabs.read().await;
                    let symtabs_marcher = self.march_symbol_tables(&symtabs, &content_path).await;
    
                    let parent_sym_typ = symtabs_marcher
                        .get_symbol(&parent_path)
                        .map(|v| v.typ())
                        .unwrap_or(SymbolType::Type);
    
                    if parent_sym_typ == SymbolType::Class {
                        for class in symtabs_marcher.class_hierarchy(&parent_path).skip(1) {
                            let base_func_path = class.path().join_component(func_name, SymbolCategory::Callable);
                            if let Some(base_func_loc) = symtabs_marcher.locate_symbol(&base_func_path) {
                                loc = Some(base_func_loc.to_owned());
                            }
                        }
                    } 
                    else if parent_sym_typ == SymbolType::State {
                        for state in symtabs_marcher.state_hierarchy(&parent_path).skip(1) {
                            let base_func_path = state.path().join_component(func_name, SymbolCategory::Callable);
                            if let Some(base_func_loc) = symtabs_marcher.locate_symbol(&base_func_path) {
                                loc = Some(base_func_loc.to_owned());
                            }
                        }
    
                        let base_func_path = BasicTypeSymbolPath::new(StateSymbol::DEFAULT_STATE_BASE_NAME).join_component(func_name, SymbolCategory::Callable);
                        if let Some(base_func_loc) = symtabs_marcher.locate_symbol(&base_func_path) {
                            loc = Some(base_func_loc.to_owned());
                        }
                    }
                }
                else if symvar.is_global_func_replacer() || symvar.is_member_func_replacer() || symvar.is_member_func_wrapper() {
                    let sympath = symvar.path();
    
                    let symtabs = self.symtabs.read().await;
                    let symtabs_marcher = self.march_symbol_tables(&symtabs, &content_path).await;
    
                    if let Some(first_loc) = symtabs_marcher.redefinition_chain(&sympath).last().and_then(|v| v.location()) {
                        loc = Some(first_loc.to_owned());
                    }
                }
                else if let Some(wrapped_method_sym) = symvar.try_as_wrapped_method_ref() {
                    let symtabs = self.symtabs.read().await;
                    let symtabs_marcher = self
                        .march_symbol_tables(&symtabs, &content_path).await
                        .skip_first_step(true);
    
                    let wrapped_loc = symtabs_marcher
                        .redefinition_chain(&wrapped_method_sym.wrapped_path())
                        .skip(1).next()
                        .and_then(|v| v.location());
    
                    if let Some(wrapped_loc) = wrapped_loc {
                        loc = Some(wrapped_loc.to_owned());
                    }
                }
            }
    
            let target_uri = loc.as_ref()
                .map(|loc| loc.abs_source_path().to_uri())
                .unwrap_or(params.text_document_position_params.text_document.uri.clone());
    
            let target_range = loc.as_ref()
                .map(|loc| loc.range)
                .unwrap_or(inspected.origin_selection_range);
    
            let target_selection_range = loc.as_ref()
                .map(|loc| loc.label_range)
                .unwrap_or(inspected.origin_selection_range);
    
            Ok(Some(lsp::request::GotoDeclarationResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range,
                    target_uri,
                    target_range,
                    target_selection_range
                }
            ])))
        } else {
            Ok(None)
        }
    }

    pub async fn goto_type_definition_impl(&self, params: lsp::request::GotoTypeDefinitionParams) -> Result<Option<lsp::request::GotoTypeDefinitionResponse>> {
        let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();
    
        if doc_path.extension().unwrap_or_default() != "ws" {
            return Ok(None);
        }
    
        let content_path;
        if let Some(path) = self.scripts.get(&doc_path).and_then(|ss| ss.content_info.as_ref().map(|ci| ci.content_path.to_owned())) {
            content_path = path;
        } 
        else {
            self.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
            return Ok(None);
        }
    
        if let Some(inspected) = self.inspect_symbol_at_position(&content_path, &doc_path, params.text_document_position_params.position).await {
            if inspected.symvar.as_ref().map(|symvar| symvar.typ().category() != SymbolCategory::Type).unwrap_or(false) {
                return Ok(None);
            }
    
            let origin_selection_range = Some(inspected.origin_selection_range);
            let loc = inspected.loc;
    
            let target_uri = loc.as_ref()
                .map(|loc| loc.abs_source_path().to_uri())
                .unwrap_or(params.text_document_position_params.text_document.uri.clone());
    
            let target_range = loc.as_ref()
                .map(|loc| loc.range)
                .unwrap_or(inspected.origin_selection_range);
    
            let target_selection_range = loc.as_ref()
                .map(|loc| loc.label_range)
                .unwrap_or(inspected.origin_selection_range);
    
            Ok(Some(lsp::request::GotoTypeDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range,
                    target_uri,
                    target_range,
                    target_selection_range
                }
            ])))
        } else {
            Ok(None)
        }
    }



    async fn inspect_symbol_at_position(&self, content_path: &AbsPath, doc_path: &AbsPath, position: lsp::Position) -> Option<Inspected> {
        let symtabs = self.symtabs.read().await;
        let symtabs_marcher = self.march_symbol_tables(&symtabs, &content_path).await;
        
        let script_state = self.scripts.get(doc_path)?;
    
        let position_target = resolve_text_document_position(position, &script_state, symtabs_marcher.clone())?;
        drop(script_state);
    
        let symvar = position_target.target_symbol_path(&symtabs_marcher)
            .and_then(|sympath| symtabs_marcher.get_symbol(&sympath))
            .and_then(|symvar| {
                let default_state_base_path: SymbolPathBuf = BasicTypeSymbolPath::new(StateSymbol::DEFAULT_STATE_BASE_NAME).into();
                let rerouted_path = match symvar {
                    SymbolVariant::Constructor(s) => Some(s.parent_type_path.as_sympath()),
                    SymbolVariant::GlobalVar(s) => Some(s.type_path().as_sympath()),
                    SymbolVariant::ThisVar(s) => Some(s.type_path()),
                    SymbolVariant::SuperVar(s) => Some(s.type_path()),
                    SymbolVariant::StateSuperVar(s) => {
                        if s.base_state_name().is_some() {
                            let state_path = s.path().root().unwrap_or_default();
                            symtabs_marcher
                                .state_hierarchy(state_path)
                                .skip(1).next()
                                .map(|sym| sym.path())
                        } else {
                            Some(default_state_base_path.as_sympath())
                        }
                    },
                    SymbolVariant::ParentVar(s) => Some(s.type_path()),
                    SymbolVariant::VirtualParentVar(s) => Some(s.type_path()),
                    _ => None
                };
    
                if let Some(rerouted_path) = rerouted_path {
                    symtabs_marcher.get_symbol(rerouted_path)
                } else {
                    Some(symvar)
                }
            })
            .map(|symvar| symvar.to_owned());
    
        let loc = symvar.as_ref().and_then(|symvar| symvar.location().cloned());
    
        Some(Inspected {
            origin_selection_range: position_target.range,
            symvar,
            loc
        })
    }
}

struct Inspected {
    origin_selection_range: lsp::Range,
    symvar: Option<SymbolVariant>,
    loc: Option<SymbolLocation>
}

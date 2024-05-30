use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript_analysis::symbol_analysis::symbol_table::SymbolLocation;
use witcherscript_analysis::symbol_analysis::symbol_path::SymbolPathBuf;
use witcherscript_analysis::symbol_analysis::symbols::*;
use crate::{providers::common::PositionTargetKind, Backend, messaging::notifications};
use super::common::resolve_text_document_position;



pub async fn goto_definition(backend: &Backend, params: lsp::GotoDefinitionParams) -> Result<Option<lsp::GotoDefinitionResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

    if doc_path.extension().unwrap_or_default() != "ws" {
        return Ok(None);
    }

    let content_path;
    if let Some(path) = backend.source_trees.containing_content_path(&doc_path) {
        content_path = path;
    } 
    else {
        backend.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
        return Ok(None);
    }

    if let Some(inspected) = inspect_symbol_at_position(backend, &content_path, &doc_path, params.text_document_position_params.position).await {
        let origin_selection_range = Some(inspected.origin_selection_range);

        let target_uri = inspected
            .loc.as_ref()
            .map(|loc| loc.abs_source_path.to_uri())
            .unwrap_or(params.text_document_position_params.text_document.uri.clone());

        let target_range = inspected
            .loc.as_ref()
            .map(|loc| loc.range)
            .unwrap_or(inspected.origin_selection_range);

        let target_selection_range = inspected
            .loc.as_ref()
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


pub async fn goto_declaration(backend: &Backend, params: lsp::request::GotoDeclarationParams) -> Result<Option<lsp::request::GotoDeclarationResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

    if doc_path.extension().unwrap_or_default() != "ws" {
        return Ok(None);
    }

    let content_path;
    if let Some(path) = backend.source_trees.containing_content_path(&doc_path) {
        content_path = path;
    } 
    else {
        backend.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
        return Ok(None);
    }

    if let Some(inspected) = inspect_symbol_at_position(backend, &content_path, &doc_path, params.text_document_position_params.position).await {
        let origin_selection_range = Some(inspected.origin_selection_range);

        let mut loc = inspected.loc;
        if let (Some(symvar), Some(loc)) = (inspected.symvar, &mut loc) {
            // if the inspected symbol is a method or an event
            // attempt to find the very first declaration of the callable
            // because normally you get the location of the last override of the function
            if symvar.is_member_func() || symvar.is_event() {
                let func_path = symvar.path().to_owned();
                let mut parent_path = func_path.clone();
                parent_path.pop();

                let func_name = symvar.name();

                let symtabs = backend.symtabs.read().await;
                let symtabs_marcher = backend.march_symbol_tables(&symtabs, &content_path).await;

                let parent_sym_typ = symtabs_marcher
                    .get(&parent_path)
                    .map(|v| v.typ())
                    .unwrap_or(SymbolType::Type);

                if parent_sym_typ == SymbolType::Class {
                    for class in symtabs_marcher.class_hierarchy(&parent_path).skip(1) {
                        let base_func_path = class.path().join_component(func_name, SymbolCategory::Callable);
                        if let Some(base_func_loc) = symtabs_marcher.locate(&base_func_path) {
                            *loc = base_func_loc;
                        }
                    }
                } 
                else if parent_sym_typ == SymbolType::State {
                    for state in symtabs_marcher.state_hierarchy(&parent_path).skip(1) {
                        let base_func_path = state.path().join_component(func_name, SymbolCategory::Callable);
                        if let Some(base_func_loc) = symtabs_marcher.locate(&base_func_path) {
                            *loc = base_func_loc;
                        }
                    }

                    let base_func_path = BasicTypeSymbolPath::new(StateSymbol::DEFAULT_STATE_BASE_NAME).join_component(func_name, SymbolCategory::Callable);
                    if let Some(base_func_loc) = symtabs_marcher.locate(&base_func_path) {
                        *loc = base_func_loc;
                    }
                }
            }
        }

        let target_uri = loc.as_ref()
            .map(|loc| loc.abs_source_path.to_uri())
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


pub async fn goto_type_definition(backend: &Backend, params: lsp::request::GotoTypeDefinitionParams) -> Result<Option<lsp::request::GotoTypeDefinitionResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

    if doc_path.extension().unwrap_or_default() != "ws" {
        return Ok(None);
    }

    let content_path;
    if let Some(path) = backend.source_trees.containing_content_path(&doc_path) {
        content_path = path;
    } 
    else {
        backend.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
        return Ok(None);
    }

    if let Some(inspected) = inspect_symbol_at_position(backend, &content_path, &doc_path, params.text_document_position_params.position).await {
        if inspected.symvar.as_ref().map(|symvar| symvar.typ().category() != SymbolCategory::Type).unwrap_or(false) {
            return Ok(None);
        }

        let origin_selection_range = Some(inspected.origin_selection_range);

        let target_uri = inspected
            .loc.as_ref()
            .map(|loc| loc.abs_source_path.to_uri())
            .unwrap_or(params.text_document_position_params.text_document.uri.clone());

        let target_range = inspected
            .loc.as_ref()
            .map(|loc| loc.range)
            .unwrap_or(inspected.origin_selection_range);

        let target_selection_range = inspected
            .loc.as_ref()
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



struct Inspected {
    origin_selection_range: lsp::Range,
    symvar: Option<SymbolVariant>,
    loc: Option<SymbolLocation>
}

async fn inspect_symbol_at_position(backend: &Backend, content_path: &AbsPath, doc_path: &AbsPath, position: lsp::Position) -> Option<Inspected> {
    let symtabs = backend.symtabs.read().await;
    let symtabs_marcher = backend.march_symbol_tables(&symtabs, &content_path).await;
    
    let script_state = backend.scripts.get(doc_path)?;

    let position_target = resolve_text_document_position(position, &script_state, symtabs_marcher.clone())?;
    drop(script_state);

    
    let sympath: Option<SymbolPathBuf> = match position_target.kind {
        PositionTargetKind::ArrayTypeIdentifier => {
            None
        },
        PositionTargetKind::TypeIdentifier(type_name) => {
            Some(BasicTypeSymbolPath::new(&type_name).into())
        },
        PositionTargetKind::StateDeclarationNameIdentifier => {
            Some(position_target.sympath_ctx)
        },
        PositionTargetKind::StateDeclarationBaseIdentifier => {
            let mut state_base_path = None;

            if let Some(target_state_sym) = symtabs_marcher.get(&position_target.sympath_ctx).and_then(|v| v.try_as_state_ref()) {
                let base_state_name = target_state_sym.base_state_name.as_ref().map(|s| s.as_str()).unwrap_or_default();

                'ancestors: for class in symtabs_marcher.class_hierarchy(target_state_sym.parent_class_path()) {
                    for state in symtabs_marcher.class_states(class.path()) {
                        if state.state_name() == base_state_name {
                            state_base_path = Some(state.path().to_owned());
                            break 'ancestors;
                        }
                    }
                }
            }
            
            state_base_path
        },
        PositionTargetKind::DataDeclarationNameIdentifier(name) => {
            if let Some(ctx_sym) = symtabs_marcher.get(&position_target.sympath_ctx) {
                if ctx_sym.is_enum() {
                    Some(GlobalDataSymbolPath::new(&name).into())
                } else {
                    Some(MemberDataSymbolPath::new(&position_target.sympath_ctx, &name).into())
                }
            } else {
                None
            }
        },
        PositionTargetKind::CallableDeclarationNameIdentifier => {
            Some(position_target.sympath_ctx)
        },
        PositionTargetKind::ThisKeyword => {
            symtabs_marcher
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::This))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::SuperKeyword => {
            symtabs_marcher
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::Super))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::ParentKeyword => {
            symtabs_marcher
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::Parent))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::VirtualParentKeyword => {
            symtabs_marcher
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::VirtualParent))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::ExpressionIdentifier(expr_type_path) => {
            Some(expr_type_path)
        }
    };

    let symvar = sympath
        .and_then(|sympath| symtabs_marcher.get(&sympath))
        .and_then(|symvar| {
            let rerouted_path = match symvar {
                SymbolVariant::Constructor(s) => Some(s.parent_type_path.as_sympath()),
                SymbolVariant::GlobalVar(s) => Some(s.type_path().as_sympath()),
                SymbolVariant::SpecialVar(s) => Some(s.type_path().as_sympath()),
                _ => None
            };

            if let Some(rerouted_path) = rerouted_path {
                symtabs_marcher.get(rerouted_path)
            } else {
                Some(symvar)
            }
        })
        .map(|symvar| symvar.to_owned());

    let loc = symvar.as_ref().and_then(|symvar| symtabs_marcher.locate(symvar.path()));

    Some(Inspected {
        origin_selection_range: position_target.range,
        symvar,
        loc
    })
}

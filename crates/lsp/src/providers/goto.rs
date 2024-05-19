use std::str::FromStr;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::ast::SyntaxNodeVisitorChain;
use witcherscript::tokens::Keyword;
use witcherscript_analysis::symbol_analysis::symbol_table::{SymbolLocation, marcher::IntoSymbolTableMarcher};
use witcherscript_analysis::symbol_analysis::symbol_path::SymbolPathBuf;
use witcherscript_analysis::symbol_analysis::symbols::*;
use witcherscript_analysis::utils::{PositionSeeker, SymbolPathBuilder};
use crate::{providers::common::PositionTargetKind, Backend, ScriptState, messaging::notifications};
use super::common::{PositionTarget, TextDocumentPositionResolver};



pub async fn goto_definition(backend: &Backend, params: lsp::GotoDefinitionParams) -> Result<Option<lsp::GotoDefinitionResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

    let content_path;
    if let Some(path) = backend.source_trees.containing_content_path(&doc_path) {
        content_path = path;
    } 
    else {
        backend.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
        return Ok(None);
    }

    if let Some(inspected) = inspect_symbol_at_position(backend, &content_path, &doc_path, params.text_document_position_params.position).await {
        Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
            lsp::LocationLink {
                origin_selection_range: Some(inspected.origin_selection_range),
                target_uri: inspected.loc.as_ref().map(|loc| loc.abs_source_path.to_uri()).unwrap_or(params.text_document_position_params.text_document.uri.clone()),
                target_range: inspected.loc.as_ref().map(|loc| loc.range).unwrap_or(inspected.origin_selection_range),
                target_selection_range: inspected.loc.as_ref().map(|loc| loc.label_range).unwrap_or(inspected.origin_selection_range)
            }
        ])))
    } else {
        Ok(None)
    }
}


pub async fn goto_declaration(backend: &Backend, params: lsp::request::GotoDeclarationParams) -> Result<Option<lsp::request::GotoDeclarationResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

    let content_path;
    if let Some(path) = backend.source_trees.containing_content_path(&doc_path) {
        content_path = path;
    } 
    else {
        backend.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
        return Ok(None);
    }

    if let Some(inspected) = inspect_symbol_at_position(backend, &content_path, &doc_path, params.text_document_position_params.position).await {
        Ok(Some(lsp::request::GotoDeclarationResponse::Link(vec![
            lsp::LocationLink {
                origin_selection_range: Some(inspected.origin_selection_range),
                target_uri: inspected.loc.as_ref().map(|loc| loc.abs_source_path.to_uri()).unwrap_or(params.text_document_position_params.text_document.uri.clone()),
                target_range: inspected.loc.as_ref().map(|loc| loc.range).unwrap_or(inspected.origin_selection_range),
                target_selection_range: inspected.loc.as_ref().map(|loc| loc.label_range).unwrap_or(inspected.origin_selection_range)
            }
        ])))
    } else {
        Ok(None)
    }
}


pub async fn goto_type_definition(backend: &Backend, params: lsp::request::GotoTypeDefinitionParams) -> Result<Option<lsp::request::GotoTypeDefinitionResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

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

        Ok(Some(lsp::request::GotoTypeDefinitionResponse::Link(vec![
            lsp::LocationLink {
                origin_selection_range: Some(inspected.origin_selection_range),
                target_uri: inspected.loc.as_ref().map(|loc| loc.abs_source_path.to_uri()).unwrap_or(params.text_document_position_params.text_document.uri.clone()),
                target_range: inspected.loc.as_ref().map(|loc| loc.range).unwrap_or(inspected.origin_selection_range),
                target_selection_range: inspected.loc.as_ref().map(|loc| loc.label_range).unwrap_or(inspected.origin_selection_range)
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
    let script_state = backend.scripts.get(doc_path)?;
    let position_target = resolve_position(position, &script_state)?;

    let content_dependency_paths: Vec<_> = 
        [content_path.clone()].into_iter()
        .chain(backend.content_graph
            .read().await
            .walk_dependencies(&content_path)
            .map(|n| n.content.path().to_owned()))
        .collect();
        
    let symtabs = backend.symtabs.read().await;

    let symtabs_marcher = 
        content_dependency_paths.iter()
        .filter_map(|p| symtabs.get(p))
        .into_marcher();

    
    let sympath: Option<SymbolPathBuf> = match position_target.kind {
        PositionTargetKind::TypeIdentifier(type_name) => {
            Some(BasicTypeSymbolPath::new(&type_name).into())
        },
        PositionTargetKind::StateDeclarationNameIdentifier => {
            Some(position_target.sympath_ctx)
        },
        PositionTargetKind::StateDeclarationBaseIdentifier => {
            let mut state_base_path = None;

            if let Some(target_state_sym) = symtabs_marcher.clone().get(&position_target.sympath_ctx).and_then(|v| v.try_as_state_ref()) {
                let base_state_name = target_state_sym.base_state_name.as_ref().map(|s| s.as_str()).unwrap_or_default();

                'ancestors: for class in symtabs_marcher.clone().class_hierarchy(target_state_sym.parent_class_path()) {
                    for state in symtabs_marcher.clone().class_states(class.path()) {
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
            if let Some(ctx_sym) = symtabs_marcher.clone().get(&position_target.sympath_ctx) {
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
            symtabs_marcher.clone()
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::This))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::SuperKeyword => {
            symtabs_marcher.clone()
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::Super))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::ParentKeyword => {
            symtabs_marcher.clone()
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::Parent))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::VirtualParentKeyword => {
            symtabs_marcher.clone()
                .get(&SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap(), SpecialVarSymbolKind::VirtualParent))
                .and_then(|v| v.try_as_special_var_ref())
                .map(|sym| sym.type_path().clone())
        },
        PositionTargetKind::DataIdentifier(name) => {
            if Keyword::from_str(&name).map(|kw| kw.is_global_var()).unwrap_or(false) {
                symtabs_marcher.clone()
                    .get(&SymbolPathBuf::new(&name, SymbolCategory::Data))
                    .and_then(|v| v.try_as_global_var_ref())
                    .map(|sym| sym.type_path().clone())
            } else {
                // not ready yet
                None
            }
        }
        // other stuff not reliably possible yet
        _ => {
            None
        }
    };

    let (symvar, loc) = sympath
        .and_then(|sympath| symtabs_marcher.get_with_location(&sympath))
        .map(|(symvar, loc)| (symvar.to_owned(), loc))
        .unzip();

    Some(Inspected {
        origin_selection_range: position_target.range,
        symvar,
        loc
    })
}

fn resolve_position(position: lsp::Position, script_state: &ScriptState) -> Option<PositionTarget> {
    let (pos_seeker, pos_seeker_payload) = PositionSeeker::new(position);
    let (sympath_builder, sympath_builder_payload) = SymbolPathBuilder::new(&script_state.buffer);
    let resolver = TextDocumentPositionResolver::new_rc(position, &script_state.buffer, pos_seeker_payload.clone(), sympath_builder_payload.clone());

    let mut chain = SyntaxNodeVisitorChain::new()
        .link(pos_seeker)
        .link(sympath_builder)
        .link_rc(resolver.clone());

    script_state.script.visit_nodes(&mut chain);

    let resolver_ref = resolver.borrow();
    resolver_ref.found_target.clone()
}
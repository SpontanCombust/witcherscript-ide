use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::ast::SyntaxNodeVisitorChain;
use witcherscript_analysis::{model::{collections::{symbol_table::SymbolLocation, IntoSymbolTableMarcher}, symbol_path::SymbolPathBuf, symbol_variant::SymbolVariant, symbols::*}, utils::PositionSeeker};
use crate::{providers::common::PositionTargetKind, Backend, ScriptState};
use super::common::{PositionTarget, TextDocumentPositionResolver};



pub async fn goto_definition(backend: &Backend, params: lsp::GotoDefinitionParams) -> Result<Option<lsp::GotoDefinitionResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

    if let Some(inspected) = inspect_symbol_at_position(backend, &doc_path, params.text_document_position_params.position).await {
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

    if let Some(inspected) = inspect_symbol_at_position(backend, &doc_path, params.text_document_position_params.position).await {
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

    if let Some(inspected) = inspect_symbol_at_position(backend, &doc_path, params.text_document_position_params.position).await {
        if inspected.symvar.as_ref().map(|symvar| symvar.as_dyn().typ().category() != SymbolCategory::Type).unwrap_or(false) {
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

async fn inspect_symbol_at_position(backend: &Backend, doc_path: &AbsPath, position: lsp::Position) -> Option<Inspected> {
    let content_path;
    if let Some(path) = backend.source_trees.containing_content_path(&doc_path) {
        content_path = path;
    } 
    else {
        return None;
    }

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

    let symtabs_iter = 
        content_dependency_paths.iter()
        .filter_map(|p| symtabs.get(p))
        .into_marcher();

    
    let sympath: Option<SymbolPathBuf> = match position_target.kind {
        PositionTargetKind::TypeIdentifier(type_name) => {
            Some(BasicTypeSymbolPath::new(&type_name).into())
        },
        PositionTargetKind::StateIdentifier { state_name, parent_name } => {
            Some(StateSymbolPath::new(&state_name, BasicTypeSymbolPath::new(&parent_name)).into())
        },
        // other stuff not reliably possible yet
        _ => {
            None
        }
    };

    let (symvar, loc) = sympath
        .and_then(|sympath| symtabs_iter.get_with_location(&sympath))
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
    let resolver = TextDocumentPositionResolver::new_rc(position, &script_state.buffer, pos_seeker_payload.clone());

    let mut chain = SyntaxNodeVisitorChain::new()
        .link(pos_seeker)
        .link_rc(resolver.clone());

    script_state.script.visit_nodes(&mut chain);

    let resolver_ref = resolver.borrow();
    resolver_ref.found_target.clone()
}
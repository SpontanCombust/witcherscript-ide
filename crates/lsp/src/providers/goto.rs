use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::ast::SyntaxNodeVisitorChain;
use witcherscript_analysis::utils::PositionSeeker;
use crate::Backend;
use super::common::TextDocumentPositionResolver;



pub async fn goto_definition(backend: &Backend, params: lsp::GotoDefinitionParams) -> Result<Option<lsp::GotoDefinitionResponse>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();
    if let Some(script_state) = backend.scripts.get(&doc_path) {
        let (pos_seeker, payload) = PositionSeeker::new_rc(params.text_document_position_params.position);
        let resolver = TextDocumentPositionResolver::new_rc(params.text_document_position_params.position, payload.clone());

        let mut chain = SyntaxNodeVisitorChain::new()
            .link_rc(pos_seeker.clone())
            .link_rc(resolver.clone());

        script_state.script.visit_nodes(&mut chain);

        let resolver_ref = resolver.borrow();
        if let Some(found) = resolver_ref.found_target.as_ref() {
            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range: Some(found.range),
                    // this is a mock implementation for now
                    // it can at best highlight the origin range
                    // will change it later
                    target_uri: params.text_document_position_params.text_document.uri,
                    target_range: found.range,
                    target_selection_range: found.range
                }
            ])))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}


pub async fn goto_declaration(_backend: &Backend, _params: lsp::request::GotoDeclarationParams) -> Result<Option<lsp::request::GotoDeclarationResponse>> {
    Ok(None)
}


pub async fn goto_type_definition(_backend: &Backend, _params: lsp::request::GotoTypeDefinitionParams) -> Result<Option<lsp::request::GotoTypeDefinitionResponse>> {
    Ok(None)
}

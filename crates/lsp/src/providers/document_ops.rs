use std::{ops::DerefMut, borrow::Borrow};
use tower_lsp::lsp_types as lsp;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::{diagnostics::{Diagnostic, DiagnosticBody}, jobs::syntax_analysis};
use crate::Backend;


// Until the witcherscript_project crate is ready, only scripts visible in the editor will be stored on the server

pub async fn did_open(backend: &Backend, params: lsp::DidOpenTextDocumentParams) {
    if params.text_document.uri.scheme() != "file" {
        backend.client.log_message(lsp::MessageType::ERROR, format!("{} works only on localhost", Backend::SERVER_NAME)).await;
        return;
    }

    if params.text_document.language_id == Backend::LANGUAGE_ID {
        let doc_path = params.text_document.uri.to_file_path().unwrap();
        if !backend.doc_buffers.contains_key(&doc_path) {
            let doc = ScriptDocument::from_str(&params.text_document.text);
            match Script::new(&doc) {
                Ok(script) => {
                    script_syntax_diagnostics(&script, backend, params.text_document.uri.clone()).await;
    
                    backend.doc_buffers.insert(doc_path.clone(), doc);
                    backend.scripts.insert(doc_path, script);
                },
                Err(err) => {
                    backend.client.log_message(lsp::MessageType::ERROR, err.to_string()).await;
                }
            }
        }
    }
}

pub async fn did_change(backend: &Backend, params: lsp::DidChangeTextDocumentParams) {
    if params.text_document.uri.scheme() != "file" {
        backend.client.log_message(lsp::MessageType::ERROR, format!("{} works only on localhost", Backend::SERVER_NAME)).await;
        return;
    }

    let doc_path = params.text_document.uri.to_file_path().unwrap();
    if let Some(mut doc) = backend.doc_buffers.get_mut(&doc_path) {
        for edit in params.content_changes {
            doc.deref_mut().edit(&edit);
        }

        let mut script = backend.scripts.get_mut(&doc_path).unwrap();
        if let Err(err) = script.update(&mut doc) {
            backend.client.log_message(lsp::MessageType::ERROR, err.to_string()).await;
        }

        script_syntax_diagnostics(&*script, backend, params.text_document.uri).await;
    }
}

// pub async fn did_save(backend: &Backend, params: lsp::DidSaveTextDocumentParams) {
    
// }

pub async fn did_close(backend: &Backend, params: lsp::DidCloseTextDocumentParams) {
    if params.text_document.uri.scheme() != "file" {
        backend.client.log_message(lsp::MessageType::ERROR, format!("{} works only on localhost", Backend::SERVER_NAME)).await;
        return;
    }

    // clear errors for the file
    backend.client.publish_diagnostics(params.text_document.uri.clone(), vec![], None).await;

    let doc_path = params.text_document.uri.to_file_path().unwrap();
    backend.doc_buffers.remove(&doc_path);
    backend.scripts.remove(&doc_path);
}


async fn script_syntax_diagnostics<S: Borrow<Script>>(script: S, backend: &Backend, path: lsp::Url) {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    syntax_analysis::syntax_analysis(script.borrow().root_node(), &mut diagnostics);

    let lsp_diags = diagnostics.into_iter()
        .map(|diag| lsp::Diagnostic {
            range: diag.range,
            severity: Some(match diag.body {
                DiagnosticBody::Error(_) => lsp::DiagnosticSeverity::ERROR,
                DiagnosticBody::Warning(_) => lsp::DiagnosticSeverity::WARNING,
                DiagnosticBody::Info(_) => lsp::DiagnosticSeverity::INFORMATION,
            }),
            source: Some(Backend::SERVER_NAME.to_string()),
            message: diag.body.to_string(),
            ..Default::default()
        })
        .collect();

    backend.client.publish_diagnostics(path, lsp_diags, None).await;
}
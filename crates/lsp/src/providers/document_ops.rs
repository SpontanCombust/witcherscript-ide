use std::{borrow::Borrow, ops::DerefMut};
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::{diagnostics::Diagnostic, jobs::syntax_analysis};
use witcherscript_project::Manifest;
use crate::{reporting::IntoLspDiagnostic, Backend};


pub async fn did_open(backend: &Backend, params: lsp::DidOpenTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    let belongs_to_workspace = backend
        .workspace_roots
        .read().await
        .iter()
        .any(|root| doc_path.starts_with(root));

    if params.text_document.language_id == Backend::LANGUAGE_ID {
        let mut in_known_content = false;
        for it in backend.source_trees.iter() {
            let source_tree = it.value();
            if doc_path.starts_with(source_tree.script_root()) {
                in_known_content = true;
                break;
            }
        }

        let doc_buff = backend
            .doc_buffers
            .entry(doc_path.clone())
            .or_insert(ScriptDocument::from_str(&params.text_document.text));

        // If script does not belong to the content graph it can be analyzed only during file operations
        // as project-wide analysis does not take it into account.
        if !in_known_content && !backend.scripts.contains_key(&doc_path) {
            backend.log_info("Opened script file not belonging to any known content").await;
            match Script::new(&doc_buff) {
                Ok(script) => {
                    script_syntax_diagnostics(&script, backend, params.text_document.uri.clone()).await;
                    backend.scripts.insert(doc_path, script);
                },
                Err(err) => {
                    backend.log_error(err).await;
                }
            }
        }
    } else if doc_path.file_name().unwrap() == Manifest::FILE_NAME && belongs_to_workspace {
        let project_is_known = backend
            .content_graph
            .read().await
            .get_workspace_projects()
            .iter()
            .any(|p| p.manifest_path() == &doc_path);

        if !project_is_known {
            backend.log_info("Opened unknown manifest file").await;
            backend.scan_workspace_projects().await;
            backend.build_content_graph().await;
        }
    }
}

pub async fn did_change(backend: &Backend, params: lsp::DidChangeTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if let Some(mut doc) = backend.doc_buffers.get_mut(&doc_path) {
        for edit in params.content_changes {
            doc.deref_mut().edit(&edit);
        }

        if let Some(mut script) = backend.scripts.get_mut(&doc_path) {
            if let Err(err) = script.update(&mut doc) {
                backend.log_error(err).await;
            }

            script_syntax_diagnostics(&*script, backend, params.text_document.uri).await;
        }
    }
}

pub async fn did_save(backend: &Backend, params: lsp::DidSaveTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();

    let belongs_to_workspace = backend
        .workspace_roots
        .read().await
        .iter()
        .any(|root| doc_path.starts_with(root));

    
    if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
        let mut containing_content_path = None;
        for it in backend.source_trees.iter() {
            let content_path = it.key();
            let source_tree = it.value();
            if doc_path.starts_with(source_tree.script_root()) {
                containing_content_path = Some(content_path.to_owned());
                break;
            }
        }

        if let Some(containing_content_path) = containing_content_path {
            backend.scan_source_tree(&containing_content_path).await;
        }


        // replace the doc content completely
        let mut doc_buff = backend
            .doc_buffers
            .entry(doc_path.clone())
            .insert(ScriptDocument::from_str(&params.text.unwrap()));

        if let Some(mut script) = backend.scripts.get_mut(&doc_path) {
            if let Err(err) = script.update(&mut doc_buff) {
                backend.log_error(err).await;
            }

            script_syntax_diagnostics(&*script, backend, params.text_document.uri).await;
        }
        
    } else if doc_path.file_name().unwrap() == Manifest::FILE_NAME && belongs_to_workspace {
        backend.build_content_graph().await;
    }
}

pub async fn did_close(backend: &Backend, params: lsp::DidCloseTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
        let belongs_to_workspace = backend
            .workspace_roots
            .read().await
            .iter()
            .any(|root| doc_path.starts_with(root));
        
        let mut belongs_to_source_tree = false;
        for it in backend.source_trees.iter() {
            if it.value().contains(&doc_path) {
                belongs_to_source_tree = true;
                break;
            }
        }
    
        // script does not belong to the pool of actively monitored scripts, so it can be let go on close
        if !belongs_to_workspace && !belongs_to_source_tree {
            backend.clear_diagnostics(params.text_document.uri.clone()).await;
            backend.scripts.remove(&doc_path);
        }
    
        backend.doc_buffers.remove(&doc_path);
    }
}


async fn script_syntax_diagnostics<S: Borrow<Script>>(script: S, backend: &Backend, url: lsp::Url) {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    syntax_analysis::syntax_analysis(script.borrow().root_node(), &mut diagnostics);

    let lsp_diags = diagnostics.into_iter().map(|diag| diag.into_lsp_diagnostic());
    backend.publish_diagnostics(url, lsp_diags).await;
}
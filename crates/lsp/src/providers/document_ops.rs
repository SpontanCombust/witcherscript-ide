use std::borrow::Borrow;
use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::{diagnostics::Diagnostic, jobs::syntax_analysis};
use witcherscript_project::{content::ProjectDirectory, Manifest};
use crate::{reporting::IntoLspDiagnostic, Backend, ScriptState};


pub async fn did_open(backend: &Backend, params: lsp::DidOpenTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    let belongs_to_workspace = backend
        .workspace_roots
        .read().await
        .iter()
        .any(|root| doc_path.starts_with(root));

    if params.text_document.language_id == Backend::LANGUAGE_ID {
        let doc_buff = ScriptDocument::from_str(&params.text_document.text);

        if let Some(mut script_entry) = backend.scripts.get_mut(&doc_path) {
            script_entry.value_mut().buffer.replace(doc_buff);
        } else {
            backend.reporter.log_info("Opened script file unknown to the content graph").await;

            let script = Script::new(&doc_buff).unwrap();
            script_syntax_diagnostics(&script, backend, &doc_path);

            backend.scripts.insert(doc_path, ScriptState {
                buffer: Some(doc_buff),
                script,
                is_foreign: true
            });
        }
    } else if doc_path.file_name().unwrap() == Manifest::FILE_NAME && belongs_to_workspace {
        let project_is_known = backend
            .content_graph
            .read().await
            .nodes()
            .filter_map(|n| n.content.as_any().downcast_ref::<ProjectDirectory>())
            .any(|p| p.manifest_path() == &doc_path);

        if !project_is_known {
            backend.reporter.log_info("Opened unknown manifest file").await;
            backend.build_content_graph().await;
        }
    }

    backend.reporter.commit_all_diagnostics().await;
}

pub async fn did_change(backend: &Backend, params: lsp::DidChangeTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if let Some(mut entry) = backend.scripts.get_mut(&doc_path) {
        let script_state = entry.value_mut();

        if let Some(buf) = &mut script_state.buffer {
            for edit in params.content_changes {
                buf.edit(&edit);
            }

            if let Err(err) = script_state.script.update(buf) {
                backend.reporter.log_error(err).await;
            }
    
            script_syntax_diagnostics(&script_state.script, backend, &doc_path);
            backend.reporter.commit_diagnostics(&doc_path).await;
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

        if let Some(mut entry) = backend.scripts.get_mut(&doc_path) {
            let script_state = entry.value_mut();

            let doc_buff = ScriptDocument::from_str(&params.text.unwrap());

            // do a fresh reparse without caring about the previous state 
            // as a fail-safe in case of bad edits or document being changed outside of the editor
            if let Err(err) = script_state.script.refresh(&doc_buff) {
                backend.reporter.log_error(err).await;
            }

            script_syntax_diagnostics(&script_state.script, backend, &doc_path);
        }        
    } else if doc_path.file_name().unwrap() == Manifest::FILE_NAME && belongs_to_workspace {
        backend.build_content_graph().await;
    }

    backend.reporter.commit_all_diagnostics().await;
}

pub async fn did_close(backend: &Backend, params: lsp::DidCloseTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
        let mut should_remove_script = false;
        if let Some(mut entry) = backend.scripts.get_mut(&doc_path) {
            let script_state = entry.value_mut();

            if script_state.is_foreign {
                backend.reporter.purge_diagnostics(&doc_path);
                backend.reporter.commit_diagnostics(&doc_path).await;
                should_remove_script = true;
            } else {
                script_state.buffer = None;
            }
        }

        if should_remove_script {
            backend.scripts.remove(&doc_path);
        }
    }
}


fn script_syntax_diagnostics<S: Borrow<Script>>(script: S, backend: &Backend, path: &AbsPath) {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    syntax_analysis::syntax_analysis(script.borrow().root_node(), &mut diagnostics);

    backend.reporter.clear_diagnostics(path);
    backend.reporter.push_diagnostics(path, diagnostics.into_iter().map(|d| d.into_lsp_diagnostic()));
}
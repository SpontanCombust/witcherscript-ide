use filetime::FileTime;
use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_project::{redkit, Manifest};
use crate::{Backend, ScriptState};


pub async fn did_open(backend: &Backend, params: lsp::DidOpenTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if params.text_document.language_id == Backend::LANGUAGE_ID {
        if !backend.scripts.contains_key(&doc_path) {
            // Scripts that are not a part of a workspace projects or their dependencies
            // are not included in content source trees and thus knowledge about them is limited.
            // Only a limited processing can be performed on them, such that only requires the context 
            // of a given isolated script.

            backend.reporter.log_info("Opened script file unknown to the content graph").await;
            
            let doc_buff = ScriptDocument::from_str(&params.text_document.text);
            let script = Script::new(&doc_buff).unwrap();
            backend.scripts.insert(doc_path.clone(), ScriptState {
                buffer: doc_buff,
                script,
                modified_timestamp: FileTime::now(),
                source_tree_path: None
            });

            backend.run_script_analysis([doc_path.clone()]).await;
            backend.reporter.commit_diagnostics(&doc_path).await;
        }
    }
}

pub async fn did_change(backend: &Backend, params: lsp::DidChangeTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();

    let mut should_notify = false;
    let mut source_path = None;
    if let Some(mut entry) = backend.scripts.get_mut(&doc_path) {
        let script_state = entry.value_mut();

        for edit in params.content_changes {
            script_state.buffer.edit(&edit);
        }

        script_state.script.update(&mut script_state.buffer).expect("Script update error!");

        script_state.modified_timestamp = FileTime::now();

        source_path = script_state.source_tree_path.clone();
        should_notify = true;
    } 

    if should_notify {
        if let (Some(source_path), Some(content_path)) = (source_path, backend.source_trees.containing_content_path(&doc_path)) {
            if let Ok(mut symtabs) = backend.symtabs.try_write() {
                let mut symtab = symtabs.get_mut(&content_path).unwrap();
                backend.scan_symbols(&mut symtab, &content_path, vec![source_path]).await;
            }
        }

        backend.run_script_analysis([doc_path.clone()]).await;
        backend.reporter.commit_diagnostics(&doc_path).await;
    }
}

// Not all circumstances can be easily handled or even detected.
// For such cases the act of saving a script file or manifest is used as a trigger
// for refreshing source trees or even entire graphs to make sure the IDE is up-to-date.
pub async fn did_save(backend: &Backend, params: lsp::DidSaveTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();

    let belongs_to_workspace = backend
        .workspace_roots
        .read().await
        .iter()
        .any(|root| doc_path.starts_with(root));

    
    if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
        if let Some(mut entry) = backend.scripts.get_mut(&doc_path) {
            let script_state = entry.value_mut();

            let doc_buff = ScriptDocument::from_str(&params.text.unwrap());

            // Do a fresh reparse without caring about the previous state.
            // This is a fail-safe in case of bad edits or document having been changed outside of the editor.
            script_state.script.refresh(&doc_buff).expect("Script refresh error!");

            script_state.modified_timestamp = FileTime::now();
        }

        if let Some(containing_content_path) = backend.source_trees.containing_content_path(&doc_path) {
            backend.scan_source_tree(&containing_content_path).await;
        }
    } else if (doc_path.file_name().unwrap() == Manifest::FILE_NAME || doc_path.extension().unwrap() == redkit::RedkitManifest::EXTENSION) && belongs_to_workspace {
        // try rebuilding the graph but only if it's not already being rebuilt
        if let Ok(mut content_graph) = backend.content_graph.try_write() {
            backend.build_content_graph(&mut content_graph).await;
        }
    }

    backend.reporter.commit_all_diagnostics().await;
}

pub async fn did_close(backend: &Backend, params: lsp::DidCloseTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
        let mut should_remove_script = false;
        if backend.scripts.get(&doc_path).map(|s| s.source_tree_path.is_none()).unwrap_or(false) {
            backend.reporter.purge_diagnostics(&doc_path).await;
            backend.reporter.commit_diagnostics(&doc_path).await;
            should_remove_script = true;
        }

        if should_remove_script {
            backend.scripts.remove(&doc_path);
        }
    }
}
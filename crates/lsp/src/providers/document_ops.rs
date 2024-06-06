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
                content_info: None
            });

            backend.run_script_analysis(vec![doc_path.clone()], true).await;
            backend.reporter.commit_diagnostics(&doc_path).await;
        }
    }
}

pub async fn did_change(backend: &Backend, params: lsp::DidChangeTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();

    let mut should_notify = false;
    let mut content_info = None;
    if let Some(mut entry) = backend.scripts.get_mut(&doc_path) {
        let script_state = entry.value_mut();

        for edit in params.content_changes {
            script_state.buffer.edit(&edit);
        }

        script_state.script.update(&mut script_state.buffer).expect("Script update error!");

        script_state.modified_timestamp = FileTime::now();

        content_info = script_state.content_info.clone();
        should_notify = true;
    } 

    if should_notify {
        if let Some(content_info) = content_info {
            let content_path = content_info.content_path;
            let source_path = content_info.source_tree_path;
            backend.scan_symbols(&content_path, vec![source_path], false).await;
        }

        backend.run_script_analysis(vec![doc_path.clone()], false).await;
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
        if let Some(text) = params.text {
            if let Some(mut entry) = backend.scripts.get_mut(&doc_path) {
                let script_state = entry.value_mut();

                // Do a fresh reparse without caring about the previous state.
                // This is a fail-safe in case of bad edits or document having been changed outside of the editor.
                script_state.buffer.replace(&text);
                script_state.script.refresh(&mut script_state.buffer).expect("Script refresh error!");
                script_state.modified_timestamp = FileTime::now();
            }
        }

        // a fail-safe for situations when the script isn't known to source trees yet
        let mut containing_content_path = None;
        for kv in backend.source_trees.iter() {
            let content_path = kv.key();
            let source_tree = kv.value();
    
            if doc_path.starts_with(source_tree.script_root()) {
                containing_content_path = Some(content_path.to_owned());
                break;
            }
        }

        if let Some(containing_content_path) = containing_content_path {
            backend.scan_source_tree(&containing_content_path).await;
        }
    } else if (doc_path.file_name().unwrap() == Manifest::FILE_NAME || doc_path.extension().unwrap() == redkit::RedkitManifest::EXTENSION) && belongs_to_workspace {
        backend.build_content_graph(false).await;
    }

    backend.reporter.commit_all_diagnostics().await;
}

pub async fn did_close(backend: &Backend, params: lsp::DidCloseTextDocumentParams) {
    let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
        let mut should_remove_script = false;
        if backend.scripts.get(&doc_path).map(|s| s.content_info.is_none()).unwrap_or(false) {
            backend.reporter.purge_diagnostics(&doc_path);
            backend.reporter.commit_diagnostics(&doc_path).await;
            should_remove_script = true;
        }

        if should_remove_script {
            backend.scripts.remove(&doc_path);
        }
    }
}
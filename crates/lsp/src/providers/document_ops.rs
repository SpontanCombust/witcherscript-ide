use std::collections::HashSet;
use filetime::FileTime;
use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_project::{redkit::RedkitManifest, Manifest};
use crate::{Backend, ScriptState};


impl Backend {
    pub async fn did_open_impl(&self, params: lsp::DidOpenTextDocumentParams) {
        let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
        if params.text_document.language_id == Backend::LANGUAGE_ID {
            if !self.scripts.contains_key(&doc_path) {
                // Scripts that are not a part of a workspace projects or their dependencies
                // are not included in content source trees and thus knowledge about them is limited.
                // Only a limited processing can be performed on them, such that only requires the context 
                // of a given isolated script.
    
                self.reporter.log_info("Opened script file unknown to the content graph").await;
                
                let doc_buff = ScriptDocument::from_str(&params.text_document.text);
                let script = Script::new(&doc_buff).unwrap();
                self.scripts.insert(doc_path.clone(), ScriptState {
                    buffer: doc_buff,
                    script,
                    modified_timestamp: FileTime::now(),
                    content_info: None
                });
    
                self.run_script_analysis(vec![doc_path.clone()], true).await;
                self.reporter.commit_diagnostics(&doc_path).await;
            }
        }
    }
    
    pub async fn did_change_impl(&self, params: lsp::DidChangeTextDocumentParams) {
        let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    
        let mut should_notify = false;
        let mut content_info = None;
        if let Some(mut entry) = self.scripts.get_mut(&doc_path) {
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
                self.scan_symbols(&content_path, vec![source_path], false).await;
            }
    
            self.run_script_analysis(vec![doc_path.clone()], false).await;
            self.reporter.commit_diagnostics(&doc_path).await;
        }
    }
    
    // Not all circumstances can be easily handled or even detected.
    // For such cases the act of saving a script file or manifest is used as a trigger
    // for refreshing source trees or even entire graphs to make sure the IDE is up-to-date.
    pub async fn did_save_impl(&self, params: lsp::DidSaveTextDocumentParams) {
        let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
    
        let belongs_to_workspace = self
            .workspace_roots
            .read().await
            .iter()
            .any(|root| doc_path.starts_with(root));
    
        
        if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
            if let Some(text) = params.text {
                if let Some(mut entry) = self.scripts.get_mut(&doc_path) {
                    let script_state = entry.value_mut();
    
                    // Do a fresh reparse without caring about the previous state.
                    // This is a fail-safe in case of bad edits or document having been changed outside of the editor.
                    script_state.buffer.replace(&text);
                    script_state.script.refresh(&mut script_state.buffer).expect("Script refresh error!");
                    script_state.modified_timestamp = FileTime::now();
                }
            }
    
            // a fail-safe for situations when the script isn't known to source trees yet
            if let Some(containing_content_path) = self.content_graph.read().await.strip_content_path_prefix(&doc_path) {
                self.scan_source_tree(&containing_content_path).await;
            }
        } else if (doc_path.file_name().unwrap() == Manifest::FILE_NAME || doc_path.extension().unwrap() == RedkitManifest::EXTENSION) && belongs_to_workspace {
            self.build_content_graph(false).await;
        }
    
        self.reporter.commit_all_diagnostics().await;
    }
    
    pub async fn did_close_impl(&self, params: lsp::DidCloseTextDocumentParams) {
        let doc_path = AbsPath::try_from(params.text_document.uri.clone()).unwrap();
        if doc_path.extension().map(|ext| ext == "ws").unwrap_or(false) {
            let mut should_remove_script = false;
            if self.scripts.get(&doc_path).map(|s| s.content_info.is_none()).unwrap_or(false) {
                self.reporter.purge_diagnostics(&doc_path);
                self.reporter.commit_diagnostics(&doc_path).await;
                should_remove_script = true;
            }
    
            if should_remove_script {
                self.scripts.remove(&doc_path);
            }
        }
    }
    
    pub async fn did_create_files_impl(&self, params: lsp::CreateFilesParams) {
        let paths: Vec<AbsPath> = 
            params.files.into_iter()
            .map(|f| f.uri)
            .filter_map(|uri_str| lsp::Url::parse(&uri_str).ok())
            .filter_map(|uri| AbsPath::try_from(uri).ok())
            .collect();
    
        let mut contents_to_update = HashSet::new();
        for p in &paths {
            if p.extension().unwrap_or_default() == "ws" || p.is_dir() {
                if let Some(content_path) = self.content_graph.read().await.strip_content_path_prefix(&p) {
                    contents_to_update.insert(content_path);
                }
            }
        }
    
        for content_path in contents_to_update {
            self.scan_source_tree(&content_path).await;
        }
    
        self.reporter.commit_all_diagnostics().await;
    }
    
    pub async fn did_delete_files_impl(&self, params: lsp::DeleteFilesParams) {
        let paths: Vec<AbsPath> = 
            params.files.into_iter()
            .map(|f| f.uri)
            .filter_map(|uri_str| lsp::Url::parse(&uri_str).ok())
            .filter_map(|uri| AbsPath::try_from(uri).ok())
            .collect();
    
        let mut contents_to_update = HashSet::new();
        let mut contents_to_be_deleted = HashSet::new();
        for p in &paths {
            if p.extension().unwrap_or_default() == "ws" || p.is_dir() {
                if let Some(content_path) = self.content_graph.read().await.strip_content_path_prefix(&p) {
                    contents_to_update.insert(content_path);
                }
            }
            else if p.file_name().unwrap_or_default() == Manifest::FILE_NAME || p.extension().unwrap_or_default() == RedkitManifest::EXTENSION {
                if let Some(content_path) = self.content_graph.read().await.strip_content_path_prefix(&p) {
                    contents_to_be_deleted.insert(content_path);
                }
            }
        }
    
        for p in &contents_to_be_deleted {
            contents_to_update.remove(p);
        }
    
        for content_path in contents_to_update {
            self.scan_source_tree(&content_path).await;
        }
    
        if !contents_to_be_deleted.is_empty() {
            self.build_content_graph(false).await;
        }
    
        self.reporter.commit_all_diagnostics().await;
    }
    
    pub async fn did_rename_files_impl(&self, params: lsp::RenameFilesParams) {
        let paths: Vec<AbsPath> = 
            params.files.into_iter()
            .map(|f| f.old_uri)
            .filter_map(|uri_str| lsp::Url::parse(&uri_str).ok())
            .filter_map(|uri| AbsPath::try_from(uri).ok())
            .collect();
    
        let mut contents_to_update = HashSet::new();
        for p in &paths {
            if p.extension().unwrap_or_default() == "ws" || p.is_dir() {
                if let Some(content_path) = self.content_graph.read().await.strip_content_path_prefix(&p) {
                    contents_to_update.insert(content_path);
                }
            }
        }
    
        for content_path in contents_to_update {
            self.scan_source_tree(&content_path).await;
        }
    
        self.reporter.commit_all_diagnostics().await;
    }
}

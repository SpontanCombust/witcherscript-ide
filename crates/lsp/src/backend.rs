use std::{collections::HashMap, sync::Arc};
use dashmap::DashMap;
use filetime::FileTime;
use shrinkwraprs::Shrinkwrap;
use tokio::sync::RwLock;
use tower_lsp::Client;
use abs_path::AbsPath;
use witcherscript::{script_document::ScriptDocument, Script};
use witcherscript_analysis::symbol_analysis::symbol_table::{marcher::SymbolTableMarcher, SymbolTable};
use witcherscript_project::{ContentGraph, SourceTree, SourceTreeFile, SourceTreePath};
use crate::{config::Config, reporting::Reporter};



#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub config: RwLock<Config>,
    pub workspace_roots: RwLock<Vec<AbsPath>>,
    pub reporter: Reporter,

    pub content_graph: RwLock<ContentGraph>,
    pub source_trees: SourceTreeMap,
    // key is path to the file
    pub scripts: Arc<ScriptStates>,
    pub symtabs: RwLock<SymbolTables>,
}

#[derive(Debug, Shrinkwrap)]
pub struct SourceTreeMap {
    // key is path to content directory
    inner: DashMap<AbsPath, SourceTree>
}

impl SourceTreeMap {
    fn new() -> Self {
        Self {
            inner: DashMap::new()
        }
    }

    /// Returns an absolute path to the content owning a source file at a given path
    pub fn containing_content_path(&self, source_path: &AbsPath) -> Option<AbsPath> {
        for it in self.inner.iter() {
            let content_path = it.key();
            let source_tree = it.value();
            if source_path.starts_with(source_tree.script_root()) {
                return Some(content_path.to_owned());
            }
        }

        None
    }

    /// Returns an absolute path to the content owning a source file at a given path and the source tree file object associated with the source path
    pub fn find_source_file(&self, source_path: &AbsPath) -> Option<(AbsPath, SourceTreeFile)> {
        for it in self.inner.iter() {
            let content_path = it.key();
            let source_tree = it.value();
            if let Some(source) = source_tree.find(source_path) {
                return Some((content_path.to_owned(), source.to_owned()))
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct ScriptState {
    pub script: Script,
    pub buffer: ScriptDocument,
    /// Timestamp for the modification of the script and not necessairly the file,
    /// i.e. the timestamp will update with `did_change` notification even if the file itself has not been saved yet.
    pub modified_timestamp: FileTime,
    /// If None it means the script is foreign, i.e. not known to any content in the content graph
    pub source_tree_path: Option<SourceTreePath>
}

#[derive(Debug, Shrinkwrap)]
pub struct ScriptStates {
    inner: DashMap<AbsPath, ScriptState>
}

impl ScriptStates {
    fn new() -> Self {
        Self {
            inner: DashMap::new()
        }
    }
}

#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SymbolTables {
    // key is path to content directory
    pub inner: HashMap<AbsPath, SymbolTable>
}

impl SymbolTables {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }
}

impl Backend {
    pub const LANGUAGE_ID: &'static str = "witcherscript";
    pub const SERVER_NAME: &'static str = "witcherscript-ide";

    pub fn new(client: Client) -> Self {
        Self {
            config: RwLock::new(Config::default()),
            workspace_roots: RwLock::new(Vec::new()),
            reporter: Reporter::new(client.clone()),
            client,

            content_graph: RwLock::new(ContentGraph::new()),
            source_trees: SourceTreeMap::new(),
            scripts: Arc::new(ScriptStates::new()),
            symtabs: RwLock::new(SymbolTables::new())
        }
    }


    pub async fn march_symbol_tables<'a>(&self, symtabs: &'a SymbolTables, content_path: &AbsPath) -> SymbolTableMarcher<'a> {
        let dependency_paths: Vec<_> =
            self.content_graph
            .read().await
            .walk_dependencies(&content_path)
            .map(|n| n.content.path().to_owned())
            .collect();

        let source_mask_iter =
            [content_path].into_iter()
            .chain(dependency_paths.iter())
            .filter_map(|p| self.source_trees.get(p).map(|kv| kv.value().source_mask()));

        let symtab_iter =
            [content_path].into_iter()
            .chain(dependency_paths.iter())
            .filter_map(|p| symtabs.get(p));
        
        let mut marcher = SymbolTableMarcher::new();
        for (symtab, source_mask) in symtab_iter.zip(source_mask_iter) {
            marcher.add_step(symtab, source_mask);
        }

        marcher
    }
}

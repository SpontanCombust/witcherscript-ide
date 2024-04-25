use std::{collections::HashSet, path::{Path, PathBuf}};
use lsp_types as lsp;
use witcherscript::attribs::StructSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StructSymbol {
    path: BasicTypeSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    pub specifiers: HashSet<StructSpecifier>
}

impl Symbol for StructSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Struct
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimarySymbol for StructSymbol {
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl LocatableSymbol for StructSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl StructSymbol {
    pub fn new(path: BasicTypeSymbolPath, local_source_path: PathBuf, range: lsp::Range) -> Self {
        Self {
            path,
            range,
            local_source_path,
            specifiers: HashSet::new()
        }
    }
}
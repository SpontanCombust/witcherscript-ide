use std::{collections::HashSet, path::{Path, PathBuf}};
use lsp_types as lsp;
use witcherscript::attribs::*;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct GlobalFunctionSymbol {
    path: GlobalCallableSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    pub specifiers: HashSet<GlobalFunctionSpecifier>,
    pub flavour: Option<GlobalFunctionFlavour>,
    pub return_type_path: TypeSymbolPath
}

impl Symbol for GlobalFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimarySymbol for GlobalFunctionSymbol {
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl LocatableSymbol for GlobalFunctionSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl GlobalFunctionSymbol {
    pub fn new(path: GlobalCallableSymbolPath, local_source_path: PathBuf, range: lsp::Range) -> Self {
        Self {
            path,
            local_source_path,
            range,
            specifiers: HashSet::new(),
            flavour: None,
            return_type_path: TypeSymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct MemberFunctionSymbol {
    path: MemberCallableSymbolPath,
    range: lsp::Range,
    pub specifiers: HashSet<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub return_type_path: TypeSymbolPath
}

impl Symbol for MemberFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for MemberFunctionSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl MemberFunctionSymbol {
    pub fn new(path: MemberCallableSymbolPath, range: lsp::Range) -> Self {
        Self {
            path,
            range,
            specifiers: HashSet::new(),
            flavour: None,
            return_type_path: TypeSymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct EventSymbol {
    path: MemberCallableSymbolPath,
    range: lsp::Range
}

impl Symbol for EventSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Event
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for EventSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl EventSymbol {
    pub fn new(path: MemberCallableSymbolPath, range: lsp::Range) -> Self {
        Self {
            path,
            range
        }
    }
}

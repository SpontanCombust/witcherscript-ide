use lsp_types as lsp;
use witcherscript::attribs::MemberVarSpecifier;
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


#[derive(Debug, Clone)]
pub struct MemberVarSymbol {
    path: MemberDataSymbolPath,
    range: lsp::Range,
    label_range: lsp::Range,
    pub specifiers: SpecifierBitmask<MemberVarSpecifier>,
    pub type_path: TypeSymbolPath,
    pub ordinal: usize // used in the context of struct constructors
}

impl Symbol for MemberVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for MemberVarSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl MemberVarSymbol {
    pub fn new(path: MemberDataSymbolPath, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            range,
            label_range,
            specifiers: SpecifierBitmask::new(),
            type_path: TypeSymbolPath::empty(),
            ordinal: 0
        }
    }
}



#[derive(Debug, Clone)]
pub struct LocalVarSymbol {
    path: MemberDataSymbolPath,
    range: lsp::Range,
    label_range: lsp::Range,
    pub type_path: TypeSymbolPath,
}

impl Symbol for LocalVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::LocalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for LocalVarSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl LocalVarSymbol {
    pub fn new(path: MemberDataSymbolPath, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            range,
            label_range,
            type_path: TypeSymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct GlobalVarSymbol {
    path: SymbolPathBuf,
    type_path: BasicTypeSymbolPath
}

impl Symbol for GlobalVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl GlobalVarSymbol {
    // there is a fixed amount of predefined globals, so a standard 'new' is not required
    pub fn new(name: &str, type_path: BasicTypeSymbolPath) -> Self {
        Self {
            path: SymbolPathBuf::new(name, SymbolCategory::Data),
            type_path
        }
    }

    pub fn type_path(&self) -> &SymbolPathBuf {
        &self.type_path
    }
}



#[derive(Debug, Clone)]
pub struct SpecialVarSymbol {
    path: SpecialVarSymbolPath,
    type_path: BasicTypeSymbolPath
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialVarSymbolKind {
    This,
    Super,
    Parent,
    VirtualParent
}

impl Symbol for SpecialVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl SpecialVarSymbol {
    pub fn new(path: SpecialVarSymbolPath, type_path: BasicTypeSymbolPath) -> Self {
        Self {
            path,
            type_path
        }
    }

    pub fn type_path(&self) -> &SymbolPathBuf {
        &self.type_path
    }

    pub fn kind(&self) -> SpecialVarSymbolKind {
        self.path.kind
    }
}

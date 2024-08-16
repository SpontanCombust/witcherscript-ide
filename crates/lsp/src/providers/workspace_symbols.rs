use std::collections::HashMap;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use rayon::prelude::*;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;

use abs_path::AbsPath;
use witcherscript_analysis::symbol_analysis::symbols::*;

use crate::Backend;


impl Backend {
    pub async fn symbol_impl(&self, params: lsp::WorkspaceSymbolParams) -> Result<Option<Vec<lsp::SymbolInformation>>> {
        if self.cache.workspace_symbols.read().await.should_refresh {
            self.refresh_workspace_symbols_cache().await;
        }

        let queried_data = if !params.query.is_empty() {
            let matcher = SkimMatcherV2::default();

            let mut qd = self.cache.workspace_symbols
                .read().await
                .unqueried_data.par_iter()
                .filter_map(|sd| {
                    matcher.fuzzy_match(&sd.id_string, &params.query)
                        .map(|rank| {
                            let mut cloned = sd.clone();
                            cloned.rank = Some(rank);
                            cloned
                        })
                })
                .collect::<Vec<_>>();

            // sort in descending order
            qd.par_sort_unstable_by(|a, b| b.cmp(a));

            qd
        } else {
            self.cache.workspace_symbols
                .read().await
                .unqueried_data
                .clone()
        };

        let ret: Vec<_> = 
            queried_data.into_iter()
            .map(|sd| sd.sym_info)
            .collect();

        Ok(Some(ret))
    }

    async fn refresh_workspace_symbols_cache(&self) {
        let only_from_workspace = !self.config.read().await.extended_search_for_workspace_symbols;

        let content_names: HashMap<AbsPath, String> =
            self.content_graph
            .read().await
            .nodes()
            .filter(|n| if only_from_workspace { n.in_workspace } else { true })
            .map(|n| (n.content.path().to_owned(), n.content.content_name().to_string()))
            .collect();
        
        self.cache.workspace_symbols
            .write().await
            .unqueried_data
            .clear();

        let symtabs = self.symtabs.read().await;
        
        for (content_path, st) in symtabs.iter() {
            // if the name is not in the map this means that content was filtered out
            // based on the extended_search_for_workspace_symbols setting
            let content_name;
            if let Some(name) = content_names.get(content_path) {
                content_name = name.to_string();
            } else {
                continue;
            }

            let content_symdata_iter = st.iter()
                .par_bridge()
                .filter_map(|(_, symvar)| symvar.to_symbol_data(&content_name));

            self.cache.workspace_symbols
                .write().await
                .unqueried_data
                .par_extend(content_symdata_iter);
        }

        self.cache.workspace_symbols
            .write().await
            .unqueried_data
            .par_sort_unstable_by(|a, b| b.cmp(a));

        self.cache.workspace_symbols
            .write().await
            .should_refresh = false;
    }
}


pub struct WorkspaceSymbolCache {
    unqueried_data: Vec<WorkspaceSymbolData>,
    pub should_refresh: bool
}

impl WorkspaceSymbolCache {
    pub fn new() -> Self {
        Self {
            unqueried_data: Vec::new(),
            should_refresh: true
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkspaceSymbolData {
    /// A string identifying the symbol
    /// It will be queried on using the "query" parameter 
    id_string: String,
    /// Rank derived from fuzzy-matching the id_string
    rank: Option<i64>,
    /// The actual LSP symbol info that should be sent to the client
    sym_info: lsp::SymbolInformation
}

impl PartialOrd for WorkspaceSymbolData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank.partial_cmp(&other.rank) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.sym_info.name.partial_cmp(&other.sym_info.name)
    }
}

impl Ord for WorkspaceSymbolData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        self.sym_info.name.cmp(&other.sym_info.name)
    }
}



trait ToWorkspaceSymbolData {
    /// None should be returned if a given symbol should not or cannot appear in the symbol list
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData>;
}


impl ToWorkspaceSymbolData for SymbolVariant {
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        match self {
            SymbolVariant::Class(s) => s.to_symbol_data(content_name),
            SymbolVariant::State(s) => s.to_symbol_data(content_name),
            SymbolVariant::Struct(s) => s.to_symbol_data(content_name),
            SymbolVariant::Enum(s) => s.to_symbol_data(content_name),
            SymbolVariant::Array(s) => s.to_symbol_data(content_name),
            SymbolVariant::ArrayFunc(s) => s.to_symbol_data(content_name),
            SymbolVariant::ArrayFuncParam(s) => s.to_symbol_data(content_name),
            SymbolVariant::GlobalFunc(s) => s.to_symbol_data(content_name),
            SymbolVariant::MemberFunc(s) => s.to_symbol_data(content_name),
            SymbolVariant::Event(s) => s.to_symbol_data(content_name),
            SymbolVariant::Constructor(s) => s.to_symbol_data(content_name),
            SymbolVariant::MemberFuncInjector(s) => s.to_symbol_data(content_name),
            SymbolVariant::MemberFuncReplacer(s) => s.to_symbol_data(content_name),
            SymbolVariant::GlobalFuncReplacer(s) => s.to_symbol_data(content_name),
            SymbolVariant::MemberFuncWrapper(s) => s.to_symbol_data(content_name),
            SymbolVariant::WrappedMethod(s) => s.to_symbol_data(content_name),
            SymbolVariant::Primitive(s) => s.to_symbol_data(content_name),
            SymbolVariant::EnumVariant(s) => s.to_symbol_data(content_name),
            SymbolVariant::FuncParam(s) => s.to_symbol_data(content_name),
            SymbolVariant::GlobalVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::MemberVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::Autobind(s) => s.to_symbol_data(content_name),
            SymbolVariant::LocalVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::ThisVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::SuperVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::StateSuperVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::ParentVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::VirtualParentVar(s) => s.to_symbol_data(content_name),
            SymbolVariant::MemberVarInjector(s) => s.to_symbol_data(content_name),
        }
    }
}


impl ToWorkspaceSymbolData for ClassSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.name().to_string(),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::CLASS,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for StateSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.name().to_string(),
            sym_info: lsp::SymbolInformation {
                name: self.state_name().to_string(),
                kind: lsp::SymbolKind::CLASS,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for StructSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.name().to_string(),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::STRUCT,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for EnumSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.name().to_string(),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::ENUM,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for ArrayTypeSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for ArrayTypeFunctionSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for ArrayTypeFunctionParameterSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for GlobalFunctionSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.name().to_string(),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::FUNCTION,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for MemberFunctionSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::METHOD,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for EventSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::EVENT,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for ConstructorSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for MemberFunctionInjectorSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::METHOD,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for MemberFunctionReplacerSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::METHOD,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for GlobalFunctionReplacerSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.name().to_string(),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::FUNCTION,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for MemberFunctionWrapperSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::METHOD,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for WrappedMethodSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for PrimitiveTypeSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for EnumVariantSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: format!("{} {}", self.parent_enum_name(), self.name()),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::ENUM_MEMBER,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for FunctionParameterSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for GlobalVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for MemberVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::FIELD,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for AutobindSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::FIELD,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}

impl ToWorkspaceSymbolData for LocalVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for ThisVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for SuperVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for StateSuperVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for ParentVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for VirtualParentVarSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, _content_name: &str) -> Option<WorkspaceSymbolData> {
        None
    }
}

impl ToWorkspaceSymbolData for MemberVarInjectorSymbol {
    #[allow(deprecated)]
    fn to_symbol_data(&self, content_name: &str) -> Option<WorkspaceSymbolData> {
        Some(WorkspaceSymbolData {
            id_string: self.path().components().map(|c| c.name).fold(String::new(), |n1, n2| format!("{} {}", n1, n2)),
            sym_info: lsp::SymbolInformation {
                name: self.name().to_string(),
                kind: lsp::SymbolKind::FIELD,
                location: lsp::Location { 
                    uri: self.location().abs_source_path().to_uri(), 
                    range: self.location().range
                },
                container_name: Some(content_name.to_string()),
                tags: None,
                deprecated: None
            },
            rank: None
        })
    }
}
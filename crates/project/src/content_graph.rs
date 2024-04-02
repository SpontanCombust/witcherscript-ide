use std::collections::HashSet;
use std::path::{Path, PathBuf};
use abs_path::AbsPath;
use thiserror::Error;
use lsp_types as lsp;
use crate::content::{try_make_content, ContentScanError, ProjectDirectory};
use crate::{Content, FileError, ContentScanner};
use crate::manifest::{DependencyValue, ManifestParseError};


#[derive(Debug, Clone, Error)]
pub enum ContentGraphError {
    #[error(transparent)]
    Io(#[from] FileError<std::io::Error>),
    #[error(transparent)]
    ManifestParse(#[from] FileError<ManifestParseError>),
    #[error("project dependency could not be found in this path")]
    DependencyPathNotFound {
        content_path: PathBuf,
        /// Manifest from which this error originated
        manifest_path: AbsPath,
        // Location in the manifest where the path is present
        manifest_range: lsp::Range
    },
    #[error("project dependency could not be found with this name")]
    DependencyNameNotFound {
        content_name: String,
        /// Manifest from which this error originated
        manifest_path: AbsPath,
        // Location in the manifest where the name is present
        manifest_range: lsp::Range
    },
    #[error("there are multiple matching dependencies with this name for this project")]
    MultipleMatchingDependencies {
        content_name: String,
        /// Manifest from which this error originated
        manifest_path: AbsPath,
        // Location in the manifest where the name is present
        manifest_range: lsp::Range
    }
}


/// Stores contents needed in the current workspace and tracks relationships between them.
#[derive(Debug)]
pub struct ContentGraph {
    repo_scanners: Vec<ContentScanner>,
    workspace_scanners: Vec<ContentScanner>,

    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,

    pub errors: Vec<ContentGraphError>
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub content: Box<dyn Content>,
    pub in_workspace: bool,
    pub in_repository: bool,
}

/// Edge direction is:
/// dependant ---> dependency
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GraphEdge {
    dependant_idx: usize,
    dependency_idx: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GraphEdgeDirection {
    Dependants,
    Dependencies
}


impl ContentGraph {
    pub fn new() -> Self {
        Self {
            repo_scanners: Vec::new(),
            workspace_scanners: Vec::new(),

            nodes: Vec::new(),
            edges: Vec::new(),

            errors: Vec::new()
        }
    }

    pub fn clear_repository_scanners(&mut self) {
        self.repo_scanners.clear();
    }

    pub fn add_repository_scanner(&mut self, scanner: ContentScanner) {
        self.repo_scanners.push(scanner);
    }

    pub fn clear_workspace_scanners(&mut self) {
        self.workspace_scanners.clear();
    }

    pub fn add_workspace_scanner(&mut self, scanner: ContentScanner) {
        self.workspace_scanners.push(scanner);
    }


    pub fn build(&mut self) -> ContentGraphDifference {
        let prev_nodes: Vec<_> = self.nodes
            .drain(..)
            .collect();

        self.edges.clear();
        self.errors.clear();


        self.create_workspace_content_nodes();

        // do not try finding dependencies etc. if workspace scanners returned no contents
        if !self.nodes.is_empty() {
            // building process will gradually remove elements from this vec when needed
            let mut repo_nodes = self.create_repository_content_nodes();

            self.fix_workspace_repo_overlap(&mut repo_nodes);

            // Now visit each node to compute its dependencies and create connections (edges) between nodes.
            // If a node has already been visited the process will be skipped.
            // While is needed here instead of a for loop, because the size of `self.nodes` can change
            let mut visited = HashSet::new();
            let mut i = 0;
            while i < self.nodes.len() {
                self.link_dependencies(i, &mut repo_nodes, &mut visited);
                i += 1;
            }
        }

        ContentGraphDifference::from_comparison(&prev_nodes, &self.nodes)
    }


    pub fn get_node_by_path(&self, content_path: &AbsPath) -> Option<&GraphNode> {
        self.get_node_index_by_path(content_path).map(|i| &self.nodes[i])
    }


    
    pub fn nodes<'g>(&'g self) -> Iter<'g> {
        let indices = (0..self.nodes.len()).collect();
        Iter::new(self, indices)
    }


    pub fn direct_dependencies<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.neighbour_indices_in_direction(idx, GraphEdgeDirection::Dependencies).collect();
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }

    pub fn direct_dependants<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.neighbour_indices_in_direction(idx, GraphEdgeDirection::Dependants).collect();
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }


    /// Iterate over all content nodes that are direct or indirect dependencies to the specified content starting from it.
    pub fn walk_dependencies<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.relatives_indices_in_direction(idx, GraphEdgeDirection::Dependencies);
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }

    /// Iterate over all content nodes that are direct or indirect dependants of the specified content starting from it.
    pub fn walk_dependants<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.relatives_indices_in_direction(idx, GraphEdgeDirection::Dependants);
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }




    /// Create nodes for contents coming from `workspace_scanners` and put them in the graph
    fn create_workspace_content_nodes(&mut self) {
        for scanner in &self.workspace_scanners {
            let (contents, errors) = scanner.scan();

            for content in contents {
                self.nodes.push(GraphNode { 
                    content,
                    in_workspace: true, 
                    in_repository: false,
                });
            }

            for err in errors {
                match err {
                    ContentScanError::Io(err) => {
                        self.errors.push(ContentGraphError::Io(err));
                    },
                    ContentScanError::ManifestParse(err) => {
                        self.errors.push(ContentGraphError::ManifestParse(err))
                    },
                    // NotContent only occurs when trying to make content manually and not when scanning
                    ContentScanError::NotContent => {},
                }
            }
        }
    }

    /// Create nodes for contents coming from `repo_scanners` and return them in a container seperate from graph's
    fn create_repository_content_nodes(&mut self) -> Vec<GraphNode> {
        let mut repo_nodes = Vec::new();

        for scanner in &self.repo_scanners {
            let (contents, errors) = scanner.scan();

            for content in contents {
                repo_nodes.push(GraphNode { 
                    content,
                    in_workspace: false, 
                    in_repository: true,
                });
            }

            for err in errors {
                match err {
                    ContentScanError::Io(err) => {
                        self.errors.push(ContentGraphError::Io(err));
                    },
                    ContentScanError::ManifestParse(err) => {
                        self.errors.push(ContentGraphError::ManifestParse(err))
                    },
                    // NotContent only occurs when trying to make content manually and not when scanning
                    ContentScanError::NotContent => {},
                }
            }
        }

        repo_nodes
    }

    /// Adresses the edge case when the workspace and repository folders overlap, e.g. if workspace is inside the repository folder.
    /// In that case nodes currently residing in the graph may already contain some nodes that are equal to those in `repo_nodes`.
    fn fix_workspace_repo_overlap(&mut self, repo_nodes: &mut Vec<GraphNode>) {
        let mut i = 0;
        while i < repo_nodes.len() {
            let repo_content_path = repo_nodes[i].content.path();
            if let Some(already_in_graph) = self.nodes.iter_mut().find(|n| n.content.path() == repo_content_path) {
                already_in_graph.in_repository = true;
                repo_nodes.remove(i);
                continue;
            }

            i += 1;
        }
    }

    fn link_dependencies(&mut self, node_idx: usize, repo_nodes: &mut Vec<GraphNode>, visited: &mut HashSet<usize>) {
        if visited.contains(&node_idx) {
            return;
        }

        visited.insert(node_idx);

        let manifest_path_and_deps = self.nodes[node_idx]
            .content.as_any()
            .downcast_ref::<ProjectDirectory>()
            .map(|proj| (proj.manifest_path().to_owned(), proj.manifest().dependencies.clone()));

        if let Some((manifest_path, dependencies)) = manifest_path_and_deps {
            for entry in dependencies.into_iter() {
                match entry.value.inner() {
                    DependencyValue::FromRepo(active) => {
                        self.link_dependencies_value_from_repo(node_idx, repo_nodes, &manifest_path, &entry.name, entry.name.range(), *active);
                    },
                    DependencyValue::FromPath { path } => {
                        self.link_dependencies_value_from_path(node_idx, repo_nodes, &manifest_path, path, entry.value.range());
                    },
                }
            }
        }
    }

    fn link_dependencies_value_from_repo(&mut self, 
        node_idx: usize,
        repo_nodes: &mut Vec<GraphNode>,
        manifest_path: &AbsPath,
        dependency_name: &str,
        dependency_name_range: &lsp::Range,
        active: bool
    ) {
        if active {
            match self.get_dependency_node_index_by_name(&dependency_name, repo_nodes) {
                Ok(dep_idx) => {
                    self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                },
                Err(dep_count) => {
                    if dep_count == 0 {
                        self.errors.push(ContentGraphError::DependencyNameNotFound { 
                            content_name: dependency_name.to_string(), 
                            manifest_path: manifest_path.to_owned(),
                            manifest_range: dependency_name_range.clone()
                        });
                    } else {
                        self.errors.push(ContentGraphError::MultipleMatchingDependencies { 
                            content_name: dependency_name.to_string(), 
                            manifest_path: manifest_path.to_owned(), 
                            manifest_range: dependency_name_range.clone()
                        });
                    }
                }
            }
        }
    }

    /// If there is just one repository content with the name returns Ok with the index.
    /// Otherwise returns Err with the number of contents encountered with that name.
    /// So if it wasn't found returns Err(0) or if more than one with than name was found returns Err(2) for example. 
    fn get_dependency_node_index_by_name(&mut self, name: &str, repo_nodes: &mut Vec<GraphNode>) -> Result<usize, usize> {
        let mut candidates = Vec::new();
        for (i, n) in self.nodes.iter().enumerate() {
            if n.in_repository && n.content.content_name() == name {
                candidates.push(i);
            }
        }

        let candidates_len = candidates.len();
        if candidates_len == 1 {
            Ok(candidates[0])
        } else if candidates_len > 1 {
            Err(candidates_len)
        } else {
            for (i, n) in repo_nodes.iter().enumerate() {
                if n.content.content_name() == name {
                    candidates.push(i);
                }
            }

            let candidates_len = candidates.len();
            if candidates_len == 0 {
                Err(0)
            } else if candidates_len == 1 {
                let target_node = repo_nodes.remove(candidates[0]);
                let target_node_idx = self.insert_node(target_node);
                Ok(target_node_idx)
            } else {
                Err(candidates_len)
            }
        }
    }

    fn link_dependencies_value_from_path(&mut self, 
        node_idx: usize, 
        repo_nodes: &mut Vec<GraphNode>,
        manifest_path: &AbsPath,
        dependency_path: &Path,
        dependency_path_range: &lsp::Range
    ) {
        let dependant_path = self.nodes[node_idx].content.path();
        let abs_dependency_path = AbsPath::resolve(dependency_path, Some(dependant_path));

        match abs_dependency_path {
            Ok(dep_path) => {
                if let Some(dep_idx) = self.get_dependency_node_index_by_path(&dep_path, repo_nodes) {
                    self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                } else {
                    match try_make_content(&dep_path) {
                        Ok(content) => {
                            let dep_idx = self.insert_node(GraphNode { 
                                content, 
                                in_workspace: false, 
                                in_repository: false
                            });

                            self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                        },
                        Err(err) => {
                            match err {
                                ContentScanError::Io(err) => {
                                    self.errors.push(ContentGraphError::Io(err));
                                },
                                ContentScanError::ManifestParse(err) => {
                                    self.errors.push(ContentGraphError::ManifestParse(err));
                                },
                                ContentScanError::NotContent => {
                                    self.errors.push(ContentGraphError::DependencyPathNotFound { 
                                        content_path: dep_path.into(), 
                                        manifest_path: manifest_path.to_owned(),
                                        manifest_range: dependency_path_range.clone()
                                    })
                                },
                            }
                        },
                    }
                }
            },
            Err(_) => {
                self.errors.push(ContentGraphError::DependencyPathNotFound { 
                    content_path: dependency_path.to_path_buf(), 
                    manifest_path: manifest_path.to_owned(),
                    manifest_range: dependency_path_range.clone()
                })
            }
        }
    }

    fn get_dependency_node_index_by_path(&mut self, path: &AbsPath, repo_nodes: &mut Vec<GraphNode>) -> Option<usize> {
        if let Some(i) = self.nodes.iter().position(|n| n.content.path() == path) {
            Some(i)
        }
        else if let Some(i) = repo_nodes.iter().position(|n| n.content.path() == path) {
            let target_node = repo_nodes.remove(i);
            let target_node_idx = self.insert_node(target_node);
            Some(target_node_idx)
        }
        else {
            None
        }
    }

    /// Returns the index of this node
    fn insert_node(&mut self, node: GraphNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    
    fn insert_edge(&mut self, edge: GraphEdge) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }




    fn get_node_index_by_path(&self, path: &AbsPath) -> Option<usize> {
        for (i, n) in self.nodes.iter().enumerate() {
            if n.content.path() == path {
                return Some(i)
            }
        }
        None
    }

    /// Get iterator over direct neighbours of a given node
    fn neighbour_indices_in_direction<'g>(&'g self, node_idx: usize, direction: GraphEdgeDirection) -> impl Iterator<Item = usize> + 'g {
        self.edges.iter()
            .filter(move |edge| {
                node_idx == match direction {
                    GraphEdgeDirection::Dependants => edge.dependency_idx,
                    GraphEdgeDirection::Dependencies => edge.dependant_idx,
                }
            })
            .map(move |edge| {
                match direction {
                    GraphEdgeDirection::Dependants => edge.dependant_idx,
                    GraphEdgeDirection::Dependencies => edge.dependency_idx,
                }
            })
    }

    /// Get a vec of all node indices related to the given node in a given direction. The starting node is included in the vec.
    fn relatives_indices_in_direction(&self, starting_idx: usize, direction: GraphEdgeDirection) -> Vec<usize> {
        let mut indices = Vec::with_capacity(self.nodes.capacity());

        indices.push(starting_idx);

        let mut i = 0;
        while i < indices.len() {
            let current_idx = indices[i];
            let neighbours = self.neighbour_indices_in_direction(current_idx, direction);

            for idx in neighbours {
                if !indices.contains(&idx) {
                    indices.push(idx);
                }
            }

            i += 1;
        }

        indices
    }
}


pub struct Iter<'g> {
    graph: &'g ContentGraph,
    node_indices: Vec<usize>,
    i: usize // index of node_indices, not of graph.nodes
}

impl<'g> Iter<'g> {
    fn new(graph: &'g ContentGraph, node_indices: Vec<usize>) -> Self {
        Self {
            graph,
            node_indices,
            i: 0
        }
    }

    #[inline(always)]
    fn current_node_idx(&self) -> Option<usize> {
        if self.i < self.node_indices.len() {
            Some(self.node_indices[self.i])
        } else {
            None
        }
    }

    #[inline(always)]
    fn current_node(&self) -> Option<&'g GraphNode> {
        self.current_node_idx().map(|idx| &self.graph.nodes[idx])
    }    
}

impl<'g> Iterator for Iter<'g> {
    type Item = &'g GraphNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.current_node() {
            self.i += 1;
            Some(n)
        } else {
            None
        }
    }
}



#[derive(Debug, Clone, Default)]
pub struct ContentGraphDifference {
    pub added: Vec<GraphNode>,
    pub removed: Vec<GraphNode>,
}

impl ContentGraphDifference {
    fn from_comparison(old_nodes: &Vec<GraphNode>, new_nodes: &Vec<GraphNode>) -> Self {
        // NewType that compares nodes based upon content paths only
        struct DiffingWrapper<'a>(&'a GraphNode);

        impl PartialEq for DiffingWrapper<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.0.content.path().eq(other.0.content.path())
            }
        }

        impl Eq for DiffingWrapper<'_> {}

        impl std::hash::Hash for DiffingWrapper<'_> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.content.path().hash(state);
            }
        }


        let old_diffable: HashSet<_> = old_nodes.iter().map(|n| DiffingWrapper(n)).collect();
        let new_diffable: HashSet<_> = new_nodes.iter().map(|n| DiffingWrapper(n)).collect();

        Self {
            added: new_diffable.difference(&old_diffable).map(|wrapper| wrapper.0.clone()).collect(),
            removed: old_diffable.difference(&new_diffable).map(|wrapper| wrapper.0.clone()).collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}
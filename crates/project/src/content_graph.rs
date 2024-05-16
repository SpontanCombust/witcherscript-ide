use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use lsp_types as lsp;
use abs_path::AbsPath;
use crate::content::{try_make_content, ContentScanError, ProjectDirectory, RedkitProjectDirectory};
use crate::{manifest, redkit, Content, ContentScanner, FileError};


#[derive(Debug, Clone, Error)]
pub enum ContentGraphError {
    #[error(transparent)]
    Io(#[from] FileError<std::io::Error>),
    #[error(transparent)]
    ManifestRead(#[from] FileError<manifest::Error>),
    #[error(transparent)]
    RedkitManifestRead(#[from] FileError<redkit::manifest::Error>),
    #[error("project dependency at path \"{}\" could not be found", .content_path.display())]
    DependencyPathNotFound {
        content_path: PathBuf,
        /// Manifest from which this error originated
        manifest_path: AbsPath,
        // Location in the manifest where the path is present
        manifest_range: lsp::Range
    },
    #[error("project dependency with name \"{}\" could not be found", .content_name)]
    DependencyNameNotFound {
        content_name: String,
        /// Manifest from which this error originated
        manifest_path: AbsPath,
        // Location in the manifest where the name is present
        manifest_range: lsp::Range
    },
    #[error("project dependency with name \"{}\" could not be found at specified path", .content_name)]
    DependencyNameNotFoundAtPath {
        content_name: String,
        /// Manifest from which this error originated
        manifest_path: AbsPath,
        // Location in the manifest where the name is present
        manifest_range: lsp::Range
    },
    #[error("there are multiple matching dependencies with name \"{}\" for this project", .content_name)]
    MultipleMatchingDependencies {
        content_name: String,
        /// Manifest from which this error originated
        manifest_path: AbsPath,
        // Location in the manifest where the name is present
        manifest_range: lsp::Range,

        matching_paths: Vec<AbsPath>
    },
    #[error("native content could not be found")]
    NativeContentNotFound(Option<ContentScanError>),
}


/// Stores contents needed in the current workspace and tracks relationships between them.
#[derive(Debug)]
pub struct ContentGraph {
    repo_scanners: Vec<ContentScanner>,
    workspace_scanners: Vec<ContentScanner>,
    native_content_path: Option<AbsPath>,

    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,

    pub errors: Vec<ContentGraphError>
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub content: Arc<dyn Content>,
    pub in_workspace: bool,
    pub in_repository: bool,
    pub is_native: bool
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
            native_content_path: None,

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

    /// Set the path to native content, which is content containing declarations of symbols that don't appear as declarations anywhere in content0.
    /// It is also used to attribute to it all the non-declarable types, i.e. stuff that a compiler always assumes to be there.
    /// This content is always set as a dependency of "content0" content.
    /// 
    /// Native content is distributed together with WitcherScript IDE's binaries.
    pub fn set_native_content_path(&mut self, native_content_path: &AbsPath) {
        self.native_content_path = Some(native_content_path.to_owned());
    }


    pub fn build(&mut self) -> ContentGraphDifference {
        self.errors.clear();

        let prev_nodes: Vec<_> = self.nodes.drain(..).collect();
        let prev_edges: Vec<_> = self.edges.drain(..).collect();

        self.create_native_content_node();
        self.create_workspace_content_nodes();

        // do not try finding dependencies etc. if workspace scanners returned no contents
        if self.nodes.iter().any(|n| n.in_workspace) {
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

        ContentGraphDifference::from_comparison(&prev_nodes, &self.nodes, &prev_edges, &self.edges)
    }


    pub fn get_node_by_path(&self, content_path: &AbsPath) -> Option<&GraphNode> {
        self.get_node_index_by_path(content_path).map(|i| &self.nodes[i])
    }


    
    pub fn nodes<'g>(&'g self) -> Iter<'g> {
        let indices = (0..self.nodes.len()).collect();
        Iter::new(self, indices)
    }


    /// Iterate over direct dependencies of specified content
    /// Iterator will be empty when either the specified content doesn't exist or it has no dependencies.
    pub fn direct_dependencies<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.neighbour_indices_in_direction(idx, GraphEdgeDirection::Dependencies).collect();
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }

    /// Iterate over direct dependants of specified content
    /// Iterator will be empty when either the specified content doesn't exist or it has no dependants.
    pub fn direct_dependants<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.neighbour_indices_in_direction(idx, GraphEdgeDirection::Dependants).collect();
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }


    /// Iterate over all content nodes that are direct or indirect dependencies to the specified content.
    /// Iterator will be empty when either the specified content doesn't exist or it has no dependencies.
    pub fn walk_dependencies<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.relatives_indices_in_direction(idx, GraphEdgeDirection::Dependencies);
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }

    /// Iterate over all content nodes that are direct or indirect dependants of the specified content.
    /// Iterator will be empty when either the specified content doesn't exist or it has no dependants.
    pub fn walk_dependants<'g>(&'g self, content_path: &AbsPath) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.relatives_indices_in_direction(idx, GraphEdgeDirection::Dependants);
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }




    fn create_native_content_node(&mut self) {
        if let Some(path) = self.native_content_path.clone() {
            match try_make_content(&path) {
                Ok(native_content) => {
                    self.insert_node(GraphNode { 
                        content: native_content.into(), 
                        in_workspace: false, 
                        in_repository: false, 
                        is_native: true 
                    });
                },
                Err(err) => {
                    self.errors.push(ContentGraphError::NativeContentNotFound(Some(err)));
                },
            }
        } else {
            self.errors.push(ContentGraphError::NativeContentNotFound(None))
        }
    }

    /// Create nodes for contents coming from `workspace_scanners` and put them in the graph
    fn create_workspace_content_nodes(&mut self) {
        for scanner in &self.workspace_scanners {
            let (contents, errors) = scanner.scan();

            for content in contents {
                self.nodes.push(GraphNode { 
                    content: Arc::from(content),
                    in_workspace: true, 
                    in_repository: false,
                    is_native: false
                });
            }

            for err in errors {
                match err {
                    ContentScanError::Io(err) => {
                        self.errors.push(ContentGraphError::Io(err));
                    },
                    ContentScanError::ManifestRead(err) => {
                        self.errors.push(ContentGraphError::ManifestRead(err))
                    },
                    ContentScanError::RedkitManifestRead(err) => {
                        self.errors.push(ContentGraphError::RedkitManifestRead(err))
                    }
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
                    content: Arc::from(content),
                    in_workspace: false, 
                    in_repository: true,
                    is_native: false
                });
            }

            for err in errors {
                match err {
                    ContentScanError::Io(err) => {
                        self.errors.push(ContentGraphError::Io(err));
                    },
                    ContentScanError::ManifestRead(err) => {
                        self.errors.push(ContentGraphError::ManifestRead(err))
                    },
                    ContentScanError::RedkitManifestRead(err) => {
                        self.errors.push(ContentGraphError::RedkitManifestRead(err))
                    }
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

        let content = Arc::clone(&self.nodes[node_idx].content);
        if let Some(proj) = content.as_any().downcast_ref::<ProjectDirectory>() {
            for entry in proj.manifest().dependencies.iter() {
                match &entry.value {
                    manifest::DependencyValue::FromRepo(active) => {
                        if *active {
                            self.link_dependencies_value_from_repo(node_idx, repo_nodes, proj.manifest_path(), &entry.name, &entry.name_range);
                        }
                    },
                    manifest::DependencyValue::FromPath { path } => {
                        self.link_dependencies_value_from_path(node_idx, repo_nodes, proj.manifest_path(), &entry.name, &entry.name_range, path, &entry.value_range);
                    },
                }
            }
        } else if let Some(redkit_proj) = content.as_any().downcast_ref::<RedkitProjectDirectory>() {
            self.link_dependencies_value_from_repo(node_idx, repo_nodes, redkit_proj.manifest_path(), "content0".into(), &lsp::Range::default());
        }

        // only content0 gets to be connected to native symbols to avoid redundant edges
        if content.content_name() == "content0" {
            if let Some(native_idx) = self.nodes.iter().position(|n| n.is_native) {
                self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: native_idx });
            }
        }
    }

    fn link_dependencies_value_from_repo(&mut self, 
        node_idx: usize,
        repo_nodes: &mut Vec<GraphNode>,
        manifest_path: &AbsPath,
        dependency_name: &str,
        dependency_name_range: &lsp::Range
    ) {
        match self.get_dependency_node_index_by_name(&dependency_name, repo_nodes) {
            Ok(dep_idx) => {
                self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
            },
            Err(matching_paths) => {
                if matching_paths.is_empty() {
                    self.errors.push(ContentGraphError::DependencyNameNotFound { 
                        content_name: dependency_name.to_owned(), 
                        manifest_path: manifest_path.to_owned(),
                        manifest_range: dependency_name_range.to_owned()
                    });
                } else {
                    self.errors.push(ContentGraphError::MultipleMatchingDependencies { 
                        content_name: dependency_name.to_owned(), 
                        manifest_path: manifest_path.to_owned(), 
                        manifest_range: dependency_name_range.to_owned(),
                        matching_paths
                    });
                }
            }
        }
    }

    /// If there is just one repository content with the name returns Ok with the index.
    /// Otherwise returns Err with the Vec of matching content paths.
    fn get_dependency_node_index_by_name(&mut self, name: &str, repo_nodes: &mut Vec<GraphNode>) -> Result<usize, Vec<AbsPath>> {
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
            Err(candidates.into_iter()
                .map(|i| self.nodes[i].content.path().to_owned())
                .collect())
        } else {
            for (i, n) in repo_nodes.iter().enumerate() {
                if n.content.content_name() == name {
                    candidates.push(i);
                }
            }

            let candidates_len = candidates.len();
            if candidates_len == 0 {
                Err(Vec::new())
            } else if candidates_len == 1 {
                let target_node = repo_nodes.remove(candidates[0]);
                let target_node_idx = self.insert_node(target_node);
                Ok(target_node_idx)
            } else {
                Err(candidates.into_iter()
                    .map(|i| repo_nodes[i].content.path().to_owned())
                    .collect())
            }
        }
    }

    fn link_dependencies_value_from_path(&mut self, 
        node_idx: usize, 
        repo_nodes: &mut Vec<GraphNode>,
        manifest_path: &AbsPath,
        dependency_name: &str,
        dependency_name_range: &lsp::Range,
        dependency_path: &PathBuf,
        dependency_path_range: &lsp::Range
    ) {
        let dependant_path = self.nodes[node_idx].content.path();
        let abs_dependency_path = AbsPath::resolve(&dependency_path, Some(dependant_path));

        if abs_dependency_path.is_err() {
            self.errors.push(ContentGraphError::DependencyPathNotFound { 
                content_path: dependency_path.to_owned(), 
                manifest_path: manifest_path.to_owned(),
                manifest_range: dependency_path_range.to_owned()
            });

            return;
        }

        let dep_path = abs_dependency_path.unwrap();

        if let Some(dep_idx) = self.get_dependency_node_index_by_path(&dep_path, repo_nodes) {
            if self.nodes[dep_idx].content.content_name() == dependency_name {
                self.insert_edge(GraphEdge { 
                    dependant_idx: node_idx, 
                    dependency_idx: dep_idx 
                });
            } else {
                self.errors.push(ContentGraphError::DependencyNameNotFoundAtPath { 
                    content_name: dependency_name.to_owned(), 
                    manifest_path: manifest_path.to_owned(), 
                    manifest_range: dependency_name_range.to_owned()
                });
            }
        } else {
            match try_make_content(&dep_path) {
                Ok(content) => {
                    if content.content_name() == dependency_name {
                        let dep_idx = self.insert_node(GraphNode { 
                            content: Arc::from(content), 
                            in_workspace: false, 
                            in_repository: false,
                            is_native: false
                        });
    
                        self.insert_edge(GraphEdge { 
                            dependant_idx: node_idx, 
                            dependency_idx: dep_idx 
                        });
                    } else {
                        self.errors.push(ContentGraphError::DependencyNameNotFoundAtPath { 
                            content_name: dependency_name.to_owned(), 
                            manifest_path: manifest_path.to_owned(), 
                            manifest_range: dependency_name_range.to_owned()
                        });
                    }
                },
                Err(err) => {
                    match err {
                        ContentScanError::Io(err) => {
                            self.errors.push(ContentGraphError::Io(err));
                        },
                        ContentScanError::ManifestRead(err) => {
                            self.errors.push(ContentGraphError::ManifestRead(err));
                        },
                        ContentScanError::RedkitManifestRead(err) => {
                            self.errors.push(ContentGraphError::RedkitManifestRead(err))
                        }
                        ContentScanError::NotContent => {
                            self.errors.push(ContentGraphError::DependencyPathNotFound { 
                                content_path: dep_path.into(), 
                                manifest_path: manifest_path.to_owned(),
                                manifest_range: dependency_path_range.to_owned()
                            })
                        },
                    }
                },
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

    /// Get a vec of all node indices related to the given node in a given direction
    fn relatives_indices_in_direction(&self, starting_idx: usize, direction: GraphEdgeDirection) -> Vec<usize> {
        let mut indices = Vec::with_capacity(self.nodes.capacity());

        for neighbour in self.neighbour_indices_in_direction(starting_idx, direction) {
            indices.push(neighbour);
        }

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


#[derive(Debug, Clone)]
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
    pub added_nodes: Vec<GraphNode>,
    pub removed_nodes: Vec<GraphNode>,
    pub added_edges: Vec<GraphEdgeWithContent>,
    pub removed_edges: Vec<GraphEdgeWithContent>
}

impl ContentGraphDifference {
    fn from_comparison(
        old_nodes: &Vec<GraphNode>, 
        new_nodes: &Vec<GraphNode>,
        old_edges: &Vec<GraphEdge>,
        new_edges: &Vec<GraphEdge>
    ) -> Self {
        // NewType that compares nodes based upon content paths only
        struct NodeDiffingWrapper<'a>(&'a GraphNode);

        impl PartialEq for NodeDiffingWrapper<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.0.content.path().eq(other.0.content.path())
            }
        }

        impl Eq for NodeDiffingWrapper<'_> {}

        impl std::hash::Hash for NodeDiffingWrapper<'_> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.content.path().hash(state);
            }
        }


        let old_nodes_diffable: HashSet<_> = old_nodes.iter().map(|n| NodeDiffingWrapper(n)).collect();
        let new_nodes_diffable: HashSet<_> = new_nodes.iter().map(|n| NodeDiffingWrapper(n)).collect();
        let old_edges_diffable: HashSet<_> = old_edges.iter().map(|e| GraphEdgeWithContent::new(e, old_nodes)).collect();
        let new_edges_diffable: HashSet<_> = new_edges.iter().map(|e| GraphEdgeWithContent::new(e, new_nodes)).collect();

        Self {
            added_nodes: new_nodes_diffable.difference(&old_nodes_diffable).map(|wrapper| wrapper.0.clone()).collect(),
            removed_nodes: old_nodes_diffable.difference(&new_nodes_diffable).map(|wrapper| wrapper.0.clone()).collect(),
            added_edges: new_edges_diffable.difference(&old_edges_diffable).cloned().collect(),
            removed_edges: old_edges_diffable.difference(&new_edges_diffable).cloned().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.added_nodes.is_empty()     && 
        self.removed_nodes.is_empty()   && 
        self.added_edges.is_empty()     && 
        self.removed_edges.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct GraphEdgeWithContent {
    pub dependant_content: Arc<dyn Content>,
    pub dependency_content: Arc<dyn Content>
}

impl GraphEdgeWithContent {
    fn new(edge: &GraphEdge, nodes: &Vec<GraphNode>) -> Self {
        Self {
            dependant_content: Arc::clone(&nodes[edge.dependant_idx].content),
            dependency_content: Arc::clone(&nodes[edge.dependency_idx].content)
        }
    }
}

impl PartialEq for GraphEdgeWithContent {
    fn eq(&self, other: &Self) -> bool {
        self.dependant_content.path() == other.dependant_content.path() 
        && self.dependency_content.path() == other.dependency_content.path()
    }
}

impl Eq for GraphEdgeWithContent {}

impl std::hash::Hash for GraphEdgeWithContent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dependant_content.path().hash(state);
        self.dependency_content.path().hash(state);
    }
}




#[cfg(test)]
mod test {
    use std::sync::OnceLock;
    use super::*;


    fn test_assets() -> &'static AbsPath {
        static TEST_ASSETS: OnceLock<AbsPath> = OnceLock::new();
        TEST_ASSETS.get_or_init(|| {
            let manifest_dir = AbsPath::resolve(env!("CARGO_MANIFEST_DIR"), None).unwrap();
            manifest_dir.join("../../test_assets/project").unwrap()
        })
    }


    #[test]
    fn test() {
        let mut graph = ContentGraph::new();

        let workspace_scanner = ContentScanner::new(test_assets().join("dir1").unwrap()).unwrap()
            .only_projects(true)
            .recursive(true);

        let repo_scanner = ContentScanner::new(test_assets().join("dir2").unwrap()).unwrap()
            .only_projects(false)
            .recursive(false);

        graph.add_workspace_scanner(workspace_scanner);
        graph.add_repository_scanner(repo_scanner);
        graph.set_native_content_path(&test_assets().join("native_content").unwrap());

        graph.build();

        assert!(graph.errors.is_empty());

        assert!(graph.get_node_by_path(&test_assets().join("dir1/proj1").unwrap()).is_some());
        assert!(graph.get_node_by_path(&test_assets().join("dir1/proj2").unwrap()).is_some());
        assert!(graph.get_node_by_path(&test_assets().join("dir1/nested/proj3").unwrap()).is_some());
        assert!(graph.get_node_by_path(&test_assets().join("dir1/nested/raw2").unwrap()).is_some());
        assert!(graph.get_node_by_path(&test_assets().join("dir1/redkit").unwrap()).is_some());
        assert!(graph.get_node_by_path(&test_assets().join("dir2/raw0").unwrap()).is_some());
        assert!(graph.get_node_by_path(&test_assets().join("dir2/content0").unwrap()).is_some());


        let it = graph.nodes();
        assert_eq!(it.clone().count(), 8);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj1").unwrap() && n.in_workspace && !n.in_repository && !n.is_native));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj2").unwrap() && n.in_workspace && !n.in_repository && !n.is_native));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/nested/proj3").unwrap() && n.in_workspace && !n.in_repository && !n.is_native));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/nested/raw2").unwrap() && !n.in_workspace && !n.in_repository && !n.is_native));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/redkit").unwrap() && n.in_workspace && !n.in_repository && !n.is_native));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir2/raw0").unwrap() && !n.in_workspace && n.in_repository && !n.is_native));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir2/content0").unwrap() && !n.in_workspace && n.in_repository && !n.is_native));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("native_content").unwrap() && !n.in_workspace && !n.in_repository && n.is_native));


        let mut it = graph.direct_dependencies(&test_assets().join("dir1/proj1").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir2/raw0").unwrap());

        let mut it = graph.direct_dependencies(&test_assets().join("dir1/proj2").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir2/raw0").unwrap());

        let it = graph.direct_dependencies(&test_assets().join("dir1/nested/proj3").unwrap());
        assert_eq!(it.clone().count(), 2);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/nested/raw2").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj2").unwrap()));

        let it = graph.direct_dependencies(&test_assets().join("dir1/nested/raw2").unwrap());
        assert_eq!(it.count(), 0);

        let it = graph.direct_dependencies(&test_assets().join("dir1/redkit").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir2/content0").unwrap()));

        let it = graph.direct_dependencies(&test_assets().join("dir2/raw0").unwrap());
        assert_eq!(it.count(), 0);

        let it = graph.direct_dependencies(&test_assets().join("dir2/content0").unwrap());
        assert_eq!(it.count(), 1);

        let it = graph.direct_dependencies(&test_assets().join("native_content").unwrap());
        assert_eq!(it.count(), 0);



        let it = graph.direct_dependants(&test_assets().join("dir1/proj1").unwrap());
        assert_eq!(it.count(), 0);

        let mut it = graph.direct_dependants(&test_assets().join("dir1/proj2").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir1/nested/proj3").unwrap());

        let it = graph.direct_dependants(&test_assets().join("dir1/nested/proj3").unwrap());
        assert_eq!(it.count(), 0);

        let mut it = graph.direct_dependants(&test_assets().join("dir1/nested/raw2").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir1/nested/proj3").unwrap());

        let it = graph.direct_dependants(&test_assets().join("dir1/redkit").unwrap());
        assert_eq!(it.count(), 0);

        let it = graph.direct_dependants(&test_assets().join("dir2/raw0").unwrap());
        assert_eq!(it.clone().count(), 2);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj1").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj2").unwrap()));

        let it = graph.direct_dependants(&test_assets().join("dir2/content0").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/redkit").unwrap()));

        let it = graph.direct_dependants(&test_assets().join("native_content").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir2/content0").unwrap()));



        let mut it = graph.walk_dependencies(&test_assets().join("dir1/proj1").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir2/raw0").unwrap());

        let mut it = graph.walk_dependencies(&test_assets().join("dir1/proj2").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir2/raw0").unwrap());

        let it = graph.walk_dependencies(&test_assets().join("dir1/nested/proj3").unwrap());
        assert_eq!(it.clone().count(), 3);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/nested/raw2").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj2").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir2/raw0").unwrap()));

        let it = graph.walk_dependencies(&test_assets().join("dir1/nested/raw2").unwrap());
        assert_eq!(it.count(), 0);

        let it = graph.walk_dependencies(&test_assets().join("dir1/redkit").unwrap());
        assert_eq!(it.clone().count(), 2);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir2/content0").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("native_content").unwrap()));

        let it = graph.walk_dependencies(&test_assets().join("dir2/raw0").unwrap());
        assert_eq!(it.count(), 0);

        let it = graph.walk_dependencies(&test_assets().join("dir2/content0").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("native_content").unwrap()));


        let it = graph.walk_dependants(&test_assets().join("dir1/proj1").unwrap());
        assert_eq!(it.count(), 0);

        let mut it = graph.walk_dependants(&test_assets().join("dir1/proj2").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir1/nested/proj3").unwrap());

        let it = graph.walk_dependants(&test_assets().join("dir1/nested/proj3").unwrap());
        assert_eq!(it.count(), 0);

        let mut it = graph.walk_dependants(&test_assets().join("dir1/nested/raw2").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.next().unwrap().content.path() == &test_assets().join("dir1/nested/proj3").unwrap());

        let it = graph.walk_dependants(&test_assets().join("dir1/redkit").unwrap());
        assert_eq!(it.count(), 0);

        let it = graph.walk_dependants(&test_assets().join("dir2/raw0").unwrap());
        assert_eq!(it.clone().count(), 3);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj1").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/proj2").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/nested/proj3").unwrap()));

        let it = graph.walk_dependants(&test_assets().join("dir2/content0").unwrap());
        assert_eq!(it.clone().count(), 1);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/redkit").unwrap()));

        let it = graph.walk_dependants(&test_assets().join("native_content").unwrap());
        assert_eq!(it.clone().count(), 2);
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir2/content0").unwrap()));
        assert!(it.clone().any(|n| n.content.path() == &test_assets().join("dir1/redkit").unwrap()));
    }
}
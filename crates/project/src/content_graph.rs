use std::collections::HashSet;
use std::path::{Path, PathBuf};
use thiserror::Error;
use lsp_types as lsp;
use crate::content::{try_make_content, ContentScanError, ProjectDirectory};
use crate::{Content, ContentRepositories, FileError};
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
        manifest_path: PathBuf,
        // Location in the manifest where the path is present
        manifest_range: lsp::Range
    },
    #[error("project dependency could not be found with this name")]
    DependencyNameNotFound {
        content_name: String,
        /// Manifest from which this error originated
        manifest_path: PathBuf,
        // Location in the manifest where the name is present
        manifest_range: lsp::Range
    },
    #[error("there are multiple matching dependencies with this name for this project")]
    MultipleMatchingDependencies {
        content_name: String,
        /// Manifest from which this error originated
        manifest_path: PathBuf,
        // Location in the manifest where the name is present
        manifest_range: lsp::Range
    }
}


/// Stores contents needed in the current workspace and tracks relationships between them.
#[derive(Debug)]
pub struct ContentGraph {
    repos: ContentRepositories,
    workspace_projects: Vec<Box<ProjectDirectory>>,

    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,

    pub errors: Vec<ContentGraphError>
}

#[derive(Debug)]
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
            repos: ContentRepositories::new(),
            workspace_projects: Vec::new(),

            nodes: Vec::new(),
            edges: Vec::new(),

            errors: Vec::new()
        }
    }

    /// Set repositories which the graph can access for any dependencies
    pub fn set_repositories(&mut self, repos: ContentRepositories) {
        self.repos = repos;
    }

    pub fn get_reposity_contents(&self) -> &[Box<dyn Content>] {
        self.repos.found_content()
    }

    /// Set paths to contents from the workspace that should be actively monitored
    pub fn set_workspace_projects(&mut self, contents: Vec<Box<ProjectDirectory>>) {
        self.workspace_projects = contents;
    }

    pub fn get_workspace_projects(&self) -> &[Box<ProjectDirectory>] {
        &self.workspace_projects
    }


    pub fn build(&mut self) -> ContentGraphDifference {
        let prev_content_paths: HashSet<_> = self.nodes.iter()
            .map(|n| n.content.path().to_owned())
            .collect();

        self.nodes.clear();
        self.edges.clear();
        self.errors.clear();

        if !self.workspace_projects.is_empty() {     
            for i in 0..self.workspace_projects.len() {
                let content = &self.workspace_projects[i];
                self.create_node_for_content(content.clone(), false, true);
            }
    
            for i in 0..self.repos.found_content().len() {
                let content = &self.repos.found_content()[i];
                self.create_node_for_content(dyn_clone::clone_box(&**content), true, false);
            }
    
            // Correct nodes if repository and workspace paths overlap
            for n in &mut self.nodes {
                if self.repos.found_content().iter().any(|repo_content| repo_content.path() == n.content.path()) {
                    n.in_repository = true;
                }
            }
    
            // Now visit each of workspace content nodes to check for their dependencies.
            let mut visited = HashSet::new();
            for i in 0..self.nodes.len() {
                if self.nodes[i].in_workspace {
                    self.link_dependencies(i, &mut visited);
                }
            }
    
            // At the start all contents found in repos were given a node.
            // Now we're going to remove nodes that are not needed anymore (the ones not used by workspace's projects).
            // Since we've built dependencies only for workspace contents, the contents that do not have any dependants are technically unnecessary.
            let unneeded_content_paths: Vec<_> = self.nodes.iter()
                .enumerate()
                .filter(|(i, n)| !n.in_workspace && !self.edges.iter().any(|e| e.dependency_idx == *i))
                .map(|(_, n)| n.content.path().to_path_buf())
                .collect();
    
            for p in unneeded_content_paths {
                self.remove_node_by_path(&p);
            }
        }

        let new_content_paths: HashSet<_> = self.nodes.iter()
            .map(|n| n.content.path().to_owned())
            .collect();

        let mut diff = ContentGraphDifference::default();
        diff.added.extend(new_content_paths.difference(&prev_content_paths).cloned());
        diff.removed.extend(prev_content_paths.difference(&new_content_paths).cloned());

        //FIXME when manifest is modified it is not registered as a modification to the graph
        diff
    }


    pub fn get_node_by_path(&self, content_path: &Path) -> Option<&GraphNode> {
        self.get_node_index_by_path(content_path).map(|i| &self.nodes[i])
    }


    /// Iterate over content nodes that are dependencies to the specified content starting from it.
    pub fn walk_dependencies<'g>(&'g self, content_path: &Path) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.relatives_indices_in_direction(idx, GraphEdgeDirection::Dependencies);
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }

    /// Iterate over content nodes that are dependants of the specified content starting from it.
    pub fn walk_dependants<'g>(&'g self, content_path: &Path) -> Iter<'g> {
        if let Some(idx) = self.get_node_index_by_path(content_path) {
            let indices = self.relatives_indices_in_direction(idx, GraphEdgeDirection::Dependants);
            Iter::new(self, indices)
        } else {
            Iter::new(self, vec![])
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &GraphNode> {
        self.nodes.iter()
    }



    /// Returns index of the node if it was inserted successfully
    fn create_node_for_content(&mut self, content: Box<dyn Content>, in_repository: bool, in_workspace: bool) {
        if self.get_node_index_by_path(content.path()).is_some() {
            // node has already been made for this content
            return;
        }

        self.insert_node(GraphNode { 
            content,
            in_workspace, 
            in_repository,
        });
    }

    fn link_dependencies(&mut self, node_idx: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&node_idx) {
            return;
        }

        visited.insert(node_idx);

        if let Some(dependencies) = self.nodes[node_idx].content.dependencies().cloned() {
            for entry in dependencies.into_iter() {
                match entry.value.inner() {
                    DependencyValue::FromRepo(active) => {
                        self.link_dependencies_value_from_repo(node_idx, visited, &entry.name, entry.name.range(), *active);
                    },
                    DependencyValue::FromPath { path } => {
                        let final_path = if path.is_relative() {
                            self.nodes[node_idx].content.path().join(path)
                        } else {
                            path.clone()
                        };

                        self.link_dependencies_value_from_path(node_idx, visited, &final_path, entry.value.range());
                    },
                }
            }
        }
    }

    fn link_dependencies_value_from_repo(&mut self, 
        node_idx: usize, 
        visited: &mut HashSet<usize>, 
        dependency_name: &str,
        dependency_name_range: &lsp::Range,
        active: bool
    ) {
        if active {
            match self.get_node_index_by_name(&dependency_name) {
                Ok(dep_idx) => {
                    self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                    self.link_dependencies(dep_idx, visited);
                },
                Err(dep_count) => {
                    if dep_count == 0 {
                        self.errors.push(ContentGraphError::DependencyNameNotFound { 
                            content_name: dependency_name.to_string(), 
                            manifest_path: self.nodes[node_idx].content.path().to_path_buf(), 
                            manifest_range: dependency_name_range.clone()
                        });
                    } else {
                        self.errors.push(ContentGraphError::MultipleMatchingDependencies { 
                            content_name: dependency_name.to_string(), 
                            manifest_path: self.nodes[node_idx].content.path().to_path_buf(), 
                            manifest_range: dependency_name_range.clone()
                        });
                    }
                }
            }
        }
    }

    fn link_dependencies_value_from_path(&mut self, 
        node_idx: usize, 
        visited: &mut HashSet<usize>,
        dependency_path: &Path,
        dependency_path_range: &lsp::Range
    ) {
        let dependant_path = self.nodes[node_idx].content.path().to_path_buf();
        let final_dependency_path = if dependency_path.is_absolute() {
            dependency_path.canonicalize()
        } else {
            dependant_path.join(&dependency_path).canonicalize()
        };

        match final_dependency_path {
            Ok(dep_path) => {
                if let Some(dep_idx) = self.get_node_index_by_path(&dep_path) {
                    self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                    self.link_dependencies(dep_idx, visited);
                } else {
                    match try_make_content(&dep_path) {
                        Ok(content) => {
                            let dep_idx = self.insert_node(GraphNode { 
                                content, 
                                in_workspace: false, 
                                in_repository: false
                            });

                            self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                            self.link_dependencies(dep_idx, visited);
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
                                        content_path: dep_path, 
                                        manifest_path: dependant_path.to_path_buf(),
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
                    manifest_path: dependant_path.to_path_buf(),
                    manifest_range: dependency_path_range.clone()
                })
            }
        }
    }




    /// Returns the index of this node
    fn insert_node(&mut self, node: GraphNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    /// Changes node indices. Be aware!
    fn remove_node_by_path(&mut self, content_path: &Path) {
        if let Some(target_idx) = self.get_node_index_by_path(content_path) {
            // first remove all edges that mention this node
            self.edges.retain(|edge| edge.dependant_idx != target_idx && edge.dependency_idx != target_idx);

            let last_idx = self.nodes.len() - 1;
            if self.nodes.len() > 1 && target_idx != last_idx {
                // swap this and the last node to retain the same indices for all but these swapped nodes
                self.nodes.swap(target_idx, last_idx);
                
                // fix references to the swapped edge
                self.edges.iter_mut()
                    .for_each(|edge| { 
                        if edge.dependant_idx == last_idx {
                            edge.dependant_idx = target_idx;
                        }
                        if edge.dependency_idx == last_idx {
                            edge.dependency_idx = target_idx;
                        }
                    });

                self.edges.sort();
            }

            // remove the last element
            // if we did a swap it is the node we've been intending to remove
            self.nodes.pop();
        }
    }

    fn get_node_index_by_path(&self, path: &Path) -> Option<usize> {
        for (i, n) in self.nodes.iter().enumerate() {
            if n.content.path() == path {
                return Some(i)
            }
        }

        None
    }

    /// If there is just one content with the name returns Ok with the index.
    /// Otherwise returns Err with the number of contents encountered with that name.
    /// So if it wasn't found returns Err(0) or if more than one with than name was found returns Err(2) for example. 
    fn get_node_index_by_name(&self, name: &str) -> Result<usize, usize> {
        let mut candidates = Vec::new();
        for (i, n) in self.nodes.iter().enumerate() {
            if n.content.content_name() == name {
                candidates.push(i);
            }
        }

        let candidates_len = candidates.len();
        if candidates_len == 0 {
            Err(0)
        } else if candidates_len == 1 {
            Ok(candidates[0])
        } else {
            Err(candidates_len)
        }
    }

    fn insert_edge(&mut self, edge: GraphEdge) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }


    fn relatives_indices_in_direction(&self, starting_idx: usize, direction: GraphEdgeDirection) -> Vec<usize> {
        let mut indices = Vec::with_capacity(self.nodes.capacity());

        indices.push(starting_idx);

        let mut i = 0;
        while i < indices.len() {
            let current_idx = indices[i];
            let neighbours = self.edges.iter()
                .filter(|edge| {
                    current_idx == match direction {
                        GraphEdgeDirection::Dependants => edge.dependency_idx,
                        GraphEdgeDirection::Dependencies => edge.dependant_idx,
                    }
                })
                .map(|edge| {
                    match direction {
                        GraphEdgeDirection::Dependants => edge.dependant_idx,
                        GraphEdgeDirection::Dependencies => edge.dependency_idx,
                    }
                });

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
    i: usize
}

impl<'g> Iter<'g> {
    fn new(graph: &'g ContentGraph, node_indices: Vec<usize>) -> Self {
        Self {
            graph,
            node_indices,
            i: 0
        }
    }
}

impl<'g> Iterator for Iter<'g> {
    type Item = &'g GraphNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.node_indices.len() {
            let n = &self.graph.nodes[self.i];
            self.i += 1;
            Some(n)
        } else {
            None
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct ContentGraphDifference {
    pub added: Vec<PathBuf>,
    pub removed: Vec<PathBuf>
}

impl ContentGraphDifference {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::content::ProjectDirectory;
use crate::{Content, ContentRepositories};
use crate::manifest::{DependencyValue, ManifestError};


#[derive(Debug, Clone, Error)]
pub enum ContentGraphError {
    #[error(transparent)]
    ManifestError(#[from] ManifestError),
    #[error("Content could not be found in this path")]
    ContentPathNotFound {
        path: PathBuf,
        origin: Option<PathBuf>
    },
    #[error("Content could not be found with this name")]
    ContentNameNotFound {
        name: String,
        origin: Option<PathBuf>
    },
    #[error("There are multiple matching contents for this content name")]
    MultipleMatchingContents {
        name: String,
        origin: Option<PathBuf>
    },
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
struct GraphNode {
    content: Box<dyn Content>,
    in_workspace: bool,
    in_repository: bool,
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

    /// Set paths to contents from the workspace that should be actively monitored
    pub fn set_workspace_projects(&mut self, contents: Vec<Box<ProjectDirectory>>) {
        self.workspace_projects = contents;
    }


    pub fn build(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.errors.clear();

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
                self.build_dependency_connections(i, &mut visited);
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

    /// Visits all content nodes that are dependencies to the specified content.
    pub fn walk_dependencies(&self, content: &impl Content, visitor: impl FnMut(usize)) {
        if let Ok(idx) = self.get_node_index_by_path(content.path(), None) {
            self.walk_by_index(idx, GraphEdgeDirection::Dependencies, visitor);
        }
    }

    /// Visits all content nodes that are dependants to the specified content.
    pub fn walk_dependants(&self, content: &impl Content, visitor: impl FnMut(usize)) {
        if let Ok(idx) = self.get_node_index_by_path(content.path(), None) {
            self.walk_by_index(idx, GraphEdgeDirection::Dependants, visitor);
        }
    }



    /// Returns index of the node if it was inserted successfully
    fn create_node_for_content(&mut self, content: Box<dyn Content>, in_repository: bool, in_workspace: bool) {
        if content.path().exists() {
            if self.get_node_index_by_path(content.path(), None).is_ok() {
                // node has already been made for this content
                return;
            }

            self.insert_node(GraphNode { 
                content,
                in_workspace, 
                in_repository,
            });
        } else {
            self.errors.push(ContentGraphError::ContentPathNotFound { 
                path: content.path().to_path_buf(),
                origin: None
            });
        }
    }

    fn build_dependency_connections(&mut self, node_idx: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&node_idx) {
            return;
        }

        visited.insert(node_idx);

        if let Some(dependencies) = self.nodes[node_idx].content.dependencies().cloned() {
            for (dep_name, dep_val) in dependencies {
                match dep_val {
                    DependencyValue::FromRepo(active) => {
                        if active {
                            match self.get_node_index_by_name(&dep_name, Some(self.nodes[node_idx].content.path())) {
                                Ok(dep_idx) => {
                                    self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                                    self.build_dependency_connections(dep_idx, visited);
                                },
                                Err(err) => {
                                    self.errors.push(err);
                                }
                            }
                        }
                    },
                    DependencyValue::FromPath { path } => {
                        match self.nodes[node_idx].content.path().join(&path).canonicalize() {
                            Ok(path) => {
                                match self.get_node_index_by_path(&path, Some(self.nodes[node_idx].content.path())) {
                                    Ok(dep_idx) => {
                                        self.insert_edge(GraphEdge { dependant_idx: node_idx, dependency_idx: dep_idx });
                                        self.build_dependency_connections(dep_idx, visited);
                                    },
                                    Err(err) => {
                                        self.errors.push(err);
                                    }
                                }
                            },
                            Err(_) => {
                                self.errors.push(ContentGraphError::ContentPathNotFound { 
                                    path, 
                                    origin: Some(self.nodes[node_idx].content.path().to_path_buf())
                                })
                            }
                        }
                    },
                }
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
        if let Ok(target_idx) = self.get_node_index_by_path(content_path, None) {
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

    fn get_node_index_by_path(&self, path: &Path, dependant: Option<&Path>) -> Result<usize, ContentGraphError> {
        for (i, n) in self.nodes.iter().enumerate() {
            if n.content.path() == path {
                return Ok(i)
            }
        }

        Err(ContentGraphError::ContentPathNotFound { 
            path: path.to_path_buf(),
            origin: dependant.map(|p| p.to_path_buf())
        })
    }

    fn get_node_index_by_name(&self, name: &str, dependant: Option<&Path>) -> Result<usize, ContentGraphError> {
        let mut candidates = Vec::new();
        for (i, n) in self.nodes.iter().enumerate() {
            if n.content.content_name() == name {
                candidates.push(i);
            }
        }

        if candidates.len() == 0 {
            Err(ContentGraphError::ContentNameNotFound { 
                name: name.to_string(),
                origin: dependant.map(|p| p.to_path_buf())
            })
        } else if candidates.len() == 1 {
            Ok(candidates[0])
        } else {
            Err(ContentGraphError::MultipleMatchingContents { 
                name: name.to_string(),
                origin: dependant.map(|p| p.to_path_buf())
            })
        }
    }

    fn insert_edge(&mut self, edge: GraphEdge) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }

    /// Visits every node according to the given direction starting from the specified node.
    fn walk_by_index(&self, node_idx: usize, direction: GraphEdgeDirection, mut visitor: impl FnMut(usize)) {
        let mut visited = HashSet::new();

        visitor(node_idx);
        visited.insert(node_idx);
        self._walk_by_index(node_idx, direction, &mut visitor, &mut visited);
    }

    fn _walk_by_index(&self, node_idx: usize, direction: GraphEdgeDirection, visitor: &mut impl FnMut(usize), visited: &mut HashSet<usize>) {
        let edge_iter = self.edges.iter().filter(|edge| {
            let current_idx = match direction {
                GraphEdgeDirection::Dependants => edge.dependency_idx,
                GraphEdgeDirection::Dependencies => edge.dependant_idx,
            };

            current_idx == node_idx
        });

        for edge in edge_iter {
            let target_idx = match direction {
                GraphEdgeDirection::Dependants => edge.dependant_idx,
                GraphEdgeDirection::Dependencies => edge.dependency_idx,
            };

            if !visited.contains(&target_idx) {
                visitor(target_idx);
                visited.insert(target_idx);
                self._walk_by_index(target_idx, direction, visitor, visited);
            }
        }
    }
}

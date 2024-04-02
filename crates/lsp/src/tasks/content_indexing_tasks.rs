use std::collections::HashMap;
use tokio::time::Instant;
use abs_path::AbsPath;
use witcherscript_project::content::{ContentScanError, ProjectDirectory};
use witcherscript_project::source_tree::SourceTreeDifference;
use witcherscript_project::{ContentGraph, ContentScanner, FileError};
use witcherscript_project::content_graph::{ContentGraphDifference, ContentGraphError, GraphEdgeWithContent, GraphNode};
use crate::{reporting::IntoLspDiagnostic, Backend};


impl Backend {
    pub async fn setup_workspace_content_scanners(&self, content_graph: &mut ContentGraph) {
        let workspace_roots = self.workspace_roots.read().await;

        content_graph.clear_workspace_scanners();

        for root in workspace_roots.iter() {
            let scanner = 
                ContentScanner::new(root.clone()).unwrap()
                .recursive(true)
                .only_projects(true);

            content_graph.add_workspace_scanner(scanner);
        }
    }

    pub async fn setup_repository_content_scanners(&self, content_graph: &mut ContentGraph) {
        let config = self.config.read().await;

        let mut repo_paths = Vec::new();
        
        for repo in &config.project_repositories {
            repo_paths.push(repo.clone());
        }
        
        repo_paths.push(config.game_directory.join("content"));
        repo_paths.push(config.game_directory.join("Mods"));


        content_graph.clear_repository_scanners();

        for repo in repo_paths {
            if !repo.as_os_str().is_empty() {
                match AbsPath::resolve(&repo, None) {
                    Ok(abs_repo) => {
                        match ContentScanner::new(abs_repo) {
                            Ok(scanner) => {
                                let scanner = scanner.recursive(false).only_projects(false);
                                content_graph.add_repository_scanner(scanner);
                            },
                            Err(err) => {
                                self.report_content_scan_error(err).await;
                            },
                        }
                    }
                    Err(_) => {
                        self.reporter.log_error(format!("Invalid project repository path: {}", repo.display())).await;
                    }
                }
            }
        }
    }
    
    pub async fn build_content_graph(&self, content_graph: &mut ContentGraph) {
        self.reporter.log_info("Building content graph...").await;

        self.reporter.clear_all_diagnostics();
        
        let diff = content_graph.build();
    
        if !content_graph.errors.is_empty() {
            for err in &content_graph.errors {
                self.report_content_graph_error(err.clone()).await;
            }
        }

        if !diff.is_empty() {
            self.on_content_graph_changed(diff).await;
        } else {
            self.reporter.log_info("Found no content graph changes.").await;
        }

        self.run_script_analysis_for_all().await;
    }

    pub async fn on_content_graph_changed(&self, diff: ContentGraphDifference) {
        let (added_node_count, removed_node_count) = (diff.added_nodes.len(), diff.removed_nodes.len());
        self.reporter.log_info(format!("Changes to the content graph: {} content(s) discovered, {} to be deprecated", added_node_count, removed_node_count)).await;

        let start = Instant::now();

        let (diff_nodes_added, diff_nodes_removed, diff_edges_added, diff_edges_removed) = (diff.added_nodes, diff.removed_nodes, diff.added_edges, diff.removed_edges);
        self.on_graph_nodes_removed(diff_nodes_removed).await;
        self.on_graph_nodes_added(diff_nodes_added).await;
        self.on_graph_edges_added(diff_edges_added).await;
        self.on_graph_edges_removed(diff_edges_removed).await;

        let duration = Instant::now() - start;
        self.reporter.log_info(format!("Handled content graph related changes in {:.3}s", duration.as_secs_f32())).await;
    }

    async fn on_graph_nodes_added(&self, added_nodes: Vec<GraphNode>) {
        let mut source_tree_diffs = HashMap::new();

        for added_node in added_nodes {
            let added_content = added_node.content;
            let added_content_path = added_content.path();

            self.reporter.log_info(format!("Discovered content \"{}\" in {}", added_content.content_name(), added_content_path)).await; 

            let source_tree = added_content.source_tree();

            if !source_tree.errors.is_empty() {
                for err in &source_tree.errors {
                    self.report_source_tree_scan_error(err.clone()).await;
                }
            }

            source_tree_diffs.insert(added_content_path.to_owned(), SourceTreeDifference {
                added: source_tree.iter().cloned().collect(),
                removed: vec![],
                modified: vec![]
            });

            self.source_trees.insert(added_content_path.clone(), source_tree);
        }

        // handling source tree changes in a seperate step to not lock resources for too long
        for (content_path, diff) in source_tree_diffs {
            self.on_source_tree_changed(&content_path, diff, false).await;
        }
    }

    async fn on_graph_nodes_removed(&self, removed_nodes: Vec<GraphNode>) {
        let mut source_tree_diffs = HashMap::new();
        for removed_node in removed_nodes {
            let removed_content = removed_node.content;
            let removed_content_path = removed_content.path();

            self.reporter.log_info(format!("Deprecated content \"{}\" from {}", removed_content.content_name(), removed_content_path)).await;

            if let Some((_, source_tree)) = self.source_trees.remove(removed_content_path) {
                source_tree_diffs.insert(removed_content_path.to_owned(), SourceTreeDifference {
                    added: vec![],
                    removed: source_tree.into_iter().collect(),
                    modified: vec![]
                });
            }

            if let Some(project) = removed_content.as_any().downcast_ref::<ProjectDirectory>() {
                self.reporter.purge_diagnostics(project.manifest_path());
            }
        }

        for (content_path, diff) in source_tree_diffs {
            self.on_source_tree_changed(&content_path, diff, false).await;
        }
    }

    async fn on_graph_edges_added(&self, added_edges: Vec<GraphEdgeWithContent>) {
        for added_edge in added_edges {
            self.reporter.log_info(format!(
                "Established \"{}\" as dependency for content {}", 
                added_edge.dependency_content.content_name(), 
                added_edge.dependant_content.path()
            )).await;
        }
    }

    async fn on_graph_edges_removed(&self, removed_edges: Vec<GraphEdgeWithContent>) {
        for removed_edge in removed_edges {
            self.reporter.log_info(format!(
                "Removed \"{}\" from dependencies of content {}", 
                removed_edge.dependency_content.content_name(), 
                removed_edge.dependant_content.path()
            )).await;
        }
    }


    async fn report_content_scan_error(&self, err: ContentScanError) {
        match err {
            ContentScanError::Io(err) => {
                self.reporter.log_warning(format!("Content scanning issue for {}: {}", err.path.display(), err.error)).await;
            },
            ContentScanError::ManifestParse(err) => {
                self.reporter.push_diagnostic(&err.path, err.clone().into_lsp_diagnostic());
            },
            ContentScanError::NotContent => {},
        }
    }

    async fn report_content_graph_error(&self, err: ContentGraphError) {
        let err_str = err.to_string();
        match err {
            ContentGraphError::Io(err) => {
                self.reporter.log_warning(format!("Content scanning issue at {}: {}", err.path.display(), err.error)).await;
            },
            ContentGraphError::ManifestParse(err) => {
                self.reporter.push_diagnostic(&err.path, err.clone().into_lsp_diagnostic());
            },
            ContentGraphError::DependencyPathNotFound { content_path: _, manifest_path, manifest_range } => {
                self.reporter.push_diagnostic(&manifest_path, (err_str, manifest_range).into_lsp_diagnostic());
            },
            ContentGraphError::DependencyNameNotFound { content_name: _, manifest_path, manifest_range } => {
                self.reporter.push_diagnostic(&manifest_path, (err_str, manifest_range).into_lsp_diagnostic());
            },
            ContentGraphError::DependencyNameNotFoundAtPath { content_name: _, manifest_path, manifest_range } => {
                self.reporter.push_diagnostic(&manifest_path, (err_str, manifest_range).into_lsp_diagnostic());
            },
            ContentGraphError::MultipleMatchingDependencies { content_name: _, manifest_path, manifest_range } => {
                self.reporter.push_diagnostic(&manifest_path, (err_str, manifest_range).into_lsp_diagnostic());
            }
        }
    }

    async fn report_source_tree_scan_error(&self, err: FileError<std::io::Error>) {
        self.reporter.log_warning(format!("Source tree scanning issue for {}: {}", err.path.display(), err.error)).await
    }
}
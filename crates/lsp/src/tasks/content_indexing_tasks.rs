use std::collections::HashMap;
use tokio::time::Instant;
use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript_analysis::symbol_analysis::symbol_table::SymbolTable;
use witcherscript_diagnostics::*;
use witcherscript_project::content::{ContentScanError, ProjectDirectory, RedkitProjectDirectory};
use witcherscript_project::source_tree::SourceTreeDifference;
use witcherscript_project::{ContentGraph, ContentScanner, FileError};
use witcherscript_project::content_graph::{ContentGraphDifference, ContentGraphError, GraphEdgeWithContent, GraphNode};
use crate::Backend;


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
        
        for repo in &config.content_repositories {
            repo_paths.push(repo.clone());
        }
        
        if !config.game_directory.as_os_str().is_empty() {
            repo_paths.push(config.game_directory.join("content"));
            repo_paths.push(config.game_directory.join("Mods"));
        }

        content_graph.clear_repository_scanners();

        if repo_paths.is_empty() {
            self.reporter.show_warning_notification("No WitcherScript content repository paths have have been configured").await;
            return;
        }

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
                        self.reporter.log_error(format!("Invalid content repository path: {}", repo.display())).await;
                    }
                }
            }
        }
    }
    
    pub async fn build_content_graph(&self, content_graph: &mut ContentGraph) {
        self.reporter.log_info("Building content graph...").await;

        self.reporter.clear_all_diagnostics().await;
        
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

    async fn on_content_graph_changed(&self, diff: ContentGraphDifference) {
        let (added_node_count, removed_node_count) = (diff.added_nodes.len(), diff.removed_nodes.len());
        self.reporter.log_info(format!("Changes to the content graph: {} content(s) discovered, {} to be deprecated", added_node_count, removed_node_count)).await;

        let start = Instant::now();

        let (diff_nodes_added, diff_nodes_removed, diff_edges_added, diff_edges_removed) = (diff.added_nodes, diff.removed_nodes, diff.added_edges, diff.removed_edges);
        if !diff_nodes_removed.is_empty() {
            self.on_graph_nodes_removed(diff_nodes_removed).await;
        }
        if !diff_nodes_added.is_empty() {
            self.on_graph_nodes_added(diff_nodes_added).await;
        }
        if !diff_edges_added.is_empty() {
            self.on_graph_edges_added(diff_edges_added).await;
        }
        if !diff_edges_removed.is_empty() {
            self.on_graph_edges_removed(diff_edges_removed).await;
        }

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

            let scripts_root = source_tree.script_root_arc();
            self.source_trees.insert(added_content_path.clone(), source_tree);

            let mut symtabs = self.symtabs.write().await;
            symtabs.insert(added_content_path.clone(), SymbolTable::new(scripts_root));
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

            let mut symtabs = self.symtabs.write().await;
            symtabs.remove(removed_content_path);
            
            if !removed_content_path.exists() || !removed_node.in_workspace {
                let mut manifest_path = None;
                if let Some(project) = removed_content.as_any().downcast_ref::<ProjectDirectory>() {
                    manifest_path = Some(project.manifest_path());
                } else if let Some(redkit_proj) = removed_content.as_any().downcast_ref::<RedkitProjectDirectory>() {
                    manifest_path = Some(redkit_proj.manifest_path());
                }

                if let Some(manifest_path) = manifest_path {
                    self.reporter.purge_diagnostics(manifest_path).await;
                }
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
                self.reporter.log_warning(format!("Content scanning issue for \"{}\": {}", err.path, err.error)).await;
            },
            ContentScanError::ManifestRead(err) => {
                self.report_manifest_read_error(err).await;
            },
            ContentScanError::RedkitManifestRead(err) => {
                self.report_redkit_manifest_read_error(err).await;
            },
            ContentScanError::NotContent => {},
        }
    }

    async fn report_content_graph_error(&self, err: ContentGraphError) {
        match err {
            ContentGraphError::Io(err) => {
                let (path, err) = (err.path, err.error);
                self.reporter.log_warning(format!("Content graph building issue for \"{path}\": {err}")).await;
            },
            ContentGraphError::ManifestRead(err) => {
                self.report_manifest_read_error(err).await;
            },
            ContentGraphError::RedkitManifestRead(err) => {
                self.report_redkit_manifest_read_error(err).await;
            },
            ContentGraphError::DependencyPathNotFound { content_path, manifest_path, manifest_range } => {
                self.reporter.push_diagnostic(&manifest_path, Diagnostic {
                    range: manifest_range,
                    kind: DiagnosticKind::ProjectDependencyPathNotFound(content_path)
                }).await;
            },
            ContentGraphError::DependencyNameNotFound { content_name, manifest_path, manifest_range, .. } => {
                self.reporter.push_diagnostic(&manifest_path, Diagnostic {
                    range: manifest_range,
                    kind: DiagnosticKind::ProjectDependencyNameNotFound(content_name)
                }).await;
            },
            ContentGraphError::DependencyNameNotFoundAtPath { content_name, manifest_path, manifest_range } => {
                self.reporter.push_diagnostic(&manifest_path, Diagnostic {
                    range: manifest_range,
                    kind: DiagnosticKind::ProjectDependencyNameNotFoundAtPath(content_name)
                }).await;
            },
            ContentGraphError::MultipleMatchingDependencies { content_name, manifest_path, manifest_range, matching_paths } => {
                self.reporter.push_diagnostic(&manifest_path, Diagnostic {
                    range: manifest_range,
                    kind: DiagnosticKind::MultipleMatchingProjectDependencies { content_name, matching_paths }
                }).await;
            }
        }
    }

    async fn report_manifest_read_error(&self, err: FileError<witcherscript_project::manifest::Error>) {
        let (path, err) = (err.path, err.error);
        match err.as_ref() {
            witcherscript_project::manifest::Error::Io(io_err) => {
                self.reporter.log_error(format!("Error reading project manifest at \"{path}\": {io_err}")).await;
            },
            witcherscript_project::manifest::Error::Toml { range, msg } => {
                self.reporter.push_diagnostic(&path, Diagnostic {
                    range: *range,
                    kind: DiagnosticKind::InvalidProjectManifest(msg.to_owned())
                }).await;
            },
            witcherscript_project::manifest::Error::InvalidNameField { range } => {
                self.reporter.push_diagnostic(&path, Diagnostic {
                    range: *range,
                    kind: DiagnosticKind::InvalidProjectName
                }).await;
            },
        }
    }

    async fn report_redkit_manifest_read_error(&self, err: FileError<witcherscript_project::redkit::manifest::Error>) {
        let (path, err) = (err.path, err.error);
        match err.as_ref() {
            witcherscript_project::redkit::manifest::Error::Io(io_err) =>{
                self.reporter.log_error(format!("Error reading REDKit project manifest at \"{path}\": {io_err}")).await;
            },
            witcherscript_project::redkit::manifest::Error::Json { position, msg } => {
                self.reporter.push_diagnostic(&path, Diagnostic {
                    range: lsp::Range::new(*position, *position),
                    kind: DiagnosticKind::InvalidRedkitProjectManifest(msg.to_owned())
                }).await;
            },
        }   
    }

    async fn report_source_tree_scan_error(&self, err: FileError<std::io::Error>) {
        self.reporter.log_error(format!("Source tree scanning issue for {}: {}", err.path, err.error)).await
    }
}
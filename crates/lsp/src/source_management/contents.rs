use std::collections::HashMap;
use std::path::PathBuf;
use witcherscript_project::content::{ContentScanError, ProjectDirectory, find_content_in_directory};
use witcherscript_project::source_tree::SourceTreeDifference;
use witcherscript_project::{Content, ContentRepositories, FileError};
use witcherscript_project::content_graph::{ContentGraphDifference, ContentGraphError};
use crate::{reporting::IntoLspDiagnostic, Backend};


impl Backend {
    pub async fn scan_workspace_projects(&self) {
        self.log_info("Scanning workspace projects...").await;

        let mut projects = Vec::new();
    
        let workspace_roots = self.workspace_roots.read().await;
        for root in workspace_roots.iter() {
            let (contents, errors) = find_content_in_directory(root, true);
        
            for content in contents {
                if let Ok(proj) = content.as_any().downcast::<ProjectDirectory>() { 
                    projects.push(proj);
                }
            }
        
            for err in errors {
                self.report_content_scan_error(err).await;    
            }
        }

        if projects.is_empty() {
            self.log_info("Found no projects in the workspace.").await;
        } else {
            for proj in &projects {
                self.log_info(format!("Found project {}", proj.content_name())).await;
            }
        }
    
        let mut lock = self.content_graph.write().await;
        lock.set_workspace_projects(projects);
    } 
    
    pub async fn scan_content_repositories(&self) {
        self.log_info("Scanning content repositories...").await;

        let mut repos = ContentRepositories::new();
    
        let config = self.config.read().await;
        for repo in &config.project_repositories {
            if !repo.as_os_str().is_empty() {
                repos.add_repository(&repo);
            }
        }
        if !config.game_directory.as_os_str().is_empty() {
            repos.add_repository(config.game_directory.join("content"));
            repos.add_repository(config.game_directory.join("Mods"));
        }
    
        repos.scan();
    
        for err in &repos.errors {
            self.report_content_scan_error(err.clone()).await;    
        }

        if repos.found_content().is_empty() {
            self.log_info("Found no script contents in repositories.").await;
        } else {
            for content in repos.found_content() {
                self.log_info(format!("Found script content {}", content.content_name())).await;
            }
        }
    
        let mut graph = self.content_graph.write().await;
        graph.set_repositories(repos);
    }
    
    pub async fn build_content_graph(&self) {
        self.log_info("Building content graph...").await;

        let mut graph = self.content_graph.write().await;
        let diff = graph.build();
    
        if !graph.errors.is_empty() {
            for err in &graph.errors {
                self.report_content_graph_error(err.clone()).await;
            }
        }

        self.log_info("Content graph built.").await;

        drop(graph);

        if !diff.is_empty() {
            self.on_content_graph_changed(diff).await;
        } else {
            self.log_info("Found no changes.").await;
        }
    }

    pub async fn on_content_graph_changed(&self, diff: ContentGraphDifference) {
        let (diff_added, diff_removed) = (diff.added, diff.removed);
        self.on_graph_contents_removed(diff_removed).await;
        self.on_graph_contents_added(diff_added).await;
    }

    async fn on_graph_contents_added(&self, added_content_paths: Vec<PathBuf>) {
        let graph = self.content_graph.read().await;
        
        let mut source_tree_diffs = HashMap::new();
        for added_path in added_content_paths {

            let added_content = &graph.get_node_by_path(&added_path).unwrap().content;
            let source_tree = added_content.source_tree();

            if !source_tree.errors.is_empty() {
                for err in &source_tree.errors {
                    self.report_source_tree_scan_error(err.clone()).await;
                }
            }

            source_tree_diffs.insert(added_path.clone(), SourceTreeDifference {
                added: source_tree.iter().cloned().collect(),
                removed: vec![]
            });

            self.source_trees.insert(added_path.clone(), source_tree);

            self.log_info(format!("Found new content: {}", added_path.display())).await; 
        }

        drop(graph);

        for (content_path, diff) in source_tree_diffs {
            let script_count = diff.added.len();
            self.on_source_tree_changed(&content_path, diff).await;
            self.log_info(format!("Found in total {} scripts in {}", script_count, content_path.display())).await;
        }
    }

    async fn on_graph_contents_removed(&self, removed_content_paths: Vec<PathBuf>) {
        for removed_path in removed_content_paths {
            self.source_trees.remove(&removed_path);
            self.log_info(format!("Delisted deprecated content: {}", removed_path.display())).await;
        }   
    }


    async fn report_content_scan_error(&self, err: ContentScanError) {
        match err {
            ContentScanError::Io(err) => {
                self.log_warning(format!("Content scanning issue for {}: {}", err.path.display(), err.error)).await;
            },
            ContentScanError::ManifestParse(err) => {
                self.publish_diagnostics(err.path.clone(), [err.into_lsp_diagnostic()]).await;
            },
            ContentScanError::NotContent => {},
        }
    }

    async fn report_content_graph_error(&self, err: ContentGraphError) {
        let err_str = err.to_string();
        match err {
            ContentGraphError::Io(err) => {
                self.log_warning(format!("Content scanning issue at {}: {}", err.path.display(), err.error)).await;
            },
            ContentGraphError::ManifestParse(err) => {
                self.publish_diagnostics(err.path.clone(), [err.into_lsp_diagnostic()]).await;
            },
            ContentGraphError::DependencyPathNotFound { content_path: _, manifest_path, manifest_range } => {
                self.publish_diagnostics(manifest_path, [(err_str, manifest_range).into_lsp_diagnostic()]).await;
            },
            ContentGraphError::DependencyNameNotFound { content_name: _, manifest_path, manifest_range } => {
                self.publish_diagnostics(manifest_path, [(err_str, manifest_range).into_lsp_diagnostic()]).await;
            },
            ContentGraphError::MultipleMatchingDependencies { content_name: _, manifest_path, manifest_range } => {
                self.publish_diagnostics(manifest_path, [(err_str, manifest_range).into_lsp_diagnostic()]).await;
            }
        }
    }

    async fn report_source_tree_scan_error(&self, err: FileError<std::io::Error>) {
        self.log_warning(format!("Source tree scanning issue for {}: {}", err.path.display(), err.error)).await
    }
}
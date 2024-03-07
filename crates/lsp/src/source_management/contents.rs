use witcherscript_project::content::{ContentScanError, ProjectDirectory, find_content_in_directory};
use witcherscript_project::{Content, ContentRepositories};
use witcherscript_project::content_graph::ContentGraphError;
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
        graph.build();
    
        if !graph.errors.is_empty() {
            self.report_content_graph_errors(graph.errors.clone()).await;
        }
    }



    async fn report_content_scan_error(&self, err: ContentScanError) {
        match err {
            ContentScanError::Io(err) => {
                self.log_warning(format!("Content scanning issue at {}: {}", err.path.display(), err.error)).await;
            },
            ContentScanError::ManifestParse(err) => {
                self.publish_diagnostics(err.path.clone(), [err.into_lsp_diagnostic()]).await;
            },
            ContentScanError::NotContent => {},
        }
    }

    async fn report_content_graph_errors(&self, errors: Vec<ContentGraphError>) {
        for err in errors {
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
    }
}
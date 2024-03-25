use abs_path::AbsPath;
use tower_lsp::lsp_types as lsp;
use crate::Backend;


pub async fn did_change_workspace_folders(backend: &Backend, params: lsp::DidChangeWorkspaceFoldersParams) {
    let added: Vec<_> = params.event.added.into_iter()
        .map(|f| AbsPath::try_from(f.uri).unwrap())
        .collect();

    let removed: Vec<_> = params.event.removed.into_iter()
        .map(|f| AbsPath::try_from(f.uri).unwrap())
        .collect();

    {
        let mut workspace_roots = backend.workspace_roots.write().await;
        workspace_roots.retain(|p| !removed.contains(p));
        workspace_roots.extend(added);
    }

    backend.setup_workspace_content_scanners().await;
    backend.build_content_graph().await;
}
use tower_lsp::lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript_diagnostics::{Diagnostic, DiagnosticDomain};
use super::Reporter;


#[derive(Debug)]
pub struct BufferedDiagnostics {
    diags: Vec<BufferedDiagnostic>,
    changed: bool,
    should_purge: bool
}

#[derive(Debug)]
struct BufferedDiagnostic {
    lsp_diag: lsp::Diagnostic,
    domain: DiagnosticDomain
}

impl Reporter {
    pub async fn push_diagnostic(&self, path: &AbsPath, diag: Diagnostic) {
        let bd = BufferedDiagnostic { domain: diag.kind.domain(), lsp_diag: diag.into() };
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.push(bd);
            v.changed = true;
        } else {
            diags.insert(path.clone(), BufferedDiagnostics {
                diags: vec![bd],
                changed: true,
                should_purge: false
            });
        }
    }

    pub async fn push_diagnostics(&self, path: &AbsPath, diags: impl IntoIterator<Item = Diagnostic>) {
        let bds = diags.into_iter().map(|diag| BufferedDiagnostic { domain: diag.kind.domain(), lsp_diag: diag.into() });
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.extend(bds);
            v.changed = true;
        } else {
            diags.insert(path.clone(), BufferedDiagnostics {
                diags: bds.collect(),
                changed: true,
                should_purge: false
            });
        }
    }

    pub async fn clear_diagnostics(&self, path: &AbsPath, domain: DiagnosticDomain) {
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.retain(|d| d.domain != domain);
            v.changed = true;
        }
    }

    /// Clears all diagnostics for a given file and additionally that file gets forgotten about by the diagnostic system
    pub async fn purge_diagnostics(&self, path: &AbsPath) {
        let mut diags = self.buffered_diagnostics.lock().await;
        if let Some(v) = diags.get_mut(path) {
            v.diags.clear();
            v.changed = true;
            v.should_purge = true;
        }
    }

    pub async fn clear_all_diagnostics(&self) {
        let mut diags = self.buffered_diagnostics.lock().await;
        for (_, v) in diags.iter_mut() {
            v.diags.clear();
            v.changed = true;
        }
    }


    pub async fn commit_diagnostics(&self, path: &AbsPath) {
        let mut diags = self.buffered_diagnostics.lock().await;
        let mut should_purge = false;
        if let Some(v) = diags.get_mut(path) {
            if v.changed {
                let uri = path.to_uri();
    
                let to_publish = v.diags.iter().map(|d| d.lsp_diag.clone()).collect();
                v.changed = false;
        
                self.client.publish_diagnostics(uri, to_publish, None).await;
            }

            should_purge = v.should_purge;            
        }

        if should_purge {
            diags.remove(path);
        }
    }

    pub async fn commit_all_diagnostics(&self) {
        let mut diags = self.buffered_diagnostics.lock().await;
        for (k, v) in diags.iter_mut().filter(|(_, v)| v.changed) {
            let uri = k.to_uri();

            let to_publish = v.diags.iter().map(|d| d.lsp_diag.clone()).collect();
            v.changed = false;

            self.client.publish_diagnostics(uri, to_publish, None).await;
        }

        diags.retain(|_, v| !v.should_purge);
    }
}

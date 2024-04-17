//! Utilities for sending informational messages to the client.

use tower_lsp::Client;
use abs_path::AbsPath;
use dashmap::DashMap;

mod diagnostics;
pub use diagnostics::*;

mod logging;

mod notifications;


#[derive(Debug)]
pub struct Reporter {
    client: Client,
    buffered_diagnostics: DashMap<AbsPath, BufferedDiagnostics>,
}

impl Reporter {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            buffered_diagnostics: DashMap::new()
        }
    }
}
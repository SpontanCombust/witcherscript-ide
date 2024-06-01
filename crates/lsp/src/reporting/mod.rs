//! Utilities for sending informational messages to the client.

use std::{collections::HashMap, sync::Mutex};
use tower_lsp::Client;
use abs_path::AbsPath;

mod diagnostics;
pub use diagnostics::*;

mod logging;

mod notifications;


#[derive(Debug)]
pub struct Reporter {
    client: Client,
    buffered_diagnostics: Mutex<HashMap<AbsPath, BufferedDiagnostics>>,
}

impl Reporter {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            buffered_diagnostics: Mutex::default()
        }
    }
}
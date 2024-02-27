use std::path::PathBuf;
use thiserror::Error;
use tower_lsp::{Client, jsonrpc};
use tower_lsp::lsp_types as lsp;
use crate::Backend;


#[derive(Debug, Clone, Default)]
pub struct Config {
    pub game_directory: PathBuf,
    pub project_repositories: Vec<PathBuf>
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Client returned an error")]
    ClientResponseError(#[from] jsonrpc::Error),
    #[error("Data coming from the client failed to deserialize")]
    DeserializationError(#[from] serde_json::Error)
}

impl Config {
    const CONFIG_ITEM_SECTIONS: [&str; 2] = [
        "witcherscript-ide.gameDirectory",
        "witcherscript-ide.projectRepositories"
    ];

    pub async fn fetch(client: &Client) -> Result<Self, ConfigError> {
        let config_items = Self::CONFIG_ITEM_SECTIONS.map(|section| lsp::ConfigurationItem {
            scope_uri: None,
            section: Some(section.to_string())
        }).to_vec();

        let values = client.configuration(config_items).await?;

        Ok(Self {
            game_directory: serde_json::from_value(values[0].clone())?,
            project_repositories: serde_json::from_value(values[1].clone())?
        })
    }
}


impl Backend {
    pub async fn fetch_config(&self) {
        self.log_info("Fetching configuration...").await;

        match Config::fetch(&self.client).await {
            Ok(config) => {
                let mut lock = self.config.write().await;
                *lock = config;
            },
            Err(err) => {
                self.show_error_notification(format!("Client configuration fetch error: {}", err)).await;
            },
        }
    }
}
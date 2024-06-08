use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tower_lsp::{Client, jsonrpc};
use tower_lsp::lsp_types as lsp;
use crate::Backend;


#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub game_directory: PathBuf,
    pub content_repositories: Vec<PathBuf>
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Client returned an error")]
    ClientResponseError(#[from] jsonrpc::Error),
    #[error("Data coming from the client failed to deserialize")]
    DeserializationError(#[from] serde_json::Error)
}

impl Config {
    const CONFIG_ITEM_SECTIONS: [&'static str; 2] = [
        "witcherscript-ide.gameDirectory",
        "witcherscript-ide.contentRepositories"
    ];

    pub async fn fetch(client: &Client) -> Result<Self, ConfigError> {
        let config_items = Self::CONFIG_ITEM_SECTIONS.map(|section| lsp::ConfigurationItem {
            scope_uri: None,
            section: Some(section.to_string())
        }).to_vec();

        let values = client.configuration(config_items).await?;

        Ok(Self {
            game_directory: serde_json::from_value(values[0].clone())?,
            content_repositories: serde_json::from_value(values[1].clone())?
        })
    }
}


impl Backend {
    pub async fn fetch_config(&self) -> ConfigDifference {
        self.reporter.log_info("Fetching configuration...").await;

        match Config::fetch(&self.client).await {
            Ok(new_config) => {
                let mut old_config = self.config.write().await;
                let diff = ConfigDifference::from_comparison(&old_config, &new_config);
                *old_config = new_config;
                drop(old_config);

                if !diff.any_changed() {
                    self.reporter.log_info("No changes to configuration detected.").await;
                }

                diff
            },
            Err(err) => {
                self.reporter.show_error_notification(format!("Client configuration fetch error: {}", err)).await;
                ConfigDifference::default()
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigDifference {
    pub game_directory_changed: bool,
    pub content_repositories_changed: bool
}

impl ConfigDifference {
    fn from_comparison(old_config: &Config, new_config: &Config) -> Self {
        let game_directory_changed = old_config.game_directory != new_config.game_directory;
        let content_repositories_changed = old_config.content_repositories != new_config.content_repositories;

        ConfigDifference {
            game_directory_changed,
            content_repositories_changed
        }
    }

    pub fn any_changed(&self) -> bool {
        self.game_directory_changed || 
        self.content_repositories_changed
    }
}

impl Default for ConfigDifference {
    fn default() -> Self {
        Self { 
            game_directory_changed: false, 
            content_repositories_changed: false 
        }
    }
}
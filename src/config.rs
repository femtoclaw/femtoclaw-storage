//! Configuration Store.
//!
//! Provides persistent storage for runtime configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub brain: BrainConfig,
    pub logging: LoggingConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainConfig {
    pub backend: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
}

impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            backend: "echo".to_string(),
            model: None,
            api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub path: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("/var/lib/femtoclaw"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            brain: BrainConfig::default(),
            logging: LoggingConfig::default(),
            storage: StorageConfig::default(),
        }
    }
}

pub struct ConfigStore {
    config: Config,
    path: Option<PathBuf>,
}

impl ConfigStore {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            path: None,
        }
    }

    pub fn load(path: PathBuf) -> Self {
        let config = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        Self {
            config,
            path: Some(path),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn brain(&self) -> &BrainConfig {
        &self.config.brain
    }

    pub fn logging(&self) -> &LoggingConfig {
        &self.config.logging
    }

    pub fn storage(&self) -> &StorageConfig {
        &self.config.storage
    }

    pub fn update_brain(
        &mut self,
        backend: String,
        model: Option<String>,
        api_key: Option<String>,
    ) {
        self.config.brain = BrainConfig {
            backend,
            model,
            api_key,
        };
    }

    pub fn update_logging(&mut self, level: String) {
        self.config.logging = LoggingConfig { level };
    }

    pub fn save(&self) -> std::io::Result<()> {
        if let Some(ref path) = self.path {
            let json = serde_json::to_string_pretty(&self.config)?;
            std::fs::write(path, json)?;
        }
        Ok(())
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::new()
    }
}

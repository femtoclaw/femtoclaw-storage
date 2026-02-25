//! Persistent Storage.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub struct Store {
    path: std::path::PathBuf,
}

impl Store {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }

    pub async fn save(&self, data: &serde_json::Value) -> Result<(), StoreError> {
        let json = serde_json::to_string_pretty(data)?;
        tokio::fs::write(&self.path, json).await?;
        Ok(())
    }

    pub async fn load(&self) -> Result<serde_json::Value, StoreError> {
        let json = tokio::fs::read_to_string(&self.path).await?;
        let data = serde_json::from_str(&json)?;
        Ok(data)
    }
}

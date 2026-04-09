//! State Snapshots.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub state: serde_json::Value,
}

impl Snapshot {
    pub fn new(state: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            state,
        }
    }
}

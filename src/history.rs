//! Execution History.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub input: String,
    pub output: String,
    pub metadata: serde_json::Value,
}

pub struct History {
    entries: Vec<HistoryEntry>,
    max_entries: usize,
}

impl History {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn push(&mut self, input: String, output: String, metadata: serde_json::Value) {
        self.entries.push(HistoryEntry {
            timestamp: chrono::Utc::now(),
            input,
            output,
            metadata,
        });
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
}

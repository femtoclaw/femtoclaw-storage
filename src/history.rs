//! Execution History.
//!
//! In-memory execution history with configurable size limits.

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

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_push() {
        let mut history = History::new(2);
        history.push(
            "input1".to_string(),
            "output1".to_string(),
            serde_json::json!({}),
        );
        history.push(
            "input2".to_string(),
            "output2".to_string(),
            serde_json::json!({}),
        );

        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_history_max_entries() {
        let mut history = History::new(2);
        history.push(
            "input1".to_string(),
            "output1".to_string(),
            serde_json::json!({}),
        );
        history.push(
            "input2".to_string(),
            "output2".to_string(),
            serde_json::json!({}),
        );
        history.push(
            "input3".to_string(),
            "output3".to_string(),
            serde_json::json!({}),
        );

        assert_eq!(history.len(), 2);
    }
}

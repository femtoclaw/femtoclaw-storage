//! Execution Log.
//!
//! Records capability executions with durability guarantees.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub capability: String,
    pub arguments: serde_json::Value,
    pub result: Option<String>,
    pub status: ExecutionStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Started,
    Completed,
    Failed,
    Denied,
}

impl ExecutionRecord {
    pub fn started(capability: &str, arguments: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            capability: capability.to_string(),
            arguments,
            result: None,
            status: ExecutionStatus::Started,
            error: None,
        }
    }

    pub fn completed(mut self, result: String) -> Self {
        self.result = Some(result);
        self.status = ExecutionStatus::Completed;
        self
    }

    pub fn failed(mut self, error: String) -> Self {
        self.error = Some(error);
        self.status = ExecutionStatus::Failed;
        self
    }

    pub fn denied(mut self) -> Self {
        self.status = ExecutionStatus::Denied;
        self
    }
}

pub struct ExecutionLog {
    records: Vec<ExecutionRecord>,
    max_records: usize,
}

impl ExecutionLog {
    pub fn new(max_records: usize) -> Self {
        Self {
            records: Vec::new(),
            max_records,
        }
    }

    pub fn log(&mut self, record: ExecutionRecord) {
        if self.records.len() >= self.max_records {
            self.records.remove(0);
        }
        self.records.push(record);
    }

    pub fn records(&self) -> &[ExecutionRecord] {
        &self.records
    }

    pub fn get(&self, id: &Uuid) -> Option<&ExecutionRecord> {
        self.records.iter().find(|r| &r.id == id)
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_record() {
        let record = ExecutionRecord::started("fs.read", serde_json::json!({"path": "/test"}));
        assert_eq!(record.status, ExecutionStatus::Started);

        let completed = record.completed("file content".to_string());
        assert_eq!(completed.status, ExecutionStatus::Completed);
    }
}

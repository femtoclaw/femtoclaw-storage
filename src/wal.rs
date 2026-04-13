//! Write-Ahead Log (WAL).
//!
//! Provides durable, append-only logging of execution events.
//! Ensures that state can be reconstructed in the event of a failure.

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalEntry {
    pub id: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub payload: serde_json::Value,
}

pub struct Wal {
    path: PathBuf,
    file: File,
    next_id: u64,
}

impl Wal {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&path)?;

        let mut wal = Self {
            path,
            file,
            next_id: 0,
        };
        
        // Recover next_id from existing file
        let entries = wal.replay()?;
        if let Some(last) = entries.last() {
            wal.next_id = last.id + 1;
        }

        Ok(wal)
    }

    pub fn append(&mut self, event_type: &str, payload: serde_json::Value) -> io::Result<u64> {
        let id = self.next_id;
        let entry = WalEntry {
            id,
            timestamp: chrono::Utc::now(),
            event_type: event_type.to_string(),
            payload,
        };

        let json = serde_json::to_string(&entry)?;
        writeln!(self.file, "{}", json)?;
        self.file.flush()?;

        self.next_id += 1;
        Ok(id)
    }

    pub fn replay(&self) -> io::Result<Vec<WalEntry>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let entry: WalEntry = serde_json::from_str(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            entries.push(entry);
        }

        Ok(entries)
    }

    pub fn truncate(&mut self) -> io::Result<()> {
        self.file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        self.next_id = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_wal_append_replay() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test.wal");
        
        {
            let mut wal = Wal::open(&wal_path).unwrap();
            wal.append("test_event", serde_json::json!({"foo": "bar"})).unwrap();
            wal.append("test_event", serde_json::json!({"baz": 123})).unwrap();
        }

        {
            let wal = Wal::open(&wal_path).unwrap();
            let entries = wal.replay().unwrap();
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].id, 0);
            assert_eq!(entries[1].id, 1);
            assert_eq!(entries[0].payload["foo"], "bar");
            assert_eq!(entries[1].payload["baz"], 123);
        }
    }
}

//! FemtoClaw Storage Library.
//!
//! Provides persistence, state snapshots, and execution history according to
//! FemtoClaw Storage Specification (FC-STORAGE-0001).
//!
//! # Architecture
//!
//! - [`ExecutionLog`] - records capability executions
//! - [`History`] - execution history management
//! - [`Snapshot`] - state snapshots
//! - [`Store`] - persistent storage
//! - [`ConfigStore`] - configuration storage

pub mod config;
pub mod execution_log;
pub mod history;
pub mod snapshot;
pub mod store;

pub use config::ConfigStore;
pub use execution_log::ExecutionLog;
pub use history::History;
pub use snapshot::Snapshot;
pub use store::Store;

//! FemtoClaw Storage Library.
//!
//! Provides persistence, state snapshots, and execution history according to
//! FemtoClaw Storage Specification (FC-STORAGE-0001).

pub mod history;
pub mod snapshot;
pub mod store;

pub use history::History;
pub use snapshot::Snapshot;
pub use store::Store;

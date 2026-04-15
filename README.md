# 💾 FemtoClaw Storage & WAL

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![Tier](https://img.shields.io/badge/Tier-Enterprise-green.svg)]()

The **FemtoClaw Storage** library provides the high-performance persistence layer required for industrial agent execution. It focuses on three core pillars: **Durability**, **Crash Resilience**, and **State Reconstruction**.

---

## 🪵 Write-Ahead Log (WAL): Spec 20

FemtoClaw implements a strict **Write-Ahead Log (WAL)** to ensure that no execution state is lost during unexpected process termination. Every state mutation (e.g., a message being pushed to memory or a tool result being recorded) is committed to durable storage *before* it is applied to the active runtime memory.

### Key Durability Features:
- **Append-only I/O**: High-throughput sequential disk writes ensure minimal latency impact on the agent control loop.
- **Atomic Commits**: Mutations are only visible to the runtime once they have been successfully flushed to the WAL.
- **Crash Recovery**: On startup, the runtime replays the WAL to reconstruct the exact state of the agent's memory and history.
- **Audit Integrity**: The log provides a permanent, immutable, and tamper-evident record of all agent interactions for compliance.

---

## 🔄 State Reconstruction (Replay)

If the runtime process terminates unexpectedly (e.g., crash, power failure, or node migration), the WAL can be used to restore the full execution context:

```rust
use femtoclaw_storage::Wal;

// 1. Open the existing durable log
let mut wal = Wal::open("~/.femtoclaw/execution.wal")?;

// 2. Replay events to restore memory history
let entries = wal.replay()?; 

for entry in entries {
    println!("Recovered event [{}]: {}", entry.id, entry.event_type);
}
```

---

## 📦 Backends & Support

| Backend | Tier | Purpose |
|---------|------|---------|
| **Filesystem (Native)** | Enterprise | Optimized for workstations and enterprise servers. |
| **In-Memory (Ephemeral)** | Local | For stateless or transient execution cycles (testing). |
| **Distributed (Reference)** | Reference | (In Progress) Cluster-wide synchronization via Spec 41. |

---

## 📄 Related Specifications
- **[FC-STORAGE-0001: Persistence and Storage](../femtoclaw-spec/FC-STORAGE-0001-Persistence_and_Storage_Specification.md)**
- **[FC-20: Persistent Storage Specification](../femtoclaw-spec/20-FemtoClaw_Persistent_Storage_Specification.md)**

Copyright © 2026 FemtoClaw Project.

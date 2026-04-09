# FemtoClaw Storage

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Normative-green.svg)]()

FemtoClaw Storage — persistence, state snapshots, and execution history.

## Overview

`femtoclaw-storage` provides the persistence layer for the FemtoClaw Industrial Agent Runtime. It implements state management and execution history storage according to the [FemtoClaw Storage Specification (FC-STORAGE-0001)](../femtoclaw-spec/FC-STORAGE-0001-Persistence_and_Storage_Specification.md).

This library handles memory persistence, execution history, and state snapshots for the runtime.

## Features

- **Execution History**: Record and retrieve all execution events
- **State Snapshots**: Save and restore runtime state
- **Persistent Storage**: Async file-based storage with JSON serialization
- **Memory Management**: Configurable history limits

## Architecture

```
┌─────────────────────────────────────────────┐
│         FemtoClaw Runtime                   │
│  ┌─────────────────────────────────────┐   │
│  │         Memory Subsystem            │   │
│  │  ┌────────────┐ ┌────────────┐      │   │
│  │  │ Short-term │ │ Long-term  │      │   │
│  │  │   Memory   │ │   Memory   │      │   │
│  │  └────────────┘ └────────────┘      │   │
│  └─────────────────────────────────────┘   │
│                    ↓                        │
│  ┌─────────────────────────────────────┐   │
│  │       femtoclaw-storage             │   │
│  │  ┌──────────┐ ┌─────────┐           │   │
│  │  │ History  │ │Snapshot │           │   │
│  │  │  Store   │ │  Store  │           │   │
│  │  └──────────┘ └─────────┘           │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

## Installation

```toml
[dependencies]
femtoclaw-storage = "1.0"
```

## Usage

```rust
use femtoclaw_storage::{History, Snapshot, Store};

// Execution history
let mut history = History::new(1000); // Max 1000 entries
history.push(
    "user message".to_string(),
    "assistant response".to_string(),
    serde_json::json!({"model": "gpt-4"})
);

// State snapshots
let state = serde_json::json!({
    "messages": [...],
    "capabilities": [...]
});
let snapshot = Snapshot::new(state);

// Persistent storage
let store = Store::new("/data/femtoclaw/state.json".into());
store.save(&snapshot.state).await?;
```

## Modules

- `history` — Execution history management
- `snapshot` — State snapshot definitions
- `store` — Async persistent storage

## Requirements

- Rust 1.75 or later
- serde 1.x
- serde_json 1.x
- tokio 1.x (with fs, rt-multi-thread, macros)
- thiserror 1.x
- chrono 0.4
- uuid 1.x

## Related Specifications

- [FC-STORAGE-0001: Persistence and Storage](../femtoclaw-spec/FC-STORAGE-0001-Persistence_and_Storage_Specification.md)
- [FC-06: Memory Subsystem](../femtoclaw-spec/06-FemtoClaw_Memory_Subsystem_Specification.md)

## License

Copyright 2026 FemtoClaw

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

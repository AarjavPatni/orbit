# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Orbit is a distributed, fault-tolerant key-value store in Rust that can be driven entirely from a CLI, much like Redis but with built-in consensus for consistency across nodes. The project evolves in demoable milestones: starting with a single-node in-memory store, then layering on persistence, consensus (both per-transaction and replicated log approaches), and multiple transport backends.

## Project Goals
1. **Fault tolerance using CLI** - Visibly demonstrate resilience from command line
2. **Consensus for data consistency** - Two approaches:
   - Consensus for each transaction  
   - Log using sequence numbers
3. **CLI tool/query language** - Redis-inspired interface for data operations

## Common Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release

# Run the project
cargo run

# Run with release optimizations
cargo run --release
```

### Testing and Quality
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Lint with all targets
cargo clippy --all-targets --all-features
```

## Architecture

Currently a minimal single-file application with `main.rs` as the entry point. As the project grows, consider organizing code into:
- `src/lib.rs` for library code
- `src/modules/` for feature modules
- `tests/` for integration tests

## Dependencies

No external dependencies currently. When adding dependencies, update Cargo.toml and consider:
- Compatibility with rust edition 2021
- License compatibility
- Maintenance status of crates

## Development Milestones

See `ROADMAP.md` for detailed task breakdown. Current milestones:

### Milestone 0: Project Setup (Complete)
- Basic Rust project structure
- Project documentation (CLAUDE.md, ROADMAP.md)

### Milestone 1: Basic CLI Store (In Progress)
- In-memory key-value operations (GET, SET, DEL, KEYS)
- Command-line interface with argument parsing
- Basic error handling and validation

### Milestone 2: Persistent Single Node
- Append-only log for durability
- Crash recovery and log replay
- Log compaction to prevent infinite growth

### Milestone 3: Network-Aware Single Node
- TCP server for remote connections
- Wire protocol for client-server communication
- Concurrent connection handling

### Milestone 4: Leader-Follower Replication
- Basic replication between nodes
- Simple leader election mechanism
- Split-brain prevention

### Milestone 5: Multi-Node Consensus (Raft)
- Full Raft consensus implementation
- Cluster membership management
- Partition tolerance demonstration

### Milestone 6: Transaction-Level Consensus
- Per-transaction consensus approach
- Performance comparison with log-based consensus
- Runtime switching between approaches

### Milestone 7: Advanced Fault Scenarios
- Dynamic membership changes
- Complex partition recovery
- Chaos engineering and automated fault injection

### Milestone 8: Production Features
- Observability (metrics, logging, tracing)
- Configuration management
- Performance optimization

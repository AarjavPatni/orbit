# Orbit

A distributed, fault-tolerant key-value store built in Rust with CLI-demonstrable fault tolerance.

## Overview

Orbit is a Redis-like key-value store that provides:
- **Distributed consensus** using the Raft algorithm
- **Fault tolerance** with visible resilience demonstrations
- **CLI-driven operations** for easy interaction and testing
- **Multiple consensus approaches** (per-transaction vs. log-based)

## Project Status

🚧 **Early Development** - Currently implementing Milestone 1 (Basic CLI Store)

See [ROADMAP.md](ROADMAP.md) for detailed development progress and upcoming features.

## Goals

1. **Fault tolerance using CLI** - Visibly demonstrate resilience from command line
2. **Consensus for data consistency** - Two approaches:
   - Consensus for each transaction  
   - Log using sequence numbers
3. **CLI tool/query language** - Redis-inspired interface for data operations

## Quick Start

```bash
# Build the project
cargo build

# Run locally
cargo run

# Run tests
cargo test
```

## Development Milestones

- [x] **Milestone 0**: Project Setup
- [ ] **Milestone 1**: Basic CLI Store (In Progress)
- [ ] **Milestone 2**: Persistent Single Node
- [ ] **Milestone 3**: Network-Aware Single Node
- [ ] **Milestone 4**: Leader-Follower Replication
- [ ] **Milestone 5**: Multi-Node Consensus (Raft)
- [ ] **Milestone 6**: Transaction-Level Consensus
- [ ] **Milestone 7**: Advanced Fault Scenarios
- [ ] **Milestone 8**: Production Features

## Architecture

The project follows a modular design that evolves through milestones:

1. **Single-node in-memory store** → Basic CLI operations
2. **Persistence layer** → Append-only log with crash recovery
3. **Network layer** → TCP server with custom protocol
4. **Replication** → Leader-follower with basic failover
5. **Consensus** → Full Raft implementation
6. **Advanced features** → Performance optimization and production readiness

## Demo Scenarios

Each milestone includes specific demonstrations:
- **CLI Demo**: Local key-value operations
- **Persistence Demo**: Data survives application restarts
- **Network Demo**: Remote access from multiple clients
- **Fault Tolerance Demo**: Kill processes, show automatic recovery
- **Partition Demo**: Network splits with consistency maintenance

## Documentation

- [CLAUDE.md](CLAUDE.md) - Development guidance and commands
- [ROADMAP.md](ROADMAP.md) - Detailed task breakdown and progress tracking
- [project-description.md](project-description.md) - Original project concept

## Contributing

This is a learning project focused on understanding distributed systems concepts. The milestone-driven approach ensures each feature is fully functional before moving to the next.

## License

MIT License - see LICENSE file for details
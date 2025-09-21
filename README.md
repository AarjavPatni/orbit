# Orbit

A distributed, encrypted file storage system that makes data resilient to failure, private, and accessible across a network of nodes.

## Overview

Orbit is a distributed file storage system that provides:
- **File encryption and sharding** - Files split into encrypted pieces for privacy
- **P2P networking** - Automatic node discovery and gossip-based coordination
- **Fault tolerance** - Files remain accessible even when nodes fail
- **Rich TUI interface** - Real-time visualization of network status and operations

## Project Status

**Beginning Development** - Starting with Milestone 1 (Basic File Operations)

**Planned Features:**
- File chunking with adaptive sizing (64KB/1MB/4MB based on file size)
- Per-shard encryption for privacy and security
- P2P node discovery using mDNS for local networks
- Gossip protocols for distributed metadata coordination
- Rich TUI with real-time progress and network status
- Simple replication for fault tolerance

See [ROADMAP.md](ROADMAP.md) for detailed development progress and milestone breakdown.

## Goals

1. **Distributed file storage** - Store files as encrypted shards across multiple nodes
2. **Privacy through encryption** - Per-shard encryption ensures data confidentiality  
3. **P2P networking** - Automatic discovery and coordination without central authority
4. **Fault tolerance** - Files remain accessible despite node failures
5. **Rich user experience** - Visual progress tracking and real-time system insights

## Quick Start

```bash
# Build the project
cargo build

# Start a storage node
cargo run -- node --port 8001

# Upload a file (from another terminal)
cargo run -- upload myfile.txt

# Check network status  
cargo run -- status

# Download the file
cargo run -- download myfile.txt

# Run tests
cargo test
```

## Development Milestones

- [ ] **Milestone 1**: Basic File Operations - Local chunking, encryption, and storage
- [ ] **Milestone 2**: Multi-Node Storage - Distribute files across multiple processes  
- [ ] **Milestone 3**: P2P Discovery - Automatic node discovery via mDNS
- [ ] **Milestone 4**: Basic TUI - Progress bars and status display
- [ ] **Milestone 5**: Gossip Protocol - Distributed metadata coordination
- [ ] **Milestone 6**: Fault Tolerance - Manual node recovery and file resilience
- [ ] **Milestone 7**: Rich TUI Dashboard - Comprehensive network visualization
- [ ] **Milestone 8**: Polish & Performance - Production readiness and optimization

## Architecture

The project follows a modular design that evolves through milestones:

1. **File operations** → Chunking, encryption, and local storage
2. **Multi-node distribution** → Store shards across multiple processes
3. **P2P networking** → Automatic discovery and peer communication
4. **User interface** → Rich TUI with progress tracking and status
5. **Distributed coordination** → Gossip protocols for metadata
6. **Fault tolerance** → Redundancy and graceful failure handling
7. **Advanced visualization** → Real-time network and file status
8. **Production features** → Performance optimization and reliability

## Demo Scenarios

Each milestone includes specific demonstrations:
- **File Operations**: Upload, chunk, encrypt, and reconstruct files locally
- **Multi-Node Demo**: Store file across multiple processes, retrieve from any
- **P2P Discovery**: Nodes automatically find each other on local network
- **TUI Progress**: Visual upload/download progress with real-time status
- **Gossip Demo**: Nodes share metadata about stored files
- **Fault Tolerance**: Kill nodes, files remain accessible via other nodes
- **Network Status**: Live dashboard showing node health and file distribution

## Documentation

- [PROBLEM_STATEMENT.md](PROBLEM_STATEMENT.md) - Core problem and solution approach
- [CLAUDE.md](CLAUDE.md) - Development guidance and commands
- [ROADMAP.md](ROADMAP.md) - Detailed milestone breakdown and progress tracking
- [project-description.md](project-description.md) - Project concept and vision


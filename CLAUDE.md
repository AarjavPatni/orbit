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

## Collaborative Learning Approach

This project follows a collaborative learning methodology where Claude guides implementation while maximizing opportunities for hands-on learning and independent thinking.

### Learning Principles
- **Collaborative brainstorming** - Discuss design decisions and architecture choices together
- **Guided implementation** - Claude provides structure and guidance while the human implements core logic
- **Learning by doing** - Focus on meaningful code contributions rather than routine boilerplate
- **Educational insights** - Share relevant programming concepts and patterns during development

### Implementation Process
1. **Planning phase** - Brainstorm approach, discuss trade-offs, and create implementation plan
2. **Skeleton setup** - Claude creates minimal structure with TODO(human) markers for learning opportunities
3. **Guided development** - Human implements key logic with Claude providing context and guidance
4. **Integration** - Claude handles integration, testing, and routine tasks while sharing insights

### When to Request Human Implementation
Request human contributions for:
- **Design decisions** - Error handling strategies, data structure choices, algorithm selection
- **Business logic** - Core functionality with multiple valid approaches
- **Key interfaces** - Function signatures and API design decisions
- **Problem-solving** - Debugging challenges and optimization opportunities

### Learning Moments
- Provide educational insights before and after code implementation
- Explain architectural patterns and their trade-offs
- Connect individual code pieces to broader system design
- Share Rust-specific best practices and idioms as they arise

### Debugging and Problem Discovery Guidelines
- **Let the human discover issues first** - Don't proactively find and fix problems
- **Wait for explicit requests for help** - Only offer solutions when asked
- **Guide through discovery** - Help analyze problems when the human encounters them
- **Focus on teaching debugging skills** - Explain how to identify and approach issues
- **Respect the learning process** - Allow natural problem-solving flow

### Collaboration Boundaries
- Human drives the implementation and discovers issues
- Claude provides structure, guidance, and explanations when requested
- Test failures and integration issues are learning opportunities for the human
- Only intervene when explicitly asked for help or guidance

This approach ensures deep understanding of the codebase while maintaining development momentum and respecting the human's learning autonomy.

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

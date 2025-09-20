## Project Overview

Orbit is a distributed, encrypted file storage system that splits files into shards and distributes them across multiple nodes for fault tolerance and privacy. The system uses P2P networking for node discovery and features a rich TUI interface that makes distributed systems concepts visible and engaging. The project evolves in demoable milestones: starting with basic file operations, then adding P2P discovery, gossip protocols, and comprehensive fault tolerance.

## Project Goals
1. **Distributed file storage** - Store encrypted file shards across multiple nodes
2. **P2P networking** - Automatic node discovery and gossip-based coordination
3. **Rich TUI interface** - Visual progress tracking and real-time system status
4. **Fault tolerance** - Files remain accessible even when nodes fail
5. **Privacy through encryption** - Per-shard encryption protects data on storage nodes

## Essential Commands
```bash
cargo check                              # Fast compile check (fastest)
cargo test                               # Run all tests  
cargo run -- node --port 8001            # Start storage node
cargo run -- upload myfile.txt           # Upload file to network
cargo run -- download myfile.txt         # Download file from network  
cargo run -- status                      # Show network and file status
```

### **MUST** Run Before Commit
```bash
cargo fmt                                # Format code
cargo clippy                             # Lint code (fix ALL warnings)
cargo test                               # Ensure tests pass
```

### Build & Advanced Testing
```bash
cargo build                              # Development build
cargo build --release                    # Optimized build
cargo test -- --nocapture                # Tests with output
cargo test test_name                     # Specific test
cargo clippy --all-targets --all-features # Full lint check
```

## Code Style (Rust-Specific)

### **IMPORTANT** Guidelines
- Use `Result<T, E>` for error handling, avoid panics in library code
- Prefer `&str` over `String` for function parameters when possible
- Use `#[derive(Debug)]` on all custom types
- Follow Rust naming conventions: `snake_case` for functions/variables, `PascalCase` for types

### Error Handling Patterns
```rust
// Prefer this pattern for file operations
fn upload_file(path: &Path) -> Result<FileId, StorageError> {
    // Implementation
}

// Chain results with ?
fn distribute_shards() -> Result<(), StorageError> {
    let shards = create_shards(&file_data)?;
    encrypt_shards(&shards)?;
    distribute_to_nodes(&encrypted_shards)?;
    Ok(())
}
```

## Planned Architecture

- `src/lib.rs` - Core library API
- `src/storage/` - File chunking, encryption, and shard management
- `src/network/` - P2P networking, discovery, and gossip protocols  
- `src/tui/` - Rich terminal user interface with real-time updates
- `src/cli/` - Command-line interface for file operations
- `tests/` - Integration tests for distributed storage scenarios


## **IMPORTANT** - Other Guidelines

- Prioritize minimal, well-maintained crates for dependencies
- See `ROADMAP.md` for complete milestone details


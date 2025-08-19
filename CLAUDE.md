## Project Overview

Orbit is a distributed, fault-tolerant key-value store in Rust that can be driven entirely from a CLI, much like Redis but with built-in consensus for consistency across nodes. The project evolves in demoable milestones: starting with a single-node in-memory store, then layering on persistence, consensus (both per-transaction and replicated log approaches), and multiple transport backends.

## Project Goals
1. **Fault tolerance using CLI** - Visibly demonstrate resilience from command line
2. **Consensus for data consistency** - Two approaches:
   - Consensus for each transaction  
   - Log using sequence numbers
3. **CLI tool/query language** - Redis-inspired interface for data operations

## Essential Commands
```bash
cargo check                              # Fast compile check (fastest)
cargo test                               # Run all tests  
cargo run                                # Run the CLI
cargo run -- get mykey                   # Run with CLI args
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
// Prefer this pattern for the distributed store
fn get_value(key: &str) -> Result<Option<String>, StoreError> {
    // Implementation
}

// Chain results with ?
fn complex_operation() -> Result<(), StoreError> {
    let value = get_value("key")?;
    validate_value(&value)?;
    Ok(())
}
```

## Planned Architecture

- `src/lib.rs` - Core library API
- `src/store/` - Key-value store implementation  
- `src/consensus/` - Raft and transaction consensus
- `src/network/` - TCP server and wire protocol
- `src/cli/` - Command-line interface
- `tests/` - Integration tests for fault tolerance scenarios


## **IMPORTANT** - Other Guidelines

- Prioritize minimal, well-maintained crates for dependencies
- See `ROADMAP.md` for complete milestone details


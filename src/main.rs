mod cli;
mod error;
mod storage;

use std::process;

use cli::parser::Cli;
use storage::memory::MemoryStore;

use crate::error::StoreError;
use crate::storage::KeyValueStore;

fn main() {
    let cli = Cli::parse_args();
    let mut store = MemoryStore::new();

    let exit_code = match cli.command {
        cli::parser::Commands::Get { key } => match store.get(&key) {
            Ok(value) => {
                println!("{}", value);
                0
            }
            Err(e @ StoreError::KeyNotFound(_)) => {
                eprintln!("Error: {}", e);
                1
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                2
            }
        },
        cli::parser::Commands::Set { key, value } => match store.set(key, value) {
            Ok(_) => {
                println!("OK");
                0
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                2
            }
        },
        cli::parser::Commands::Del { key } => match store.delete(&key) {
            Ok(Some(old_value)) => {
                println!("{}", old_value);
                0
            }
            Ok(None) => {
                println!("(nil)");
                0 // Idempotent success - key doesn't exist is OK
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                2
            }
        },
        cli::parser::Commands::Keys { pattern } => {
            let keys = store.keys(pattern.as_deref());
            for (i, key) in keys.iter().enumerate() {
                println!("{}) {}", i + 1, key);
            }
            0
        }
    };

    process::exit(exit_code);
}

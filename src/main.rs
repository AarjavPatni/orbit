mod cli;
mod error;
mod storage;

use cli::parser::Cli;
use storage::memory::MemoryStore;

use crate::storage::KeyValueStore;

fn main() {
    let cli = Cli::parse_args();
    let mut store = MemoryStore::new();

    match cli.command {
        cli::parser::Commands::Get { key } => match store.get(&key) {
            Ok(value) => println!("{}", value),
            Err(e) => println!("Error: {:?}", e),
        },
        cli::parser::Commands::Set { key, value } => match store.set(key, value) {
            Ok(_) => println!("OK"),
            Err(e) => println!("Error: {:?}", e),
        },
        cli::parser::Commands::Del { key } => match store.delete(&key) {
            Some(old_value) => println!("{}", old_value),
            None => println!("(nil)"),
        },
        cli::parser::Commands::Keys { pattern } => {
            let keys = store.keys(pattern.as_deref());
            for (i, key) in keys.iter().enumerate() {
                println!("{}) {}", i + 1, key);
            }
        }
    }
}

use blake3::hash;
use std::{fs, path::Path};

use clap::{Parser, Subcommand};
use orbit::storage::chunker::chunk_file;

#[derive(Parser)]
#[command(name = "orbit")]
#[command(about = "Distributed file storage system")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Chunk a file and store locally
    Chunk {
        filename: String,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chunk { filename } => {
            // Call your chunker here
            match chunk_file(&filename) {
                Ok(chunk_iterator) => {
                    for (index, chunk) in chunk_iterator.enumerate() {
                        let full_hash = hash(filename.as_bytes());
                        let hashed_filename = &full_hash.to_string()[..8];

                        /*
                        ? src/main.rs|38 col 113-114 error| the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`) the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

                        Ans:
                        The ? operator can only be used in functions that return Result or Option. Since main() returns (), you can't use ?. That's why you switched to .expect() - perfect solution for now!
                        */

                        fs::write(
                            format!("/Users/aarjav/.orbit/{}_chunk_{}", hashed_filename, index),
                            chunk.data,
                        )
                        .expect("Unable to write");
                    }
                }

                Err(_) => {
                    println!("Unable to write");
                }
            }
        }

        Commands::List => {
            // List files in ~/.orbit/
            // TODO: Convert this to list files and not chunks
            let chunks_path = fs::read_dir("Users/aarjav/.orbit").unwrap();

            for chunk in chunks_path {
                let chunk_name = chunk.unwrap().file_name().into_string().unwrap();
                println!("{}", chunk_name);
            }
        }
    }
}

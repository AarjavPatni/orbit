use clap::{Parser, Subcommand};

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
        file: String,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chunk { file } => {
            // Call your chunker here
        }

        Commands::List => {
            // List files in ~/.orbit/
        }
    }
}

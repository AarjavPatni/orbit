use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "orbit")]
#[command(about = "A distributed, fault-tolerant key-value store")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Get the value of a key")]
    Get {
        #[arg(help = "The key to retrieve")]
        key: String,
    },
    #[command(about = "Set the value of the key if it exits, otherwise create a new one")]
    Set {
        #[arg(help = "The key to set the value for")]
        key: String,
        #[arg(help = "The value to store")]
        value: String,
    },

    #[command(
        about = "Delete the key if it exists, else return an error informing the user the key doesn't exist"
    )]
    Del {
        #[arg(help = "The key to delete")]
        key: String,
    },

    #[command(about = "List all the keys and optionally accept a filtering pattern")]
    Keys {
        #[arg(help = "The search pattern to find keys")]
        pattern: Option<String>,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

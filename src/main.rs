mod cli;

use cli::parser::Cli;

fn main() {
    let cli = Cli::parse_args();

    match cli.command {
        cli::parser::Commands::Get { key } => {
            println!("GET command: key = {}", key);
        }
        cli::parser::Commands::Set { key, value } => {
            println!("SET command: key = {}, value = {}", key, value);
        }
        cli::parser::Commands::Del { key } => {
            println!("DEL command: key = {}", key);
        }
        cli::parser::Commands::Keys { pattern } => match pattern {
            Some(p) => println!("KEYS command with pattern: {}", p),
            None => println!("KEYS command: list all keys"),
        },
    }
}

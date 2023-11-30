use clap::Parser;
use rust_tools::cli::{Cli, Commands};

fn main() {
    // use try_parse to avoid panic
    match Cli::try_parse() {
        Ok(cli) => match cli.command {
            Commands::Grep(args) => {
                if let Err(e) = args.run() {
                    println!("Command Grep error: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("Application error: {}", e);
        }
    }
}

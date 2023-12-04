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
            Commands::Find(args) => {
                if let Err(e) = args.run() {
                    println!("Command Find error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::StaticServer(args) => {
                if let Err(e) = args.run() {
                    println!("Command StaticServer error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::JsonServer(args) => {
                if let Err(e) = args.run() {
                    println!("Command JsonServer error: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}

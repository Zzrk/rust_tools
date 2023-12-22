use clap::Parser;
use rust_tools::cli::{Cli, Commands, RunCommand};

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
            Commands::ImagePreview(args) => {
                if let Err(e) = args.run() {
                    println!("Command ImagePreview error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::Base64(args) => {
                if let Err(e) = args.run() {
                    println!("Command Base64 error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::MD5(args) => {
                if let Err(e) = args.run() {
                    println!("Command MD5 error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::Sha(args) => {
                if let Err(e) = args.run() {
                    println!("Command Sha error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::Aes(args) => {
                if let Err(e) = args.run() {
                    println!("Command Aes error: {}", e);
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

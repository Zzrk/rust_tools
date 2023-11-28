use clap::Parser;
use rust_tools::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Grep(args) => {
            if let Err(e) = args.run() {
                println!("Application error: {}", e);
                std::process::exit(1);
            }
        }
    };
}

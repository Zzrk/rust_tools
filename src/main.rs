use rust_tools::Config;
use std::process;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search keywords in the file
    Grep(GrepArgs),
}

#[derive(Args)]
struct GrepArgs {
    query: String,
    file_path: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Grep(args) => {
            let mut config: Vec<String> = Vec::new();
            config.push(String::from("rust_tools"));
            config.push(args.query.clone());
            config.push(args.file_path.clone());
            let config = Config::build(&config).unwrap_or_else(|err| {
                println!("Problem parsing arguments: {err}");
                process::exit(1);
            });
            if let Err(e) = rust_tools::run(config) {
                println!("Application error: {e}");
                process::exit(1);
            }
        }
    }
}

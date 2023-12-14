use crate::{
    find::FindArgs, grep::GrepArgs, json_server::JsonServerArgs, static_server::StaticServerArgs,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search keyword in the file
    Grep(GrepArgs),
    /// Find file by keyword
    Find(FindArgs),
    /// Static file server
    StaticServer(StaticServerArgs),
    /// Start a json server
    JsonServer(JsonServerArgs),
}

trait Run {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

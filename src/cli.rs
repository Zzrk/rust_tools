use crate::{find::FindArgs, grep::GrepArgs, static_server::StaticServerArgs};
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
    /// Search keywords in the file
    Grep(GrepArgs),
    /// Static file server
    StaticServer(StaticServerArgs),
    /// Find file by keyword
    Find(FindArgs),
}

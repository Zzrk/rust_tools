use crate::subcommands::{
    aes::AesArgs, base64::Base64Args, find::FindArgs, grep::GrepArgs,
    image_preview::ImagePreviewArgs, json_server::JsonServerArgs, md5::Md5Args, sha::ShaArgs,
    static_server::StaticServerArgs,
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
    /// Preview image
    ImagePreview(ImagePreviewArgs),
    /// Base64 encode or decode
    Base64(Base64Args),
    /// MD5 hash
    Md5(Md5Args),
    /// Sha hash
    Sha(ShaArgs),
    /// Aes encrypt or decrypt
    Aes(AesArgs),
}

pub trait RunCommand {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

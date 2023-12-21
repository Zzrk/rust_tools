use crate::cli::RunCommand;
use clap::Args;
use sha2::{Digest, Sha256, Sha512};
use std::fs;

#[derive(Args)]
pub struct ShaArgs {
    /// sha type, only support sha256, sha512
    sha_type: String,
    /// message to hash
    message: String,
    /// file mode, whether the message represents a file path
    #[arg(short, long)]
    file_mode: bool,
}

impl ShaArgs {
    /// get message from file or string
    fn get_message(&self) -> Vec<u8> {
        match self.file_mode {
            true => fs::read(self.message.as_str()).unwrap(),
            false => self.message.as_bytes().to_vec(),
        }
    }

    fn sha256(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.get_message());
        format!("{:x}", hasher.finalize())
    }

    fn sha512(&self) -> String {
        let mut hasher = Sha512::new();
        hasher.update(self.get_message());
        format!("{:x}", hasher.finalize())
    }

    /// hash message
    fn hash(&self) -> String {
        match self.sha_type.as_str() {
            "256" => self.sha256(),
            "512" => self.sha512(),
            _ => panic!("unsupported sha type"),
        }
    }
}

impl RunCommand for ShaArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", self.hash());
        Ok(())
    }
}

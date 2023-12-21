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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let args = ShaArgs {
            sha_type: String::from("256"),
            message: String::from("hello world"),
            file_mode: false,
        };

        assert_eq!(
            args.sha256(),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_sha512() {
        let args = ShaArgs {
            sha_type: String::from("512"),
            message: String::from("hello world"),
            file_mode: false,
        };

        assert_eq!(
            args.sha512(),
            "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f\
            989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
        );
    }
}

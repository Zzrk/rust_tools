use crate::cli::RunCommand;
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct Md5Args {
    /// message to hash
    message: String,
    /// file mode, whether the message represents a file path
    #[arg(short, long)]
    file_mode: bool,
}

impl Md5Args {
    /// hash message
    fn hash(&self) -> String {
        let message = match self.file_mode {
            true => fs::read(self.message.as_str()).unwrap(),
            false => self.message.as_bytes().to_vec(),
        };
        let digest = md5::compute(message);
        format!("{:x}", digest)
    }
}

impl RunCommand for Md5Args {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", self.hash());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let args = Md5Args {
            message: String::from("hello world"),
            file_mode: false,
        };
        assert_eq!(args.hash(), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }
}

use crate::cli::RunCommand;
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct MD5Args {
    /// message to hash
    message: String,
    /// file mode, whether the message represents a file path
    #[arg(short, long)]
    file_mode: bool,
}

impl RunCommand for MD5Args {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let message = match self.file_mode {
            true => fs::read(self.message.as_str())?,
            false => self.message.as_bytes().to_vec(),
        };
        let digest = md5::compute(message);
        let res = format!("{:x}", digest);
        println!("{}", res);
        Ok(())
    }
}

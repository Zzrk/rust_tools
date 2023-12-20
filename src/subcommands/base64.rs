use crate::{cli::RunCommand, tools::print_debug};
use base64::{engine::general_purpose, Engine as _};
use clap::Args;
use std::error::Error;
use std::fs;

#[derive(Args)]
pub struct Base64Args {
    /// encode or decode message
    message: String,
    /// encode message, default
    #[arg(short, long)]
    encode: bool,
    /// decode message, should not be used with encode
    #[arg(short, long)]
    decode: bool,
    /// use url safe encoding, default is false
    #[arg(short, long)]
    url_safe: bool,
    /// file to encode or decode
    /// in case of encode, the file will be encoded and printed to stdout
    /// in case of decode, the message will be decoded and written to the file
    #[arg(short, long)]
    file: Option<String>,
}

impl Base64Args {
    /// encode message
    fn encode(&self) -> String {
        match self.url_safe {
            true => general_purpose::URL_SAFE.encode(&self.message.as_bytes()),
            false => general_purpose::STANDARD.encode(&self.message.as_bytes()),
        }
    }

    /// encode file
    fn encode_file(&self) -> Result<String, Box<dyn Error>> {
        let file = fs::read_to_string(self.file.as_ref().unwrap())?;
        match self.url_safe {
            true => Ok(general_purpose::URL_SAFE.encode(&file.as_bytes())),
            false => Ok(general_purpose::STANDARD.encode(&file.as_bytes())),
        }
    }

    /// decode message
    fn decode(&self) -> Result<String, Box<dyn Error>> {
        match self.url_safe {
            true => Ok(String::from_utf8(
                general_purpose::URL_SAFE.decode(&self.message.as_bytes())?,
            )?),
            false => Ok(String::from_utf8(
                general_purpose::STANDARD.decode(&self.message.as_bytes())?,
            )?),
        }
    }

    /// decode message and write to file
    fn decode_file(&self) -> Result<String, Box<dyn Error>> {
        let file_path = self.file.as_ref().unwrap();
        let file = match self.url_safe {
            true => general_purpose::URL_SAFE.decode(&self.message.as_bytes())?,
            false => general_purpose::STANDARD.decode(&self.message.as_bytes())?,
        };
        fs::write(file_path, file)?;
        Ok(format!("write file to {}", file_path))
    }
}

impl RunCommand for Base64Args {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.encode && self.decode {
            return Err("encode and decode can't be used together".into());
        } else if self.decode {
            match self.file.is_some() {
                true => println!("{}", self.decode_file()?),
                false => println!("{}", self.decode()?),
            }
        } else {
            match self.file.is_some() {
                true => println!("{}", self.encode_file()?),
                false => println!("{}", self.encode()),
            }
        }
        Ok(())
    }
}

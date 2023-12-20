use crate::cli::RunCommand;
use base64::{engine::general_purpose, Engine as _};
use clap::Args;
use std::error::Error;

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
}

impl RunCommand for Base64Args {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.encode && self.decode {
            return Err("encode and decode can't be used together".into());
        } else if self.decode {
            let decoded = match self.url_safe {
                true => general_purpose::URL_SAFE.decode(&self.message.as_bytes())?,
                false => general_purpose::STANDARD.decode(&self.message.as_bytes())?,
            };
            println!("{}", String::from_utf8(decoded)?);
        } else {
            let encoded = match self.url_safe {
                true => general_purpose::URL_SAFE.encode(&self.message.as_bytes()),
                false => general_purpose::STANDARD.encode(&self.message.as_bytes()),
            };
            println!("{}", encoded);
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         let args: GrepArgs = GrepArgs {
//             query: String::from(query),
//             file_path: String::from(""),
//             ignore_case: false,
//         };

//         assert_eq!(vec!["safe, fast, productive."], args.search(contents));
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         let args: GrepArgs = GrepArgs {
//             query: String::from(query),
//             file_path: String::from(""),
//             ignore_case: false,
//         };

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             args.search_case_insensitive(contents)
//         );
//     }
// }

use crate::cli::RunCommand;
use clap::Args;
use std::error::Error;
use std::fs;

#[derive(Args)]
pub struct GrepArgs {
    /// Search keyword
    query: String,
    /// Search file path
    file_path: String,
    #[arg(short, long)]
    /// Ignore case
    ignore_case: bool,
}

impl RunCommand for GrepArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.file_path)?;

        let results = if self.ignore_case {
            self.search_case_insensitive(&contents)
        } else {
            self.search(&contents)
        };

        for line in results {
            println!("{line}");
        }

        Ok(())
    }
}

impl GrepArgs {
    // 不忽略大小写
    pub fn search<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(&self.query) {
                results.push(line);
            }
        }

        results
    }

    /// 忽略大小写
    pub fn search_case_insensitive<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        let query = self.query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let args: GrepArgs = GrepArgs {
            query: String::from(query),
            file_path: String::from(""),
            ignore_case: false,
        };

        assert_eq!(vec!["safe, fast, productive."], args.search(contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let args: GrepArgs = GrepArgs {
            query: String::from(query),
            file_path: String::from(""),
            ignore_case: false,
        };

        assert_eq!(
            vec!["Rust:", "Trust me."],
            args.search_case_insensitive(contents)
        );
    }
}

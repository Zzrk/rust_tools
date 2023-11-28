use clap::Args;
use std::error::Error;
use std::fs;

#[derive(Args)]
pub struct GrepArgs {
    query: String,
    file_path: String,
    #[arg(short, long)]
    ignore_case: bool,
}

impl GrepArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
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

    pub fn search<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(&self.query) {
                results.push(line);
            }
        }

        results
    }

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

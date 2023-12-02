use clap::Args;
use std::error::Error;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Args)]
pub struct FindArgs {
    /// Search keyword
    keyword: String,
    /// Search root path, default current path
    #[arg(short, long)]
    path: Option<String>,
    /// Search result limit, default 10
    #[arg(short, long)]
    limit: Option<u16>,
}

impl FindArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let path = self.path.clone().unwrap_or(".".to_string());
        let path = Path::new(path.as_str());
        let limit = self.limit.unwrap_or(10);
        let mut count = 0;

        for result in WalkDir::new(path) {
            if count >= limit {
                break;
            }
            let entry = result.unwrap();
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains(self.keyword.as_str()) {
                    count += 1;
                    let file_path = entry.path();
                    let relative_path = file_path.strip_prefix(path).unwrap();
                    let relative_path_str = relative_path.to_str().unwrap();
                    let full_path = file_path.canonicalize().unwrap();
                    let full_path_str = full_path.to_str().unwrap();
                    // remove prefix \\?\ for windows
                    let full_path_without_prefix = if full_path_str.starts_with("\\\\?\\") {
                        full_path_str.strip_prefix("\\\\?\\").unwrap()
                    } else {
                        full_path_str
                    };

                    // print header
                    if count == 1 {
                        println!(
                            "{:<5} {:<20} {:<30} {}",
                            "No", "File Name", "Relative Path", "Full Path"
                        );
                    }
                    // print result
                    println!(
                        "{:<5} {:<20} {:<30} {}",
                        count, file_name, relative_path_str, full_path_without_prefix
                    );
                }
            }
        }
        Ok(())
    }
}

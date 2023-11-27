use rust_tools::Config;
use std::io::{self, Write};
use std::process;

fn main() {
    print!("1. Search keywords in the file.\n");

    let mut input = String::new();
    while input.trim() != "1" {
        print!("Please enter the number before to select the desired function:\n");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
    }

    print!("Please enter the keywords you want to search:\n");
    io::stdout().flush().unwrap();
    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();

    print!("Please enter the file path:\n");
    io::stdout().flush().unwrap();
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).unwrap();

    let mut config: Vec<String> = Vec::new();
    config.push(String::from("rust_tools"));
    config.push(query.trim().to_string());
    config.push(file_path.trim().to_string());
    let config = Config::build(&config).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = rust_tools::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

use std::env;
use std::fs;
use rust_cli::{
    run,
    Config,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Searching for: {}", config.query);
    println!("File Path: {}", config.file_path);

    let _ = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", config.file_path, err);
        std::process::exit(1);
    });

    // println!("File Content:\n{}", content);
    println!("==========\nStarting search...");

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
    println!("Search completed successfully.");
}

// fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
//     let content = fs::read_to_string(&config.file_path)?;
//     if content.contains(&config.query) {
//         println!("Found '{}' in file '{}'", config.query, config.file_path);
//     } else {
//         println!("'{}' not found in file '{}'", config.query, config.file_path);
//     }
//     Ok(())
// }

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             eprintln!("Usage: {} <query> <file_path>", args[0]);
//             std::process::exit(1);
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         if !fs::metadata(&file_path).is_ok() {
//             eprintln!("Error: File '{}' does not exist.", file_path);
//             std::process::exit(1);
//         }

//         Config { query, file_path }
//     }
// }

// Uncomment the following function if you want to use it instead of the Config struct 


// fn parse_config(args: &[String]) -> Config {
//     if args.len() < 3 {
//         eprintln!("Usage: {} <query> <file_path>", args[0]);
//         std::process::exit(1);
//     }

//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     if !fs::metadata(&file_path).is_ok() {
//         eprintln!("Error: File '{}' does not exist.", file_path);
//         std::process::exit(1);
//     }

//     Config { query, file_path}
// }

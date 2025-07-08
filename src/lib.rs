use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    if content.contains(&config.query) {
        println!("Found '{}' in file '{}'", config.query, config.file_path);
    } else {
        println!("'{}' not found in file '{}'", config.query, config.file_path);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            eprintln!("Usage: {} <query> <file_path>", args[0]);
            std::process::exit(1);
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        if !fs::metadata(&file_path).is_ok() {
            eprintln!("Error: File '{}' does not exist.", file_path);
            std::process::exit(1);
        }

        Config { query, file_path }
    }
}
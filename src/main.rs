use std::error::Error;
use std::process;
use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Config {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    file: String,
}

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    for line in trgrep::search(&config.pattern, &contents) {
        println!("{line}");
    }

    Ok(())
}

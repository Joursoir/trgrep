use std::error::Error;
use std::process;
use std::fs;
use std::io::{self, BufReader, BufRead};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Config {
    /// The pattern to look for
    pattern: String,
    /// The path to files to read. A path of "-" stands for standard input.
    ///
    /// If no FILE is given, read standard input.
    files: Vec<String>,
}

fn main() {
    let mut config = Config::parse();

    if config.files.is_empty() {
        config.files.push(String::from("-"));
    }

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file in config.files {
        // On-Stack Dynamic Dispatch
        let (mut stdin_read, mut file_read);

        // We need to ascribe the type to get dynamic dispatch.
        let reader: &mut dyn BufRead = if file == "-" {
            stdin_read = BufReader::new(io::stdin());
            &mut stdin_read
        } else {
            file_read = BufReader::new(fs::File::open(&file)?);
            &mut file_read
        };

        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        for line in trgrep::search(&config.pattern, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

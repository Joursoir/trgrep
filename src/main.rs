use std::error::Error;
use std::process;
use std::fs;
use std::io::{self, BufReader, BufRead};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[clap(disable_help_flag = true)]
struct Config {
    /// The pattern to look for
    pattern: String,
    /// The path to files to read. A path of "-" stands for standard input.
    ///
    /// If no FILE is given, read standard input.
    files: Vec<String>,

    /// Print a help text and exit
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Ignores the case of the search string
    #[arg(short, long)]
    ignore_case: bool,

    /// Selects non-matching lines
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// Prefixes each matching line with the line number
    #[arg(short = 'n', long)]
    line_number: bool,

    /// Matches only whole words
    #[arg(short, long)]
    word_regexp: bool,

    /// Displays only the filenames of files that contain matches
    #[arg(short = 'l', long)]
    files_with_matches: bool,

    /// Suppresses the display of filenames
    #[arg(short = 'h', long)]
    no_filename: bool,
}

fn main() {
    let mut config = Config::parse();

    match config.files.len() {
        0 => {
            config.files.push(String::from("-"));
            config.no_filename = true;
        }
        1 => config.no_filename = true,
        _ => (),
    }

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    'outer: for file in config.files {
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

        for (idx, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
            let match_flag = trgrep::contains_pattern(&line, &config.pattern, config.ignore_case, config.word_regexp);
            let match_flag = if config.invert_match { !match_flag } else { match_flag };
            if !match_flag {
                continue;
            }

            if config.files_with_matches {
                println!("{file}");
                continue 'outer;
            }

            let formatted_output = if !config.no_filename && config.line_number {
                format!("{}:{}:{}", file, idx + 1, line)
            } else if !config.no_filename {
                format!("{}:{}", file, line)
            } else if config.line_number {
                format!("{}:{}", idx + 1, line)
            } else {
                format!("{}", line)
            };

            println!("{}", formatted_output);
        }
    }

    Ok(())
}

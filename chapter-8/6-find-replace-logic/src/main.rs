use clap::Args;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version = "1.0",
    about = "A command-line text manipulation utility",
    name = "Text Manipulation Utility"
)]
pub struct Cli {
    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    /// Find and Replace commands.
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Subcommand for handling find operations.
    Find(FindCommands),
    /// Subcommand for handling replace operations.
    Replace(ReplaceCommands),
}

/// Represents find-related commands.
#[derive(Debug, Args)]
pub struct FindCommands {
    /// Sets the input file to process
    #[clap(short = 'i', long = "input")]
    pub input: Option<String>,
    /// Sets the pattern to find.
    #[clap(short = 'p', long = "pattern")]
    pub pattern: Option<String>,
}

/// Represents repalce-related commands.
#[derive(Debug, Args)]
pub struct ReplaceCommands {
    /// Sets the input file to process
    #[clap(short = 'i', long = "input")]
    pub input: Option<String>,
    /// Sets the pattern to find.
    #[clap(short = 'p', long = "pattern")]
    pub pattern: Option<String>,
    /// Sets the replacement text.
    #[clap(short = 'r', long = "replace")]
    pub replace: Option<String>,
}

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn find_and_replace(content: &mut String, pattern: &str, replacement: &str) {
    *content = content.replace(pattern, replacement);
}

fn main() {
    let args = Cli::parse();
    let mut input_content = "".to_string();
    let mut pattern_to_find = "".to_string();
    let mut replacement_text = "".to_string();

    match args.command {
        Some(Commands::Find(command)) => {
            if let Some(input) = &command.input {
                input_content = read_file_to_string(input).unwrap();
                println!("Input file content: {:?}", input_content);
            } else {
                println!("Please provide a file name.");
            }
            if let Some(pattern) = &command.pattern {
                pattern_to_find = pattern.clone();
                println!("Pattern: {:?}", pattern);
            } else {
                println!("Please provide a pattern.");
            }
        }
        Some(Commands::Replace(command)) => {
            if let Some(input) = &command.input {
                input_content = read_file_to_string(input).unwrap();
                println!("Input file content: {:?}", input_content);
            } else {
                println!("Please provide a file name.");
            }
            if let Some(pattern) = &command.pattern {
                pattern_to_find = pattern.clone();
                println!("Pattern: {:?}", pattern_to_find);
            } else {
                println!("Please provide a pattern.");
            }
            if let Some(replace) = &command.replace {
                replacement_text = replace.clone();
                println!("Replacement text: {:?}", replacement_text);
            } else {
                println!("Please provide a replacement text.");
            }
        }
        None => println!("Unknown command. Use '--help' for usage instructions.")
    };

    println!("Content Before: {:?}", input_content);
    find_and_replace(&mut input_content, &pattern_to_find, &replacement_text);
    println!("Content After: {:?}", input_content);
}
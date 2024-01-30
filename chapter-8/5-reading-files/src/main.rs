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

fn read_file_to_string(file_path: &str) -> io::Result<String> { // <1>
    let mut file = File::open(file_path)?; // <2>
    let mut contents = String::new(); // <2>
    file.read_to_string(&mut contents)?; // <2>

    Ok(contents) // <2>
}

fn main() { // <3>
    let args = Cli::parse();
    let input_content;
    match args.command {
        Some(Commands::Find(command)) => { // <4>
            match command.input {
                Some(ref input) => {
                    // cargo run -- find --input file.txt
                    input_content = read_file_to_string(input).unwrap();
                    println!("Input file content: {:?}", input_content);
                }
                None => {
                    println!("Please provide a file name.");
                }
            };
            match command.pattern {
                Some(ref pattern) => {
                    // cargo run -- find --pattern custom_pattern
                    println!("{:?}", pattern);
                }
                None => {
                    println!("Please provide a pattern.");
                }
            };
        }
        Some(Commands::Replace(command)) => { // <5>
            match command.input {
                Some(ref input) => {
                    // cargo run -- replace --input file_name
                    println!("{:?}", input);
                }
                None => {
                    println!("Please provide a file name.");
                }
            };
            match command.pattern {
                Some(ref pattern) => {
                    // cargo run -- replace --pattern custom_pattern
                    println!("{:?}", pattern);
                }
                None => {
                    println!("Please provide a pattern.");
                }
            };
            match command.replace {
                Some(ref replace) => {
                    // cargo run -- replace --replace string
                    println!("{:?}", replace);
                }
                None => {
                    println!("Please provide a pattern.");
                }
            };
        }
        None => println!(
            "Unknown command. Use '--help' for usage instructions."
        )
    };
}
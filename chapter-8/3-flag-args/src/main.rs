use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version = "1.0",
    about = "A command-line find and replace utility",
    name = "Find and Replace"
)]
struct Args {
    /// Sets the input file to process
    #[arg(short = 'i', long = "input")]
    input: String,
    /// Sets the pattern to find
    #[arg(short = 'f', long = "find")]
    find: String,
    /// Sets the replacement text
    #[arg(short = 'r', long = "replace")]
    replace: String,
    /// Perform a case-insensitive search and replace
    #[arg(short = 'c', long = "ignore-case")]
    ignore_case: bool,
}

fn main() {
    // Access and use the `input`, `find`, `replace`, and `ignore-case` arguments here
    let _args = Args::parse();
}

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
}

fn main() {
    // Access and use the `input`, `find`, and `replace` arguments here
    let _args = Args::parse();
}

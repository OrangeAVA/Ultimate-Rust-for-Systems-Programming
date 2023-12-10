use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version = "1.0",
    about = "A command-line find and replace utility",
    name = "find-replace"
)]
struct Args {
    /// Sets the input file to process
    #[arg(short = 'i', long = "input")]
    input: String,
}

fn main() {
    // Access and use the `input` argument here
    let args = Args::parse();

    println!("Parsed Input file name: {}!", args.input);
}

use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
struct Args {
    /// Files to show
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(short = 'l', long = "lines", default_value = "false")]
    lines: bool,

    /// Number of words
    #[arg(
        short = 'w',
        long = "words",
        default_value = "false",
        conflicts_with = "bytes"
    )]
    words: bool,

    /// Number of bytes
    #[arg(
        short = 'c',
        long = "bytes",
        default_value = "false",
        conflicts_with = "words"
    )]
    bytes: bool,

    /// Number of characters
    #[arg(short = 'm', long = "chars", default_value = "false")]
    chars: bool,
}

fn main() {
    let mut args = Args::parse();

    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|&x| !x)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    if let Err(e) = run(args) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> MyResult<()> {
    println!("{:?}", args);
    Ok(())
}

use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
struct Args {
    /// Input file
    #[arg(default_value = "-")]
    in_file: String,

    /// Output file
    out_file: Option<String>,

    /// Show counts
    #[arg(short = 'c', long = "count", default_value = "false")]
    count: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> MyResult<()> {
    println!("{:?}", args);
    Ok(())
}

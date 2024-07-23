use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

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
    let mut file = open(&args.in_file).map_err(|e| format!("{}: {}", &args.in_file, e))?;
    let mut out_file: Box<dyn Write> = match args.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    // This closure must be declared as mut to refer mutable out_file.
    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if args.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        }
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

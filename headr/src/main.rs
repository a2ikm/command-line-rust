use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
struct Args {
    /// Files to show
    #[arg(default_value = "-")]
    file: Vec<String>,

    /// Number to show characters in bytes
    #[arg(short = 'c', long = "bytes", value_name = "BYTES", value_parser = clap::value_parser!(u64).range(1..))]
    num_bytes: Option<u64>,

    /// Number to show lines
    #[arg(short = 'n', long = "lines", default_value = "10", value_name = "LINES", value_parser = clap::value_parser!(u64).range(1..))]
    num_lines: u64,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let num_files = args.file.len();

    for (file_num, filename) in args.file.iter().enumerate() {
        if num_files > 1 {
            println!(
                "{}==> {} <==",
                if file_num > 0 { "\n" } else { "" },
                filename,
            );
        }

        match open(&filename) {
            Ok(mut reader) => {
                if let Some(num_bytes) = args.num_bytes {
                    show_head_by_bytes(&mut reader, num_bytes)?;
                } else {
                    show_head_by_lines(&mut reader, args.num_lines)?;
                };
            }
            Err(e) => {
                eprintln!("{}: {}", filename, e);
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn show_head_by_bytes(reader: &mut Box<dyn BufRead>, num_bytes: u64) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; num_bytes.try_into().unwrap()];
    let n = reader.read(&mut buffer[..])?;
    print!("{}", String::from_utf8_lossy(&buffer[0..n]));
    Ok(())
}

fn show_head_by_lines(reader: &mut Box<dyn BufRead>, num_lines: u64) -> Result<(), Box<dyn Error>> {
    for _ in 0..num_lines {
        let mut buffer = String::new();
        let n = reader.read_line(&mut buffer)?;
        if n == 0 {
            break;
        }
        print!("{}", buffer);
    }
    Ok(())
}

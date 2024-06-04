use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

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
    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let info = count(file)?;
                print(&args, filename, &info);

                total.num_lines += info.num_lines;
                total.num_words += info.num_words;
                total.num_bytes += info.num_bytes;
                total.num_chars += info.num_chars;
            }
        }
    }

    if args.files.len() > 1 {
        print(&args, "total", &total);
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }

        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += line_bytes;
        num_chars += line.chars().count();

        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn print(args: &Args, filename: &str, info: &FileInfo) {
    let mut output = String::new();
    if args.lines {
        output.push_str(&format!("{:>8}", info.num_lines));
    }
    if args.words {
        output.push_str(&format!("{:>8}", info.num_words));
    }
    if args.bytes {
        output.push_str(&format!("{:>8}", info.num_bytes));
    }
    if args.chars {
        output.push_str(&format!("{:>8}", info.num_chars));
    }
    if filename != "-" {
        output.push_str(&format!(" {}", filename));
    }
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}

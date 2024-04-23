use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_num = 0;
                for line_result in file.lines() {
                    let line = line_result?;

                    if config.number_lines {
                        line_num += 1;
                    }

                    if config.number_lines {
                        print!("{:6}  ", line_num + 1);
                    } else if config.number_nonblank_lines && !line.trim().is_empty() {
                        print!("{:6}  ", line_num + 1);
                    }
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .value_name("FILES")
                .help("Files to concatenate")
                .required(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number all output lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number nonempty output lines, overrides -n"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        number_lines: matches.contains_id("number_lines"),
        number_nonblank_lines: matches.contains_id("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

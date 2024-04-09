use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
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
                .num_args(1..),
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

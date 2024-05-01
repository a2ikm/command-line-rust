use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    strings: Vec<String>,

    /// omit newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    for (i, arg) in args.strings.iter().enumerate() {
        if i > 0 {
            print!(" ")
        }
        print!("{}", arg)
    }

    if !args.omit_newline {
        print!("\n")
    }
}

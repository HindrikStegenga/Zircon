use clap::*;
use std::fs::*;
use std::io::*;
use std::path::*;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input of the compiler. If given a directory tries to compile all appropriate shader files in the directory it can find.
    #[clap(short, long)]
    input: String,
    /// Output file
    #[clap(short, long, default_value = "./out")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let path = PathBuf::from_str(&args.input).expect("Given input not a valid path!");
}

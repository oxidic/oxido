use clap::Parser as clap_parser;
use std::fs;

use crate::errors::Error;

mod errors;
mod token;
mod parser;
mod lexer;

#[derive(clap_parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to output debug information
    #[clap(short, long, value_parser)]
    debug: bool,

    #[clap()]
    input: String,
}

fn main() {
    let args = Args::parse();
    let contents = match fs::read_to_string(&args.input) {
        Ok(text) => text,
        Err(_) => {
            Error::throw(
                &args.input,
                &args.input,
                0,
                &format!("File '{}' was not found", args.input),
                false,
            );
            String::new()
        }
    };

    parser::Parser::new(args.input, contents.lines().map(|f| f.trim().to_string()).collect()).parse();

}

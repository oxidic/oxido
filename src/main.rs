use clap::Parser as clap_parser;
use std::fs;

mod lexer;
mod parser;
mod token;

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
        Err(e) => {
            parser::Parser::new(String::new(), vec![String::new()]).throw(
                0,
                format!("Error while reading file {}:\n\t{e}", args.input),
                false,
            );
            String::new()
        }
    };

    parser::Parser::new(
        args.input,
        contents.lines().map(|f| f.trim().to_string()).collect(),
    )
    .run();
}

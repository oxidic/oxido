use clap::Parser as ClapParser;
use lexer::Lexer;
use std::fs;

mod lexer;
mod token;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to output debug information
    #[clap(short, long, value_parser)]
    debug: bool,

    /// Whether to not run the actual code
    #[clap(short, long, value_parser)]
    no_run: bool,

    #[clap()]
    input: String,
}

fn main() {
    let args = Args::parse();
    let filename = String::from(&args.input);

    if args.debug {
        println!("D: {}\nN: {}", args.debug, args.no_run);
    }

    run(filename, args.debug, args.no_run);
}

fn run(mut file: String, debug: bool, no_run: bool) {
    if fs::metadata(&file).unwrap().is_dir() {
        file = file.to_owned() + "/main.o";
    }

    let contents = match fs::read_to_string(&file) {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    Lexer::new(contents).lexer();

}

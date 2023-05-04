use clap::Parser;
use lexer::Lexer;
use std::fs::{metadata, read_to_string};
use std::process::exit;
use std::time::Instant;

mod ast;
mod data;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod standardlibrary;
mod token;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to output debug information
    #[clap(short, long, value_parser)]
    debug: bool,

    /// Whether to dry run
    #[clap(long, value_parser)]
    dry_run: bool,

    /// Whether to print the time elapsed
    #[clap(short, long, value_parser)]
    time: bool,

    /// The code which is to be executed
    #[clap(short, long, value_parser)]
    code: Option<String>,

    /// The path of file which is to be executed
    #[clap()]
    input: Option<String>,
}

struct Config {
    debug: bool,
    dry_run: bool,
    time: bool,
}

impl Config {
    pub fn new(debug: bool, dry_run: bool, time: bool) -> Self {
        Self {
            debug,
            dry_run,
            time,
        }
    }
}

fn main() {
    let args = Args::parse();

    let contents = if args.code.is_some() {
        args.code.unwrap()
    } else if args.input.is_some() {
        let mut input = args.input.clone().unwrap();

        let root = metadata(&input);
        if root.unwrap().is_dir() {
            let main = metadata(input.clone() + "/main.oxi");
            if main.unwrap().is_file() {
                input += "/main.oxi";
            } else {
                let src = metadata(input.clone() + "/src");
                if src.is_ok() && !src.unwrap().is_dir() {
                    println!("error while reading, `{input}` is a dir and does not has `src` or is a file");
                    exit(1);
                }
                let main = metadata(input.to_string() + "/src/main.oxi");
                if main.is_ok() && main.unwrap().is_dir() {
                    println!(
                        "error while reading, `{input}` is a dir and does not has `src/main.oxi` or is a dir"
                    );
                    exit(1);
                }
                input += "/src/main.oxi"
            }
        }

        match read_to_string(input) {
            Ok(text) => text,
            Err(error) => panic!("error while reading file, {error}"),
        }
    } else {
        println!("expected either file name or contents with -c flag");
        exit(1);
    };

    let config = Config::new(args.debug, args.dry_run, args.time);

    run(args.input.unwrap_or_default(), contents, config);
}

fn run(name: String, contents: String, config: Config) {
    let main = Instant::now();

    let mut lexer = Lexer::new(&name, &contents);
    let tokens = lexer.run().unwrap();

    if config.debug {
        let duration = main.elapsed();
        println!("LEXER: {tokens:?}\n\nTIME: {duration:?}\n");
    }

    let parser = parser::Parser::new(&name, &contents);
    let ast = parser.run(tokens.to_vec()).unwrap();

    if config.debug {
        let duration = main.elapsed();
        println!("AST: {ast:?}\n\nTIME: {duration:?}\n");
    }
    if config.dry_run {
        return;
    }

    let mut interpreter = interpreter::Interpreter::new(&name, &contents);
    interpreter.run(ast.to_vec());

    if config.debug || config.time {
        let duration = main.elapsed();
        println!("\nTIME: {duration:?}");
    }

    println!()
}

use clap::Parser;
use lexer::Lexer;
use std::fs;
use std::time::Instant;

mod ast;
mod datatype;
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

    #[clap()]
    input: String,
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
    let filename = String::from(&args.input);

    let config = Config::new(args.debug, args.dry_run, args.time);

    let contents = readfile(&filename);

    run(filename, contents, config);
}

fn readfile(file: &str) -> String {
    let mut file = file.to_string();

    if fs::metadata(&file).unwrap().is_dir() {
        file += "/main.oxi";
    }

    match fs::read_to_string(&file) {
        Ok(text) => text,
        Err(error) => panic!("error while reading file {error}"),
    }
}

fn run(name: String, contents: String, config: Config) {
    let main = Instant::now();

    let mut lexer = Lexer::new(&name, &contents);
    let tokens = lexer.run();

    if config.debug {
        let duration = main.elapsed();
        println!("LEXER: {tokens:?}\n\nTIME: {duration:?}\n");
    }

    let mut parser = parser::Parser::new(tokens.to_vec(), contents.clone(), name.clone());
    let ast = parser.run();

    if config.debug {
        let duration = main.elapsed();
        println!("AST: {ast:?}\n\nTIME: {duration:?}\n");
    }
    if config.dry_run {
        return;
    }

    let mut interpreter = interpreter::Interpreter::new(ast.to_vec(), contents, name);
    interpreter.run();

    if config.debug || config.time {
        let duration = main.elapsed();
        println!("\nTIME: {duration:?}");
    }

    println!()
}

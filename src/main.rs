use clap::Parser;
use lexer::Lexer;
use std::fs;

mod ast;
mod lexer;
mod parser;
mod token;
mod interpreter;
mod datatype;

#[derive(Parser, Debug)]
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

    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.run();

    if debug {
        println!("LEXER: {tokens:?}\n");
    }

    let mut parser = parser::Parser::new(tokens.to_vec());
    let ast = parser.run();

    if debug {
        println!("AST: {ast:?}\n");
    }

    if no_run {
        return;
    }

    let mut interpreter = interpreter::Interpreter::new(ast.to_vec());
    interpreter.run();
}

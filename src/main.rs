use lexer::Lexer;
use std::fs;
use clap::Parser;

mod ast;
mod datatype;
mod globals;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod error;

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

    readfile(filename, args.debug, args.no_run);
}

fn readfile(mut file: String, debug: bool, no_run: bool) {
    if fs::metadata(&file).unwrap().is_dir() {
        file = file.to_owned() + "/main.o";
    }

    let contents = match fs::read_to_string(&file) {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    run(file, contents, debug, no_run)
}

pub fn run(name: String, contents: String, debug: bool, no_run: bool) {
    let mut lexer = Lexer::new(&name ,&contents);
    let tokens = lexer.run();

    if debug {
        println!("LEXER: {tokens:?}\n");
    }

    let mut parser = parser::Parser::new(tokens.to_vec(), contents.clone(), name.clone());
    let ast = parser.run();

    if debug {
        println!("AST: {ast:?}\n");
    }

    if no_run {
        return;
    }

    let mut interpreter = interpreter::Interpreter::new(ast.to_vec(), contents, name);
    interpreter.run();
}
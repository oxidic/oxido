use lexer::Lexer;
use std::time::Instant;

mod ast;
mod data;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod standardlibrary;
mod token;
pub struct Config {
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

pub fn run(name: String, contents: String, config: Config) {
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

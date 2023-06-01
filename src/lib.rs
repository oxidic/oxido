use instant::Instant;
use lexer::Lexer;
use wasm_bindgen::prelude::wasm_bindgen;

extern crate console_error_panic_hook;
use std::panic;

mod ast;
mod data;
mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
mod standardlibrary;
mod token;

#[wasm_bindgen]
pub struct Config {
    debug: bool,
    dry_run: bool,
    time: bool,
}

#[wasm_bindgen]
impl Config {
    pub fn new(debug: bool, dry_run: bool, time: bool) -> Self {
        Self {
            debug,
            dry_run,
            time,
        }
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn run(name: String, contents: String, config: Config) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
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

    let mut interpreter = interpreter::Interpreter::new(&name, contents);
    interpreter.run(ast.to_vec());

    if config.debug || config.time {
        let duration = main.elapsed();
        println!("\nTIME: {duration:?}");
    }

    println!()
}

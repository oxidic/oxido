use clap::Parser;
use std::fs;

mod lexer;
mod token;
mod expression;
mod ast;

#[derive(Parser, Debug)]
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
    let filename = String::from(&args.input);

    run(filename, args.debug);
}

fn run(mut file: String, _debug: bool) {
    if fs::metadata(&file).unwrap().is_dir() {
        file = file.to_owned() + "/main.o";
    }

    let contents = match fs::read_to_string(&file) {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    lexer::Lexer::new(contents).tokenize().lex();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn declaration() {
        run("tests/declaration.o".to_string(), false)
    }

    #[test]
    fn reassignment() {
        run("tests/reassignment.o".to_string(), false)
    }

    #[test]
    fn function() {
        run("tests/function.o".to_string(), false)
    }

    #[test]
    fn r#if() {
        run("tests/if.o".to_string(), false)
    }

    #[test]
    fn r#loop() {
        run("tests/loop.o".to_string(), false)
    }
}

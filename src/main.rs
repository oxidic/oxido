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
    let filename = String::from(&args.input);

    run(filename, args.debug);
}

fn run(mut file: String, debug: bool) {
    if fs::metadata(&file).unwrap().is_dir() {
        file = file.to_owned() + "/main.o";
    }

    let contents = match fs::read_to_string(&file) {
        Ok(text) => text,
        Err(e) => {
            parser::Parser::new(String::new(), vec![String::new()]).throw(
                0,
                format!("Error while reading file {}:\n\t{e}", file),
                false,
            );
            String::new()
        }
    };

    let parser = parser::Parser::new(
        file,
        contents.lines().map(|f| f.trim().to_string()).collect(),
    )
    .run();

    if debug {
        println!("{parser:#?}");
    }
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

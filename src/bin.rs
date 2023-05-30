use clap::Parser;
use oxidolib::{run, Config, version};
use std::fs::{metadata, read_to_string};
use std::process::exit;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

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
        println!("Welcome to Oxido v{}.\nTo exit, press CTRL+C or CTRL+D", version());
        let mut rl = DefaultEditor::new().unwrap();
        let mut code = String::new();
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(&line).unwrap();
                    code.push_str(&line);
                    run(String::from("REPL"), code.clone(), Config::new(false, false, false))
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        exit(1);
    };

    let config = Config::new(args.debug, args.dry_run, args.time);

    run(args.input.unwrap_or_default(), contents, config);
}

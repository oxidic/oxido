use std::env;
use std::{collections::HashMap, fs};

mod util;
mod parser;
mod token;

fn main() {
    let mut store: HashMap<&str, String> = util::get_hash();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return println!("CLI: file to run was not provided!")
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().filter(|line| line != &"").collect();

    for line in lines {
        store = parser::parse(line, store);
    }

    println!("DEBUG: {:?}", store);
}

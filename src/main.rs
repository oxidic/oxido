use crate::store::Store;
use std::env;
use std::fs;

mod parser;
mod store;
mod token;
mod util;

fn main() {
    let mut store: Store = Store::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return println!("CLI: file to run was not provided!");
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();

    for line in lines {
        store.increment_line(line.to_string());
        store = parser::parse(line, store);
    }

    if args.contains(&String::from("-D")) {
        println!("DEBUG: {}", store);
    }
}

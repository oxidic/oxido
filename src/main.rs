use crate::store::Store;
use std::env;
use std::fs;

mod parser;
mod store;
mod token;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return println!("CLI: file to run was not provided!");
    }

    let mut filename: String = (&args[1]).to_string();

    if !filename.ends_with(".o") {
        filename = String::from(filename.to_owned() + "/main.o");
    }

    let contents = fs::read_to_string(filename.clone()).expect("Something went wrong reading the file");

    let mut store: Store = Store::new(
        filename,
        contents.lines().map(|f| f.to_string()).collect(),
    );

    loop {
        if store.current_line == store.total_lines && !store.is_looping {
            break;
        }
        if store.is_looping {
            if store.loop_line == store.loop_stack.len() {
                store.loop_line = 0;
            }
            store = parser::parse(
                store.loop_stack.get(store.loop_line).unwrap().to_string(),
                store.clone(),
            );
            store.loop_line += 1;
            continue;
        }
        store.increment_line(store.lines.get(0).unwrap().to_string());
        store = parser::parse(store.lines.remove(0).to_string(), store);
    }

    if args.contains(&String::from("-D")) {
        println!("DEBUG: {}", store);
    }
}

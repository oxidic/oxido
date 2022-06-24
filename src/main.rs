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

    let filename = &args[1];

    run(filename);
}

fn run(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut store: Store = Store::new(
        filename.to_string(),
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
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn declaration() {
        run("tests/declaration.o");
    }
    #[test]
    fn reassignment() {
        run("tests/reassignment.o");
    }
    #[test]
    fn if_statement() {
        run("tests/if_statement.o");
    }
    #[test]
    fn loop_statement() {
        run("tests/loop_statement.o");
    }
}
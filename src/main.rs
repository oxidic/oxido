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
        if store.states.functions.running {
            let mut function = store
                .functions
                .get(&store.states.functions.current)
                .unwrap()
                .clone();
            let variables = store.variables.clone();
            for (i, v) in store.states.functions.args.iter() {
                store.variables.insert(i.to_string(), v.to_string());
            }
            loop {
                if function.lines.len() == 0 {
                    break;
                }
                store = parser::parse(function.lines.remove(0).to_string(), store);
            }
            store.states.functions.running = false;
            store.states.functions.current = String::new();
            store.variables = variables;
            continue;
        }
        if store.lines.at == store.lines.total && !store.states.loops.looping {
            break;
        }
        if store.states.loops.looping {
            if store.states.loops.loop_line == store.states.loops.stack.len().try_into().unwrap() {
                store.states.loops.loop_line = 0;
            }
            store = parser::parse(
                store
                    .states
                    .loops
                    .stack
                    .get(store.states.loops.loop_line as usize)
                    .unwrap()
                    .to_string(),
                store.clone(),
            );
            store.states.loops.loop_line += 1;
            continue;
        }
        store.increment_line(store.lines.lines.get(0).unwrap().to_string());
        store = parser::parse(store.lines.lines.remove(0).to_string(), store);
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
    #[test]
    fn function() {
        run("tests/function.o");
    }
}

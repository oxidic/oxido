use crate::{
    store::Store,
    token::Token,
    util::{check_syntax, parse_ident},
};
use logos::Logos;

mod assignment;
mod r#break;
mod declaration;
mod expression;
mod function;
mod if_statement;
mod r#loop;
mod print;

pub fn parse(line: String, mut store: Store) -> Store {
    let lex = Token::lexer(&line);

    let token = lex.clone().next();

    // NEWLINES
    if token == None {
        return store;
    }

    /*
    IF a CURLYBRACE is closed then we determine whether it was an IF CURLYBRACE or LOOP CURLYBRACE

    IF: Subtract 1 from scopes._if as IF has ended

    LOOP: Loop can be ran now and Subtract 1 from scopes._loop as LOOP has endede
     */
    if token.unwrap() == Token::CurlyBraceClose {
        let token = store.states.stack.last().unwrap();
        if token == "if" {
            store.scopes._if -= 1;
            if store.scopes._loop > 0 {
                store.states.loops.stack.push(store.clone().lines.text);
            }
        } else if token == "loop" {
            store.scopes._loop -= 1;
            store.states.loops.looping = true;
        } else if token == "function" {
            store.states.functions.capturing = false;
        }
        store.states.stack.pop();
        return store;
    }

    /*
    IF LOOP is still capturing input, push it
     */
    let functions = store.clone().states.functions;
    if functions.capturing {
        let mut function = store.functions.get(&functions.current).unwrap().clone();
        function.lines.push(store.lines.text.clone());
        store.functions.insert(functions.current, function);
        return store;
    }

    /*
    IF LOOP is still capturing input, push it
     */
    if store.scopes._loop > 0 {
        store.states.loops.stack.push(store.clone().lines.text);
    }

    /*
    IF CURCLYBRACE is opened then determine if it was an IF open or LOOP one and handle accordingly
     */
    if store.scopes._if > 0 || store.scopes._loop > 0 {
        if store.clone().lines.text.contains("{") {
            if store.clone().lines.text.contains("if") {
                store.scopes._if += 1;
                store.states.stack.push(String::from("if"));
            } else if store.clone().lines.text.contains("loop") {
                store.scopes._loop += 1;
                store.states.stack.push(String::from("loop"));
            }
        }
        return store;
    }

    match token.unwrap() {
        Token::Let => store = declaration::parse_declaration(lex, store),
        Token::Print => store = print::parse(lex, store),
        Token::If => store = if_statement::parse_if_statement(lex, store),
        Token::Loop => store = r#loop::parse_loop(lex, store),
        Token::Function => store = function::parse_function(lex, store),
        Token::Break => store = r#break::parse_break(lex, store),
        _ => {
            let mut lex_clone = lex.clone();
            // TOKEN:: IDENT
            check_syntax(lex_clone.next(), Token::Ident, &store);

            let var = store.variables.get(lex_clone.slice());
            let function = store.functions.get(lex_clone.slice());

            if var != None {
                store = assignment::parse_assignment(lex, store);
            } else if function != None {
                store.states.functions.current = lex_clone.slice().to_string();
                let mut c = 0;
                check_syntax(lex_clone.next(), Token::SquareBraceOpen, &store);
                loop {
                    if lex_clone.next().unwrap() == Token::SquareBraceClose {
                        break;
                    }
                    let arg = lex_clone.slice().to_string();
                    if arg == "," {
                        continue;
                    }
                    store.states.functions.args.insert(
                        function.unwrap().arguments[c].clone(),
                        parse_ident(&arg, &store),
                    );
                    c += 1;
                }
                store.states.functions.running = true;
            }
        }
    }
    store
}

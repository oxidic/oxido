use crate::{store::Store, token::Token, util::check_syntax};
use logos::{Logos};

mod assignment;
mod declaration;
mod expression;
mod if_statement;
mod r#loop;
mod print;

pub fn parse<'a>(line: &'a str, mut store: Store<'a>) -> Store<'a> {
    let lex = Token::lexer(line);

    let token = lex.clone().next();

    // NEWLINES
    if token == None {
        return store;
    }

    if token.unwrap() == Token::CurlyBraceClose {
        let token = store.bracket_stack.last().unwrap();
        println!("{}", token);
        if token == "if" {
            store.decrement_scope();
        } else if token == "loop" {
            store.decrement_loop();
        }
        store.bracket_stack.pop();
        return store;
    }

    if store.get_scope() > 0 || store.get_loop() > 0 {
        if store.line_text().contains("{") {
            if store.line_text().contains("if") {
                store.increment_scope();
                store.bracket_stack.push(String::from("if"));
            } else if store.line_text().contains("loop") {
                store.increment_loop();
                store.bracket_stack.push(String::from("loop"));
            }
        }
        return store;
    }

    if store.get_loop() > 0 {
        store.loop_stack.push(store.line_text());
    }

    match token.unwrap() {
        Token::Let => store = declaration::parse_declaration(lex, store),
        Token::Print => store = print::parse(lex, store),
        Token::If => store = if_statement::parse_if_statement(lex, store),
        Token::Loop => store = r#loop::parse_loop(lex, store),
        _ => {
            let mut lex_clone = lex.clone();
            // TOKEN:: IDENT
            check_syntax(lex_clone.next(), Token::Ident, &store);

            let var = store.get_variable(lex_clone.slice());

            if var != None {
                store = assignment::parse_assignment(lex, store);
            } else {
                panic!("SyntaxError: unexpected token",);
            }
        }
    }
    store
}

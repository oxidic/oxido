use crate::{store::Store, token::Token, util::check_syntax};
use logos::Logos;

mod assignment;
mod r#break;
mod declaration;
mod expression;
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

    if token.unwrap() == Token::CurlyBraceClose {
        let token = store.bracket_stack.last().unwrap();
        if token == "if" {
            store.scope -= 1;
            if store.r#loop > 0 {
                store.loop_stack.push(store.clone().line_text);
            }
        } else if token == "loop" {
            store.r#loop -= 1;
            store.is_looping = true;
        }
        store.bracket_stack.pop();
        return store;
    }

    if store.r#loop > 0 {
        store.loop_stack.push(store.clone().line_text);
    }

    if store.scope > 0 || store.r#loop > 0 {
        if store.clone().line_text.contains("{") {
            if store.clone().line_text.contains("if") {
                store.scope += 1;
                store.bracket_stack.push(String::from("if"));
            } else if store.clone().line_text.contains("loop") {
                store.r#loop += 1;
                store.bracket_stack.push(String::from("loop"));
            }
        }
        return store;
    }

    match token.unwrap() {
        Token::Let => store = declaration::parse_declaration(lex, store),
        Token::Print => store = print::parse(lex, store),
        Token::If => store = if_statement::parse_if_statement(lex, store),
        Token::Loop => store = r#loop::parse_loop(lex, store),
        Token::Break => store = r#break::parse_break(lex, store),
        _ => {
            let mut lex_clone = lex.clone();
            // TOKEN:: IDENT
            check_syntax(lex_clone.next(), Token::Ident, &store);

            let var = store.get_variable(lex_clone.slice());

            if var != None {
                store = assignment::parse_assignment(lex, store);
            }
        }
    }
    store
}

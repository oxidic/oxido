use crate::token::Token;
use logos::Lexer;
use std::collections::HashMap;
use super::expression::parse_expression;

pub fn parse_assignment<'a>(
    mut lex: Lexer<'a, Token>,
    mut store: HashMap<&'a str, String>,
) -> HashMap<&'a str, String> {
    // TOKEN: IDENT
    lex.next();
    let ident = lex.slice();

    // TOKEN: =
    lex.next();

    // TOKEN: TEXT
    let value: String;

    if lex.clone().count() > 2 {
        (value, store) = parse_expression(&lex, store);
    } else {
        match lex.next().unwrap() {
            Token::Number => value = lex.slice().parse().unwrap(),
            Token::String => value = lex.slice().parse().unwrap(),
            _ => value = String::new(),
        }
    }

    store.insert(ident, value.replace('"', ""));

    store
}
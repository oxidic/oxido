use crate::{token::Token, util::check_data_type, store::Store};
use logos::Lexer;
use super::expression::parse_expression;

pub fn parse_assignment<'a>(
    mut lex: Lexer<'a, Token>,
    mut store: Store<'a>,
) -> Store<'a> {
    // TOKEN: IDENT
    check_data_type(lex.next(), Token::Ident, &store);

    let ident = lex.slice();

    // TOKEN: =
    check_data_type(lex.next(), Token::Assignment, &store);

    // TOKEN: TEXT
    let value: String;

    if lex.clone().count() > 2 {
        (value, store) = parse_expression(&lex, store);
    } else {
        match lex.next().unwrap() {
            Token::Integer => value = lex.slice().parse().unwrap(),
            Token::String => value = lex.slice().parse().unwrap(),
            _ => value = String::new(),
        }
    }

    store.set_variable(ident, value.replace('"', ""));

    check_data_type(lex.last(), Token::Semicolon, &store);

    store
}
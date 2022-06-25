use super::expression::parse_expression;
use crate::{store::Store, token::Token, util::check_syntax};
use logos::Lexer;

pub fn parse_assignment(mut lex: Lexer<Token>, mut store: Store) -> Store {
    // TOKEN: IDENT
    check_syntax(lex.next(), Token::Ident, &store);

    let ident = lex.slice();

    // TOKEN: =
    check_syntax(lex.next(), Token::Assignment, &store);

    // TOKEN: TEXT
    let value: String;

    if lex.clone().count() > 2 {
        (value, store) = parse_expression(&mut lex, store);
    } else {
        match lex.next().unwrap() {
            Token::Integer => value = lex.slice().parse().unwrap(),
            Token::String => value = lex.slice().parse().unwrap(),
            Token::Bool => value = lex.slice().parse().unwrap(),
            _ => value = String::new(),
        }
    }

    store
        .variables
        .insert(ident.to_string(), value.replace('"', ""));

    check_syntax(lex.last(), Token::Semicolon, &store);

    store
}

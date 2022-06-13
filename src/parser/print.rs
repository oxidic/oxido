use crate::token::Token;
use logos::Lexer;
use std::collections::HashMap;

pub fn parse<'a>(
    mut lex: Lexer<'a, Token>,
    store: HashMap<&'a str, String>,
) -> HashMap<&'a str, String> {
    // TOKEN: PRINT
    lex.next();
    // TOKEN: BRACKET
    lex.next();

    lex.next();
    let idnt = lex.slice();
    let value = store.get(idnt).unwrap();

    println!("{}", value);

    store
}
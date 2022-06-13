use std::collections::HashMap;
use logos::Lexer;
use crate::token::Token;
use super::assignment::parse_assignment;

pub fn parse_declaration<'a>(
    mut lex: Lexer<'a, Token>,
    store: HashMap<&'a str, String>,
) -> HashMap<&'a str, String> {
    // TOKEN: LET
    lex.next();

    parse_assignment(lex, store)
}
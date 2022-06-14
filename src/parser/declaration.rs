use logos::Lexer;
use crate::{token::Token, util::check_data_type, store::Store};
use super::assignment::parse_assignment;

pub fn parse_declaration<'a>(
    mut lex: Lexer<'a, Token>,
    store: Store<'a>,
) -> Store<'a> {
    // TOKEN: LET
    let token = lex.next();

    check_data_type(token, Token::Let, &store);

    parse_assignment(lex, store)
}
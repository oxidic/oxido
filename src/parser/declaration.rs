use logos::Lexer;
use crate::{token::Token, util::check_syntax, store::Store};
use super::assignment::parse_assignment;

pub fn parse_declaration(
    mut lex: Lexer<Token>,
    store: Store,
) -> Store {
    // TOKEN: LET
    let token = lex.next();

    check_syntax(token, Token::Let, &store);

    parse_assignment(lex, store)
}
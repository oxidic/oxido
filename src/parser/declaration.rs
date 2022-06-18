use super::assignment::parse_assignment;
use crate::{store::Store, token::Token, util::check_syntax};
use logos::Lexer;

pub fn parse_declaration(mut lex: Lexer<Token>, store: Store) -> Store {
    // TOKEN: LET
    let token = lex.next();

    check_syntax(token, Token::Let, &store);

    parse_assignment(lex, store)
}

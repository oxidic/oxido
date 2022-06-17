use logos::Lexer;
use crate::{token::Token, store::Store, util::check_syntax};

pub fn parse_loop<'a>(
    mut lex: Lexer<'a, Token>,
    mut store: Store<'a>,
) -> Store<'a> {

    check_syntax( lex.next(), Token::Loop, &store);
    check_syntax( lex.next(), Token::CurlyBraceOpen, &store);

    store.increment_loop();
    store.bracket_stack.push(String::from("loop"));

    store
}
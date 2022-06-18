use logos::Lexer;
use crate::{token::Token, store::Store, util::check_syntax};

pub fn parse_loop(
    mut lex: Lexer<Token>,
    mut store: Store,
) -> Store {
    check_syntax( lex.next(), Token::Loop, &store);
    check_syntax( lex.next(), Token::CurlyBraceOpen, &store);

    store.r#loop += 1;
    store.bracket_stack.push(String::from("loop"));

    store
}
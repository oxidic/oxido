use logos::Lexer;
use crate::{token::Token, store::Store};

pub fn parse_break(
    _: Lexer<Token>,
    mut store: Store,
) -> Store {
    store.is_looping = false;
    store
}
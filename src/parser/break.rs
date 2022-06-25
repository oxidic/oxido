use crate::{store::Store, token::Token};
use logos::Lexer;

pub fn parse_break(_: Lexer<Token>, mut store: Store) -> Store {
    store.states.loops.looping = false;
    store
}

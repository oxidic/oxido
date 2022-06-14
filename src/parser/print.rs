use crate::{token::Token, store::Store};
use logos::Lexer;

pub fn parse<'a>(
    mut lex: Lexer<'a, Token>,
    store: Store<'a>,
) -> Store<'a> {
    // TOKEN: PRINT
    lex.next();
    // TOKEN: BRACKET
    lex.next();

    lex.next();
    let idnt = lex.slice();
    let value = store.get_variable(idnt).unwrap();

    println!("{}", value);

    store
}
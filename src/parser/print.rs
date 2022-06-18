use crate::{token::Token, store::Store, parser::expression::parse_expression};
use logos::Lexer;

pub fn parse(
    mut lex: Lexer<Token>,
    store: Store,
) -> Store {
    // TOKEN: PRINT
    lex.next();
    // TOKEN: BRACKET
    lex.next();

    let (r, store) = parse_expression(&mut lex, store);

    println!("{}", r);

    store
}
use crate::{parser::expression::parse_expression, store::Store, token::Token};
use logos::Lexer;

pub fn parse(mut lex: Lexer<Token>, store: Store) -> Store {
    // TOKEN: PRINT
    lex.next();
    // TOKEN: BRACKET
    lex.next();

    let (r, store) = parse_expression(&mut lex, store);

    println!("{}", r);

    store
}

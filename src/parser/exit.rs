use crate::{parser::expression::parse_expression, store::Store, token::Token};
use logos::Lexer;

pub fn parse(mut lex: Lexer<Token>, store: Store) -> ! {
    // TOKEN: EXIT
    lex.next();
    // TOKEN: BRACKET
    lex.next();

    let (r, _store) = parse_expression(&mut lex, store);

    std::process::exit(
        r.parse::<i32>()
            .expect("Value passed to exit must be a number"),
    );
}

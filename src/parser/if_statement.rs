use crate::{store::Store, token::Token, util::{check_data_type, parse_ident}, parser::parse_with_lex};
use logos::Lexer;

pub fn parse_if_statement<'a>(mut lex: Lexer<'a, Token>, mut store: Store<'a>) -> Store<'a> {
    // TOKEN: if
    check_data_type(lex.next(), Token::If, &store);

    let mut is_equal = false;

    lex.next();
    let lhs = parse_ident(&lex.slice().to_string(), &store);
    let op = lex.next().unwrap();
    lex.next();
    let rhs = parse_ident(&lex.slice().to_string(), &store);

    match op {
        Token::Equality => {
            if lhs == rhs {
                is_equal = true
            } else {
                is_equal = false
            }
        }
        _ => {}
    }

    if is_equal {
        store = parse_with_lex(lex, store);
    }

    store
}

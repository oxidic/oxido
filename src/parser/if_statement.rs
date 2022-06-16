use crate::{
    store::Store,
    token::Token,
    util::{check_data_type, parse_ident},
};
use logos::Lexer;

pub fn parse_if_statement<'a>(mut lex: Lexer<'a, Token>, mut store: Store<'a>) -> Store<'a> {
    // TOKEN: if
    check_data_type(lex.next(), Token::If, &store);

    lex.next();
    let lhs = parse_ident(&lex.slice().to_string(), &store).replace("\"", "");
    let op = lex.next().unwrap();
    lex.next();
    let rhs = parse_ident(&lex.slice().to_string(), &store).replace("\"", "");

    match op {
        Token::Equality => {
            if lhs != rhs {
                store.increment_scope();
            }
        }
        _ => {}
    }

    check_data_type(lex.next(), Token::CurlyBraceOpen, &store);

    store
}

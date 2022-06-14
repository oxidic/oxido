use crate::{token::Token, util::check_data_type, store::Store};
use logos::Logos;

mod declaration;
mod assignment;
mod print;
mod expression;

pub fn parse<'a>(line: &'a str, mut store: Store<'a>) -> Store<'a> {
    let lex = Token::lexer(line);
    let token = lex.clone().next();

    if token == None {
        return store;
    }

    match token.unwrap() {
        Token::Let => store = declaration::parse_declaration(lex, store),
        Token::Print => store = print::parse(lex, store),
        _ => {
            let mut lex_clone = lex.clone();
            // TOKEN:: IDENT
            check_data_type(lex_clone.next(), Token::Ident, &store);

            let var = store.get_variable(lex_clone.slice());

            if var != None {
                store = assignment::parse_assignment(lex, store);
            } else {
                panic!("SyntaxError: unexpected token",);
            }
        }
    }

    store
}

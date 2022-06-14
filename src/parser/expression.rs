use crate::{token::Token, util::parse_ident, store::Store};
use logos::Lexer;

pub fn parse_expression<'a>(
    lex: &Lexer<Token>,
    store: Store<'a>,
) -> (String, Store<'a>) {

    if lex.clone().collect::<Vec<Token>>().contains(&Token::String) {
        let string = parse_ident(&lex.remainder().replace(";", ""), &store);
        let array = string.split("+");
        let mut r = String::new();
        for s in array {
            r += &s.trim().replace('"', "");
        }
        return (r, store)
    }

    let eval_string = parse_ident(&lex.remainder().replace(";", ""), &store);

    let r = meval::eval_str(eval_string).unwrap();

    (r.to_string(), store)
}


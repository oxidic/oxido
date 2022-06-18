use crate::{store::Store, token::Token, util::parse_ident};
use logos::Lexer;

pub fn parse_expression(lex: &mut Lexer<Token>, store: Store) -> (String, Store) {
    let puntuators = vec![Token::CurlyBraceOpen, Token::CurlyBraceClose, Token::Semicolon, Token::Bracket];

    let clone = lex
        .clone()
        .filter(|f| !puntuators.contains(f))
        .collect::<Vec<Token>>();

    if clone.len() == 1 && clone.contains(&Token::Ident) {
        lex.next();
        let r = parse_ident(&lex.slice().to_string(), &store);
        return (r, store);
    } else if clone.contains(&Token::String) {
        let string = parse_ident(
            &lex.remainder()
                .replace("(", "")
                .replace(")", "")
                .replace(";", ""),
            &store,
        );
        let array = string.split("+");
        let mut r = String::new();
        for s in array {
            r += &s.trim().replace('"', "");
        }
        return (r, store);
    } else {
        let eval_string = parse_ident(
            &lex.remainder()
                .replace("(", "")
                .replace(")", "")
                .replace(";", ""),
            &store,
        );

        let r = meval::eval_str(eval_string).unwrap();

        (r.to_string(), store)
    }
}

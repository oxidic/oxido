use crate::token::Token;
use logos::Logos;
use std::collections::HashMap;

mod declaration;
mod assignment;
mod print;
mod expression;

pub fn parse<'a>(line: &'a str, mut store: HashMap<&'a str, String>) -> HashMap<&'a str, String> {
    let lex = Token::lexer(line);

    match lex.clone().next().unwrap() {
        Token::Let => store = declaration::parse_declaration(lex, store),
        Token::Print => store = print::parse(lex, store),
        _ => {
            let mut lex_clone = lex.clone();
            // TOKEN:: IDENT
            lex_clone.next();

            if store.get(lex_clone.slice()).unwrap() != "" {
                store = assignment::parse_assignment(lex, store);
            }
        }
    }

    store
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::get_hash;

    #[test]
    fn parse_assignment() {
        let mut hash = get_hash();
        hash.insert("a", String::from("7"));
        assert_eq!(parse("let a = 7;", get_hash()), hash)
    }

    #[test]
    fn parse_expression() {
        let mut hash = get_hash();
        hash.insert("a", String::from("343"));
        assert_eq!(parse("let a = 7 ** 3;", get_hash()), hash)
    }
}

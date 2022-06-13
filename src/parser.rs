use logos::{Logos, Lexer};
use crate::{token::Token, util::is_numeric};
use std::collections::HashMap;

pub fn parse<'a>(line: &'a str, mut store: HashMap<&'a str, String>) -> HashMap<&'a str, String> {
    let lex = Token::lexer(line);

    match lex.clone().next().unwrap() {
        Token::Let => store = parse_declaration(lex, store),
        Token::Print => store = parse_print(lex, store),
        _ => {
            let mut lex_clone = lex.clone();
            // TOKEN:: IDENT
            lex_clone.next();
            if store.get(lex_clone.slice()).unwrap() != "" {
                store = parse_assignment(lex, store);
            }
        }
    }

    store
}

fn parse_declaration<'a>(
    mut lex: Lexer<'a, Token>,
    store: HashMap<&'a str, String>,
) -> HashMap<&'a str, String> {
    // TOKEN: LET
    lex.next();

    parse_assignment(lex, store)
}

fn parse_assignment<'a>(
    mut lex: Lexer<'a, Token>,
    mut store: HashMap<&'a str, String>,
) -> HashMap<&'a str, String> {
    // TOKEN: IDENT
    lex.next();
    let ident = lex.slice();

    // TOKEN: =
    lex.next();

    // TOKEN: TEXT
    let value: String;

    if lex.clone().count() > 2 {
        (value, store) = parse_expression(lex, store);
    } else {
        match lex.next().unwrap() {
            Token::Number => value = lex.slice().parse().unwrap(),
            Token::String => value = lex.slice().parse().unwrap(),
            _ => value = String::new(),
        }
    }

    store.insert(ident, value.replace('"', ""));

    store
}

fn parse_expression<'a>(
    mut lex: Lexer<Token>,
    store: HashMap<&'a str, String>,
) -> (String, HashMap<&'a str, String>) {
    let mut lhs_type = lex.next().unwrap();
    let mut lhs = lex.slice();
    let op = lex.next().unwrap();
    let rhs_type = lex.next().unwrap();
    let mut rhs = lex.slice();

    if lhs_type != rhs_type {
        if lhs_type != Token::Ident && rhs_type != Token::Ident {
            panic!(
            "TypeError: Expected types of {} and {} to be same, found types {:?} and {:?} instead!",
            lhs, rhs, lhs_type, rhs_type
        )
        } else {
            if lhs_type == Token::Ident {
                lhs = store.get(lhs).unwrap();
                if is_numeric(lhs) {
                    lhs_type = Token::Number;
                } else {
                    lhs_type = Token::String;
                }
            }
            if rhs_type == Token::Ident {
                rhs = store.get(rhs).unwrap();
            };
        }
    }

    match lhs_type {
        Token::Number => match op {
            Token::AddOperator => (
                (lhs.parse::<i128>().unwrap() + rhs.parse::<i128>().unwrap()).to_string(),
                store,
            ),
            Token::SubOperator => (
                (lhs.parse::<i128>().unwrap() - rhs.parse::<i128>().unwrap()).to_string(),
                store,
            ),
            Token::MulOperator => (
                (lhs.parse::<i128>().unwrap() * rhs.parse::<i128>().unwrap()).to_string(),
                store,
            ),
            Token::DivOperator => (
                (lhs.parse::<i128>().unwrap() / rhs.parse::<i128>().unwrap()).to_string(),
                store,
            ),
            Token::PowerOperator => (
                lhs.parse::<i128>()
                    .unwrap()
                    .pow(rhs.parse::<i128>().unwrap().try_into().unwrap())
                    .to_string(),
                store,
            ),
            _ => (String::new(), store),
        },
        Token::String => match op {
            Token::AddOperator => ((lhs.to_owned() + rhs).to_string(), store),
            _ => (String::new(), store),
        },
        _ => (String::new(), store),
    }
}

fn parse_print<'a>(
    mut lex: Lexer<'a, Token>,
    store: HashMap<&'a str, String>,
) -> HashMap<&'a str, String> {
    // TOKEN: PRINT
    lex.next();
    // TOKEN: BRACKET
    lex.next();

    lex.next();
    let idnt = lex.slice();
    let value = store.get(idnt).unwrap();

    println!("{}", value);

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

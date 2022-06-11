use std::collections::HashMap;

use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, PartialEq)]
enum Token {
    #[token("let")]
    Let,

    #[token(";")]
    Semicolon,

    #[token("+")]
    AddOperator,

    #[token("-")]
    SubOperator,

    #[token("*")]
    MulOperator,

    #[token("/")]
    DivOperator,

    #[token("^")]
    #[token("**")]
    PowerOperator,

    #[token("=")]
    Assignment,

    #[regex("[A-Za-z][A-Za-z0-9]+")]
    Ident,

    #[regex("[0-9]+")]
    Number,

    #[error]
    #[regex(" +", logos::skip)]
    Error,
}

fn main() {
    let mut store: HashMap<&str, i128> = HashMap::new();
    store = parse("let a = 5 ^ 3;", store);
    store = parse("let d = 7 ** 2;", store);

    println!("{:#?}", store);
}

fn parse<'a>(line: &'a str, mut store: HashMap<&'a str, i128>) -> HashMap<&'a str, i128> {
    let lex = Token::lexer(line);

    match lex.clone().next().unwrap() {
        Token::Let => store = parse_assignment(lex, store),
        _ => {}
    }

    store
}

fn parse_assignment<'a>(mut lex: Lexer<'a, Token>, mut store: HashMap<&'a str, i128>) -> HashMap<&'a str, i128> {
    // let operator
    lex.next();

    // i identifier
    lex.next();
    let ident = lex.slice();

    // = operator
    lex.next();

    // v value
    let value: i128;

    if lex.clone().count() > 2 {
        value = parse_expression(lex);
    } else {
        match lex.next().unwrap() {
            Token::Number => value = lex.slice().parse().unwrap(),
            _ => value = 0,
        }
    }

    store.insert(ident, value);

    store
}

fn parse_expression(mut lex: Lexer<Token>) -> i128 {
    lex.next();
    let lhs: i128 = lex.slice().parse().unwrap();
    let op = lex.next().unwrap();
    lex.next();
    let rhs: i128 = lex.slice().parse().unwrap();

    match op {
        Token::AddOperator => lhs + rhs,
        Token::SubOperator => lhs - rhs,
        Token::MulOperator => lhs * rhs,
        Token::DivOperator => lhs / rhs,
        Token::PowerOperator => lhs.pow(rhs.try_into().unwrap()),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_assignment() {
        let mut hash = HashMap::new();
        hash.insert("a", 343);
        assert_eq!(parse("let a = 7 ** 3;", HashMap::new()), hash)
    }
}
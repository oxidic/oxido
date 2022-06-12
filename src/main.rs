use logos::{Lexer, Logos};
use std::env;
use std::{collections::HashMap, fs};

mod util;

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

    #[regex(r"[\(\)]+")]
    Bracket,

    #[regex("[A-Za-z]+")]
    Ident,

    #[regex("\"[A-Za-z0-9 !]+\"")]
    String,
    #[regex("[0-9]+")]
    Number,

    #[token("print")]
    Print,

    #[error]
    #[regex(" +", logos::skip)]
    Error,
}

fn main() {
    let mut store: HashMap<&str, String> = util::get_hash();
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().filter(|line| line != &"").collect();

    for line in lines {
        store = parse(line, store);
    }

    println!("DEBUG: {:?}", store);
}

fn parse<'a>(line: &'a str, mut store: HashMap<&'a str, String>) -> HashMap<&'a str, String> {
    let mut lex = Token::lexer(line);

    match lex.clone().next().unwrap() {
        Token::Let => store = parse_assignment(lex, store),
        Token::Print => store = parse_print(lex, store),
        _ => {
            lex.next();
            let idnt = lex.slice();
            if store.get(idnt).unwrap() != "" {
                lex.next();

                lex.next();
                store.insert(idnt, lex.slice().to_string().replace('"', ""));
            }
        }
    }

    store
}

fn parse_assignment<'a>(
    mut lex: Lexer<'a, Token>,
    mut store: HashMap<&'a str, String>,
) -> HashMap<&'a str, String> {
    // TOKEN: LET
    lex.next();

    // TOKEN: IDENT
    lex.next();
    let ident = lex.slice();

    // TOKEN: =
    lex.next();

    // TOKEN: TEXT
    let value: String;

    if lex.clone().count() > 2 {
        value = parse_expression(lex);
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

fn parse_expression(mut lex: Lexer<Token>) -> String {
    let lhs_type = lex.next().unwrap();
    let lhs = lex.slice();
    let op = lex.next().unwrap();
    let rhs_type = lex.next().unwrap();
    let rhs = lex.slice();

    if lhs_type != rhs_type {
        panic!(
            "TypeError: Expected types of {} and {} to be same, found types {:?} and {:?} instead!",
            lhs, rhs, lhs_type, rhs_type
        )
    }
    match lhs_type {
        Token::Number => match op {
            Token::AddOperator => {
                (lhs.parse::<i128>().unwrap() + rhs.parse::<i128>().unwrap()).to_string()
            }
            Token::SubOperator => {
                (lhs.parse::<i128>().unwrap() - rhs.parse::<i128>().unwrap()).to_string()
            }
            Token::MulOperator => {
                (lhs.parse::<i128>().unwrap() * rhs.parse::<i128>().unwrap()).to_string()
            }
            Token::DivOperator => {
                (lhs.parse::<i128>().unwrap() / rhs.parse::<i128>().unwrap()).to_string()
            }
            Token::PowerOperator => lhs
                .parse::<i128>()
                .unwrap()
                .pow(rhs.parse::<i128>().unwrap().try_into().unwrap())
                .to_string(),
            _ => String::new(),
        },
        Token::String => match op {
            Token::AddOperator => (lhs.to_owned() + rhs).to_string(),
            _ => String::new(),
        },
        _ => String::new(),
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

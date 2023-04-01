use std::error;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    If,
    Then,
    Loop,
    Fn,
    Semicolon,
    Comma,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
    Equal,
    IsEqual,
    IsNotEqual,
    IsGreater,
    IsLesser,
    IsGreaterEqual,
    IsLesserEqual,
    Identifier(String),
    FunctionName(String),
    FunctionParameter(String, String),
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    LParen,
    RParen,
    LCurly,
    RCurly,
    Get,
    Exit,
    Break,
    Return,
    Error,
}

impl Token {
    pub fn is_keyword(ident: &Vec<char>) -> Result<Token, Box<dyn error::Error>> {
        Ok(match &ident.iter().collect::<String>() as &str {
            "let" => Token::Let,
            "if" => Token::If,
            "then" => Token::Then,
            "loop" => Token::Loop,
            "fn" => Token::Fn,
            "exit" => Token::Exit,
            "break" => Token::Break,
            "return" => Token::Return,
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

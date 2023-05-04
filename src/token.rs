use std::fmt::Debug;

use crate::data::DataType;

pub type Tokens = Vec<(Token, usize)>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Let,
    If,
    Loop,
    Fn,

    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
    Not,
    Equal,
    IsEqual,
    IsNotEqual,
    IsGreater,
    IsLesser,
    IsGreaterEqual,
    IsLesserEqual,

    Identifier(String),
    FunctionName(String),
    DataType(DataType),

    Str(String),
    Int(i64),
    Bool(bool),

    Semicolon,
    Comma,
    LParen,
    RParen,
    LCurly,
    RCurly,

    Exit,
    Break,
    Return,
}

impl Token {
    pub fn as_string(&self) -> String {
        match &self {
            Token::Let => String::from("let"),
            Token::If => String::from("if"),
            Token::Loop => String::from("loop"),
            Token::Fn => String::from("fn"),

            Token::Addition => String::from("+"),
            Token::Subtraction => String::from("-"),
            Token::Multiplication => String::from("*"),
            Token::Division => String::from("/"),
            Token::Power => String::from("^"),
            Token::Not => String::from("!"),
            Token::Equal => String::from("="),
            Token::IsEqual => String::from("=="),
            Token::IsNotEqual => String::from("!="),
            Token::IsGreater => String::from(">"),
            Token::IsLesser => String::from("<"),
            Token::IsGreaterEqual => String::from(">="),
            Token::IsLesserEqual => String::from("<="),

            Token::Identifier(ident) => ident.to_string(),
            Token::FunctionName(fname) => fname.to_string(),
            Token::DataType(datatype) => {
                match datatype {
                    DataType::Str => String::from("Str"),
                    DataType::Int => String::from("Int"),
                    DataType::Bool => String::from("Bool"),
                }
            },

            Token::Str(string) => string.to_string(),
            Token::Int(i) => format!("{i}"),
            Token::Bool(b) => format!("{b}"),

            Token::Semicolon => String::from(";"),
            Token::Comma => String::from(","),
            Token::LParen => String::from("("),
            Token::RParen => String::from(")"),
            Token::LCurly => String::from("{"),
            Token::RCurly => String::from("}"),

            Token::Exit => String::from("exit"),
            Token::Break => String::from("break"),
            Token::Return => String::from("return"),
        }
    }

    pub fn len(&self) -> usize {
        self.as_string().len()
    }
}

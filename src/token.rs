use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,

    If,

    Then,

    Loop,

    Function,

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

    FunctionCall(String),

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

    Comment,

    Error(String, String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}
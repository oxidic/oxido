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

    Call(String),

    FunctionSignature(String, Vec<String>),

    String(String),

    Integer(i64),

    Bool(bool),

    LParen,

    RParen,

    LCurly,

    RCurly,

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

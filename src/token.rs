use std::fmt::{Debug, Display, Error, Formatter};

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

    String(String),
    Integer(i64),
    Bool(bool),

    Semicolon,
    Comma,
    LParen,
    RParen,
    LCurly,
    RCurly,
    
    Exit,
    Break,
    Return
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

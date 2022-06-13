use logos::{Logos};
use std::fmt::{Display, Formatter, Error};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#?}", self)
    }
}
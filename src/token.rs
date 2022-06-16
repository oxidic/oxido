use logos::{Logos};
use std::fmt::{Display, Formatter, Error};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
    #[token("let")]
    Let,

    #[token("if")]
    If,

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
    #[token("==")]
    Equality,
    
    #[regex(r"[\(\)]+")]
    Bracket,

    #[token("{")]
    CurlyBraceOpen,

    #[token("}")]
    CurlyBraceClose,

    #[regex("[A-Za-z]+")]
    Ident,

    #[regex("\"[A-Za-z0-9 !]+\"")]
    String,
    #[regex("[0-9]+")]
    Integer,

    #[token("print")]
    Print,

    #[regex(" +", logos::skip)]
    NewLine,

    #[error]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#?}", self)
    }
}
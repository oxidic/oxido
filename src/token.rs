use logos::Logos;
use std::fmt::{Display, Error, Formatter};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
    /// Keyword: let
    #[token("let")]
    Let,
    /// Keyword: if
    #[token("if")]
    If,
    /// Keyword: loop
    #[token("loop")]
    Loop,
    /// Keyword: fn
    #[token("fn")]
    Function,

    /// Keyword: ;
    #[token(";", logos::skip)]
    Semicolon,
    /// Keyword: ,
    #[token(",")]
    Comma,

    /// Keyword: +
    #[token("+")]
    Addition,
    /// Keyword: -
    #[token("-")]
    Subtraction,
    /// Keyword: *
    #[token("*")]
    Multiplication,
    /// Keyword: /
    #[token("/")]
    Division,
    /// Keyword: ^ OR **
    #[token("^")]
    #[token("**")]
    Power,
    /// Keyword: =
    #[token("=")]
    Equal,
    /// Keyword: ==
    #[token("==")]
    IsEqual,
    /// Keyword: !=
    #[token("!=")]
    IsNotEqual,
    /// Keyword: >
    #[token(">")]
    IsGreater,
    /// Keyword: <
    #[token("<")]
    IsLesser,
    /// Keyword: >=
    #[token(">=")]
    IsGreaterEqual,
    /// Keyword: <=
    #[token("<=")]
    IsLesserEqual,

    #[regex("[A-Za-z]+")]
    Ident,
    #[regex("\"[A-Za-z0-9 !]+\"")]
    String,
    #[regex("[0-9]+")]
    Integer,

    /// Keyword: true|false
    #[regex("true|false")]
    Bool,

    /// Keyword: (
    #[token("(")]
    LParen,
    /// Keyword: )
    #[token(")")]
    RParen,
    /// Keyword: {
    #[token("{")]
    LCurly,
    /// Keyword: }
    #[token("}")]
    RCurly,

    /// Keyword: print
    #[token("print")]
    Print,
    /// Keyword: println
    #[token("println")]
    Println,
    /// Keyword: exit
    #[token("exit")]
    Exit,
    /// Keyword: break
    #[token("break")]
    Break,

    #[regex(r"/\*([^*]|\*[^/])+\*/", logos::skip)]
    Comment,
    #[regex(" +", logos::skip)]
    NewLine,
    #[error]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

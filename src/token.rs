use logos::Logos;
use std::fmt::{Display, Error, Formatter};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("loop")]
    Loop,
    #[token("fn")]
    Function,

    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,

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
    #[token("!=")]
    NotEquality,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LesserThan,

    #[token("(")]
    ParenthesisOpen,
    #[token(")")]
    ParenthesisClose,
    #[token("{")]
    CurlyBraceOpen,
    #[token("}")]
    CurlyBraceClose,
    #[token("[")]
    SquareBraceOpen,
    #[token("]")]
    SquareBraceClose,

    #[regex("[A-Za-z]+")]
    Ident,
    #[regex("\"[A-Za-z0-9 !]+\"")]
    String,
    #[regex("[0-9]+")]
    Integer,
    #[regex("true|false")]
    Bool,

    #[token("print")]
    Print,
    #[token("exit")]
    Exit,
    #[token("break")]
    Break,

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

use std::fmt::Debug;

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
    FunctionCall(String),

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
    Return,
}

impl Token {
    pub fn to_string(&self) -> String {
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
            Token::FunctionCall(fcall) => fcall.to_string(),

            Token::String(string) => string.to_string(),
            Token::Integer(i) => format!("{i}"),
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

    pub fn size(&self) -> usize {
        self.to_string().len()
    }
}

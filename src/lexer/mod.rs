use crate::token::Token;
use logos::{Lexer, Logos};

pub fn lexer(contents: &str) -> Lexer<Token> {
    Token::lexer(contents)
}

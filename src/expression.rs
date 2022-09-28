use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    BinaryOperation(Box<Expression>, Token, Box<Expression>),
    Integer(i64),
    Identifier(String),
    Bool(bool),
    String(String),
    Placeholder,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Integer(i64),
    Bool(bool),
    String(String),
    Placeholder,
}
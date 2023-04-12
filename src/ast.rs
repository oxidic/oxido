use crate::{token::Token};

#[derive(Clone, Debug)]
pub enum AstNode {
    Assignment(String, Expression),
    If(Expression, Vec<AstNode>),
    Loop(Vec<AstNode>),
    FunctionCall(String, Vec<Expression>),
    Break,
    Return,
    Exit
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryOperation(Box<Expression>, Token, Box<Expression>),
    String(String),
    Integer(i64),
    Bool(bool),
    Identifier(String),
    Unexpected
}

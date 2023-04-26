use std::ops::Range;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum AstNode {
    Assignment(String, Expression),
    ReAssignment(String, Expression),
    If(Expression, Vec<(AstNode, Range<usize>)>),
    Loop(Vec<(AstNode, Range<usize>)>),
    FunctionCall(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<String>, Vec<(AstNode, Range<usize>)>),
    Break,
    Return(Expression),
    Exit
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryOperation(Box<Expression>, Token, Box<Expression>),
    Str(String),
    Integer(i64),
    Bool(bool),
    FunctionCall(String, Vec<Expression>),
    Identifier(String)
}

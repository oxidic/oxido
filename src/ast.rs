use std::ops::Range;
use crate::{token::Token, data::{DataType, Param}};

pub type Ast = Vec<(AstNode, Range<usize>)>;

#[derive(Clone, Debug)]
pub enum AstNode {
    Assignment(String, DataType, Expression),
    ReAssignment(String, DataType, Expression),
    If(Expression, Ast),
    Loop(Ast),
    FunctionCall(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<Param>, Ast),
    Break,
    Return(Expression),
    Exit(Expression),
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryOperation(Box<Expression>, Token, Box<Expression>),
    Str(String),
    Int(i64),
    Bool(bool),
    FunctionCall(String, Vec<Expression>),
    Identifier(String)
}

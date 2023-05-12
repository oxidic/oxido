use std::ops::Range;
use crate::{token::Token, data::{DataType, Param}};

pub type Ast = Vec<(AstNode, Range<usize>)>;

#[derive(Clone, Debug)]
pub enum AstNode {
    Assignment(String, Option<DataType>, Expression),
    ReAssignment(String, Expression),
    VecReAssignment(String, Expression, Expression),
    If(Expression, Ast),
    IfElse(Expression, Ast, Ast),
    Loop(Ast),
    FunctionCall(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<Param>, Option<DataType>, Ast),
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
    Identifier(String),
    Vector(Vec<Expression>, Option<DataType>),
    VecIndex(String, Box<Expression>),
}

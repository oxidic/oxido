use crate::expression::Expression;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Placeholder,
    Declaration(String, Expression),
    Redeclaration(String, Expression),
    If(Expression, Vec<AstNode>),
    Loop(Vec<AstNode>),
    Break,
    Return(Expression),
    FunctionDeclaration(String, Vec<String>, Vec<AstNode>),
    Call(String, Vec<Expression>),
}

impl Display for AstNode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

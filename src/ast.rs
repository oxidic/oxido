use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Placeholder,
    Declaration(String, Expression),
    Redeclaration(String, Expression),
    If(Expression, Vec<Ast>),
    Loop(Vec<Ast>),
    Break,
    Return(Expression),
    Function(String, Vec<String>, Vec<Ast>),
}

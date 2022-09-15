use crate::{expression::Expression};


#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Placeholder,
    Declaration(String, Expression),
    Redeclaration(String, Expression),
    If(Expression, Vec<Ast>),
}
use std::ops::Range;

use crate::ast::AstNode;

#[derive(Clone, Debug)]
pub enum Data {
    Str(String),
    Integer(i64),
    Bool(bool),
}

impl Data {
    pub fn type_as_str(&self) -> &str {
        match self {
            Data::Str(_) => "Str",
            Data::Integer(_) => "Integer",
            Data::Bool(_) => "Bool",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub statements: Vec<(AstNode, Range<usize>)>,
}

impl Function {
    pub fn new(name: String, params: Vec<String>, statements: Vec<(AstNode, Range<usize>)>) -> Self {
        Self {
            name,
            params,
            statements,
        }
    }
}

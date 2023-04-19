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
            Data::Str(_) => "String",
            Data::Integer(_) => "int",
            Data::Bool(_) => "bool",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub statements: Vec<(AstNode, usize)>,
}

impl Function {
    pub fn init(name: String, params: Vec<String>, statements: Vec<(AstNode, usize)>) -> Self {
        Self {
            name,
            params,
            statements,
        }
    }
}

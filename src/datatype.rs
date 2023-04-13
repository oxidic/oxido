use crate::ast::AstNode;

#[derive(Clone, Debug)]
pub enum Data {
    String(String),
    Integer(i64),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub statements: Vec<AstNode>,
}

impl Function {
    pub fn init(name: String, params: Vec<String>, statements: Vec<AstNode>) -> Self {
        Self {
            name,
            params,
            statements,
        }
    }
}

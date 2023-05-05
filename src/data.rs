use std::fmt::{self, Display};

use crate::ast::Ast;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Data {
    Str(String),
    Int(i64),
    Bool(bool),
}

impl Data {
    pub fn to_string(&self) -> &str {
        match self {
            Data::Str(_) => "str",
            Data::Int(_) => "int",
            Data::Bool(_) => "bool",
        }
    }

    pub fn r#type(&self) -> DataType {
        match self {
            Data::Str(_) => DataType::Str,
            Data::Int(_) => DataType::Int,
            Data::Bool(_) => DataType::Bool,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataType {
    Str,
    Int,
    Bool,
}

impl Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                DataType::Str => "str",
                DataType::Int => "int",
                DataType::Bool => "bool",
            })
        )
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub datatype: DataType,
    pub data: Data,
}

impl Variable {
    pub fn new(datatype: DataType, data: Data) -> Self {
        Self { datatype, data }
    }
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub datatype: DataType,
}

impl Param {
    pub fn new(name: String, datatype: DataType) -> Self {
        Self { name, datatype }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub datatype: Option<DataType>,
    pub statements: Ast,
}

impl Function {
    pub fn new(name: String, params: Vec<Param>, datatype: Option<DataType>, statements: Ast) -> Self {
        Self {
            name,
            params,
            datatype,
            statements,
        }
    }
}

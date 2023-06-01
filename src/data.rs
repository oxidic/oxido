use crate::ast::Ast;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Data {
    Str(String),
    Int(i64),
    Bool(bool),
    Vector(Vec<Data>, DataType),
}

impl Data {
    pub fn r#type(&self) -> DataType {
        match self {
            Data::Str(_) => DataType::Str,
            Data::Int(_) => DataType::Int,
            Data::Bool(_) => DataType::Bool,
            Data::Vector(_, t) => DataType::Vector(Box::new(t.clone())),
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.r#type())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataType {
    Vector(Box<DataType>),
    Str,
    Int,
    Bool,
}

impl Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn match_type(d: &DataType) -> String {
            match d {
                DataType::Str => String::from("str"),
                DataType::Int => String::from("int"),
                DataType::Bool => String::from("bool"),
                DataType::Vector(t) => "vec<".to_owned() + &match_type(t) + ">",
            }
        }
        write!(
            f,
            "{}",
            match_type(self)
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
    pub fn new(
        name: String,
        params: Vec<Param>,
        datatype: Option<DataType>,
        statements: Ast,
    ) -> Self {
        Self {
            name,
            params,
            datatype,
            statements,
        }
    }
}

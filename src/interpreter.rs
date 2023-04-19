use std::{collections::HashMap, process};

use crate::{
    ast::{AstNode, Expression},
    datatype::{Data, Function},
    error::error,
    globals::Globals,
    token::Token,
};

pub struct Interpreter {
    name: String,
    file: String,
    ast: Vec<(AstNode, usize)>,
    stop: bool,
    variables: HashMap<String, Data>,
    functions: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new(ast: Vec<(AstNode, usize)>, file: String, name: String) -> Self {
        Self {
            name,
            file,
            ast,
            stop: false,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut stream = self.ast.clone().into_iter().peekable();
        loop {
            if stream.peek().is_none() {
                break;
            }

            self.match_node(stream.next().unwrap());
        }
    }

    pub fn match_node(&mut self, node: (AstNode, usize)) {
        if self.stop {
            return;
        }
        match node.0 {
            AstNode::Assignment(ident, expression) => {
                self.variables
                    .insert(ident, self.parse_expression(expression, node.1));
            }
            AstNode::If(condition, statements) => {
                let data = self.parse_expression(condition, node.1);

                if let Data::Bool(bool) = data {
                    if bool {
                        let mut stream = statements.into_iter().peekable();
                        loop {
                            if stream.peek().is_none() {
                                break;
                            }

                            self.match_node(stream.next().unwrap());
                        }
                    }
                } else {
                    println!("{}", node.1);
                    error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        node.1..node.1,
                    );
                }
            }
            AstNode::Loop(statements) => {
                let mut stream = statements.clone().into_iter().peekable();
                loop {
                    if stream.peek().is_none() {
                        stream = statements.clone().into_iter().peekable();
                    }

                    if self.stop {
                        self.stop = false;
                        break;
                    }

                    self.match_node(stream.next().unwrap());
                }
            }
            AstNode::FunctionCall(name, params) => {
                let mut args = vec![];

                for param in params {
                    args.push(self.parse_expression(param, node.1))
                }

                if Globals::_has(&name) {
                    Globals::call(&name, args)
                } else if self.functions.contains_key(&name) {
                    let function = self.functions.get(&name).unwrap().to_owned();

                    if args.len() != function.params.len() {
                        error(
                            &self.name,
                            &self.file,
                            "0002",
                            "not enough arguments were passed",
                            &format!(
                                "{} arguments were expected but {} were passed",
                                function.params.len(),
                                args.len()
                            ),
                            node.1..node.1,
                        );
                    }

                    for (i, arg) in args.iter().enumerate() {
                        let param = function.params.get(i).unwrap();

                        self.variables.insert(param.to_string(), arg.to_owned());
                    }

                    let mut stream = function.statements.into_iter().peekable();
                    loop {
                        if stream.peek().is_none() {
                            break;
                        }

                        self.match_node(stream.next().unwrap());
                    }
                }
            }
            AstNode::FunctionDeclaration(name, params, statements) => {
                self.functions
                    .insert(name.clone(), Function::init(name, params, statements));
            }
            AstNode::Break => {
                self.stop = true;
            }
            AstNode::Return => todo!(),
            AstNode::Exit => process::exit(0),
        }
    }

    pub fn parse_expression(&self, expr: Expression, pos: usize) -> Data {
        match expr {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().to_owned(),
            Expression::Bool(b) => Data::Bool(b),
            Expression::String(s) => Data::String(s),
        }
    }

    pub fn parse_binary_operation(&self, lhs: Expression, op: Token, rhs: Expression, pos: usize) -> Data {
        let lhs = match lhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().to_owned(),
            Expression::String(s) => Data::String(s),
            Expression::Bool(b) => Data::Bool(b),
        };
        let operator = op;
        let rhs = match rhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().to_owned(),
            Expression::String(s) => Data::String(s),
            Expression::Bool(b) => Data::Bool(b),
        };
        match operator {
            Token::Addition => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::String(str + &s),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n + m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    &self.name,
                    &self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `String` or `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `String` or `int` was expected",
                    pos..pos,
                ),
            },
            Token::Subtraction => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n - m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    &self.name,
                    &self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::Multiplication => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n * m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    &self.name,
                    &self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::Division => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n / m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    &self.name,
                    &self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::Power => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n.pow(m as u32)),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    &self.name,
                    &self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::IsEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str == s),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n == m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b == d),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsNotEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str != s),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n != m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b != d),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsGreater => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str > s),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n > m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b & !d),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsLesser => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str < s),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n < m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(!b & d),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsGreaterEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str >= s),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n >= m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b >= d),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsLesserEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str <= s),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n <= m),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b <= d),
                    data => error(
                        &self.name,
                        &self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            _ => unreachable!(),
        }
    }
}

use std::{collections::HashMap, process};

use crate::{
    ast::{AstNode, Expression},
    datatype::Data,
    token::Token,
};

pub struct Interpreter {
    ast: Vec<AstNode>,
    stop: bool,
    variables: HashMap<String, Data>,
}

impl Interpreter {
    pub fn new(ast: Vec<AstNode>) -> Self {
        Self {
            ast,
            stop: false,
            variables: HashMap::new(),
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

    pub fn match_node(&mut self, node: AstNode) {
        match node {
            AstNode::Assignment(ident, expression) => {
                self.variables
                    .insert(ident, self.parse_expression(expression));
            }
            AstNode::If(condition, statements) => {
                let data = self.parse_expression(condition);

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
                    panic!("expected bool data type")
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
            AstNode::Break => {
                self.stop = true;
            },
            AstNode::Return => todo!(),
            AstNode::Exit => process::exit(0),
        }
    }

    pub fn parse_expression(&self, expr: Expression) -> Data {
        match expr {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().to_owned(),
            Expression::Bool(b) => Data::Bool(b),
            Expression::String(s) => Data::String(s),
            Expression::Unexpected => unimplemented!(),
        }
    }

    pub fn parse_binary_operation(&self, lhs: Expression, op: Token, rhs: Expression) -> Data {
        let lhs = match lhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().to_owned(),
            Expression::String(s) => Data::String(s),
            Expression::Bool(b) => Data::Bool(b),
            Expression::Unexpected => unimplemented!(),
        };
        let operator = op;
        let rhs = match rhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().to_owned(),
            Expression::String(s) => Data::String(s),
            Expression::Bool(b) => Data::Bool(b),
            Expression::Unexpected => unimplemented!(),
        };
        match operator {
            Token::Addition => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::String(str + &s),
                    _ => unimplemented!(),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n + m),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            Token::Subtraction => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n - m),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            Token::Multiplication => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n * m),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            Token::Division => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n / m),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            Token::Power => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n.pow(m as u32)),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            Token::IsEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str == s),
                    _ => unimplemented!(),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n == m),
                    _ => unimplemented!(),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b == d),
                    _ => unimplemented!(),
                },
            },
            Token::IsNotEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str != s),
                    _ => unimplemented!(),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n != m),
                    _ => unimplemented!(),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b != d),
                    _ => unimplemented!(),
                },
            },
            Token::IsGreater => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str > s),
                    _ => unimplemented!(),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n > m),
                    _ => unimplemented!(),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b & !d),
                    _ => unimplemented!(),
                },
            },
            Token::IsLesser => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str < s),
                    _ => unimplemented!(),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n < m),
                    _ => unimplemented!(),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(!b & d),
                    _ => unimplemented!(),
                },
            },
            Token::IsGreaterEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str >= s),
                    _ => unimplemented!(),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n >= m),
                    _ => unimplemented!(),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b >= d),
                    _ => unimplemented!(),
                },
            },
            Token::IsLesserEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str <= s),
                    _ => unimplemented!(),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n <= m),
                    _ => unimplemented!(),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b <= d),
                    _ => unimplemented!(),
                },
            },
            _ => unimplemented!(),
        }
    }
}

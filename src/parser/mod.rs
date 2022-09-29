pub mod ast;
pub mod lexer;

use std::collections::HashMap;

use crate::{
    ast::AstNode,
    expression::{Data, Expression},
    globals::Globals,
    parser::ast::Ast,
    token::Token,
};

pub struct Parser {
    pub ast: Vec<AstNode>,
    pub variables: HashMap<String, Data>,
    pub functions: HashMap<String, (Vec<String>, Vec<AstNode>)>,
    pub scope_variables: HashMap<String, Data>,
    r#break: bool,
}

impl Parser {
    pub fn new(raw: String) -> Self {
        let mut ast = Ast::new(raw);
        Self {
            ast: ast.tree(),
            variables: HashMap::new(),
            scope_variables: HashMap::new(),
            functions: HashMap::new(),
            r#break: false,
        }
    }

    pub fn match_ast(&mut self, ast: AstNode) {
        match ast {
            AstNode::Declaration(name, value) => {
                self.variables.insert(name, self.parse_expression(value));
            }
            AstNode::Redeclaration(name, value) => {
                self.variables.insert(name, self.parse_expression(value));
            }
            AstNode::If(expr, lines) => {
                let value = self.parse_expression(expr);

                if let Data::Bool(b) = value {
                    if b {
                        for line in lines {
                            self.match_ast(line);
                        }
                    }
                }
            }
            AstNode::Loop(lines) => loop {
                for line in lines.clone() {
                    self.match_ast(line);
                }
                if self.r#break {
                    self.r#break = false;
                    break;
                }
            },
            AstNode::Break => self.r#break = true,
            AstNode::Return(_) => todo!(),
            AstNode::FunctionDeclaration(name, args, code) => {
                self.functions.insert(name, (args, code));
            }
            AstNode::Call(name, args) => {
                println!("{name}");
                if self.functions.get(&name).is_some() {
                    let (parameters, lines) = self.functions.get(&name).unwrap();
                    let mut arguments = HashMap::new();
                    for (i, parameter) in parameters.iter().enumerate() {
                        arguments.insert(parameter.to_string(), args.get(i).unwrap().clone());
                    }
                    println!("{arguments:?}");
                } else {
                    match name.as_str() {
                        "print" => {
                            Globals::print(self.parse_expression(args.get(0).unwrap().clone()))
                        }
                        "println" => {
                            Globals::println(self.parse_expression(args.get(0).unwrap().clone()))
                        }
                        _ => {
                            panic!("function {name} not found!");
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn run(&mut self) {
        for ast in self.ast.clone() {
            println!("{ast}");
            self.match_ast(ast);
        }
    }

    pub fn parse_expression(&self, expr: Expression) -> Data {
        match expr {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().clone(),
            Expression::Bool(b) => Data::Bool(b),
            Expression::String(s) => Data::String(s),
            Expression::Placeholder => todo!(),
        }
    }

    pub fn parse_binary_operation(&self, lhs: Expression, op: Token, rhs: Expression) -> Data {
        let lhs = match lhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().clone(),
            Expression::String(s) => Data::String(s),
            Expression::Bool(b) => Data::Bool(b),
            Expression::Placeholder => Data::Placeholder,
        };
        let operator = op;
        let rhs = match rhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&i).unwrap().clone(),
            Expression::String(s) => Data::String(s),
            Expression::Bool(b) => Data::Bool(b),
            Expression::Placeholder => Data::Placeholder,
        };
        match operator {
            Token::Addition => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::String(str + &s),
                    _ => Data::Placeholder,
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n + m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Subtraction => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n - m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Multiplication => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n * m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Division => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n / m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Power => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n.pow(m as u32)),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str == s),
                    _ => Data::Placeholder,
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n == m),
                    _ => Data::Placeholder,
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b == d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsNotEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str != s),
                    _ => Data::Placeholder,
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n != m),
                    _ => Data::Placeholder,
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b != d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsGreater => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str > s),
                    _ => Data::Placeholder,
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n > m),
                    _ => Data::Placeholder,
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b & !d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsLesser => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str < s),
                    _ => Data::Placeholder,
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n < m),
                    _ => Data::Placeholder,
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(!b & d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsGreaterEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str >= s),
                    _ => Data::Placeholder,
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n >= m),
                    _ => Data::Placeholder,
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b >= d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsLesserEqual => match lhs {
                Data::String(str) => match rhs {
                    Data::String(s) => Data::Bool(str <= s),
                    _ => Data::Placeholder,
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n <= m),
                    _ => Data::Placeholder,
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b <= d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            _ => Data::Placeholder,
        }
    }
}

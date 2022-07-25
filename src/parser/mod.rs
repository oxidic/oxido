use crate::{
    errors::{
        error_keys::{syntax_error_message, SYNTAX_ERROR_CODE},
        Error,
    },
    lexer::{self},
    parser::expression::{Boolean, Expression, Text},
    token::Token,
};
use logos::Lexer;
use std::collections::HashMap;

use self::{
    expression::{BinaryOperation, Identifier, Number},
    variable::Data,
};

mod expression;
mod variable;

#[derive(Clone, Debug)]
pub struct Parser {
    pub file: String,
    pub lines: Vec<String>,
    pub variables: HashMap<String, Data>,
}

impl Parser {
    pub fn new(file: String, lines: Vec<String>) -> Self {
        Self {
            file,
            lines,
            variables: HashMap::new(),
        }
    }

    pub fn file_name(&self) -> String {
        self.file.clone()
    }

    pub fn parse(&mut self) {
        for line in self.lines.clone() {
            let lexer = lexer::lexer(&line);
            let token = lexer.clone().next();

            match token {
                Some(Token::Let) => self.parse_declaration(lexer),
                None | Some(_) => {
                    Error::throw(
                        &self.file_name(),
                        &line,
                        SYNTAX_ERROR_CODE,
                        &syntax_error_message("Let"),
                        true,
                    );
                }
            }
        }
    }

    pub fn parse_declaration(&mut self, mut lexer: Lexer<Token>) {
        lexer.next();
        lexer.next();

        let identifier = lexer.slice().to_string();

        lexer.next();

        let value = self.parse_expression(lexer);

        self.variables.insert(identifier, value);

    }

    pub fn parse_binary_operation(&self, op: BinaryOperation) -> Data {
        let lhs = match *op.lhs {
            Expression::BinaryOperation(op) => self.parse_binary_operation(op),
            Expression::Number(n) => Data::Number(n.value),
            Expression::Identifier(var) => self.variables.get(&var.name).unwrap().clone(),
            Expression::Text(str) => Data::Text(str.value),
            _ => Data::Number(0), // placeholder
        };
        let operator = op.operator;
        let rhs = match *op.rhs {
            Expression::BinaryOperation(op) => self.parse_binary_operation(op),
            Expression::Number(n) => Data::Number(n.value),
            Expression::Identifier(var) => self.variables.get(&var.name).unwrap().clone(),
            Expression::Text(str) => Data::Text(str.value),
            _ => Data::Placeholder,
        };

        match operator {
            Token::Addition => match lhs {
                Data::Text(str) => match rhs {
                    Data::Text(s) => Data::Text(str + &s),
                    _ => Data::Placeholder,
                },
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Number(n + m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Subtraction => match lhs {
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Number(n - m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Multiplication => match lhs {
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Number(n * m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Division => match lhs {
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Number(n / m),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::Power => match lhs {
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Number(n.pow(m as u32)),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            _ => Data::Placeholder,
        }
    }

    pub fn parse_expression(&mut self, lexer: Lexer<Token>) -> Data {
        let (expr, _) = self.pratt_parser(lexer, 0);

        let result = match expr {
            Expression::BinaryOperation(op) => self.parse_binary_operation(op),
            Expression::Number(n) => Data::Number(n.value),
            Expression::Identifier(ident) => self.variables.get(&ident.name).unwrap().clone(),
            Expression::Boolean(b) => Data::Boolean(b.value),
            Expression::Text(str) => Data::Text(str.value),
        };

        result
    }

    pub fn pratt_parser<'a>(
        &mut self,
        mut lexer: Lexer<'a, Token>,
        prec: u16,
    ) -> (Expression, Lexer<'a, Token>) {
        let token = lexer.next().unwrap();
        let mut expr: Expression;

        match token {
            Token::Ident => {
                expr = Expression::Identifier(Identifier {
                    name: lexer.slice().to_string(),
                });
            }
            Token::Bool => {
                expr = Expression::Boolean(Boolean {
                    value: lexer.slice().parse::<bool>().unwrap(),
                })
            }
            Token::String => {
                expr = Expression::Text(Text {
                    value: lexer.slice().to_string(),
                })
            }
            Token::LParen => {
                lexer.next();
                (expr, lexer) = self.pratt_parser(lexer, 0);
            }
            Token::Subtraction => {
                lexer.next();
                expr = Expression::Number(Number {
                    value: -lexer.slice().parse::<i128>().unwrap(),
                })
            }
            _ => {
                expr = Expression::Number(Number {
                    value: lexer.slice().parse::<i128>().unwrap(),
                });
            }
        };

        loop {
            let token = lexer.next();

            if token == None {
                break;
            }

            let op = token.unwrap();

            if op == Token::Power && self.infix_binding_power(op) < prec {
                break;
            }
            if op != Token::Power && self.infix_binding_power(op) <= prec {
                break;
            }

            let rhs;
            (rhs, lexer) = self.pratt_parser(lexer, self.infix_binding_power(op));
            expr = Expression::BinaryOperation(BinaryOperation {
                lhs: Box::new(expr),
                operator: op,
                rhs: Box::new(rhs),
            })
        }

        (expr, lexer)
    }

    pub fn infix_binding_power(&self, op: Token) -> u16 {
        match op {
            Token::Addition => 1,
            Token::Subtraction => 2,
            Token::Multiplication => 3,
            Token::Division => 4,
            _ => 5,
        }
    }
}

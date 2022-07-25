use crate::{
    errors::{error_keys::check_syntax, Error},
    lexer::lexer,
    parser::{
        expression::{BinaryOperation, Boolean, Expression, Identifier, Number, Text},
        variable::Data,
    },
    token::Token,
};
use logos::Lexer;
use std::collections::HashMap;

mod expression;
mod variable;

#[derive(Clone, Debug)]
pub struct Parser {
    pub file: String,
    pub lines: Vec<String>,
    pub line: String,
    pub variables: HashMap<String, Data>,
    pub stacks: Vec<String>,
    pub can_run: i64,
}

impl Parser {
    pub fn new(file: String, lines: Vec<String>) -> Self {
        Self {
            file,
            lines,
            line: String::new(),
            variables: HashMap::new(),
            stacks: vec![],
            can_run: 0,
        }
    }

    pub fn file(&self) -> String {
        self.file.clone()
    }

    pub fn parse(&mut self) {
        for line in self.lines.clone() {
            self.line = line.clone();
            let lexer = lexer(&line);
            let token = lexer.clone().next();

            match token {
                Some(Token::RCurly) => {
                    self.stacks.pop();
                    self.can_run += 1;
                }
                Some(t) => {
                    if self.can_run < 0 {
                        continue;
                    };
                    match t {
                        Token::Let => self.parse_declaration(lexer),
                        Token::Print => self.parse_print(lexer),
                        Token::If => self.parse_if(lexer),
                        _ => check_syntax(&self.file(), &line, Token::Let, token.unwrap()),
                    }
                }
                None => {}
            }
        }
    }

    pub fn parse_if(&mut self, mut lexer: Lexer<Token>) {
        check_syntax(&self.file(), &self.line, Token::If, lexer.next().unwrap());
        match self.parse_expression(lexer) {
            Data::Boolean(run) => {
                self.stacks.push(String::from("If"));
                if !run {
                    self.can_run -= 1;
                }
            }
            t => Error::throw(&self.file(), &self.line, 3, &format!("Unexpected data type {t:?}"), true),
        }
    }

    pub fn parse_print(&mut self, mut lexer: Lexer<Token>) {
        check_syntax(
            &self.file(),
            &self.line,
            Token::Print,
            lexer.next().unwrap(),
        );
        match self.parse_expression(lexer) {
            Data::Text(str) => println!("{str}"),
            Data::Number(n) => println!("{n}"),
            Data::Boolean(b) => println!("{b}"),
            t => Error::throw(
                &self.file(),
                &self.line,
                3,
                &format!("Unexpected data type {t:?}"),
                true,
            ),
        }
    }

    pub fn parse_declaration(&mut self, mut lexer: Lexer<Token>) {
        check_syntax(&self.file(), &self.line, Token::Let, lexer.next().unwrap());
        check_syntax(
            &self.file(),
            &self.line,
            Token::Ident,
            lexer.next().unwrap(),
        );

        let identifier = lexer.slice().to_string();

        check_syntax(
            &self.file(),
            &self.line,
            Token::Equal,
            lexer.next().unwrap(),
        );

        let value = self.parse_expression(lexer);

        self.variables.insert(identifier, value);
    }

    pub fn parse_binary_operation(&self, op: BinaryOperation) -> Data {
        let lhs = match *op.lhs {
            Expression::BinaryOperation(op) => self.parse_binary_operation(op),
            Expression::Number(n) => Data::Number(n.value),
            Expression::Identifier(var) => self.variables.get(&var.name).unwrap().clone(),
            Expression::Text(str) => Data::Text(str.value),
            Expression::Boolean(b) => Data::Boolean(b.value),
        };
        let operator = op.operator;
        let rhs = match *op.rhs {
            Expression::BinaryOperation(op) => self.parse_binary_operation(op),
            Expression::Number(n) => Data::Number(n.value),
            Expression::Identifier(var) => self.variables.get(&var.name).unwrap().clone(),
            Expression::Text(str) => Data::Text(str.value),
            Expression::Boolean(b) => Data::Boolean(b.value),
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
            Token::IsEqual => match lhs {
                Data::Text(str) => match rhs {
                    Data::Text(s) => Data::Boolean(str == s),
                    _ => Data::Placeholder,
                },
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Boolean(n == m),
                    _ => Data::Placeholder,
                },
                Data::Boolean(b) => match rhs {
                    Data::Boolean(d) => Data::Boolean(b == d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsNotEqual => match lhs {
                Data::Text(str) => match rhs {
                    Data::Text(s) => Data::Boolean(str != s),
                    _ => Data::Placeholder,
                },
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Boolean(n != m),
                    _ => Data::Placeholder,
                },
                Data::Boolean(b) => match rhs {
                    Data::Boolean(d) => Data::Boolean(b != d),
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
                    value: lexer.slice().to_string().replace('"', ""),
                })
            }
            Token::LParen => {
                (expr, lexer) = self.pratt_parser(lexer, 0);
            }
            Token::Subtraction => {
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

            if token == None || token == Some(Token::RParen) {
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
            Token::Power => 5,
            Token::IsEqual => 6,
            _ => 0,
        }
    }
}

use crate::{
    lexer,
    parser::{
        expression::{BinaryOperation, Boolean, Expression, Identifier, Number, Text},
        variable::Data,
    },
    token::Token,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use logos::Lexer;
use std::collections::HashMap;

mod expression;
mod variable;

#[derive(Clone, Debug)]
pub struct Parser {
    pub file: String,
    pub lines: Vec<String>,
    pub line: String,
    pub line_number: usize,
    pub variables: HashMap<String, Data>,
    pub stacks: Vec<String>,
    pub char_sum: usize,
    pub to_break: bool,
}

impl Parser {
    pub fn new(file: String, lines: Vec<String>) -> Self {
        Self {
            file,
            lines,
            line: String::new(),
            variables: HashMap::new(),
            stacks: vec![String::from("main")],
            line_number: 0,
            char_sum: 0,
            to_break: false,
        }
    }

    pub fn run(mut self) -> Self {
        loop {
            if self.line_number + 1 > self.lines.len() {
                break;
            }
            self.parse(self.lines.get(self.line_number).unwrap().to_string());
            self.line_number += 1;
        }

        self
    }

    pub fn tokenize(&mut self, line: String) -> Option<Token> {
        lexer::lexer(&line).next()
    }

    pub fn parse(&mut self, line: String) -> Token {
        self.line = line.clone();
        let lexer = lexer::lexer(&line);
        let token = self.tokenize(self.line.clone());

        match token {
            Some(t) => match t {
                Token::Let => self.parse_declaration(lexer),
                Token::Print => self.parse_print(lexer, false),
                Token::Println => self.parse_print(lexer, true),
                Token::Ident => self.parse_assignment(lexer),
                Token::If => self.parse_if(lexer),
                Token::Loop => self.parse_loop(lexer),
                Token::Break => self.parse_break(),
                _ => {}
            },
            None => {}
        }

        self.char_sum += self.line.chars().count() + 1;
        if token == None {
            return Token::NewLine;
        }
        token.unwrap()
    }

    pub fn parse_break(&mut self) {
        // let mut index = 0;

        // for i in 0..self.stacks.len() {
        //     if self
        //         .stacks
        //         .get(i)
        //         .unwrap()
        //         .starts_with(&String::from("Loop"))
        //     {
        //         index = i;
        //     }
        // }

        // for _ in index..self.stacks.len() {
        //     self.stacks.pop();
        // }

        self.to_break = true;
    }

    pub fn parse_loop(&mut self, mut lexer: Lexer<Token>) {
        self.check(Token::Loop, lexer.next());

        let loop_signature = "Loop".to_owned() + &self.stacks.len().to_string();

        self.stacks.push(loop_signature.clone());

        self.line_number += 1;

        let loop_start = self.line_number;
        let mut ignore = false;

        loop {
            if self.line_number > self.lines.len() {
                self.line_number = loop_start;
            }

            if self.to_break {
                ignore = true;
                self.to_break = false;
            }

            let line = self.lines.get(self.line_number).unwrap().to_string();

            if ignore {
                let token = self.tokenize(line).unwrap();
                if token == Token::RCurly  {
                    if self.stacks.last().unwrap() == &loop_signature {
                        break;
                    } else {
                        self.stacks.pop();
                    }
                }
                self.line_number += 1;
                continue;
            }

            let token = self.parse(line.clone());

            if token == Token::RCurly && self.stacks.last().unwrap() == &loop_signature {
                self.line_number = loop_start;
                continue;
            }

            self.line_number += 1;
        }
    }

    pub fn parse_if(&mut self, mut lexer: Lexer<Token>) {
        self.check(Token::If, lexer.next());
        match self.parse_expression(lexer) {
            Data::Boolean(run) => {
                let if_signature = "If".to_string();

                self.stacks.push(if_signature.clone());

                let stack_len = self.stacks.len();
                let lines = self.lines.clone();
                self.line_number += 1;

                loop {
                    if self.line_number + 1 >= lines.len() {
                        break;
                    }

                    let line = lines.get(self.line_number).unwrap().to_string();

                    if run
                        && self.parse(line.clone()) == Token::RCurly
                        && self.stacks.last().unwrap() == &if_signature
                    {
                        self.stacks.pop();
                    }

                    if !run
                        && self.tokenize(line) == Some(Token::RCurly)
                        && self.stacks.last().unwrap() == &if_signature
                    {
                        self.stacks.pop();
                    }

                    if self.stacks.len() < stack_len {
                        break;
                    }

                    self.line_number += 1;
                }
            }
            t => self.throw(3, format!("unexpected data type {t:?}"), true),
        }
    }

    pub fn parse_print(&mut self, mut lexer: Lexer<Token>, new_line: bool) {
        if new_line {
            self.check(Token::Println, lexer.next());
        } else {
            self.check(Token::Print, lexer.next());
        }

        let mut lex = lexer.clone();
        lex.next();

        if lex.next() == Some(Token::RParen) {
            return println!();
        }

        let data: String = match self.parse_expression(lexer) {
            Data::Text(str) => str,
            Data::Number(n) => format!("{n}"),
            Data::Boolean(b) => format!("{b}"),
            t => {
                self.throw(3, format!("unexpected data type {t:?}"), true);
                String::new()
            }
        };

        if new_line {
            println!("{data}")
        } else {
            print!("{data}")
        }
    }

    pub fn parse_declaration(&mut self, mut lexer: Lexer<Token>) {
        self.check(Token::Let, lexer.next());
        self.check(Token::Ident, lexer.next());

        let identifier = lexer.slice().to_string();

        self.check(Token::Equal, lexer.next());

        let value = self.parse_expression(lexer);

        self.variables.insert(identifier, value);
    }

    pub fn parse_assignment(&mut self, mut lexer: Lexer<Token>) {
        self.check(Token::Ident, lexer.next());

        let identifier = lexer.slice().to_string();

        if self.variables.get(&identifier).is_none() {
            self.throw(4, "assignment to undeclared variable".to_string(), true)
        }

        self.check(Token::Equal, lexer.next());

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
            Token::IsGreater => match lhs {
                Data::Text(str) => match rhs {
                    Data::Text(s) => Data::Boolean(str > s),
                    _ => Data::Placeholder,
                },
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Boolean(n > m),
                    _ => Data::Placeholder,
                },
                Data::Boolean(b) => match rhs {
                    Data::Boolean(d) => Data::Boolean(b & !d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsLesser => match lhs {
                Data::Text(str) => match rhs {
                    Data::Text(s) => Data::Boolean(str < s),
                    _ => Data::Placeholder,
                },
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Boolean(n < m),
                    _ => Data::Placeholder,
                },
                Data::Boolean(b) => match rhs {
                    Data::Boolean(d) => Data::Boolean(!b & d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsGreaterEqual => match lhs {
                Data::Text(str) => match rhs {
                    Data::Text(s) => Data::Boolean(str >= s),
                    _ => Data::Placeholder,
                },
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Boolean(n >= m),
                    _ => Data::Placeholder,
                },
                Data::Boolean(b) => match rhs {
                    Data::Boolean(d) => Data::Boolean(b >= d),
                    _ => Data::Placeholder,
                },
                _ => Data::Placeholder,
            },
            Token::IsLesserEqual => match lhs {
                Data::Text(str) => match rhs {
                    Data::Text(s) => Data::Boolean(str <= s),
                    _ => Data::Placeholder,
                },
                Data::Number(n) => match rhs {
                    Data::Number(m) => Data::Boolean(n <= m),
                    _ => Data::Placeholder,
                },
                Data::Boolean(b) => match rhs {
                    Data::Boolean(d) => Data::Boolean(b <= d),
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

    pub fn build(
        &self,
        code: i32,
        message: String,
        label: bool,
    ) -> (Diagnostic<usize>, SimpleFiles<String, String>) {
        let mut files = SimpleFiles::new();
        let file = self
            .lines
            .clone()
            .into_iter()
            .map(|f| f + "\n")
            .collect::<String>();

        let file_id = files.add(self.file.clone(), file);

        let mut diagnostic: Diagnostic<usize> = Diagnostic::error()
            .with_message(message)
            .with_code("E".to_owned() + &code.to_string());

        if label {
            diagnostic = diagnostic.with_labels(vec![Label::primary(
                file_id,
                self.char_sum..self.char_sum + self.line.len(),
            )]);
        }

        (diagnostic, files)
    }

    pub fn throw(&self, code: i32, message: String, label: bool) {
        let (diagnostic, files) = &self.build(code, message, label);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        term::emit(&mut writer.lock(), &config, files, diagnostic).unwrap();
        std::process::exit(1);
    }

    pub fn check(&self, expectation: Token, reality: Option<Token>) {
        if expectation != reality.unwrap() {
            self.throw(1, format!("Expected {expectation} here"), true);
        }
    }

    pub fn infix_binding_power(&self, op: Token) -> u16 {
        match op {
            Token::Addition => 1,
            Token::Subtraction => 2,
            Token::Multiplication => 3,
            Token::Division => 4,
            Token::Power => 5,
            Token::IsNotEqual | Token::IsEqual => 6,
            _ => 0,
        }
    }
}

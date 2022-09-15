use std::vec::IntoIter;

use crate::{ast::Ast, expression::Expression, token::Token};

pub struct Lexer {
    pub lines: Vec<String>,
    pub tokens: Vec<Vec<Token>>,
    pub ast: Vec<Ast>,
}

impl Lexer {
    pub fn new(raw: String) -> Self {
        Self {
            lines: raw.lines().map(String::from).collect(),
            tokens: vec![],
            ast: vec![],
        }
    }

    pub fn ast(&mut self, line: Vec<Token>, mut i: usize, push: bool) -> (Ast, usize) {
        let mut ast: Ast = Ast::Placeholder;
        match line.first().unwrap() {
            Token::Let => {
                if let Token::Identifier(ident) = line.clone().get(1).unwrap() {
                    let (expr, _) = self.pratt_parser(
                        line.into_iter()
                            .enumerate()
                            .filter(|(i, _)| i > &2)
                            .map(|(_, v)| v)
                            .collect::<Vec<Token>>()
                            .into_iter(),
                        0,
                    );
                    ast = Ast::Declaration(ident.to_string(), expr);
                }
            }
            Token::Identifier(ident) => {
                let (expr, _) = self.pratt_parser(
                    line.clone()
                        .into_iter()
                        .enumerate()
                        .filter(|(i, _)| i > &1)
                        .map(|(_, v)| v)
                        .collect::<Vec<Token>>()
                        .into_iter(),
                    0,
                );
                ast = Ast::Redeclaration(ident.to_string(), expr);
            }
            Token::If => {
                let lcurly_pos = line.clone().into_iter().position(|f| f == Token::LCurly);
                let then_pos = line.clone().into_iter().position(|f| f == Token::Then);
                if lcurly_pos != None {
                    let (c, s) = line.split_at(lcurly_pos.unwrap() + 1);
                    let (mut condition, mut statement) = (c.to_vec(), s.to_vec());
                    condition.pop();
                    let (expr, _) = self.pratt_parser(
                        condition
                            .iter()
                            .cloned()
                            .enumerate()
                            .filter(|(i, _)| i > &0)
                            .map(|(_, v)| v)
                            .collect::<Vec<Token>>()
                            .into_iter(),
                        0,
                    );
                    if statement.is_empty() {
                        let mut brackets_open = 1;
                        let mut ast_vec = vec![];
                        let range = i..self.tokens.len();
                        for _ in range {
                            if brackets_open == 0 {
                                break;
                            }
                            statement = self.tokens.get(i + 1).unwrap().to_vec();
                            if statement.iter().any(|f| f == &Token::RCurly) {
                                brackets_open -= 1;
                            } else if statement.iter().any(|f| f == &Token::LCurly) {
                                brackets_open += 1;
                            }
                            if brackets_open == 0 {
                                i += 1;
                                break;
                            }
                            let (temp_ast, _) = self.ast(statement, i, false);
                            ast_vec.push(temp_ast);
                            i += 1;
                        }
                        ast = Ast::If(expr, ast_vec);
                    } else {
                        statement.pop();
                        (ast, i) = self.ast(statement, i, false);
                        i += 1;
                        ast = Ast::If(expr, vec![ast.clone()]);
                    }
                } else if then_pos != None {
                    let (c, s) = line.split_at(then_pos.unwrap() + 1);
                    let (mut condition, mut statement) = (c.to_vec(), s.to_vec());
                    condition.pop();
                    let (expr, _) = self.pratt_parser(
                        condition
                            .iter()
                            .cloned()
                            .enumerate()
                            .filter(|(i, _)| i > &0)
                            .map(|(_, v)| v)
                            .collect::<Vec<Token>>()
                            .into_iter(),
                        0,
                    );
                    if statement.is_empty() {
                        statement = self.tokens.get(i + 1).unwrap().to_vec();
                        i += 1;
                    }
                    (ast, i) = self.ast(statement, i, false);
                    i += 1;
                    ast = Ast::If(expr, vec![ast.clone()]);
                }
            }
            t => {
                panic!("{t} serves no purpose in AST tree!")
            }
        }
        if push {
            self.ast.push(ast.clone())
        }
        (ast, i)
    }

    pub fn lex(&mut self) {
        let mut c = 0;
        for mut i in 0..self.tokens.len() {
            if c > i {
                i = c
            } else {
                c = i
            }
            if self.tokens.get(c) == None {
                break;
            }
            let line = self.tokens.get(c).unwrap().to_vec();
            if line.first() == None {
                continue;
            }
            let (_, j) = self.ast(line, i, true);
            c = j;
            if c + 1 == self.tokens.len() {
                break;
            }
        }
        println!("{:?}", self.ast)
    }

    pub fn tokenize(&mut self) -> &mut Lexer {
        for mut line in self.lines.clone() {
            let mut array = vec![];
            line = line.trim().replace(';', "");
            for word in line.split_whitespace() {
                let token = match word {
                    "let" => Token::Let,
                    "if" => Token::If,
                    "then" => Token::Then,
                    "loop" => Token::Loop,
                    "," => Token::Comma,
                    "+" => Token::Addition,
                    "-" => Token::Subtraction,
                    "*" => Token::Multiplication,
                    "/" => Token::Division,
                    "^" => Token::Power,
                    "=" => Token::Equal,
                    "==" => Token::IsEqual,
                    "!=" => Token::IsNotEqual,
                    ">" => Token::IsGreater,
                    "<" => Token::IsLesser,
                    ">=" => Token::IsGreaterEqual,
                    "<=" => Token::IsLesserEqual,
                    "(" => Token::LParen,
                    ")" => Token::RParen,
                    "{" => Token::LCurly,
                    "}" => Token::RCurly,
                    "exit" => Token::Exit,
                    "break" => Token::Break,
                    "//" => Token::Comment,
                    "true" | "false" => Token::Bool(word.parse::<bool>().unwrap()),
                    _ => {
                        if word.ends_with("()") {
                            Token::FunctionName(word.to_string())
                        } else if word.starts_with('"') && word.ends_with('"') {
                            Token::String(word.to_string())
                        } else if word.parse::<i64>().is_ok() {
                            Token::Integer(word.parse::<i64>().unwrap())
                        } else if word.chars().all(|f| f.is_alphanumeric()) {
                            Token::Identifier(word.to_string())
                        } else {
                            Token::Error(word.to_string(), line.clone())
                        }
                    }
                };
                array.push(token);
            }
            self.tokens.push(array);
        }
        self
    }

    pub fn pratt_parser(
        &mut self,
        mut lexer: IntoIter<Token>,
        prec: u16,
    ) -> (Expression, IntoIter<Token>) {
        let token = lexer.next().unwrap();
        let mut expr: Expression = Expression::Placeholder;

        match token {
            Token::Identifier(i) => {
                expr = Expression::Identifier(i);
            }
            Token::Bool(bool) => {
                expr = Expression::Bool(bool);
            }
            Token::String(str) => {
                expr = Expression::String(str);
            }
            Token::LParen => {
                (expr, lexer) = self.pratt_parser(lexer, 0);
            }
            Token::Subtraction => {
                if let Token::Integer(i) = token {
                    expr = Expression::Integer(-i);
                }
            }
            _ => {
                if let Token::Integer(i) = token {
                    expr = Expression::Integer(i);
                }
            }
        };

        loop {
            let token = lexer.next();

            if token == None || token == Some(Token::RParen) {
                break;
            }

            let op = token.unwrap();

            if op == Token::Power && self.infix_binding_power(&op) < prec {
                break;
            }
            if op != Token::Power && self.infix_binding_power(&op) <= prec {
                break;
            }

            let rhs;
            (rhs, lexer) = self.pratt_parser(lexer, self.infix_binding_power(&op));
            expr = Expression::BinaryOperation(Box::new(expr), op, Box::new(rhs))
        }

        (expr, lexer)
    }

    pub fn infix_binding_power(&self, op: &Token) -> u16 {
        match op {
            Token::RCurly | Token::LCurly => 0,
            Token::Addition => 1,
            Token::Subtraction => 2,
            Token::Multiplication => 3,
            Token::Division => 4,
            Token::Power => 5,
            Token::IsNotEqual
            | Token::IsEqual
            | Token::IsGreater
            | Token::IsLesser
            | Token::IsGreaterEqual
            | Token::IsLesserEqual => 6,
            _ => {
                println!("infix binding power not set for operator {op:?}");
                0
            }
        }
    }
}

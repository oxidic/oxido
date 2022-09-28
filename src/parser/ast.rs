use crate::{ast::AstNode, expression::Expression, parser::lexer::Lexer, token::Token};
use std::{iter::Peekable, vec::IntoIter};
pub struct Ast {
    pub tokens: Vec<Vec<Token>>,
    pub ast: Vec<AstNode>,
}

impl Ast {
    pub fn new(raw: String) -> Self {
        let mut lexer = Lexer::new(raw);
        Self {
            tokens: lexer.tokenize(),
            ast: vec![],
        }
    }

    pub fn ast(&mut self, mut line: Vec<Token>, mut i: usize, push: bool) -> (AstNode, usize) {
        let mut ast: AstNode = AstNode::Placeholder;
        match line.clone().first().unwrap() {
            Token::Let => {
                if let Token::Identifier(ident) = line.clone().get(1).unwrap() {
                    if line.pop() != Some(Token::Semicolon) {
                        panic!("expected semicolon on line: {line:?}")
                    }
                    let (expr, _) = self.pratt_parser(
                        line.into_iter()
                            .enumerate()
                            .filter(|(i, _)| i > &2)
                            .map(|(_, v)| v)
                            .collect::<Vec<Token>>()
                            .into_iter()
                            .peekable(),
                        0,
                    );
                    ast = AstNode::Declaration(ident.to_string(), expr);
                }
            }
            Token::Identifier(ident) => {
                if line.pop() != Some(Token::Semicolon) {
                    panic!("expected semicolon on line: {line:?}")
                }
                let (expr, _) = self.pratt_parser(
                    line.clone()
                        .into_iter()
                        .enumerate()
                        .filter(|(i, _)| i > &1)
                        .map(|(_, v)| v)
                        .collect::<Vec<Token>>()
                        .into_iter()
                        .peekable(),
                    0,
                );
                ast = AstNode::Redeclaration(ident.to_string(), expr);
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
                            .into_iter()
                            .peekable(),
                        0,
                    );
                    if statement.is_empty() {
                        let mut brackets_open = 1;
                        let mut ast_vec = vec![];
                        let range = i..self.tokens.len();
                        for _ in range {
                            if brackets_open == 0 || self.tokens.get(i + 1) == None {
                                break;
                            }
                            statement = self.tokens.get(i + 1).unwrap().to_vec();
                            if statement.iter().any(|f| f == &Token::RCurly) {
                                brackets_open -= 1;
                            } else if statement.iter().any(|f| f == &Token::LCurly)
                                && !statement.iter().any(|f| f == &Token::If)
                                && !statement.iter().any(|f| f == &Token::Loop)
                            {
                                brackets_open += 1;
                            }
                            if brackets_open == 0 {
                                i += 1;
                                break;
                            }
                            let (temp_ast, j) = self.ast(statement, i + 1, false);
                            ast_vec.push(temp_ast);
                            i = j;
                        }
                        ast = AstNode::If(expr, ast_vec);
                        i += 1;
                    } else {
                        statement.pop();
                        let mut statements = vec![];
                        let mut temp_statements = vec![];
                        for token in statement.clone() {
                            temp_statements.push(token.clone());
                            if token == Token::Semicolon {
                                (ast, i) = self.ast(temp_statements.clone(), i, false);
                                statements.push(ast);
                                temp_statements.clear();
                            }
                        }
                        ast = AstNode::If(expr, statements);
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
                            .into_iter()
                            .peekable(),
                        0,
                    );
                    if statement.is_empty() {
                        statement = self.tokens.get(i + 1).unwrap().to_vec();
                        i += 1;
                    }
                    (ast, i) = self.ast(statement, i, false);
                    i += 1;
                    ast = AstNode::If(expr, vec![ast.clone()]);
                }
            }
            Token::Loop => {
                let mut brackets_open = 1;
                let mut ast_vec = vec![];
                let range = i..self.tokens.len();
                for _ in range {
                    if brackets_open == 0 || self.tokens.get(i + 1) == None {
                        break;
                    }
                    let statement = self.tokens.get(i + 1).unwrap().to_vec();
                    if statement.iter().any(|f| f == &Token::RCurly) {
                        brackets_open -= 1;
                    } else if statement.iter().any(|f| f == &Token::LCurly)
                        && !statement.iter().any(|f| f == &Token::If)
                        && !statement.iter().any(|f| f == &Token::Loop)
                    {
                        brackets_open += 1;
                    }
                    if brackets_open == 0 {
                        i += 1;
                        break;
                    }
                    let (temp_ast, j) = self.ast(statement, i + 1, false);
                    ast_vec.push(temp_ast);
                    i = j;
                }
                ast = AstNode::Loop(ast_vec);
                i += 1;
            }
            Token::Break => {
                ast = AstNode::Break;
            }
            //TODO: Make this work with args
            Token::Function => {
                if let Token::FunctionName(_name) = line.get(1).unwrap() {
                    let mut brackets_open = 1;
                    let mut ast_vec = vec![];
                    let range = i..self.tokens.len();
                    for _ in range {
                        if brackets_open == 0 || self.tokens.get(i + 1) == None {
                            break;
                        }
                        let statement = self.tokens.get(i + 1).unwrap().to_vec();
                        if statement.iter().any(|f| f == &Token::RCurly) {
                            brackets_open -= 1;
                        } else if statement.iter().any(|f| f == &Token::LCurly)
                            && !statement.iter().any(|f| f == &Token::If)
                            && !statement.iter().any(|f| f == &Token::Loop)
                        {
                            brackets_open += 1;
                        }
                        if brackets_open == 0 {
                            i += 1;
                            break;
                        }
                        let (temp_ast, j) = self.ast(statement, i + 1, false);
                        ast_vec.push(temp_ast);
                        i = j;
                    }
                    // ast = AstNode::Function(name.to_string(), args.to_vec(), ast_vec);
                    i += 1;
                }
            }
            Token::Return => {
                if line.pop() != Some(Token::Semicolon) {
                    panic!("expected semicolon on line: {line:?}")
                }
                let (expr, _) = self.pratt_parser(
                    line.clone()
                        .into_iter()
                        .enumerate()
                        .filter(|(i, _)| i > &0)
                        .map(|(_, v)| v)
                        .collect::<Vec<Token>>()
                        .into_iter()
                        .peekable(),
                    0,
                );
                ast = AstNode::Return(expr);
            }
            //TODO: Deal with this error
            // Token::FunctionSignature(name, args) => {
            //     let mut parsed_args = vec![];
            //     for arg in args {
            //         let mut lexer = Lexer::new(arg.to_string());
            //         let (expr, _) = self.pratt_parser(lexer.tokenize().get(0).unwrap().clone().into_iter().peekable(), 0);
            //         parsed_args.push(expr);
            //     }

            //     ast = AstNode::Call(
            //         name.to_string(),
            //         parsed_args,
            //     );
            // }
            Token::Comment => {}
            t => {
                println!("{t} serves no purpose in AST tree!")
            }
        }
        if push {
            self.ast.push(ast.clone())
        }
        (ast, i)
    }

    pub fn tree(&mut self) -> Vec<AstNode> {
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
        self.ast.clone()
    }

    pub fn pratt_parser(
        &mut self,
        mut lexer: Peekable<IntoIter<Token>>,
        prec: u16,
    ) -> (Expression, Peekable<IntoIter<Token>>) {
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
            let mut lex = lexer.clone();
            let op = lex.peek();

            if op == None || op == Some(&Token::RParen) {
                break;
            }

            if op.unwrap() == &Token::Power && self.infix_binding_power(op.unwrap()) < prec {
                break;
            }

            if op.unwrap() != &Token::Power && self.infix_binding_power(op.unwrap()) <= prec {
                break;
            }
            lexer.next();
            let rhs;
            (rhs, lexer) = self.pratt_parser(lexer, self.infix_binding_power(op.unwrap()));
            expr = Expression::BinaryOperation(Box::new(expr), op.unwrap().clone(), Box::new(rhs))
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

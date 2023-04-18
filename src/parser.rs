use std::{iter::Peekable, vec::IntoIter};

use crate::{
    ast::{AstNode, Expression},
    error::error,
    token::Token,
};

pub struct Parser {
    name: String,
    file: String,
    tokens: Vec<(Token, usize)>,
    ast: Vec<AstNode>,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, usize)>, file: String, name: String) -> Self {
        Self {
            name,
            file,
            tokens,
            ast: vec![],
        }
    }

    pub fn run(&mut self) -> &Vec<AstNode> {
        self.ast = self.match_tokens(self.tokens.clone());

        &self.ast
    }

    pub fn match_tokens(&self, tokens: Vec<(Token, usize)>) -> Vec<AstNode> {
        let mut pos = 0;
        let mut statements = vec![];
        let mut nodes: Vec<AstNode> = vec![];

        loop {
            let token = tokens.get(pos);

            if token.is_none() {
                break;
            }

            let token = token.unwrap();

            if token.0 == Token::Let {
                loop {
                    let token = tokens.get(pos);

                    if token.is_none() {
                        break;
                    }

                    let token = token.unwrap();

                    if token.0 == Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements.clone()));
                        statements.clear();
                        break;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else if let Token::Identifier(_) = token.0 {
                loop {
                    let token = tokens.get(pos);

                    if token.is_none() {
                        break;
                    }

                    let token = token.unwrap();

                    if token.0 == Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements.clone()));
                        statements.clear();
                        break;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else if let Token::FunctionCall(_) = token.0 {
                loop {
                    let token = tokens.get(pos);

                    if token.is_none() {
                        break;
                    }

                    let token = token.unwrap();

                    if token.0 == Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements.clone()));
                        statements.clear();
                        break;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else if token.0 == Token::If || token.0 == Token::Loop || token.0 == Token::Fn {
                let mut depth = 0;
                loop {
                    let token = tokens.get(pos);

                    if token.is_none() {
                        break;
                    }

                    let token = token.unwrap();

                    if token.0 == Token::RCurly {
                        depth -= 1;
                        if depth == 0 {
                            statements.push(token);
                            nodes.push(self.parse(statements.clone()));
                            statements.clear();
                            break;
                        }
                    }
                    if token.0 == Token::LCurly {
                        depth += 1;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else {
                match &token.0 {
                    Token::Break => nodes.push(AstNode::Break),
                    Token::Return => nodes.push(AstNode::Return),
                    Token::Exit => nodes.push(AstNode::Exit),
                    Token::Semicolon => {}
                    t => {
                        println!("{tokens:?}");
                        error(
                            &self.name,
                            &self.file,
                            "0001",
                            &format!("token `{}` was not expected here", t.to_string()),
                            &format!("token `{}` was not expected here", t.to_string()),
                            token.1..token.1 + token.0.size(),
                        );
                    }
                }
            }

            statements.clear();

            pos += 1;
        }

        nodes
    }

    pub fn parse(&self, tokens: Vec<&(Token, usize)>) -> AstNode {
        let mut stream = tokens.iter().peekable();

        let token = *stream.next().unwrap();

        let node = if token.0 == Token::Let {
            if let Token::Identifier(ident) = &stream.next().unwrap().0 {
                let t = stream.next().unwrap();
                if t.0 != Token::Equal {
                    error(
                        &self.name,
                        &self.file,
                        "0001",
                        &format!("expected `=` found {}", t.0.to_string()),
                        "use `=` here",
                        t.1..t.1 + t.0.size(),
                    );
                };

                let mut tokens = stream.collect::<Vec<_>>();

                let t = tokens.pop().unwrap();

                if t.0 != Token::Semicolon {
                    error(
                        &self.name,
                        &self.file,
                        "0001",
                        &format!("expected `;` found {}", t.0.to_string()),
                        "use `;` here",
                        t.1..t.1 + t.0.size(),
                    );
                }

                let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

                let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

                AstNode::Assignment(ident.to_string(), expression)
            } else {
                panic!("unexpected token {token:?} after let")
            }
        } else if let Token::Identifier(ident) = &token.0 {
            let t = &stream.next().unwrap();

            if t.0 != Token::Equal {
                error(
                    &self.name,
                    &self.file,
                    "0001",
                    &format!("expected `=` found {}", t.0.to_string()),
                    "note: use `=` here",
                    t.1..t.1 + t.0.size(),
                );
            };

            let mut tokens = stream.collect::<Vec<_>>();

            let t = tokens.pop().unwrap();

            if t.0 != Token::Semicolon {
                error(
                    &self.name,
                    &self.file,
                    "0001",
                    &format!("expected `;` found {}", t.0.to_string()),
                    "use `;` here",
                    t.1..t.1 + t.0.size(),
                );
            }

            let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

            let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

            AstNode::Assignment(ident.to_string(), expression)
        } else if token.0 == Token::If {
            let mut tokens = vec![];
            let mut statements = vec![];
            let mut flag = false;

            for token in stream {
                if token.0 == Token::LCurly {
                    flag = true;
                    continue;
                }
                if flag {
                    statements.push((*token).to_owned());
                } else {
                    tokens.push(token);
                }
            }

            let t = tokens.pop().unwrap();

            if t.0 != Token::LCurly {
                error(
                    &self.name,
                    &self.file,
                    "0001",
                    &format!("expected `{{` found {}", t.0.to_string()),
                    "use `{` here",
                    t.1..t.1 + t.0.size(),
                );
            }

            let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

            let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

            AstNode::If(expression, self.match_tokens(statements))
        } else if token.0 == Token::Loop {
            let mut statements = vec![];

            let t = &stream.next().unwrap();

            if t.0 != Token::LCurly {
                error(
                    &self.name,
                    &self.file,
                    "0001",
                    &format!("expected `{{` found {}", t.0.to_string()),
                    "note: use `{` here",
                    t.1..t.1 + t.0.size(),
                );
            };

            for token in stream {
                statements.push((*token).to_owned());
            }

            AstNode::Loop(self.match_tokens(statements))
        } else if let Token::FunctionCall(ident) = &token.0 {
            let t = &stream.next().unwrap();
            if t.0 != Token::LParen {
                error(
                    &self.name,
                    &self.file,
                    "0001",
                    &format!("expected `(` found {}", t.0.to_string()),
                    "use `(` here",
                    t.1..t.1 + t.0.size(),
                );
            };

            let tokens = stream.map(|f| f.to_owned()).collect::<Vec<_>>();
            let mut params = vec![];
            let mut expression = vec![];

            for token in tokens {
                if token.0 == Token::RParen {
                    let (data, _) = self.pratt_parser(expression.clone().into_iter().peekable(), 0);

                    params.push(data);
                    break;
                }

                if token.0 == Token::Comma {
                    let (data, _) = self.pratt_parser(expression.clone().into_iter().peekable(), 0);

                    params.push(data);

                    expression.clear();
                    continue;
                }

                expression.push(token);
            }

            AstNode::FunctionCall(ident.to_string(), params)
        } else if token.0 == Token::Fn {
            let t = &stream.next().unwrap();
            if let Token::FunctionName(name) = &t.0 {
                let mut params = vec![];

                loop {
                    let token = stream.peek();

                    if token.is_none() {
                        break;
                    }

                    let token = &stream.next().unwrap().0;

                    if token == &Token::RParen {
                        break;
                    }

                    if token == &Token::Comma {
                        continue;
                    }

                    if let Token::Identifier(name) = token {
                        params.push(name.to_string());
                    }
                }

                let mut statements = vec![];

                loop {
                    let token = stream.peek();

                    if token.is_none() {
                        break;
                    }

                    let token = stream.next().unwrap();

                    statements.push((*token).to_owned());
                }

                AstNode::FunctionDeclaration(
                    name.to_string(),
                    params,
                    self.match_tokens(statements),
                )
            } else {
                error(
                    &self.name,
                    &self.file,
                    "0001",
                    &format!("expected name of function found {}", t.0.to_string()),
                    "use function name here",
                    t.1..t.1 + t.0.size(),
                );
            }
        } else {
            error(
                &self.name,
                &self.file,
                "0001",
                &format!("expected name of function found {}", token.0.to_string()),
                "use function name here",
                token.1..token.1 + token.0.size(),
            );
        };

        node
    }

    pub fn pratt_parser<'a>(
        &self,
        mut lexer: Peekable<IntoIter<&'a (Token, usize)>>,
        prec: u16,
    ) -> (Expression, Peekable<IntoIter<&'a (Token, usize)>>) {
        if lexer.clone().next().is_none() {
            return (Expression::String(String::new()), lexer);
        }
        let token = &lexer.next().unwrap().0;
        let mut expr: Expression = Expression::Unexpected;

        match token {
            Token::Identifier(i) => {
                expr = Expression::Identifier(i.to_string());
            }
            Token::Bool(bool) => {
                expr = Expression::Bool(*bool);
            }
            Token::String(str) => {
                expr = Expression::String(str.to_string());
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
                    expr = Expression::Integer(*i);
                }
            }
        };

        loop {
            let mut lex = lexer.clone();
            let op = lex.peek();

            if op.is_none() || op.unwrap().0 == Token::RParen {
                break;
            }

            if op.unwrap().0 == Token::Power && self.infix_binding_power(op.unwrap()) < prec {
                break;
            }

            if op.unwrap().0 != Token::Power && self.infix_binding_power(op.unwrap()) <= prec {
                break;
            }
            lexer.next();
            let rhs;
            (rhs, lexer) = self.pratt_parser(lexer, self.infix_binding_power(op.unwrap()));
            expr = Expression::BinaryOperation(Box::new(expr), op.unwrap().0.clone(), Box::new(rhs))
        }

        (expr, lexer)
    }

    pub fn infix_binding_power(&self, op: &(Token, usize)) -> u16 {
        match op.0 {
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
                error(
                    &self.name,
                    &self.file,
                    "0001",
                    &format!("expected an operator found {}", op.0.to_string()),
                    "use an operator here",
                    op.1..op.1 + op.0.size(),
                );
            }
        }
    }
}

use std::{iter::Peekable, vec::IntoIter};

use crate::{
    ast::{AstNode, Expression},
    token::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    ast: Vec<AstNode>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ast: vec![],
        }
    }

    pub fn run(&mut self) -> &Vec<AstNode> {
        self.ast = self.match_tokens(self.tokens.clone());

        &self.ast
    }

    pub fn match_tokens(&self, tokens: Vec<Token>) -> Vec<AstNode> {
        let mut pos = 0;
        let mut statements = vec![];
        let mut nodes: Vec<AstNode> = vec![];

        loop {
            let token = tokens.get(pos);

            if token == None {
                break;
            }

            let token = token.unwrap();

            if token == &Token::Let {
                loop {
                    let token = tokens.get(pos);

                    if token == None {
                        break;
                    }

                    let token = token.unwrap();

                    if token == &Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements.clone()));
                        statements.clear();
                        break;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else if let Token::Identifier(_) = token {
                loop {
                    let token = tokens.get(pos);

                    if token == None {
                        break;
                    }

                    let token = token.unwrap();

                    if token == &Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements.clone()));
                        statements.clear();
                        break;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else if token == &Token::If || token == &Token::Loop {
                let mut depth = 0;
                loop {
                    let token = tokens.get(pos);

                    if token == None {
                        break;
                    }

                    let token = token.unwrap();

                    if token == &Token::RCurly {
                        depth -= 1;
                        if depth == 0 {
                            statements.push(token);
                            nodes.push(self.parse(statements.clone()));
                            statements.clear();
                            break;
                        }
                    }
                    if token == &Token::LCurly {
                        depth += 1;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else {
                match token {
                    Token::Break => nodes.push(AstNode::Break),
                    Token::Return => nodes.push(AstNode::Return),
                    Token::Exit => nodes.push(AstNode::Exit),
                    // TODO: Handle none case
                    _ => {}
                }
            }

            statements.clear();

            pos += 1;
        }

        nodes
    }

    pub fn parse(&self, tokens: Vec<&Token>) -> AstNode {
        let mut stream = tokens.iter().peekable();

        let token = *stream.next().unwrap();

        let node = if token == &Token::Let {
            if let Token::Identifier(ident) = stream.next().unwrap() {
                if *stream.next().unwrap() != &Token::Equal {
                    panic!("expected equal sign")
                };

                let mut tokens = stream.map(|f| f.to_owned()).collect::<Vec<_>>();

                if *tokens.pop().unwrap() != Token::Semicolon {
                    panic!("expected semicolon")
                }

                let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

                AstNode::Assignment(ident.to_string(), expression)
            } else {
                panic!("unexpected token {token} after let")
            }
        } else if let Token::Identifier(ident) = token {
            let equal = *stream.next().unwrap();
            if equal != &Token::Equal {
                panic!("expected equal sign, found {equal}")
            };

            let mut tokens = stream.map(|f| f.to_owned()).collect::<Vec<_>>();

            if *tokens.pop().unwrap() != Token::Semicolon {
                panic!("expected semicolon")
            }

            let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

            AstNode::Assignment(ident.to_string(), expression)
        } else if token == &Token::If {
            let mut tokens = vec![];
            let mut statements = vec![];
            let mut flag = false;

            for token in stream {
                if *token == &Token::LCurly {
                    flag = true;
                }
                if flag {
                    statements.push((*token).to_owned());
                } else {
                    tokens.push(*token);
                }
            }

            let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

            AstNode::If(expression, self.match_tokens(statements))
        } else if token == &Token::Loop {
            let mut statements = vec![];

            for token in stream {
                statements.push((*token).to_owned());
            }

            AstNode::Loop(self.match_tokens(statements))
        } else {
            unimplemented!("token {token} somwhow reached parser match")
        };

        node
    }

    pub fn pratt_parser<'a>(
        &self,
        mut lexer: Peekable<IntoIter<&'a Token>>,
        prec: u16,
    ) -> (Expression, Peekable<IntoIter<&'a Token>>) {
        if lexer.clone().next() == None {
            return (Expression::String(String::new()), lexer);
        }
        let token = lexer.next().unwrap();
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

            if op == None || op == Some(&&Token::RParen) {
                break;
            }

            if op.unwrap() == &&Token::Power && self.infix_binding_power(op.unwrap()) < prec {
                break;
            }

            if op.unwrap() != &&Token::Power && self.infix_binding_power(op.unwrap()) <= prec {
                break;
            }
            lexer.next();
            let rhs;
            (rhs, lexer) = self.pratt_parser(lexer, self.infix_binding_power(op.unwrap()));
            expr = Expression::BinaryOperation(
                Box::new(expr),
                (*op.unwrap()).clone().to_owned(),
                Box::new(rhs),
            )
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
                panic!("infix binding power not set for operator {op:?}");
            }
        }
    }
}

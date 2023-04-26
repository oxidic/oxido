use std::{iter::Peekable, ops::Range, vec::IntoIter};

use crate::{
    ast::{AstNode, Expression},
    error::error,
    token::Token,
};

pub struct Parser<'a> {
    name: &'a str,
    file: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(name: &'a str, file: &'a str) -> Self {
        Self { name, file }
    }

    pub fn run(&'a self, tokens: Vec<(Token, usize)>) -> Option<Vec<(AstNode, Range<usize>)>> {
        let ast = self.match_tokens(tokens)?;

        Some(ast)
    }

    pub fn match_tokens(
        &'a self,
        tokens: Vec<(Token, usize)>,
    ) -> Option<Vec<(AstNode, Range<usize>)>> {
        let mut pos = 0;
        let mut nodes: Vec<(AstNode, Range<usize>)> = vec![];

        loop {
            let mut statements = vec![];
            let token = tokens.get(pos);

            if token.is_none() {
                break;
            }

            let token = token?;

            if token.0 == Token::Let {
                loop {
                    let token = tokens.get(pos);

                    if token.is_none() {
                        break;
                    }

                    let token = token?;

                    if token.0 == Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements)?);
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

                    let token = token?;

                    if token.0 == Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements)?);
                        break;
                    }

                    statements.push(token);

                    pos += 1;
                }
            } else if let Token::FunctionName(_) = token.0 {
                loop {
                    let token = tokens.get(pos);

                    if token.is_none() {
                        break;
                    }

                    let token = token?;

                    if token.0 == Token::Semicolon {
                        statements.push(token);
                        nodes.push(self.parse(statements)?);
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

                    let token = token?;

                    if token.0 == Token::RCurly {
                        depth -= 1;
                        if depth == 0 {
                            statements.push(token);
                            nodes.push(self.parse(statements)?);
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
                    Token::Break => nodes.push((AstNode::Break, token.1..token.0.len())),
                    Token::Return => loop {
                        let token = tokens.get(pos);

                        if token.is_none() {
                            break;
                        }

                        let token = token?;

                        if token.0 == Token::Semicolon {
                            statements.push(token);
                            nodes.push(self.parse(statements)?);
                            break;
                        }

                        statements.push(token);

                        pos += 1;
                    },
                    Token::Exit => nodes.push((AstNode::Exit, token.1..token.0.len())),
                    Token::Semicolon => {}
                    t => {
                        error(
                            self.name,
                            self.file,
                            "0001",
                            &format!("token `{}` was not expected here", t.as_string()),
                            &format!("token `{}` was not expected here", t.as_string()),
                            &(token.1..token.1 + token.0.len()),
                        );
                    }
                }
            }

            pos += 1;
        }

        Some(nodes)
    }

    pub fn parse(&'a self, tokens: Vec<&'a (Token, usize)>) -> Option<(AstNode, Range<usize>)> {
        let mut stream = tokens.iter().peekable();

        let token = *stream.next()?;

        let node: (AstNode, Range<usize>) = if token.0 == Token::Let {
            let t = &stream.next()?;
            if let Token::Identifier(ident) = &t.0 {
                let t = stream.next()?;
                if t.0 != Token::Equal {
                    error(
                        self.name,
                        self.file,
                        "0001",
                        &format!("expected `=` found {}", t.0.as_string()),
                        "use `=` here",
                        &(t.1 - 1..t.1 + t.0.len() - 1),
                    );
                };

                let mut tokens = stream.collect::<Vec<_>>();

                let t = tokens.pop()?;

                if t.0 != Token::Semicolon {
                    error(
                        self.name,
                        self.file,
                        "0001",
                        &format!("expected `;` found {}", t.0.as_string()),
                        "use `;` here",
                        &(t.1 - 1..t.1 + t.0.len() - 1),
                    );
                }

                let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

                let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

                (
                    AstNode::Assignment(ident.to_string(), expression),
                    token.1..t.1,
                )
            } else {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected identifier found {}", t.0.as_string()),
                    "use an identifier here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            }
        } else if let Token::Identifier(ident) = &token.0 {
            let t = &stream.next()?;

            if t.0 != Token::Equal {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("unexpected punctuator {}", t.0.as_string()),
                    "for declaring a value `=` should be used",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            };

            let mut tokens = stream.collect::<Vec<_>>();

            let t = tokens.pop()?;

            if t.0 != Token::Semicolon {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected `;` found {}", t.0.as_string()),
                    "use `;` here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            }

            let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

            let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

            (
                AstNode::ReAssignment(ident.to_string(), expression),
                token.1..t.1,
            )
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

            let t = statements.pop()?;

            if t.0 != Token::RCurly {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected `}}` found {}", t.0.as_string()),
                    "use `}` here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            }

            let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

            let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

            (
                AstNode::If(expression, self.match_tokens(statements)?),
                token.1..t.1,
            )
        } else if token.0 == Token::Loop {
            let mut statements = vec![];

            let t = &stream.next()?;

            if t.0 != Token::LCurly {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected `{{` found {}", t.0.as_string()),
                    "use `{` here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            };

            for token in stream {
                statements.push((*token).to_owned());
            }

            let t = statements.pop()?;

            if t.0 != Token::RCurly {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected `}}` found {}", t.0.as_string()),
                    "use `}` here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            };

            (AstNode::Loop(self.match_tokens(statements)?), token.1..t.1)
        } else if let Token::FunctionName(ident) = &token.0 {
            let t = &stream.next()?;
            if t.0 != Token::LParen {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected `(` found {}", t.0.as_string()),
                    "use `(` here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            };

            let tokens = stream.map(|f| f.to_owned()).collect::<Vec<_>>();
            let mut params = vec![];
            let mut expression = vec![];

            let mut end = 0;
            let mut depth = 1;

            for token in tokens {
                if token.0 == Token::LParen {
                    depth += 1;
                }
                if token.0 == Token::RParen {
                    depth -= 1;
                    if !expression.is_empty() {
                        if depth > 0 {
                            expression.push(token);
                        }
                        let (data, _) =
                            self.pratt_parser(expression.clone().into_iter().peekable(), 0);

                        params.push(data);

                        expression.clear();
                        continue;
                    }
                    if depth == 0 {
                        end = token.1;
                        break;
                    }
                }

                if token.0 == Token::Comma {
                    let (data, _) = self.pratt_parser(expression.clone().into_iter().peekable(), 0);

                    params.push(data);

                    expression.clear();
                    continue;
                }

                expression.push(token);
            }

            (
                AstNode::FunctionCall(ident.to_string(), params),
                token.1..end,
            )
        } else if token.0 == Token::Fn {
            let t = &stream.next()?;
            if let Token::FunctionName(name) = &t.0 {
                let mut params = vec![];

                loop {
                    let token = stream.peek();

                    if token.is_none() {
                        break;
                    }

                    let token = &stream.next()?.0;

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

                let t = &stream.next()?;

                if t.0 != Token::LCurly {
                    error(
                        self.name,
                        self.file,
                        "0001",
                        &format!("expected `{{` found {}", t.0.as_string()),
                        "use `{` here",
                        &(t.1 - 1..t.1 + t.0.len() - 1),
                    );
                };

                let mut statements = vec![];

                loop {
                    let token = stream.peek();

                    if token.is_none() {
                        break;
                    }

                    let token = stream.next()?;

                    statements.push((*token).to_owned());
                }

                let t = statements.pop()?;

                if t.0 != Token::RCurly {
                    error(
                        self.name,
                        self.file,
                        "0001",
                        &format!("expected `}}` found {}", t.0.as_string()),
                        "use `}` here",
                        &(t.1 - 1..t.1 + t.0.len() - 1),
                    );
                };

                (
                    AstNode::FunctionDeclaration(
                        name.to_string(),
                        params,
                        self.match_tokens(statements)?,
                    ),
                    token.1..t.1,
                )
            } else {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected name of function found {}", t.0.as_string()),
                    "use function name here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            }
        } else if token.0 == Token::Return {
            let mut tokens = stream.collect::<Vec<_>>();

            let t = tokens.pop()?;

            if t.0 != Token::Semicolon {
                error(
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected `;` found {}", t.0.as_string()),
                    "use `;` here",
                    &(t.1 - 1..t.1 + t.0.len() - 1),
                );
            }

            let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

            let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

            (AstNode::Return(expression), token.1..t.1)
        } else {
            error(
                self.name,
                self.file,
                "0001",
                &format!("{} was not expected", token.0.as_string()),
                "did not expect this",
                &(token.1 - 1..token.1 + token.0.len()),
            );
        };

        Some(node)
    }

    pub fn pratt_parser(
        &'a self,
        mut lexer: Peekable<IntoIter<&'a (Token, usize)>>,
        prec: u16,
    ) -> (Expression, Peekable<IntoIter<&'a (Token, usize)>>) {
        if lexer.clone().next().is_none() {
            return (Expression::Str(String::new()), lexer);
        }
        let token = &lexer.next().unwrap();
        let mut expr: Option<Expression> = None;

        match &token.0 {
            Token::Identifier(i) => {
                expr = Some(Expression::Identifier(i.to_string()));
            }
            Token::Bool(bool) => {
                expr = Some(Expression::Bool(*bool));
            }
            Token::Str(str) => {
                expr = Some(Expression::Str(str.to_string()));
            }
            Token::LParen => {
                let exp;
                (exp, lexer) = self.pratt_parser(lexer, 0);
                expr = Some(exp);
            }
            Token::Subtraction => {
                if let Token::Integer(i) = token.0 {
                    expr = Some(Expression::Integer(-i));
                }
            }
            Token::FunctionName(f) => {
                let t = &lexer.next().unwrap();
                if t.0 != Token::LParen {
                    error(
                        self.name,
                        self.file,
                        "0001",
                        &format!("expected `(` found {}", t.0.as_string()),
                        "use `(` here",
                        &(t.1 - 1..t.1 + t.0.len() - 1),
                    );
                };

                let mut tokens = vec![];
                let mut depth = 1;

                loop {
                    let t = lexer.next();

                    if t.is_none() {
                        break;
                    }
                    let t = t.unwrap();

                    if t.0 == Token::LParen {
                        depth += 1;
                    } else if t.0 == Token::RParen {
                        depth -= 1;
                    }

                    tokens.push(t);

                    if depth == 0 {
                        break;
                    }
                }

                let mut params = vec![];
                let mut expression: Vec<(Token, usize)> = vec![];

                for token in tokens {
                    if token.0 == Token::RParen {
                        if !expression.is_empty() {
                            let lex = expression.iter().collect::<Vec<_>>().into_iter().peekable();
                            let (data, _) = self.pratt_parser(lex, 0);

                            params.push(data);
                        }
                        break;
                    }

                    if token.0 == Token::Comma {
                        let lex = expression.iter().collect::<Vec<_>>().into_iter().peekable();
                        let (data, _) = self.pratt_parser(lex, 0);

                        params.push(data);

                        expression.clear();
                        continue;
                    }

                    expression.push(token.to_owned());
                }

                expr = Some(Expression::FunctionCall(f.to_string(), params));
            }
            _ => {
                if let Token::Integer(i) = token.0 {
                    expr = Some(Expression::Integer(i));
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
            expr = Some(Expression::BinaryOperation(
                Box::new(expr.unwrap()),
                op.unwrap().0.clone(),
                Box::new(rhs),
            ))
        }

        if expr.is_none() {
            error(
                self.name,
                self.file,
                "0003",
                "could not parse expression",
                "expression couldn't be parsed",
                &(token.1..token.1),
            );
        }

        (expr.unwrap(), lexer)
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
                    self.name,
                    self.file,
                    "0001",
                    &format!("expected an operator found {}", op.0.as_string()),
                    "use an operator here",
                    &(op.1..op.1 + op.0.len()),
                );
            }
        }
    }
}

use std::{iter::Peekable, ops::Range, vec::IntoIter};

use crate::{
	ast::{Ast, AstNode, Expression},
	data::Param,
	error::error,
	token::{Token, Tokens},
};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
	name: &'a str,
	file: &'a str,
}

impl<'a> Parser<'a> {
	pub fn new(name: &'a str, file: &'a str) -> Self {
		Self { name, file }
	}

	pub fn run(&'a self, tokens: Tokens) -> Option<Ast> {
		let ast = self.match_tokens(tokens)?;

		Some(ast)
	}

	fn match_tokens(&'a self, tokens: Tokens) -> Option<Ast> {
		let mut pos = 0;
		let mut nodes: Ast = vec![];

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
						break;
					}

					statements.push(token);

					pos += 1;
				}

				nodes.push(self.parse(statements)?);
			} else if let Token::Identifier(_) = token.0 {
				loop {
					let token = tokens.get(pos);

					if token.is_none() {
						break;
					}

					let token = token?;

					if token.0 == Token::Semicolon {
						statements.push(token);
						break;
					}

					statements.push(token);

					pos += 1;
				}

				nodes.push(self.parse(statements)?);
			} else if let Token::FunctionName(_) = token.0 {
				loop {
					let token = tokens.get(pos);

					if token.is_none() {
						break;
					}

					let token = token?;

					if token.0 == Token::Semicolon {
						statements.push(token);
						break;
					}

					statements.push(token);

					pos += 1;
				}

				nodes.push(self.parse(statements)?);
			} else if token.0 == Token::If {
				let mut depth = 0;
				loop {
					let token = tokens.get(pos);

					if token.is_none() {
						break;
					}

					let token = token?;

					if token.0 == Token::LCurly {
						depth += 1;
					}

					if token.0 == Token::RCurly {
						depth -= 1;
						if depth == 0 {
							statements.push(token);
							if tokens.get(pos + 1).is_some() && tokens.get(pos + 1)?.0 == Token::Else {
								pos += 1;
								statements.push(tokens.get(pos + 1)?);
								continue;
							}
							break;
						}
					}

					statements.push(token);

					pos += 1;
				}
				nodes.push(self.parse(statements)?);
			} else if token.0 == Token::Loop || token.0 == Token::Fn {
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
							break;
						}
					}
					if token.0 == Token::LCurly {
						depth += 1;
					}

					statements.push(token);

					pos += 1;
				}

				nodes.push(self.parse(statements)?);
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
					Token::Exit => loop {
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

	fn parse(&'a self, tokens: Vec<&'a (Token, usize)>) -> Option<(AstNode, Range<usize>)> {
		let mut stream = tokens.iter().peekable();

		let token = *stream.next()?;

		let node: (AstNode, Range<usize>) = if token.0 == Token::Let {
			let t = &stream.next()?;
			if let Token::Identifier(ident) = &t.0 {
				let t = stream.peek()?;
				let datatype = if let Token::DataType(datatype) = &t.0 {
					stream.next()?;
					datatype.clone()
				} else {
					error(
						self.name,
						self.file,
						"E0010",
						"expected data type",
						"expected data type",
						&(t.1 - 1..t.1 + t.0.len() - 1),
					)
				};

				self.check(stream.next().copied(), Token::Equal);

				let mut tokens = stream.collect::<Vec<_>>();

				let t = tokens.pop();

				self.check(t.copied(), Token::Semicolon);

				let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

				let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

				(
					AstNode::Assignment(ident.to_string(), datatype, expression),
					token.1..t?.1,
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
			if stream.peek()?.0 == Token::LSquare {
				stream.next()?;
				let mut tokens = stream.collect::<Vec<_>>();

				let t = tokens.pop();

				self.check(t.copied(), Token::Semicolon);

				let mut index_tokens = vec![];
				let mut expr_tokens = vec![];

				let mut flag = false;
				for token in tokens {
					if token.0 == Token::RSquare {
						flag = true;
						continue;
					}

					if !flag {
						index_tokens.push(*token);
					} else {
						expr_tokens.push(*token);
					}
				}

				let (index, _) = self.pratt_parser(index_tokens.into_iter().peekable(), 0);

				self.check(Some(expr_tokens.remove(0)), Token::Equal);

				let (expression, _) = self.pratt_parser(expr_tokens.into_iter().peekable(), 0);

				(
					AstNode::VecReAssignment(ident.to_string(), index, expression),
					token.1..t?.1,
				)
			} else {
				self.check(stream.next().copied(), Token::Equal);

				let mut tokens = stream.collect::<Vec<_>>();

				let t = tokens.pop();

				self.check(t.copied(), Token::Semicolon);

				let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

				let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

				(
					AstNode::ReAssignment(ident.to_string(), expression),
					token.1..t?.1,
				)
			}
		} else if token.0 == Token::If {
			let mut tokens = vec![];
			let mut then = vec![];
			let mut otherwise = None;
			let mut flag = false;

			for token in stream {
				if token.0 == Token::LCurly {
					flag = true;
					continue;
				} else if token.0 == Token::Else {
					otherwise = Some(vec![]);
				}
				if flag {
					if otherwise.is_some() {
						otherwise.as_mut()?.push((*token).to_owned());
					} else {
						then.push((*token).to_owned());
					}
				} else {
					tokens.push(token);
				}
			}

			let mut t = then.pop();

			self.check(t.as_ref(), Token::RCurly);

			let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

			let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

			if otherwise.is_some() {
				t = otherwise.as_mut()?.pop();
				self.check(Some(&otherwise.as_mut()?.remove(0)), Token::Else);
				self.check(t.as_ref(), Token::RCurly);
				(
					AstNode::IfElse(
						expression,
						self.match_tokens(then)?,
						self.match_tokens(otherwise?)?,
					),
					token.1..t?.1,
				)
			} else {
				(
					AstNode::If(expression, self.match_tokens(then)?),
					token.1..t?.1,
				)
			}
		} else if token.0 == Token::Loop {
			let mut statements = vec![];

			self.check(stream.next().copied(), Token::LCurly);

			for token in stream {
				statements.push((*token).to_owned());
			}

			let t = statements.pop();

			self.check(t.as_ref(), Token::RCurly);

			(AstNode::Loop(self.match_tokens(statements)?), token.1..t?.1)
		} else if let Token::FunctionName(ident) = &token.0 {
			self.check(stream.next().copied(), Token::LParen);

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

					if depth == 0 {
						if !expression.is_empty() {
							let (data, _) = self.pratt_parser(expression.clone().into_iter().peekable(), 0);

							params.push(data);

							expression.clear();
							continue;
						}
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
						if let Token::DataType(datatype) = &stream.next()?.0 {
							params.push(Param::new(name.to_string(), datatype.clone()));
						}
					}
				}

				let t = stream.next()?;
				let datatype = if let Token::DataType(datatype) = &t.0 {
					datatype
				} else {
					error(
						self.name,
						self.file,
						"E0010",
						"expected data type",
						"expected data type",
						&(t.1 - 1..t.1 + t.0.len() - 1),
					)
				};

				self.check(stream.next().copied(), Token::LCurly);

				let mut statements = vec![];

				loop {
					let token = stream.peek();

					if token.is_none() {
						break;
					}

					let token = stream.next()?;

					statements.push((*token).to_owned());
				}

				let t = statements.pop();

				self.check(t.as_ref(), Token::RCurly);

				(
					AstNode::FunctionDeclaration(
						name.to_string(),
						params,
						Some(datatype.clone()),
						self.match_tokens(statements)?,
					),
					token.1..t?.1,
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

			let t = tokens.pop();

			self.check(t.copied(), Token::Semicolon);

			let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

			let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

			(AstNode::Return(expression), token.1..t?.1)
		} else if token.0 == Token::Exit {
			let mut tokens = stream.collect::<Vec<_>>();

			let t = tokens.pop();

			self.check(t.copied(), Token::Semicolon);

			let tokens = tokens.iter().map(|f| **f).collect::<Vec<_>>();

			let (expression, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

			(AstNode::Exit(expression), token.1..t?.1)
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

	fn check(&self, t1: Option<&(Token, usize)>, t2: Token) -> bool {
		if t1.is_none() || t1.unwrap().0 != t2 {
			error(
				self.name,
				self.file,
				"0001",
				&format!(
					"expected `{}` found {}",
					t2.as_string(),
					t1.unwrap().0.as_string()
				),
				&format!("use `{}` here", t2.as_string()),
				&(t1.unwrap().1 - 1..t1.unwrap().1 + t2.len() - 1),
			);
		};

		true
	}

	fn pratt_parser(
		&'a self,
		mut lexer: Peekable<IntoIter<&'a (Token, usize)>>,
		prec: u16,
	) -> (Expression, Peekable<IntoIter<&'a (Token, usize)>>) {
		let token = &lexer.next().unwrap();
		let mut expr: Option<Expression> = None;

		match &token.0 {
			Token::Identifier(i) => {
				let t = lexer.peek();

				if t.is_some() && t.unwrap().0 == Token::LSquare {
					lexer.next().unwrap();
					let mut tokens = vec![];
					let mut depth = 1;

					loop {
						let t = lexer.next();

						if t.is_none() {
							break;
						}
						let t = t.unwrap();

						if t.0 == Token::LSquare {
							depth += 1;
						} else if t.0 == Token::RSquare {
							depth -= 1;
						}

						tokens.push(t);

						if depth == 0 {
							break;
						}
					}

					let t = tokens.pop();

					self.check(t, Token::RSquare);

					let (index, _) = self.pratt_parser(tokens.into_iter().peekable(), 0);

					expr = Some(Expression::VecIndex(i.to_string(), Box::new(index)));
				} else {
					expr = Some(Expression::Identifier(i.to_string()));
				}
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
			Token::LSquare => {
				let mut tokens = vec![];
				let mut depth = 1;

				loop {
					let t = lexer.next();

					if t.is_none() {
						break;
					}
					let t = t.unwrap();

					if t.0 == Token::LSquare {
						depth += 1;
					} else if t.0 == Token::RSquare {
						depth -= 1;
					}

					tokens.push(t);

					if depth == 0 {
						break;
					}
				}

				let mut params = vec![];
				let mut expression: Tokens = vec![];

				for token in tokens {
					if token.0 == Token::RSquare {
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
				let datatype = if params.first().is_some() {
					params.first().unwrap().infer_datatype()
				} else {
					None
				};
				expr = Some(Expression::Vector(params, datatype))
			}
			Token::Subtraction => {
				if let Token::Int(i) = token.0 {
					expr = Some(Expression::Int(-i));
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
				let mut expression: Tokens = vec![];

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
				if let Token::Int(i) = token.0 {
					expr = Some(Expression::Int(i));
				}
			}
		};

		loop {
			let op = lexer.peek();

			if op.is_none() || op.unwrap().0 == Token::RParen {
				lexer.next();
				break;
			}

			if op.unwrap().0 == Token::Power && self.infix_binding_power(op.unwrap()) < prec {
				break;
			}

			if op.unwrap().0 != Token::Power && self.infix_binding_power(op.unwrap()) <= prec {
				break;
			}
			let op = lexer.next().unwrap();
			let rhs;
			(rhs, lexer) = self.pratt_parser(lexer, self.infix_binding_power(op));
			expr = Some(Expression::BinaryOperation(
				Box::new(expr.unwrap()),
				op.0.clone(),
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

	fn infix_binding_power(&self, op: &(Token, usize)) -> u16 {
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

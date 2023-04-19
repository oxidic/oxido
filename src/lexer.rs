use crate::{error::error, token::Token};

pub struct Lexer<'a> {
    name: &'a str,
    file: &'a str,
    at: usize,
    tokens: Vec<(Token, usize)>,
}

impl<'a> Lexer<'a> {
    pub fn new(name: &'a str, file: &'a str) -> Self {
        Self {
            name,
            file,
            at: 0,
            tokens: vec![],
        }
    }

    pub fn run(&mut self) -> &Vec<(Token, usize)> {
        let file = self.file.split("").collect::<Vec<&'a str>>();

        let mut stream = file.iter().peekable();

        let size = self.file.len();
        loop {
            let mut token = String::new();

            let ch = stream.peek();
            self.at += 1;

            if ch.is_none() {
                if self.at > size {
                    break;
                }
                stream.next();
                continue;
            }

            let ch = if ch.unwrap().is_empty() {
                stream.next();
                continue;
            } else {
                stream.next().unwrap().chars().next().unwrap()
            };

            if ch.is_whitespace() {
                continue;
            }

            if ch.is_alphabetic() {
                token.push(ch);
                loop {
                    let ch = stream.peek();
                    self.at += 1;

                    if ch.is_none() {
                        if self.at > size {
                            break;
                        }
                        stream.next();
                        continue;
                    }

                    if !ch.unwrap().chars().next().unwrap().is_alphabetic() {
                        self.at -= 1;
                        break;
                    }

                    let ch = if ch.unwrap().is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next().unwrap().chars().next().unwrap()
                    };

                    token.push(ch);
                }
            } else if ch == '"' {
                loop {
                    let ch = stream.peek();
                    self.at += 1;

                    if ch.is_none() {
                        if self.at > size {
                            break;
                        }
                        stream.next();
                        continue;
                    }

                    if ch.unwrap().starts_with('\"') {
                        self.at -= 1;
                        stream.next();
                        break;
                    }

                    let ch = if ch.unwrap().is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next().unwrap().chars().next().unwrap()
                    };

                    token.push(ch);
                }
                let t = Token::Str(token);
                let size = t.size();
                self.tokens.push((t, self.at - size));
                continue;
            } else if ch.is_numeric() {
                token.push(ch);
                loop {
                    let ch = stream.peek();
                    self.at += 1;

                    if ch.is_none() {
                        if self.at > size {
                            break;
                        }
                        stream.next();
                        continue;
                    }

                    if !ch.unwrap().chars().next().unwrap().is_numeric() {
                        self.at -= 1;
                        break;
                    }

                    let ch = if ch.unwrap().is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next().unwrap().chars().next().unwrap()
                    };

                    token.push(ch);
                }
                let t = Token::Integer(token.parse::<i64>().unwrap());
                let size = t.size();
                self.tokens.push((t, self.at - size));
                continue;
            } else if ['+', '-', '*', '/', '^', '!', '=', '>', '<'].contains(&ch) {
                let t = match ch {
                    '+' => Token::Addition,
                    '-' => Token::Subtraction,
                    '*' => Token::Multiplication,
                    '/' => Token::Division,
                    '^' => Token::Power,
                    '!' => {
                        let ch = stream.peek();
                        self.at += 1;

                        if ch.is_none() {
                            if self.at > size {
                                break;
                            }
                            stream.next();
                            continue;
                        }

                        let ch = if ch.unwrap().is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek().unwrap().chars().next().unwrap()
                        };

                        if ch == '=' {
                            stream.next();
                            Token::IsNotEqual
                        } else {
                            self.at -= 1;
                            Token::Not
                        }
                    }
                    '=' => {
                        let ch = stream.peek();
                        self.at += 1;

                        if ch.is_none() {
                            if self.at > size {
                                break;
                            }
                            stream.next();
                            continue;
                        }

                        let ch = if ch.unwrap().is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek().unwrap().chars().next().unwrap()
                        };

                        if ch == '=' {
                            stream.next();
                            Token::IsEqual
                        } else {
                            self.at -= 1;
                            Token::Equal
                        }
                    }
                    '>' => {
                        let ch = stream.peek();
                        self.at += 1;

                        if ch.is_none() {
                            if self.at > size {
                                break;
                            }
                            stream.next();
                            continue;
                        }

                        let ch = if ch.unwrap().is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek().unwrap().chars().next().unwrap()
                        };

                        if ch == '=' {
                            stream.next();
                            Token::IsGreaterEqual
                        } else {
                            self.at -= 1;
                            Token::IsGreater
                        }
                    }
                    '<' => {
                        let ch = stream.peek();
                        self.at += 1;

                        if ch.is_none() {
                            if self.at > size {
                                break;
                            }
                            stream.next();
                            continue;
                        }

                        let ch = if ch.unwrap().is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek().unwrap().chars().next().unwrap()
                        };

                        if ch == '=' {
                            stream.next();
                            Token::IsLesserEqual
                        } else {
                            self.at -= 1;
                            Token::IsLesser
                        }
                    }
                    _ => unimplemented!(),
                };
                let size = t.size();
                self.tokens.push((t, self.at - size))
            } else {
                let t = match ch {
                    ';' => Token::Semicolon,
                    ',' => Token::Comma,
                    ')' => Token::RParen,
                    '(' => Token::LParen,
                    '}' => Token::RCurly,
                    '{' => Token::LCurly,
                    _ => {
                        self.at -= 2;
                        error(
                            self.name,
                            self.file,
                            "0001",
                            &format!("character `{ch}` was not expected here"),
                            &format!("character `{ch}` was not expected here"),
                            self.at..self.at + 1,
                        )
                    }
                };
                let size = t.size();
                self.tokens.push((t, self.at - size))
            }

            if !token.is_empty() {
                let t = match token.as_str() {
                    "let" => Token::Let,
                    "if" => Token::If,
                    "loop" => Token::Loop,
                    "fn" => Token::Fn,
                    "exit" => Token::Exit,
                    "break" => Token::Break,
                    "return" => Token::Return,
                    "true" => Token::Bool(true),
                    "false" => Token::Bool(false),
                    _ => {
                        if self.tokens.last().is_some()
                            && self.tokens.last().unwrap().0 == Token::Fn
                        {
                            Token::FunctionName(token)
                        } else {
                            let mut cloned_stream = stream.clone();
                            if cloned_stream.peek().is_some()
                                && cloned_stream.next().unwrap().starts_with('(')
                            {
                                Token::FunctionCall(token)
                            } else {
                                Token::Identifier(token)
                            }
                        }
                    }
                };
                let size = t.size();
                self.tokens.push((t, self.at - size))
            }
        }
        &self.tokens
    }
}

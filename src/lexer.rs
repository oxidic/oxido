use crate::token::Token;

pub struct Lexer<'a> {
    file: &'a str,
    at: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a str) -> Self {
        Self {
            file,
            at: 0,
            tokens: vec![],
        }
    }

    pub fn run(&mut self) -> &Vec<Token> {
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

                    let ch = if ch.unwrap().is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next().unwrap().chars().next().unwrap()
                    };

                    if !ch.is_alphabetic() {
                        self.at -= 1;
                        break;
                    }

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
            } else if ['+', '-', '*', '/', '^', '!', '=', '>', '<'].contains(&ch) {
                self.tokens.push(match ch {
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
                            Token::IsLesser
                        }
                    }
                    _ => unimplemented!(),
                })
            } else {
                self.tokens.push(match ch {
                    ';' => Token::Semicolon,
                    ',' => Token::Comma,
                    ')' => Token::RParen,
                    '(' => Token::LParen,
                    '}' => Token::RCurly,
                    '{' => Token::LCurly,
                    _ => unimplemented!("token: {ch}"),
                })
            }

            if !token.is_empty() {
                self.tokens.push(match token.as_str() {
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
                        if token.starts_with('"') {
                            Token::String(token)
                        } else if token
                            .split("")
                            .map(|f| f.chars().next().unwrap_or(' ').is_numeric())
                            .any(|x| x)
                        {
                            Token::Integer(token.parse::<i64>().unwrap())
                        } else if self.tokens.last().unwrap() == &Token::Fn {
                            Token::FunctionName(token)
                        } else {
                            Token::Identifier(token)
                        }
                    }
                })
            }
        }
        &self.tokens
    }
}

use crate::{
    data::DataType,
    error::error,
    token::{Token, Tokens},
};

#[derive(Debug, Clone)]
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

    pub fn run(&mut self) -> Option<&Tokens> {
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

            let ch = if ch?.is_empty() {
                stream.next();
                continue;
            } else {
                stream.next()?.chars().next()?
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

                    if !ch?.chars().next()?.is_alphabetic() {
                        self.at -= 1;
                        break;
                    }

                    let ch = if ch?.is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next()?.chars().next()?
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

                    if ch?.starts_with('\"') {
                        self.at -= 1;
                        stream.next();
                        break;
                    }

                    let ch = if ch?.is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next()?.chars().next()?
                    };

                    token.push(ch);
                }
                let t = Token::Str(token);
                let size = t.len();
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

                    if !ch?.chars().next()?.is_numeric() {
                        self.at -= 1;
                        break;
                    }

                    let ch = if ch?.is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next()?.chars().next()?
                    };

                    token.push(ch);
                }
                let t = Token::Int(token.parse::<i64>().unwrap());
                let size = t.len();
                self.tokens.push((t, self.at - size));
                continue;
            } else if ch == ':' {
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
                    let char = ch?.chars().next()?;

                    if char.is_whitespace() {
                        stream.next();
                        continue;
                    }

                    if !char.is_alphabetic() && char != '<' && char != '>' {
                        self.at -= 1;
                        break;
                    }

                    let ch = if ch?.is_empty() {
                        stream.next();
                        continue;
                    } else {
                        stream.next()?.chars().next()?
                    };

                    token.push(ch);
                }

                fn match_str(token: &str, name: &str, file: &str, at: usize) -> DataType {
                    match token {
                        "str" => DataType::Str,
                        "int" => DataType::Int,
                        "bool" => DataType::Bool,
                        t => {
                            if t.starts_with("vec") {
                                let chars = t.chars();
                                let mut chars = chars.skip(3);
                                let ch = chars.next();

                                if ch.unwrap() != '<' {
                                    error(
                                        name,
                                        file,
                                        "0001",
                                        &format!("expected `<` found `{t}`"),
                                        &format!("token `{t}` was not expected here"),
                                        &(at..at + 1),
                                    )
                                }

                                let mut chars = chars.collect::<String>();

                                if chars.pop().unwrap() != '>' {
                                    error(
                                        name,
                                        file,
                                        "0001",
                                        &format!("expected `<` found `{t}`"),
                                        &format!("token `{t}` was not expected here"),
                                        &(at..at + 1),
                                    )
                                }

                                return DataType::Vector(Box::new(match_str(
                                    &chars, name, file, at,
                                )));
                            }
                            error(
                                name,
                                file,
                                "0001",
                                &format!("expected datatype found `{t}`"),
                                &format!("token `{t}` was not expected here"),
                                &(at..at + 1),
                            )
                        }
                    }
                }

                let t = Token::DataType(match_str(&token, self.name, self.file, self.at));

                let size = t.len();

                self.tokens.push((t, self.at - size));

                continue;
            } else if ['+', '-', '*', '/', '^', '!', '=', '>', '<'].contains(&ch) {
                let t = match ch {
                    '+' => Token::Addition,
                    '-' => {
                        let ch = stream.peek();
                        self.at += 1;

                        if ch.is_none() {
                            if self.at > size {
                                break;
                            }
                            stream.next();
                            continue;
                        }

                        let ch = if ch?.is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek()?.chars().next()?
                        };

                        if ch == '>' {
                            stream.next();
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

                                if ch?.chars().next()?.is_whitespace() {
                                    stream.next();
                                    continue;
                                }

                                if !ch?.chars().next()?.is_alphabetic() {
                                    self.at -= 1;
                                    break;
                                }

                                let ch = if ch?.is_empty() {
                                    stream.next();
                                    continue;
                                } else {
                                    stream.next()?.chars().next()?
                                };

                                token.push(ch);
                            }

                            match token.as_str() {
                                "str" => Token::DataType(DataType::Str),
                                "int" => Token::DataType(DataType::Int),
                                "bool" => Token::DataType(DataType::Bool),
                                _ => error(
                                    self.name,
                                    self.file,
                                    "0001",
                                    &format!("expected datatype found `{ch}`"),
                                    &format!("token `{ch}` was not expected here"),
                                    &(self.at..self.at + 1),
                                ),
                            }
                        } else {
                            self.at -= 1;
                            Token::Subtraction
                        }
                    }
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

                        let ch = if ch?.is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek()?.chars().next()?
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

                        let ch = if ch?.is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek()?.chars().next()?
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

                        let ch = if ch?.is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek()?.chars().next()?
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

                        let ch = if ch?.is_empty() {
                            stream.next();
                            continue;
                        } else {
                            stream.peek()?.chars().next()?
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
                let size = t.len();
                self.tokens.push((t, self.at - size));
                continue;
            } else {
                let t = match ch {
                    ';' => Token::Semicolon,
                    ',' => Token::Comma,
                    ')' => Token::RParen,
                    '(' => Token::LParen,
                    '}' => Token::RCurly,
                    '{' => Token::LCurly,
                    ']' => Token::RSquare,
                    '[' => Token::LSquare,
                    _ => {
                        self.at -= 2;
                        error(
                            self.name,
                            self.file,
                            "0001",
                            &format!("character `{ch}` was not expected here"),
                            &format!("character `{ch}` was not expected here"),
                            &(self.at..self.at + 1),
                        )
                    }
                };
                let size = t.len();
                self.tokens.push((t, self.at - size))
            }

            if !token.is_empty() {
                let t = match token.as_str() {
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "loop" => Token::Loop,
                    "fn" => Token::Fn,
                    "exit" => Token::Exit,
                    "break" => Token::Break,
                    "return" => Token::Return,
                    "true" => Token::Bool(true),
                    "false" => Token::Bool(false),
                    _ => {
                        if self.tokens.last().is_some() && self.tokens.last()?.0 == Token::Fn {
                            Token::FunctionName(token)
                        } else {
                            let mut cloned_stream = stream.clone();
                            if cloned_stream.peek().is_some()
                                && cloned_stream.next()?.starts_with('(')
                            {
                                Token::FunctionName(token)
                            } else {
                                Token::Identifier(token)
                            }
                        }
                    }
                };
                let size = t.len();
                self.tokens.push((t, self.at - size))
            }
        }
        Some(&self.tokens)
    }
}

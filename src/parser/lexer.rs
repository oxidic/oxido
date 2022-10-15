use crate::token::Token;

pub struct Lexer {
    pub lines: Vec<String>,
    pub tokens: Vec<Vec<Token>>,
    pub last_token: Token,
    pub debug: bool,
}

impl Lexer {
    pub fn new(raw: String, debug: bool) -> Self {
        Self {
            lines: raw.lines().map(String::from).collect(),
            tokens: vec![],
            last_token: Token::Semicolon,
            debug,
        }
    }

    pub fn get_token(&mut self, line: String) -> Vec<Token> {
        let mut chars = line.chars().peekable();
        let mut tokens = vec![];

        loop {
            let peeked = chars.peek();
            let mut next = if peeked == None {
                break;
            } else {
                peeked.unwrap()
            };
            let mut current_token = String::new();
            if next.is_whitespace() {
                loop {
                    next = chars.peek().unwrap();
                    if !next.is_whitespace() {
                        break;
                    }
                    chars.next();
                }
                continue;
            } else if next == &'#' {
                loop {
                    next = chars.peek().unwrap();
                    if next == &'\n' {
                        break;
                    }
                    chars.next();
                }
                continue;
            } else if next.is_alphabetic() {
                loop {
                    let peeked = chars.peek();
                    next = if peeked == None {
                        break;
                    } else {
                        peeked.unwrap()
                    };

                    if !next.is_alphanumeric() {
                        if next == &'(' {
                            current_token.push('(');
                        }
                        break;
                    }
                    current_token.push(*next);
                    chars.next();
                }
                tokens.push(match current_token.as_str() {
                    "let" => Token::Let,
                    "if" => Token::If,
                    "then" => Token::Then,
                    "loop" => Token::Loop,
                    "fn" => Token::Fn,
                    "exit" => Token::Exit,
                    "break" => Token::Break,
                    "return" => Token::Return,
                    "true" | "false" => Token::Bool(current_token.parse::<bool>().unwrap()),
                    _ => {
                        if self.last_token == Token::Fn {
                            Token::FunctionName(current_token)
                        } else if let Token::FunctionName(name) = &self.last_token {
                            Token::FunctionParameter(name.clone(), current_token)
                        } else if current_token.ends_with('(') {
                            current_token.pop();
                            Token::FunctionName(current_token)
                        } else {
                            Token::Identifier(current_token)
                        }
                    }
                });
            } else if next == &'"' {
                chars.next();
                loop {
                    next = chars.peek().unwrap();
                    let to_add: char = if next == &'\\' {
                        chars.next();
                        next = chars.peek().unwrap();
                        match next {
                            't' => '\t',
                            'b' => '\x08',
                            'n' => '\n',
                            'r' => '\r',
                            'f' => '\x0c',
                            '"' => '"',
                            '\\' => '\\',
                            _ => panic!("unknown escaped character"),
                        }
                    } else if next == &'"' {
                        chars.next();
                        break;
                    } else {
                        *next
                    };
                    chars.next();
                    current_token.push(to_add);
                }
                tokens.push(Token::String(current_token));
            } else if next.is_digit(10) {
                loop {
                    let peeked = chars.peek();
                    next = if peeked == None {
                        break;
                    } else {
                        peeked.unwrap()
                    };
                    if !(next.is_digit(10) || next == &'.') {
                        break;
                    }
                    if next == &'.' && current_token.contains('.') {
                        panic!("multiple decimal points in number");
                    }
                    current_token.push(*next);
                    chars.next();
                }
                if current_token.contains('.') {
                    tokens.push(Token::Float(
                        current_token
                            .parse::<f64>()
                            .expect("error reading float literal"),
                    ));
                } else {
                    tokens.push(Token::Integer(
                        current_token
                            .parse::<i64>()
                            .expect("error reading integer literal"),
                    ));
                }
            } else if next == &'/' {
                chars.next();
                tokens.push(Token::Division);
            } else if next == &'!' {
                chars.next();
                next = chars.peek().unwrap();
                if next == &'=' {
                    tokens.push(Token::IsNotEqual);
                } else {
                    panic!("unknown character");
                }
            } else if next == &'=' {
                chars.next();
                let peeked = chars.peek();
                next = if peeked == None {
                    tokens.push(Token::Equal);
                    continue;
                } else {
                    peeked.unwrap()
                };
                if next == &'=' {
                    chars.next();
                    tokens.push(Token::IsEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            } else if next == &'<' {
                chars.next();
                let peeked = chars.peek();
                next = if peeked == None {
                    tokens.push(Token::IsLesser);
                    continue;
                } else {
                    peeked.unwrap()
                };
                if next == &'=' {
                    chars.next();
                    tokens.push(Token::IsLesserEqual);
                } else {
                    tokens.push(Token::IsLesser);
                }
            } else if next == &'>' {
                chars.next();
                let peeked = chars.peek();
                next = if peeked == None {
                    tokens.push(Token::IsGreater);
                    continue;
                } else {
                    peeked.unwrap()
                };
                if next == &'=' {
                    chars.next();
                    tokens.push(Token::IsGreaterEqual);
                } else {
                    tokens.push(Token::IsGreater);
                }
            } else {
                let token = match next {
                    '{' => Token::LCurly,
                    '(' => Token::LParen,
                    '}' => Token::RCurly,
                    ')' => Token::RParen,
                    ';' => Token::Semicolon,
                    '.' => Token::Get,
                    ',' => Token::Comma,
                    '+' => Token::Addition,
                    '-' => Token::Subtraction,
                    '*' => Token::Multiplication,
                    '/' => Token::Division,
                    '^' => Token::Power,
                    _ => panic!("unknown character"),
                };
                chars.next();
                tokens.push(token);
            }
            self.last_token = tokens.last().unwrap().clone();
        }
        tokens
    }

    pub fn tokenize(&mut self) -> Vec<Vec<Token>> {
        for line in self.lines.clone() {
            let tokens = self.get_token(line);
            self.tokens.push(tokens);
        }
        if self.debug {
            println!("T: {:?}", self.tokens);
        }
        self.tokens.clone()
    }
}

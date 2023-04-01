use crate::token::Token;

pub struct Lexer {
    pub tokens: Vec<Vec<Token>>,
    pub input: String,
    pub ch: char,
    pub position: usize,
    pub read_position: usize,
}

impl Lexer {
    pub fn new(content: String) -> Lexer {
        Lexer {
            tokens: vec![],
            input: content,
            ch: '0',
            position: 0,
            read_position: 0,
        }
    }

    pub fn lexer(&mut self) {
        for _ in self.input.lines() {
            self.tokens.push(self.next_token());
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().collect::<Vec<char>>()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && Lexer::is_letter(l.ch) {
                l.read_char();
            }
            l.input.chars().collect::<Vec<char>>()[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && Lexer::is_digit(l.ch) {
                l.read_char();
            }
            l.input.chars().collect::<Vec<char>>()[position..l.position].to_vec()
        };

        self.skip_whitespace();
        
        let token: Token = match self.ch {
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
            _ => {
                if Lexer::is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match Token::is_keyword(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        }
                        Err(_err) => {
                            return Token::Identifier(ident.iter().collect::<String>());
                        }
                    }
                } else if Lexer::is_digit(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return Token::Integer(ident.iter().collect::<String>().parse::<i64>().unwrap());
                } else {
                    return Token::Error;
                }
            }
        };
        self.read_char();
        token
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }
}

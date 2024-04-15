use crate::types::{Token};

#[derive(Debug, PartialEq)]

pub struct Lexer <'a> {
    code: &'a str,
    pos: usize
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Lexer {
            code,
            pos: 0
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            match c {
                ' ' | '\t' | '\n' => {
                    self.pos += 1;
                }
                'a'..='z' | 'A'..='Z' => {
                    let token = self.read_identifier();
                    return token;
                }
                '0'..='9' => {
                    let token = self.read_number();
                    return token;
                }
                '\'' => {
                    let token = self.read_string_literal();
                    return token;
                }
                '=' => {
                    self.pos += 1;
                    return Some(Token::Equals);
                }
                '+' => {
                    self.pos += 1;
                    return Some(Token::Addition);
                }
                '-' => {
                    self.pos += 1;
                    return Some(Token::Subtraction);
                }
                '*' => {
                    self.pos += 1;
                    return Some(Token::Multiplication);
                }
                ';' => {
                    self.pos += 1;
                    return Some(Token::Semicolon);
                }
                '(' => {
                    self.pos += 1;
                    return Some(Token::ParenOpen);
                }
                ')' => {
                    self.pos += 1;
                    return Some(Token::ParenClose);
                }
                '/' =>{
                    let next_char = self.code.chars().nth(self.pos + 1)?;
                    let token = match next_char {
                        '/' => self.read_comment(),
                        _ => {
                            self.pos += 1;
                            return Some(Token::Division);
                        },
                        _ => panic!("Expected division"),
                    };

                    return token;
                }
                _ => {
                    self.pos += 1;
                }
            }
        }

        None
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let start = self.pos;
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            if !c.is_alphabetic() {
                break;
            }

            self.pos += 1;
        }

        let ident = &self.code[start..self.pos];

        match ident {
            "let" => Some(Token::Let),
            "log" => Some(Token::Log),
            _ => Some(Token::Identifier(ident.to_string())),
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let start = self.pos;
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            if !c.is_numeric() {
                break;
            }

            self.pos += 1;
        }

        let num = &self.code[start..self.pos];
        Some(Token::Number(num.parse().unwrap()))
    }

    fn read_string_literal(&mut self) -> Option<Token> {
        let start: usize = self.pos;

        // Skip the opening quote
        self.pos += 1;

        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            if c == '\'' {
                break;
            }

            self.pos += 1;
        }

        let literal = &self.code[start + 1..self.pos];

        // Skip the closing quote
        self.pos += 1;

        Some(Token::StringLiteral(literal.to_string()))
    }

    fn read_comment(&mut self) -> Option<Token> {
        let start = self.pos;
        self.pos += 1;

        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            if c == '\n' {
                break;
            }

            self.pos += 1;
        }

        let comment = &self.code[start..self.pos];
        Some(Token::Comment(comment.to_string()))
    }
}
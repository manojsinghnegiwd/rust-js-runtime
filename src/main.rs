mod parser;
mod types;
mod interpreter;

use types::Token;
use parser::Parser;
use interpreter::Interpreter;

#[derive(Debug, PartialEq)]

struct Lexer <'a> {
    code: &'a str,
    pos: usize
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        Lexer {
            code,
            pos: 0
        }
    }

    fn next_token(&mut self) -> Option<Token> {
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
                '=' => {
                    self.pos += 1;
                    return Some(Token::Equals);
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
}

fn main() {
    let code = "let x = 3; log(x);";
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:?}", ast);

    let mut interpreter = Interpreter::new();
    interpreter.eval(ast);
}
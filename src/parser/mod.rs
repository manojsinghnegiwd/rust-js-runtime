use crate::types::{Expr, Stmt, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                Token::Let => stmts.push(self.parse_let()),
                Token::Log => stmts.push(self.parse_log()),
                _ => (),
            }
        }

        stmts
    }

    fn parse_let (&mut self) -> Stmt {
        let name = match self.next_token() {
            Some(Token::Identifier(name)) => name,
            _ => panic!("Expected identifier after let")
        };

        let value = match self.next_token() {
            Some(Token::Equals) => self.parse_expr(),
            _ => panic!("Expected equals after identifier")
        };

        Stmt::Let(name, value)
    }

    fn parse_expr(&mut self) -> Expr {
        match self.next_token() {
            Some(Token::Number(num)) => Expr::Number(num),
            Some(Token::Identifier(name)) => Expr::Identifier(name),
            Some(Token::StringLiteral(literal)) => Expr::StringLiteral(literal),
            _ => panic!("Expected number or identifier")
        }
    }

    fn parse_log(&mut self) -> Stmt {
        match self.next_token() {
            Some(Token::ParenOpen) => {
                let expr = self.parse_expr();

                match self.next_token() {
                    Some(Token::ParenClose) => Stmt::Log(expr),
                    _ => panic!("Expected closing parenthesis")
                }
            }
            _ => panic!("Expected opening parenthesis")
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        println!("Token: {:?}", token);
        self.pos += 1;
        token
    }
}
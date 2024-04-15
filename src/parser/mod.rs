use crate::types::{Expr, Stmt, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                Token::Let => stmts.push(self.parse_let()),
                Token::Log => stmts.push(self.parse_log()),
                Token::Comment(comment) => stmts.push(Stmt::Comment(comment)),
                _ => (),
            }
        }

        stmts
    }

    fn parse_let(&mut self) -> Stmt {
        let name = match self.next_token() {
            Some(Token::Identifier(name)) => name,
            _ => panic!("Expected identifier after let"),
        };

        let value = match self.next_token() {
            Some(Token::Equals) => self.parse_expr(),
            _ => panic!("Expected equals after identifier"),
        };

        Stmt::Let(name, value)
    }

    fn parse_expr(&mut self) -> Expr {
        let mut expr = Vec::new();

        while let token = self.next_token() {
            if token == Some(Token::Semicolon) || token == Some(Token::ParenClose) {
                break;
            }

            let current_token_expr = match token {
                Some(Token::Number(num)) => Expr::Number(num),
                Some(Token::Identifier(name)) => Expr::Identifier(name),
                Some(Token::StringLiteral(literal)) => Expr::StringLiteral(literal),
                Some(Token::Addition) => {
                    let left = expr.pop().expect("Expected left side of addition");
                    let right = self.parse_expr();

                    return Expr::Addition(Box::new(left), Box::new(right))
                },
                Some(Token::Subtraction) => {
                    let left = expr.pop().expect("Expected left side of subtraction");
                    let right = self.parse_expr();

                    return Expr::Subtraction(Box::new(left), Box::new(right))
                }
                ,
                Some(Token::Multiplication) => {
                    let left = expr.pop().expect("Expected left side of multiplication");
                    let right = self.parse_expr();

                    return Expr::Multiplication(Box::new(left), Box::new(right))
                }
                ,
                Some(Token::Division) => {
                    let left = expr.pop().expect("Expected left side of division");
                    let right = self.parse_expr();

                    return Expr::Division(Box::new(left), Box::new(right))
                }
                _ => panic!("Expected number or identifier"),
            };

            expr.push(current_token_expr)
        }

        // If we reached here, it means we have a single expression
        // roll back the position for semicolon and closing parenthesis
        self.pos -= 1;

        return expr.pop().expect("Expected expression");
    }

    fn parse_log(&mut self) -> Stmt {
        match self.next_token() {
            Some(Token::ParenOpen) => {
                let expr = self.parse_expr();

                match self.next_token() {
                    Some(Token::ParenClose) => Stmt::Log(expr),
                    _ => panic!("Expected closing parenthesis"),
                }
            }
            _ => panic!("Expected opening parenthesis"),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }
}

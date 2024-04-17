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
                Token::If => stmts.push(self.parse_if()),
                Token::Comment(comment) => stmts.push(Stmt::Comment(comment)),
                Token::Identifier(name) => stmts.push(Stmt::Assignment(name, self.parse_assignment())),
                _ => (),
            }
        }

        stmts
    }

    fn parse_assignment (&mut self) -> Expr {
        let _token = self.tokens.get(self.pos);
        match self.next_token() {
            Some(Token::Assign) => self.parse_expr(),
            _ => panic!("Expected equals after identifier in assignment"),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        let name = match self.next_token() {
            Some(Token::Identifier(name)) => name,
            _ => panic!("Expected identifier after let"),
        };

        let value = match self.next_token() {
            Some(Token::Assign) => self.parse_expr(),
            _ => panic!("Expected equals after identifier"),
        };

        Stmt::Let(name, value)
    }

    fn parse_expr(&mut self) -> Expr {
        let mut expr = Vec::new();

        loop {
            let token = self.next_token();
            if token == Some(Token::Semicolon) || token == Some(Token::ParenClose) {
                break;
            }

            let current_token_expr = match token {
                Some(Token::Float(num)) => Expr::Float(num),
                Some(Token::Identifier(name)) => Expr::Identifier(name),
                Some(Token::StringLiteral(literal)) => Expr::StringLiteral(literal),
                Some(Token::Boolean(bool)) => Expr::Boolean(bool),
                Some(Token::Equals) => {
                    let left = expr.pop().expect("Expected left side of equals");
                    let right = self.parse_expr();

                    return Expr::Equals(Box::new(left), Box::new(right));
                }
                Some(Token::TypeCheckEquals) => {
                    let left = expr.pop().expect("Expected left side of equals");
                    let right = self.parse_expr();

                    return Expr::TypeCheckEquals(Box::new(left), Box::new(right));
                }
                Some(Token::NotEquals) => {
                    let left = expr.pop().expect("Expected left side of not equals");
                    let right = self.parse_expr();

                    return Expr::NotEquals(Box::new(left), Box::new(right));
                }
                Some(Token::TypeNotEquals) => {
                    let left = expr.pop().expect("Expected left side of not equals");
                    let right = self.parse_expr();

                    return Expr::TypeNotEquals(Box::new(left), Box::new(right));
                }
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
                },
                Some(Token::Division) => {
                    let left = expr.pop().expect("Expected left side of division");
                    let right = self.parse_expr();

                    return Expr::Division(Box::new(left), Box::new(right))
                }
                _ => panic!("Expected Float or identifier"),
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

    fn parse_if(&mut self) -> Stmt {
        let expr = match self.next_token() {
            Some(Token::ParenOpen) => {
                let condition = self.parse_expr();

                // skip closing parenthesis
                self.pos += 1;

                let _token = self.tokens.get(self.pos);

                match self.next_token() {
                    Some(Token::BraceOpen) => {
                        let if_ast = self.parse_scope();

                        let next_token = self.next_token();

                        match next_token {
                            Some(Token::ElseIf) => {
                                let else_if_stmt = self.parse_if();
                                Expr::If(Box::new(condition), if_ast, Box::new(else_if_stmt))
                            }
                            Some(Token::Else) => {
                                // 
                                self.pos += 1;

                                let else_ast = self.parse_scope();

                                let _token = self.tokens.get(self.pos);

                                Expr::If(
                                    Box::new(condition),
                                    if_ast,
                                    Box::new(
                                        Stmt::If(
                                            Box::new(
                                                Expr::Boolean(true)
                                            ),
                                            else_ast,
                                            Box::new(
                                                Stmt::Comment(
                                                    "No else".to_string()
                                                )
                                            )
                                        )
                                    )
                                )
                            }
                            _ => {
                                self.pos -= 1;
                                Expr::If(Box::new(condition), if_ast, Box::new(Stmt::Comment("No else".to_string())))
                            },
                        }
                    },
                    _ => panic!("Expected opening brace"),
                }
            },
            _ => panic!("Expected opening parenthesis"),
        };

        match expr {
            Expr::If(condition, stmts, else_stmt) => Stmt::If(condition, stmts, else_stmt),
            _ => panic!("Expected if"),
        }
    }

    fn parse_scope (&mut self) -> Vec<Stmt> {
        let mut scope_tokens = Vec::new();

        while let Some(token) = self.next_token() {
            if token == Token::BraceClose {
                break;
            } 

            scope_tokens.push(token);
        }

        let mut parser: Parser = Parser::new(scope_tokens);
        let ast = parser.parse();

        ast
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }
}
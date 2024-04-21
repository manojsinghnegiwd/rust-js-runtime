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
                Token::Comment(_) => (),
                Token::BraceOpen => stmts.push(Stmt::CodeBlock(self.parse_scope())),
                Token::Function => stmts.push(self.parse_function()),
                Token::Identifier(name) => stmts.push(self.parse_identifier(name)),
                Token::Return => {
                    stmts.push(self.parse_return());
                    break;
                },
                _ => (),
            }
        }

        stmts
    }

    fn get_tokens(&mut self) -> Vec<Token> {
        let mut scope_tokens = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                Token::BraceClose => {
                    scope_tokens.push(token);
                    break;
                },
                Token::BraceOpen => {
                    scope_tokens.push(token);
                    scope_tokens.append(&mut self.get_tokens());
                }
                _ => scope_tokens.push(token),
            }
        }

        scope_tokens
    }

    fn parse_scope (&mut self) -> Vec<Stmt> {
        let scope_tokens = self.get_tokens();

        let mut parser: Parser = Parser::new(scope_tokens);
        let ast = parser.parse();

        ast
    }

    fn parse_return (&mut self) -> Stmt {
        let expr_to_return = self.parse_expr();
        Stmt::Return(Box::new(expr_to_return))
    }

    fn parse_function (&mut self) -> Stmt {
        let mut args: Vec<String> = Vec::new();
        
        let function_name = match self.next_token() {
            Some(Token::Identifier(name)) => name,
            _ => panic!("Expected a function name")
        };

        if self.next_token() != Some(Token::ParenOpen) {
            panic!("Expected opening paranthesis after function name {}", function_name)
        }

        while let Some(token) = self.next_token() {
            match token {
                Token::ParenClose => break,
                Token::Identifier(name) => {
                    args.push(name)
                }
                Token::Comma => (),
                _ => panic!("Unexpected token in function defination")
            }
        }

        if self.next_token() != Some(Token::BraceOpen) {
            panic!("Expected opening braces after function arguments {}", function_name)
        }

        let mut stmts = Vec::new();

        stmts.append(&mut self.parse_scope());

        Stmt::Function(function_name, args, Box::new(Stmt::CodeBlock(stmts)))
    }

    fn parse_function_call(&mut self, name: String) -> Expr {
        let mut args = Vec::new();
        while let Some(token) = self.next_token() {
            match token {
                Token::ParenClose => break,
                Token::Identifier(name) => {
                    args.push(Expr::Identifier(name))
                },
                Token::StringLiteral(literal) => {
                    args.push(Expr::StringLiteral(literal))
                },
                Token::Float(num) => {
                    args.push(Expr::Float(num))
                },
                Token::Boolean(bool) => {
                    args.push(Expr::Boolean(bool))
                },
                Token::Comma => (),
                _ => panic!("Unexpected token in function call")
            }
        }

        Expr::FunctionCall(name, args)
    }

    fn parse_identifier (&mut self, name: String) -> Stmt {
        match self.next_token() {
            Some(Token::Assign) => Stmt::Assignment(name, self.parse_expr()),
            Some(Token::ParenOpen) => {
                let expr = self.parse_function_call(name);

                println!("{:?}, {:?}", expr, self.pos);

                match expr {
                    Expr::FunctionCall(name, args) => Stmt::FunctionCall(name, args),
                    _ => panic!("Expected function call expression")
                }
            },
            Some(Token::Addition) => {
                match self.next_token() {
                    Some(Token::Addition) => {
                        let name_clone = name.clone();
                        Stmt::Assignment(name, Expr::Addition(Box::new(Expr::Identifier(name_clone)), Box::new(Expr::Float(1.0))))
                    }
                    _ => panic!("Expected increment operator")
                }
            },
            Some(Token::Subtraction) => {
                match self.next_token() {
                    Some(Token::Subtraction) => {
                        let name_clone = name.clone();
                        Stmt::Assignment(name, Expr::Subtraction(Box::new(Expr::Identifier(name_clone)), Box::new(Expr::Float(1.0))))
                    }
                    _ => panic!("Expected increment operator")
                }
            },
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
                                Expr::ControlFlow(Box::new(condition), Box::new(Stmt::CodeBlock(if_ast)), Box::new(else_if_stmt))
                            }
                            Some(Token::Else) => {
                                // 
                                self.pos += 1;

                                let else_ast = self.parse_scope();

                                let _token = self.tokens.get(self.pos);

                                Expr::ControlFlow(
                                    Box::new(condition),
                                    Box::new(Stmt::CodeBlock(if_ast)),
                                    Box::new(
                                        Stmt::ControlFlow(
                                            Box::new(
                                                Expr::Boolean(true)
                                            ),
                                            Box::new(Stmt::CodeBlock(else_ast)),
                                            Box::new(Stmt::None)
                                        )
                                    )
                                )
                            }
                            _ => {
                                self.pos -= 1;
                                Expr::ControlFlow(Box::new(condition), Box::new(Stmt::CodeBlock(if_ast)), Box::new(Stmt::None))
                            },
                        }
                    },
                    _ => panic!("Expected opening brace"),
                }
            },
            _ => panic!("Expected opening parenthesis"),
        };

        match expr {
            Expr::ControlFlow(condition, stmts, else_stmt) => Stmt::ControlFlow(condition, stmts, else_stmt),
            _ => panic!("Expected if"),
        }
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
                Some(Token::Identifier(name)) => {
                    let token = self.next_token();

                    match token {
                        Some(Token::ParenOpen) => self.parse_function_call(name),
                        Some(Token::Addition) => {
                            match self.next_token() {
                                Some(Token::Addition) => {
                                    Expr::Addition(Box::new(Expr::Identifier(name)), Box::new(Expr::Float(1.0)))
                                }
                                _ => {
                                    // roll back to previous position
                                    self.pos -= 2;
                                    Expr::Identifier(name)
                                }
                            }
                        },
                        _ => {
                            // roll back to previous position
                            self.pos -= 1;
                            Expr::Identifier(name)
                        }
                    }
                },
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

    fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }
}
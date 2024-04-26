use std::ptr::null;

use crate::types::{Expr, Stmt, Token};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                Token::Let => stmts.push(self.parse_let()),
                Token::Log => stmts.push(self.parse_log()),
                Token::If => stmts.push(self.parse_if()),
                Token::Loop => stmts.push(self.parse_loop()),
                Token::While => stmts.push(self.parse_while()),
                Token::Comment(_) => (),
                Token::BraceOpen => stmts.push(Stmt::CodeBlock(self.parse_scope())),
                Token::Function => stmts.push(self.parse_function()),
                Token::ForLoop => stmts.push(self.parse_for_loop()),
                Token::Return => {
                    stmts.push(self.parse_return());
                    break;
                },
                Token::Semicolon => (),
                Token::Comma => (),
                Token::BraceClose => (),
                Token::Break => stmts.push(Stmt::Break),
                _ => {
                    // roll back the position
                    // because we are not consuming the token
                    self.pos -= 1;

                    let expr = self.parse_expr();

                    stmts.push(
                        match expr {
                            Expr::Assignment(name, value) => Stmt::Assignment(name, *value),
                            Expr::FunctionCall(name, args) => Stmt::FunctionCall(name, args),
                            _ => Stmt::Expression(Box::new(expr)),
                        }
                    );
                },
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
                }
                Token::BraceOpen => {
                    scope_tokens.push(token);
                    scope_tokens.append(&mut self.get_tokens());
                }
                _ => scope_tokens.push(token),
            }
        }

        scope_tokens
    }

    fn parse_scope(&mut self) -> Vec<Stmt> {
        let scope_tokens = self.get_tokens();

        let mut parser: Parser = Parser::new(&scope_tokens);
        let ast = parser.parse();

        ast
    }

    fn parse_return(&mut self) -> Stmt {
        let expr_to_return = self.parse_expr();
        Stmt::Return(Box::new(expr_to_return))
    }

    fn parse_function(&mut self) -> Stmt {
        let mut args: Vec<String> = Vec::new();

        let function_name = match self.next_token() {
            Some(Token::Identifier(name)) => name,
            _ => panic!("Expected a function name"),
        };

        if self.next_token() != Some(Token::ParenOpen) {
            panic!(
                "Expected opening paranthesis after function name {}",
                function_name
            )
        }

        while let Some(token) = self.next_token() {
            match token {
                Token::ParenClose => break,
                Token::Identifier(name) => args.push(name),
                Token::Comma => (),
                _ => panic!("Unexpected token in function defination"),
            }
        }

        if self.next_token() != Some(Token::BraceOpen) {
            panic!(
                "Expected opening braces after function arguments {}",
                function_name
            )
        }

        Stmt::Function(function_name, args, Box::new(Stmt::CodeBlock(self.parse_scope())))
    }

    fn parse_function_call(&mut self, name: String) -> Expr {
        let mut args = Vec::new();
        let mut arg_expressions = Vec::new();

        arg_expressions.push(Vec::new());

        while let Some(token) = self.next_token() {
            match token {
                Token::ParenClose => break,
                Token::Comma => {
                    let current_arg_tokens = Vec::new();
                    arg_expressions.push(current_arg_tokens);
                }
                _ => arg_expressions.last_mut().unwrap().push(token),
            }
        }

        println!("{:?}", arg_expressions);

        for mut arg_tokens in arg_expressions {
            arg_tokens.push(Token::Semicolon);
            let mut parser: Parser = Parser::new(&arg_tokens);

            let mut ast = parser.parse();
            let expr = ast.pop().expect("Expected expression");

            match expr {
                Stmt::Expression(expr) => args.push(*expr),
                _ => panic!("Expected expression"),
            }
        }

        // while let Some(token) = self.next_token() {
        //     match token {
        //         Token::ParenClose => break,
        //         Token::Comma => (),
        //         _ => {
        //             args.push(self.parse_expr());
        //         },
        //     }
        // }

        Expr::FunctionCall(name, args)
    }

    fn parse_for_loop(&mut self) -> Stmt {
        let mut i = 0;
        let mut initiation = Stmt::None;
        let mut condition = Stmt::None;
        let mut increment = Stmt::None;

        match self.next_token() {
            Some(Token::ParenOpen) => {
                let mut tokens: Vec<Token> = Vec::new();
                while let Some(token) = self.next_token() {
                    match token {
                        Token::ParenClose => break,
                        _ => tokens.push(token),
                    }
                }

                let mut parser: Parser = Parser::new(&tokens);
                let expr_list = parser.parse();

                if expr_list.len() == 3 {
                    initiation = expr_list[0].clone();
                    condition = expr_list[1].clone();
                    increment = expr_list[2].clone();
                } else {
                    panic!("Expected 3 expressions in for loop");
                }
            }
            _ => panic!("Expected opening paranthesis after for"),
        };


        let code_block = match self.next_token() {

            Some(Token::BraceOpen) => self.parse_scope(),
            _ => panic!("Expected for loop body")
        };

        Stmt::ForLoop(
            Box::new(initiation),
            Box::new(condition),
            Box::new(increment),
            Box::new(Stmt::CodeBlock(code_block))
        )
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
                                Expr::ControlFlow(
                                    Box::new(condition),
                                    Box::new(Stmt::CodeBlock(if_ast)),
                                    Box::new(else_if_stmt),
                                )
                            }
                            Some(Token::Else) => {
                                //
                                self.pos += 1;

                                let else_ast = self.parse_scope();

                                let _token = self.tokens.get(self.pos);

                                Expr::ControlFlow(
                                    Box::new(condition),
                                    Box::new(Stmt::CodeBlock(if_ast)),
                                    Box::new(Stmt::ControlFlow(
                                        Box::new(Expr::Boolean(true)),
                                        Box::new(Stmt::CodeBlock(else_ast)),
                                        Box::new(Stmt::None),
                                    )),
                                )
                            }
                            _ => {
                                self.pos -= 1;
                                Expr::ControlFlow(
                                    Box::new(condition),
                                    Box::new(Stmt::CodeBlock(if_ast)),
                                    Box::new(Stmt::None),
                                )
                            }
                        }
                    }
                    _ => panic!("Expected opening brace"),
                }
            }
            _ => panic!("Expected opening parenthesis"),
        };

        match expr {
            Expr::ControlFlow(condition, stmts, else_stmt) => {
                Stmt::ControlFlow(condition, stmts, else_stmt)
            }
            _ => panic!("Expected if"),
        }
    }

    fn parse_loop(&mut self) -> Stmt {
        let code_block = match self.next_token() {
            Some(Token::BraceOpen) => self.parse_scope(),
            _ => panic!("Expected loop body"),
        };

        Stmt::Loop(Box::new(Stmt::CodeBlock(code_block)))
    }

    fn parse_while(&mut self) -> Stmt {
        let condition = match self.next_token() {
            Some(Token::ParenOpen) => self.parse_expr(),
            _ => panic!("Expected opening parenthesis"),
        };

        // skip closing parenthesis
        self.pos += 1;

        let code_block = match self.next_token() {
            Some(Token::BraceOpen) => self.parse_scope(),
            _ => {
                panic!("Expected while loop body")
            },
        };

        Stmt::While(Box::new(condition), Box::new(Stmt::CodeBlock(code_block)))
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
                        _ => {
                            self.pos -= 1;
                            Expr::Identifier(name)
                        }
                    }
                }
                Some(Token::StringLiteral(literal)) => Expr::StringLiteral(literal),
                Some(Token::Boolean(bool)) => Expr::Boolean(bool),
                Some(Token::Equals) => {
                    let left = expr.pop().expect("Expected left side of equals");
                    let right = self.parse_expr();

                    return Expr::Equals(Box::new(left), Box::new(right));
                }
                Some(Token::Assign) => {
                    let left = expr.pop().expect("Expected left side of equals");
                    let right = self.parse_expr();

                    match left {
                        Expr::Identifier(name) => {
                            return Expr::Assignment(name, Box::new(right));
                        }
                        _ => panic!("Expected identifier on left side of assignment"),
                    }
                }
                Some(Token::LogicalAnd) => {
                    let left = expr.pop().expect("Expected left side of equals");
                    let right = self.parse_expr();

                    return Expr::LogicalAnd(Box::new(left), Box::new(right));
                }
                Some(Token::LogicalOr) => {
                    let left = expr.pop().expect("Expected left side of equals");
                    let right = self.parse_expr();

                    return Expr::LogicalOr(Box::new(left), Box::new(right));
                }
                Some(Token::LogicalNot) => {
                    let right = self.parse_expr();
                    return Expr::LogicalNot(Box::new(right));
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

                    return Expr::Addition(Box::new(left), Box::new(right));
                }
                Some(Token::Increment) => {
                    let left = expr.pop().expect("Expected left side of addition");
                    let right = Expr::Float(1.0);

                    match left {
                        Expr::Identifier(name) => {
                            let name_clone = name.clone();

                            return Expr::Assignment(
                                name,
                                Box::new(Expr::Addition(
                                    Box::new(Expr::Identifier(name_clone)),
                                    Box::new(right),
                                )),
                            )
                        }
                        _ => panic!("Expected identifier on left side of increment"),
                    }
                }
                Some(Token::Decrement) => {
                    let left = expr.pop().expect("Expected left side of subtraction");
                    let right = Expr::Float(1.0);

                    match left {
                        Expr::Identifier(name) => {
                            let name_clone = name.clone();

                            return Expr::Assignment(
                                name,
                                Box::new(Expr::Subtraction(
                                    Box::new(Expr::Identifier(name_clone)),
                                    Box::new(right),
                                )),
                            )
                        }
                        _ => panic!("Expected identifier on left side of decrement"),
                    }
                }
                Some(Token::Subtraction) => {
                    let left = expr.pop().expect("Expected left side of subtraction");
                    let right = self.parse_expr();

                    return Expr::Subtraction(Box::new(left), Box::new(right));
                }
                Some(Token::Multiplication) => {
                    let left = expr.pop().expect("Expected left side of multiplication");
                    let right = self.parse_expr();

                    return Expr::Multiplication(Box::new(left), Box::new(right));
                }
                Some(Token::Division) => {
                    let left = expr.pop().expect("Expected left side of division");
                    let right = self.parse_expr();

                    return Expr::Division(Box::new(left), Box::new(right));
                }
                Some(Token::GreaterThan) => {
                    let left = expr.pop().expect("Expected left side of greater than");
                    let right = self.parse_expr();

                    return Expr::GreaterThan(Box::new(left), Box::new(right));
                }
                Some(Token::LessThan) => {
                    let left = expr.pop().expect("Expected left side of less than");
                    let right = self.parse_expr();

                    return Expr::LessThan(Box::new(left), Box::new(right));
                }
                Some(Token::GreaterThanEquals) => {
                    let left = expr
                        .pop()
                        .expect("Expected left side of greater than equals");
                    let right = self.parse_expr();

                    return Expr::GreaterThanEquals(Box::new(left), Box::new(right));
                }
                Some(Token::LessThanEquals) => {
                    let left = expr.pop().expect("Expected left side of less than equals");
                    let right = self.parse_expr();

                    return Expr::LessThanEquals(Box::new(left), Box::new(right));
                }
                _ => {
                    panic!("Expected Float or identifier")
                }
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

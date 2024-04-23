use crate::types::Token;

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
                    let token = self.read_float();
                    return token;
                }
                '\'' => {
                    let token = self.read_string_literal('\'');
                    return token;
                }
                '"' => {
                    let token = self.read_string_literal('\"');
                    return token;
                },
                '&' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '&' => {
                            self.pos += 2;
                            return Some(Token::LogicalAnd)
                        },
                        _ => panic!("Expected &")
                    }
                },
                '|' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '|' => {
                            self.pos += 2;
                            return Some(Token::LogicalOr)
                        },
                        _ => panic!("Expected |")
                    }
                },
                '!' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '=' => {
                            let char = self.code.chars().nth(self.pos + 2)?;

                            match char {
                                '=' => {
                                    self.pos += 3;
                                    return Some(Token::TypeNotEquals);
                                },
                                _ => {
                                    self.pos += 2;
                                    return Some(Token::NotEquals);
                                }
                            }
                        },
                        _ => {
                            self.pos += 1;
                            return Some(Token::LogicalNot);
                        }
                    }
                }
                '=' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '=' => {
                            let char = self.code.chars().nth(self.pos + 2)?;

                            match char {
                                '=' => {
                                    self.pos += 3;
                                    return Some(Token::TypeCheckEquals);
                                },
                                _ => {
                                    self.pos += 2;
                                    return Some(Token::Equals);
                                }
                            }
                        },
                        _ => {
                            self.pos += 1;
                            return Some(Token::Assign);
                        }
                    }
                },
                '<' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '=' => {
                            self.pos += 2;
                            return Some(Token::LessThanEquals)
                        },
                        _ => {
                            self.pos += 1;
                            return Some(Token::LessThan)
                        }
                    }
                },
                '>' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '=' => {
                            self.pos += 2;
                            return Some(Token::GreaterThanEquals)
                        },
                        _ => {
                            self.pos += 1;
                            return Some(Token::GreaterThan)
                        }
                    }
                },
                '+' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '+' => {
                            self.pos += 2;
                            return Some(Token::Increment)
                        },
                        _ => {
                            self.pos += 1;
                            return Some(Token::Addition)
                        }
                    }
                }
                '-' => {
                    let char = self.code.chars().nth(self.pos + 1)?;

                    match char {
                        '-' => {
                            self.pos += 2;
                            return Some(Token::Decrement)
                        },
                        _ => {
                            self.pos += 1;
                            return Some(Token::Subtraction)
                        }
                    }
                }
                '*' => {
                    self.pos += 1;
                    return Some(Token::Multiplication);
                }
                ';' => {
                    self.pos += 1;
                    return Some(Token::Semicolon);
                }
                '{' => {
                    self.pos += 1;
                    return Some(Token::BraceOpen);
                }
                '}' => {
                    self.pos += 1;
                    return Some(Token::BraceClose);
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
                        }
                    };

                    return token;
                }
                ',' => {
                    self.pos += 1;
                    return Some(Token::Comma);
                },
                _ => {
                    self.pos += 1;
                }
            }
        }

        None
    }

    fn is_valid_variable_char (&mut self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let start = self.pos;
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            if !self.is_valid_variable_char(c) {
                break;
            }

            self.pos += 1;
        }

        let ident = &self.code[start..self.pos];

        match ident {
            "let" => Some(Token::Let),
            "log" => Some(Token::Log),
            "return" => Some(Token::Return),
            "function" => Some(Token::Function),
            "true" => Some(Token::Boolean(true)),
            "false" => Some(Token::Boolean(false)),
            "if" => Some(Token::If),
            "for" => Some(Token::ForLoop),
            "else" => {
                self.pos += 1;

                let next_token = self.next_token();

                match next_token {
                    Some(Token::If) => Some(Token::ElseIf),
                    Some(Token::BraceOpen) => {
                        self.pos -= 1;
                        Some(Token::Else)
                    },
                    _ => panic!("Unexpected token")
                }
            },
            _ => Some(Token::Identifier(ident.to_string())),
        }
    }

    fn read_float(&mut self) -> Option<Token> {
        let start = self.pos;
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            if !c.is_numeric() {
                break;
            }

            self.pos += 1;
        }

        let num = &self.code[start..self.pos];
        Some(Token::Float(num.parse().unwrap()))
    }

    fn read_string_literal(&mut self, delimiter: char) -> Option<Token> {
        let start: usize = self.pos;

        // Skip the opening quote
        self.pos += 1;

        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos)?;

            if c == delimiter {
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
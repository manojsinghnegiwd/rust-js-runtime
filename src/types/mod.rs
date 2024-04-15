#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Identifier(String),
    Equals,
    Number(usize),
    StringLiteral(String),
    Semicolon,
    Log,
    ParenClose,
    ParenOpen,
    Comment(String),
    Addition,
    subtraction
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Number(usize),
    StringLiteral(String),
    Addition(Box<Expr>, Box<Expr>),
    subtraction(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Log(Expr),
    Comment(String),
}
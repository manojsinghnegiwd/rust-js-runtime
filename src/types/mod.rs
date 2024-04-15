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
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Number(usize),
    StringLiteral(String),
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Log(Expr),
    Comment(String),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Float(f64),
    StringLiteral(String),
    Equals,
    TypeCheckEquals,
    Assign,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Boolean(bool),
    Semicolon,
    ParenClose,
    ParenOpen,
    Comment(String),
    Log,
    Let,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Float(f64),
    StringLiteral(String),
    Boolean(bool),
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    TypeCheckEquals(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Assignment(String, Expr),
    Log(Expr),
    Comment(String),
}
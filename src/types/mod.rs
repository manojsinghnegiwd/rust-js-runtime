#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Float(f64),
    StringLiteral(String),
    Not,
    Equals,
    TypeCheckEquals,
    NotEquals,
    TypeNotEquals,
    Assign,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Boolean(bool),
    Semicolon,
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    Comment(String),
    If,
    ElseIf,
    Else,
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
    NotEquals(Box<Expr>, Box<Expr>),
    TypeNotEquals(Box<Expr>, Box<Expr>),
    ControlFlow(Box<Expr>, Vec<Stmt>, Box<Stmt>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Assignment(String, Expr),
    ControlFlow(Box<Expr>, Vec<Stmt>, Box<Stmt>),
    None,
    Log(Expr)
}
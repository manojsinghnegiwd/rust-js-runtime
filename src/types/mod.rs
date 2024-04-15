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
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Number(usize),
    StringLiteral(String),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Log(Expr),
}
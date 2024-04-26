#[derive(Debug, PartialEq, Clone)]
pub enum Token {

    // literals
    Identifier(String),
    Float(f64),
    StringLiteral(String),
    Boolean(bool),

    // operators
    Assign,

    // logical operators
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // comparison operators
    Equals,
    TypeCheckEquals,
    NotEquals,
    TypeNotEquals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,

    // arithmetic operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Increment,
    Decrement,

    // punctuation
    Semicolon,
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    Comma,
    Comment(String),

    // keywords
    If,
    ElseIf,
    Else,
    Log,
    Let,
    Function,
    Return,
    ForLoop,
    FunctionCall(String),
    Loop,
    Break,
    While
}

#[derive(Clone, Debug, PartialEq)]
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
    GreaterThan(Box<Expr>, Box<Expr>),
    GreaterThanEquals(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    LessThanEquals(Box<Expr>, Box<Expr>),
    ControlFlow(Box<Expr>, Box<Stmt>, Box<Stmt>),
    FunctionCall(String, Vec<Expr>),
    LogicalAnd(Box<Expr>, Box<Expr>),
    LogicalOr(Box<Expr>, Box<Expr>),
    LogicalNot(Box<Expr>),
    Assignment(String, Box<Expr>),
    Loop(Box<Stmt>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Assignment(String, Expr),
    ControlFlow(Box<Expr>, Box<Stmt>, Box<Stmt>),
    CodeBlock(Vec<Stmt>),
    Log(Expr),
    Function(String, Vec<String>, Box<Stmt>),
    Return(Box<Expr>),
    FunctionCall(String, Vec<Expr>),
    ForLoop(Box<Stmt>, Box<Stmt>, Box<Stmt>, Box<Stmt>),
    Expression(Box<Expr>),
    Loop(Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
    None,
    Break
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Float(f64),
    StringLiteral(String),
    Boolean(bool),
    FunctionDef(Vec<String>, Box<Stmt>),
    Return(Box<Value>),
    Break,
    None
}

pub enum Signal {
    Return,
    Break,
    None
}
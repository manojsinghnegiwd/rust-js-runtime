use std::collections::HashMap;
use crate::types::{Expr, Stmt};

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Float(f64),
    StringLiteral(String),
    Boolean(bool),
}

pub struct Interpreter {
    global_scope: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global_scope = HashMap::new();

        Interpreter {
            global_scope,
        }
    }

    pub fn eval(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            match stmt {
                Stmt::Let(name, value) => self.eval_let(name, value),
                Stmt::Log(expr) => self.eval_log(expr),
                Stmt::Assignment(name, value) => self.eval_let(name, value),
                Stmt::Comment(_) => (),
                Stmt::If(condition, stmts, else_stmt) => self.eval_if(condition, stmts, else_stmt),
            }
        }
    }

    fn eval_let(&mut self, name: String, value: Expr) {
        let value = self.eval_expr(value);
        self.global_scope.insert(name, value);
    }

    fn eval_log(&mut self, expr: Expr) {
        let value = self.eval_expr(expr);
        println!("{:?}", value);
    }

    fn eval_if(&mut self, condition: Box<Expr>, stmts: Vec<Stmt>, else_stmt: Box<Stmt>) {
        let result = self.eval_expr(*condition);

        match result {
            Value::Boolean(true) => {
                self.eval(stmts);
            },
            Value::Boolean(false) => {
                match *else_stmt {
                    Stmt::If(condition, stmts, nested_else_stmt) => {
                        self.eval_if(condition, stmts, nested_else_stmt);
                    },
                    _ => ()
                }
            },
            _ => panic!("Expected a boolean expression"),
        };
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Identifier(name) => self.global_scope.get(&name).expect(&format!("Trying to access an undefined variable: {}", name)).clone(),
            Expr::Float(num) => Value::Float(num),
            Expr::StringLiteral(literal) => Value::StringLiteral(literal),
            Expr::Boolean(bool) => Value::Boolean(bool),
            Expr::Equals(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left) {
                    Value::Float(left) => {
                        let corced_right = match (right) {
                            Value::Float(right) => Some(right),
                            Value::StringLiteral(right) => right.parse::<f64>().ok(),
                            Value::Boolean(right) => Some(right as i32 as f64),
                            _ => Some(0.0),
                        };

                        match (corced_right) {
                            Some(right) => Value::Boolean(left == right),
                            None => Value::Boolean(false),
                        }
                    },
                    Value::StringLiteral(left) => {
                        let corced_right = match (right) {
                            Value::Float(right) => Some(right.to_string()),
                            Value::StringLiteral(right) => Some(right),
                            Value::Boolean(right) => Some(right.to_string()),
                        };

                        Value::Boolean(left == corced_right.unwrap())
                    }
                    Value::Boolean(left) => {
                        let corced_right = match (right) {
                            Value::Float(right) => Some(right as i32 != 0),
                            Value::StringLiteral(right) => {
                                let value = right.parse::<i32>().ok();

                                match value {
                                    Some(value) => Some(value != 0),
                                    None => Some(false),
                                }
                            }
                            Value::Boolean(right) => Some(right),
                        };

                        Value::Boolean(left == corced_right.unwrap())
                    },
                    _ => panic!("Expected two expressions to compare"),
                
                }
            }
            Expr::TypeCheckEquals(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left == right),
                    (Value::StringLiteral(left), Value::StringLiteral(right)) => Value::Boolean(left == right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left == right),
                    _ => Value::Boolean(false),
                }
            }
            Expr::NotEquals(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left) {
                    Value::Float(left) => {
                        let corced_right = match (right) {
                            Value::Float(right) => Some(right),
                            Value::StringLiteral(right) => right.parse::<f64>().ok(),
                            Value::Boolean(right) => Some(right as i32 as f64),
                            _ => Some(0.0),
                        };

                        match (corced_right) {
                            Some(right) => Value::Boolean(left != right),
                            None => Value::Boolean(false),
                        }
                    },
                    Value::StringLiteral(left) => {
                        let corced_right = match (right) {
                            Value::Float(right) => Some(right.to_string()),
                            Value::StringLiteral(right) => Some(right),
                            Value::Boolean(right) => Some(right.to_string()),
                        };

                        Value::Boolean(left != corced_right.unwrap())
                    }
                    Value::Boolean(left) => {
                        let corced_right = match (right) {
                            Value::Float(right) => Some(right as i32 == 0),
                            Value::StringLiteral(right) => {
                                let value = right.parse::<i32>().ok();

                                match value {
                                    Some(value) => Some(value == 0),
                                    None => Some(false),
                                }
                            }
                            Value::Boolean(right) => Some(right),
                        };

                        Value::Boolean(left != corced_right.unwrap())
                    },
                    _ => panic!("Expected two expressions to compare"),
                
                }
            }
            Expr::TypeNotEquals(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left != right),
                    (Value::StringLiteral(left), Value::StringLiteral(right)) => Value::Boolean(left != right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left != right),
                    _ => Value::Boolean(false),
                }
            }
            Expr::Addition(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Float(left + right),
                    _ => panic!("Expected two numbers"),
                }
            },
            Expr::Subtraction(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Float(left - right),
                    _ => panic!("Expected two numbers"),
                }
            },
            Expr::Multiplication(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Float(left * right),
                    _ => panic!("Expected two numbers"),
                }
            },
            Expr::Division(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Float(left / right),
                    _ => panic!("Expected two numbers"),
                }
            }
            _ => panic!("Expected an expression"),
        }
    }
}

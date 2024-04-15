use std::collections::HashMap;
use crate::types::{Expr, Stmt};

#[derive(Clone, Debug)]
enum Value {
    Float(f64),
    StringLiteral(String),
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
                Stmt::Comment(_) => (),
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

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Identifier(name) => self.global_scope.get(&name).expect(&format!("Trying to access an undefined variable: {}", name)).clone(),
            Expr::Float(num) => Value::Float(num),
            Expr::StringLiteral(literal) => Value::StringLiteral(literal),
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
            },
        }
    }
}

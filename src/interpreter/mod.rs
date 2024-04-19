use crate::scope::Scope;
use crate::types::{Expr, Stmt, Value};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    scope: Option<Rc<RefCell<Scope>>>,
}

impl Interpreter {
    pub fn new(scope: Option<Scope>) -> Self {
        match scope {
            Some(scope) => Interpreter {
                scope: Some(Rc::new(RefCell::new(scope))),
            },
            None => Interpreter {
                scope: None,
            }
        }
    }

    pub fn eval(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            match stmt {
                Stmt::Let(name, value) => self.eval_let(name, value),
                Stmt::Log(expr) => self.eval_log(expr),
                Stmt::Assignment(name, value) => self.eval_assignment(name, value),
                Stmt::None => (),
                Stmt::ControlFlow(condition, stmts, else_stmt) => self.eval_if(condition, stmts, else_stmt),
                Stmt::CodeBlock(stmts) => self.eval_code_block(stmts),
            }
        };
    }

    fn eval_code_block(&mut self, stmts: Vec<Stmt>) {
        let scope = Scope::with_rc(self.scope.clone());
        let mut interpreter = Interpreter::new(Some(scope));
        interpreter.eval(stmts);
    }

    fn eval_let(&mut self, name: String, value: Expr) {
        let value = self.eval_expr(value);
        match &mut self.scope {
            Some(scope) => {
                if scope.borrow().contains_key_local(&name) {
                    panic!("Variable already defined: {}", name);
                }
                
                scope.borrow_mut().define(name, value);
            },
            None => (),
        }
    }

    fn eval_assignment(&mut self, name: String, value: Expr) {
        let value = self.eval_expr(value);

        match &mut self.scope {
            Some(scope) => {
                scope.borrow_mut().assign(name, value);
            },
            None => (),
        }
    }

    fn eval_log(&mut self, expr: Expr) {
        let value = self.eval_expr(expr);
        println!("{:#?}", value);
    }

    fn eval_if(&mut self, condition: Box<Expr>, stmts: Box<Stmt>, else_stmt: Box<Stmt>) {
        let result = self.eval_expr(*condition);

        match result {
            Value::Boolean(true) => {
                match *stmts {
                    Stmt::CodeBlock(code_block_stmts) => {
                        self.eval_code_block(code_block_stmts);
                    },
                    _ => ()
                }
            },
            Value::Boolean(false) => {
                match *else_stmt {
                    Stmt::ControlFlow(condition, stmts, nested_else_stmt) => {
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
            Expr::Identifier(name) => {
                match &mut self.scope {
                    Some(scope) => {
                        match scope.borrow().get(&name) {
                            Some(value) => value.clone(),
                            None => panic!("Variable not found: {}", name),
                        }
                    },
                    None => Value::Boolean(false),
                }
            },
            Expr::Float(num) => Value::Float(num),
            Expr::StringLiteral(literal) => Value::StringLiteral(literal),
            Expr::Boolean(bool) => Value::Boolean(bool),
            Expr::Equals(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match left {
                    Value::Float(left) => {
                        let corced_right = match right {
                            Value::Float(right) => Some(right),
                            Value::StringLiteral(right) => right.parse::<f64>().ok(),
                            Value::Boolean(right) => Some(right as i32 as f64),
                        };

                        match corced_right {
                            Some(right) => Value::Boolean(left == right),
                            None => Value::Boolean(false),
                        }
                    },
                    Value::StringLiteral(left) => {
                        let corced_right = match right {
                            Value::Float(right) => Some(right.to_string()),
                            Value::StringLiteral(right) => Some(right),
                            Value::Boolean(right) => Some(right.to_string()),
                        };

                        Value::Boolean(left == corced_right.unwrap())
                    }
                    Value::Boolean(left) => {
                        let corced_right = match right {
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
                    }
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

                match left {
                    Value::Float(left) => {
                        let corced_right = match right {
                            Value::Float(right) => Some(right),
                            Value::StringLiteral(right) => right.parse::<f64>().ok(),
                            Value::Boolean(right) => Some(right as i32 as f64),
                        };

                        match corced_right {
                            Some(right) => Value::Boolean(left != right),
                            None => Value::Boolean(false),
                        }
                    },
                    Value::StringLiteral(left) => {
                        let corced_right = match right {
                            Value::Float(right) => Some(right.to_string()),
                            Value::StringLiteral(right) => Some(right),
                            Value::Boolean(right) => Some(right.to_string()),
                        };

                        Value::Boolean(left != corced_right.unwrap())
                    }
                    Value::Boolean(left) => {
                        let corced_right = match right {
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
                    }
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
                    (Value::Float(left), Value::StringLiteral(right)) => Value::StringLiteral(format!("{}{}", left, right)),
                    (Value::StringLiteral(left), Value::Float(right)) => Value::StringLiteral(format!("{}{}", left, right)),
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

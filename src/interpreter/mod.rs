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

    pub fn eval(&mut self, stmts: Vec<Stmt>) -> Value {
        let mut return_value = Value::None;

        for stmt in stmts {
            match stmt {
                Stmt::Let(name, value) => self.eval_let(name, value),
                Stmt::Log(expr) => self.eval_log(expr),
                Stmt::Assignment(name, value) => self.eval_assignment(name, value),
                Stmt::ControlFlow(condition, stmts, else_stmt) => {
                    return_value = self.eval_if(condition, stmts, else_stmt);
                },
                Stmt::CodeBlock(stmts) => {
                    return_value = self.eval_code_block(stmts);
                },
                Stmt::Function(function_name, args, function_body) => self.eval_let_function(function_name, args, function_body),
                Stmt::FunctionCall(name, arguments) => {
                    return_value = self.eval_function_call(name, arguments);
                },
                Stmt::Break => {
                    return_value = Value::Break;
                },
                Stmt::Return(expr) => {
                    match *expr {
                        expr => {
                            return_value = self.eval_expr(expr);
                            break;
                        },
                    }
                },
                Stmt::ForLoop(_, _, _, _) => (),
                Stmt::Loop(stmts) => self.eval_loop(stmts),
                Stmt::Expression(expr) => {
                    return_value = self.eval_expr(*expr);
                },
                Stmt::None => (),
            }
        }

        return_value
    }

    fn eval_code_block(&mut self, stmts: Vec<Stmt>) -> Value {
        let scope = Scope::with_rc(self.scope.clone());
        let mut interpreter = Interpreter::new(Some(scope));
        interpreter.eval(stmts)
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

    fn eval_let_function (&mut self, name: String, args: Vec<String>, value: Box<Stmt>) {
        match &mut self.scope {
            Some(scope) => {
                if scope.borrow().contains_key_local(&name) {
                    panic!("Variable already defined: {}", name);
                }
                
                scope.borrow_mut().define(name, Value::FunctionDef(args, value));
            },
            None => (),
        }
    }

    fn eval_function_call (&mut self, name: String, args: Vec<Expr>) -> Value {
        let value = self.eval_expr(Expr::Identifier(name));

        match value {
            Value::FunctionDef(params, body) => {
                let mut stmts = Vec::new();

                let mut i = 0;

                for param in &params {
                    stmts.push(Stmt::Let(param.to_string(), args.get(i).unwrap().clone() ));
                    i += 1;
                };

                match *body {
                    Stmt::CodeBlock(function_body) => {
                        for stmt in function_body {
                            stmts.push(stmt);
                        }
                    },
                    _ => panic!("Expected function body")
                }

                self.eval_code_block(stmts)
            },
            _ => panic!("Expected function body")
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

    fn eval_if(&mut self, condition: Box<Expr>, stmts: Box<Stmt>, else_stmt: Box<Stmt>) -> Value {
        let result = self.eval_expr(*condition);
        
        let result_coerced = match result {
            Value::Boolean(value) => value,
            Value::Float(value) => value != 0.0,
            Value::StringLiteral(value) => value.len() != 0,
            Value::FunctionDef(_, _) => true,
            Value::None => false,
            _ => panic!("Expected a boolean expression"),
        };

        match result_coerced {
            true => {
                match *stmts {
                    Stmt::CodeBlock(code_block_stmts) => {
                        self.eval_code_block(code_block_stmts)
                    },
                    _ => Value::None,
                }
            },
            false => {
                match *else_stmt {
                    Stmt::ControlFlow(condition, stmts, nested_else_stmt) => {
                        self.eval_if(condition, stmts, nested_else_stmt)
                    },
                    _ => Value::None,
                }
            },
            _ => panic!("Expected a boolean expression"),
        }
    }

    fn eval_loop(&mut self, stmts: Box<Stmt>) {
        let code_block = match *stmts {
            Stmt::CodeBlock(code_block_stmts) => code_block_stmts,
            _ => panic!("Expected a code block"),
        };

        loop {
            let scope = Scope::with_rc(self.scope.clone());
            let mut interpreter = Interpreter::new(Some(scope));
            let return_value = interpreter.eval(code_block.clone());

            match return_value {
                Value::Break => break,
                _ => (),
            }
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
                            _ => panic!("Expected a number or string"),
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
                            _ => panic!("Expected a number or string"),
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
                            _ => panic!("Expected a number or string"),
                        };

                        Value::Boolean(left == corced_right.unwrap())
                    },
                    _ => panic!("Expected a number or string"),
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
                            _ => panic!("Expected a number or string"),
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
                            _ => panic!("Expected a number or string"),
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
                            _ => panic!("Expected a number or string"),
                        };

                        Value::Boolean(left != corced_right.unwrap())
                    }
                    _ => panic!("Expected a number or string"),
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
                    (Value::StringLiteral(left), Value::StringLiteral(right)) => Value::StringLiteral(format!("{}{}", left, right)),
                    _ => panic!("Expected a number or string"),
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
            Expr::GreaterThan(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left > right),
                    _ => panic!("Expected two numbers"),
                }
            },
            Expr::GreaterThanEquals(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left >= right),
                    _ => panic!("Expected two numbers"),
                }
            },
            Expr::LessThan(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left < right),
                    _ => panic!("Expected two numbers"),
                }
            },
            Expr::LessThanEquals(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match (left, right) {
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left <= right),
                    _ => panic!("Expected two numbers"),
                }
            }
            Expr::LogicalOr(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match left {
                    Value::Float(left) => {
                        if left == 0.0 {
                            return right
                        } else {
                            return Value::Float(left)
                        }
                    },
                    Value::StringLiteral(left) => {
                        if left.len() == 0 {
                            return right
                        } else {
                            return Value::StringLiteral(left)
                        }
                    },
                    Value::Boolean(left) => {
                        if !left {
                            return right
                        } else {
                            return Value::Boolean(left)
                        }
                    }
                    _ => panic!("Expected a valid logical expression")
                }
            },
            Expr::LogicalAnd(left, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match left {
                    Value::Float(left) => {
                        if left != 0.0 {
                            return right
                        } else {
                            return Value::Float(left)
                        }
                    },
                    Value::StringLiteral(left) => {
                        if left.len() != 0 {
                            return right
                        } else {
                            return Value::StringLiteral(left)
                        }
                    },
                    Value::Boolean(left) => {
                        if left {
                            return right
                        } else {
                            return Value::Boolean(left)
                        }
                    }
                    _ => panic!("Expected a valid logical expression")
                }
            },
            Expr::LogicalNot(expr) => {
                let right = self.eval_expr(*expr);

                match right {
                    Value::Float(right) => {
                        if right == 0.0 {
                            return Value::Boolean(true)
                        } else {
                            return Value::Boolean(false)
                        }
                    },
                    Value::StringLiteral(right) => {
                        if right.len() == 0 {
                            return Value::Boolean(true)
                        } else {
                            return Value::Boolean(false)
                        }
                    },
                    Value::Boolean(right) => {
                        if !right {
                            return Value::Boolean(true)
                        } else {
                            return Value::Boolean(false)
                        }
                    }
                    _ => panic!("Expected a valid logical expression")
                }
            },
            Expr::FunctionCall(args, value) => self.eval_function_call(args, value),
            _ => {
                panic!("Expected an expression")
            },
        }
    }
}

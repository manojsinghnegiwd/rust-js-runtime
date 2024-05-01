use std::io::{self, Write};
use crate::scope::Scope;
use crate::parser::Parser;
use crate::interpreter::{self, Interpreter};
use crate::lexer::Lexer;
use crate::types::{Token, Stmt, Value};

pub struct Runtime<'a> {
    tokens: Vec<Token>,
    ast: Vec<Stmt>,
    output: Value,
    code: &'a str
}

impl<'a> Runtime<'a> {
    pub fn new(code: &'a str) -> Self {
        Runtime {
            tokens: Vec::new(),
            ast: Vec::new(),
            output: Value::None,
            code
        }
    }

    pub fn repl (&mut self) {
        let scope = Scope::new(None);

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "exit" {
                break;
            }

            let ast = self.parse_ast(input);

            let mut interpreter = Interpreter::new(Some(scope.clone()));
            let (value, signal) = interpreter.eval(ast);

            println!("{:?}", value);
        }
    }

    pub fn parse_ast (&mut self, code: String) -> Vec<Stmt> {
        let mut lexer = Lexer::new(&code);
        println!("Lexer started... \n");

        while let Some(token) = lexer.next_token() {
            self.tokens.push(token);
        }

        let mut i = 0;

        for token in &self.tokens {
            println!("{:?} => {:?}", i, token);
            i += 1;
        }

        println!("\nLexer completed... \n");

        println!("Parsing started... \n");

        let mut j = 0;

        let mut parser = Parser::new(&self.tokens);
        self.ast = parser.parse();

        let local_ast = self.ast.clone();

        for stmt in &local_ast {
            println!("{:?} => {:#?}", j, stmt);
            j += 1;
        }

        println!("\nParsing completed... \n");

        return local_ast;
    }

    pub fn execute (&mut self) -> Value {
        println!("Execution started... \n");

        let local_ast = self.parse_ast(self.code.to_string());
        let scope = Scope::new(None);

        let mut interpreter = Interpreter::new(Some(scope));
        let (value, _) = interpreter.eval(local_ast);

        return value;
    }
}
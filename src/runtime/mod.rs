use crate::scope::Scope;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::types::{Token, Stmt, Value};

pub struct Runtime<'a> {
    tokens: Vec<Token>,
    ast: Vec<Stmt>,
    output: Value,
    scope: Scope,
    code: &'a str
}

impl<'a> Runtime<'a> {
    pub fn new(code: &'a str) -> Self {
        Runtime {
            tokens: Vec::new(),
            ast: Vec::new(),
            output: Value::None,
            scope: Scope::new(None),
            code
        }
    }

    pub fn execute (&mut self) -> Value {
        let mut lexer = Lexer::new(&self.code);
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

        println!("Execution started... \n");

        let scope = Scope::new(None);

        let mut interpreter = Interpreter::new(Some(scope));
        let (value, _) = interpreter.eval(local_ast);

        return value;
    }
}
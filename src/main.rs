mod parser;
mod types;
mod interpreter;
mod lexer;
mod scope;

use parser::Parser;
use interpreter::Interpreter;
use lexer::Lexer;
use scope::Scope;

fn main() {
    let code = r#"
        let x = "Hello";
        let y = "World";
        let c = "People";
        let i = 0;

        {
            let y = 8;
            let x = 1;
            log(x);

            if (x == 1) {
                let y = 2;
                log(x + " hello");
                log(y);
            }

            log(x + 1);
            log(y);
        }

        function add(a, b) {
            return a + " " + b + " " + c;
        }

        function log_something (something) {
            log(something);
        }

        let sum = add("Hello", "World");
        
        log(i+1);

        log_something(i);

    "#;

    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();

    println!("Lexer started... \n");

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let mut i = 0;

    for token in &tokens {
        println!("{:?} => {:?}", i, token);
        i += 1;
    }

    println!("\nLexer completed... \n");

    println!("Parsing started... \n");

    let mut j = 0;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    for stmt in &ast {
        println!("{:?} => {:#?}", j, stmt);
        j += 1;
    }

    println!("\nParsing completed... \n");

    println!("Execution started... \n");

    let scope = Scope::new(None);

    let mut interpreter = Interpreter::new(Some(scope));
    interpreter.eval(ast);

    println!("\nExecution completed... \n");
}
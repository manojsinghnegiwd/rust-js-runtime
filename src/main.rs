mod parser;
mod types;
mod interpreter;
mod lexer;

use parser::Parser;
use interpreter::Interpreter;
use lexer::Lexer;

fn main() {
    let code = r#"
        let x = 3;
        let y = x;
        let m = 'Manoj';
        log(x);
        log(m);
        log(y);

        // {
        //     let x = 5;
        //     log(x);
        // }

        // reactive {
        //     log(x);
        // }
    "#;
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    for stmt in &ast {
        println!("{:?}", stmt);
    }

    let mut interpreter = Interpreter::new();
    interpreter.eval(ast);
}
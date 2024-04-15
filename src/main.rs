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
        let y = x + 3 + 5;
        let m = 'Manoj';
        log(x + 1);
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

    println!("Lexer started... \n");

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    for token in &tokens {
        println!("{:?}", token);
    }

    println!("\nLexer completed... \n");

    println!("Parsing started... \n");

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    for stmt in &ast {
        println!("{:?}", stmt);
    }

    println!("\nParsing completed... \n");

    println!("Execution started... \n");

    let mut interpreter = Interpreter::new();
    interpreter.eval(ast);

    println!("\nExecution completed... \n");
}
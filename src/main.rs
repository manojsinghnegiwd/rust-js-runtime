mod parser;
mod types;
mod interpreter;
mod lexer;

use parser::Parser;
use interpreter::Interpreter;
use lexer::Lexer;

fn main() {
    let code = r#"
        let x = 6;

        if (x == 3) {
            log(x);
        }

        if (x == 4) {
            log('x is 4');
        } else if (x == 6) {
            log('x is 6');
        } else {
            log('x is 3');
        }
    "#;
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();

    println!("Lexer started... \n");

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let mut i = 0;

    for token in &tokens {
        println!("{:?}, {:?} token", token, i);
        i += 1;
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
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
        // let y = x + 3 + 3;
        // let m = 'Manoj';
        // log(x + 1);
        // log(m);
        // log(y - x);
        // log(y * x);
        // y = 10;
        // let isWorking = 'true' == false;
        // log(true == 1);
        // log(true === 1)

        log(x != 2);

        if (x == 4) {
            log('x is 3');
        }
        // } else {
        //     log('x is not 3');
        // }

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
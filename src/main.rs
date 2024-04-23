mod parser;
mod types;
mod interpreter;
mod lexer;
mod scope;
mod runtime;

use runtime::Runtime;

fn main() {
    let code = r#"
        let a = 10;
        return
    "#;


    let mut runtime = Runtime::new(code);
    runtime.execute();
}
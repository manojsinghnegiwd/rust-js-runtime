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
        let b = 20;
        
        function add(a, b) {
            return a + b;
        }

        return add(a, b);
    "#;

    let mut runtime = Runtime::new(code);
    runtime.execute();
}
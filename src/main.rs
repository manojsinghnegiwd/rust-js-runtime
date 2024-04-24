mod interpreter;
mod lexer;
mod parser;
mod runtime;
mod scope;
mod types;

use runtime::Runtime;

fn main() {
    let code = r#"
        let j = 0;

        for (let i = 0; i < 10; i++) {
            j = i;
        }
        
        return j;
    "#;

    let mut runtime = Runtime::new(code);
    runtime.execute();
}

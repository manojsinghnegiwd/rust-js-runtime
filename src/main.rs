mod interpreter;
mod lexer;
mod parser;
mod runtime;
mod scope;
mod types;

use runtime::Runtime;

fn main() {
    let code = r#"
        function fib (n) {
            if (n <= 1) {
                return n;
            }

            return fib(n - 1);
        }

        log(fib(100))
    "#;

    let mut runtime = Runtime::new(code);
    runtime.execute();
}

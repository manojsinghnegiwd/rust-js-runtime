mod parser;
mod types;
mod interpreter;
mod lexer;
mod scope;
mod runtime;

use runtime::Runtime;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assignment() {
        let code = r#"
            let a = 10;
            return a;
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(10.0));
    }
}
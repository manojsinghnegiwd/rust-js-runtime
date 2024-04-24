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

    #[test]
    fn addition() {
        let code = r#"
            let a = 10;
            let b = 20;
            return a + b;
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(30.0));
    }

    #[test]
    fn increment () {
        let code = r#"
            let a = 0;
            a++;
            return a;
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(1.0));
    }

    #[test]
    fn decrement () {
        let code = r#"
            let a = 1;
            a--;
            return a;
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(0.0));
    }

    #[test]
    fn function_call () {
        let code: &str = r#"
            function add (a, b) {
                return a + b;
            }

            return add(10, 20);
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(30.0));
    }

    #[test]
    fn function_call_stmt () {
        let code: &str = r#"
            function log_some (a) {
                return a;
            }

            1 + 1;

            return log_some(30);
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(30.0));
    }

    #[test]
    fn nested_function_call () {
        let code: &str = r#"
            function add (a, b) {
                return a + b;
            }

            function main (x, y) {
                return add(x, y);
            }

            return main(10, 20);
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();
    }

    #[test]
    fn loop_test () {
        let code: &str = r#"
            let i = 0;

            loop {
                if (i >= 10) {
                    break;
                }
                i++;
            }

            return i;
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(10.0));
    }

    #[test]
    fn if_test () {
        let code: &str = r#"
            let a = 10;

            if (a == 10) {
                return a;
            }
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(10.0));
    }

    #[test]
    fn while_test () {
        let code: &str = r#"
            let i = 0;

            while (i < 10) {
                i++;
            }

            return i;
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(10.0));
    }

    #[test]
    fn for_loop () {
        let code = r#"
            let j = 0;

            for (let i = 0; i < 10; i++) {
                j = i;
            }
            
            return j;
        "#;

        let mut runtime = Runtime::new(code);
        let output = runtime.execute();

        assert_eq!(output, types::Value::Float(9.0));
    }
}
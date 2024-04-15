# JS Compiler

This is a JavaScript runtime written in Rust.

# What it can do

* Define variable which can be number or string `let x = 0; let m = 'string'`
* Log variables `log(x)`
* Define comments `// This is a comment`

# Roadmap
* ~~Implement comments~~
* Implement math calculations
* Implement scope
* Implement function

## Project Structure

- `src/`: Contains the source code for the compiler.
  - `interpreter/`: Contains the interpreter for the JavaScript code.
  - `parser/`: Contains the parser for the JavaScript code.
  - `types/`: Contains the types used in the compiler.
- `target/`: Contains the build output from Cargo.

## Running

To run the project, run:

```sh
cargo run
```

## Building

To build the project, run:

```sh
cargo build
```
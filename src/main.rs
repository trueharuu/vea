#![allow(unused)]
#![feature(is_some_and, let_chains, option_result_contains)]

use lox::Lox;

pub mod ast;
pub mod ast_printer;
pub mod interpreter;
pub mod literal;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod env;
fn main() {
    let r =
        r#"var i = 0;
        while (i < 10) {
            print i;
            i = i + 1;
        }"#;
    Lox::new().run(r.to_string())
}


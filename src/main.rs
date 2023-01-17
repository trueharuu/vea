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
        r#"print 0.1"#;
    // inner a
    // outer b
    // global c
    // outer a
    // outer b
    // global c
    // global a
    // global b
    // global c
    Lox::new().run(r.to_string())
}
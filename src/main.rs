#![allow(dead_code)]
#![feature(is_some_and, let_chains, option_result_contains)]

use lox::Lox;

// use lox::Lox;
pub mod ast;
pub mod ast_printer;
pub mod interpreter;
pub mod literal;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;
fn main() {
    Lox::new().run("1 - 2".to_string())
}

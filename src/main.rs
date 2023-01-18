#![allow(unused)]
#![feature(is_some_and, let_chains, option_result_contains, result_flattening)]

use everest::Everest;

pub mod ast;
pub mod ast_printer;
pub mod interpreter;
pub mod literal;
pub mod everest;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod env;
mod callable;
fn main() {
    let r =
        r#"fn f(x) {
              print x;
           }
           
           f(1)"#;
    Everest::new().run(r.to_string())
}
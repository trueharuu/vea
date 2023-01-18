#![allow(unused)]
#![feature(is_some_and, let_chains, option_result_contains)]

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
        r#"var a = 0;
        var temp;
        
        for (var b = 1; a < 10000; b = temp + b) {
          print a;
          temp = a;
          a = b;
        }"#;
    Everest::new().run(r.to_string())
}
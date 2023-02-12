#![allow(unused_braces)]
#![feature(proc_macro_hygiene, iter_next_chunk)]
use std::collections::HashMap;

use interpreter::interp;
use lexer::Lexer;
use parser::parse;
pub mod lexer;
pub mod token;
pub mod ast;
pub mod tools;
pub mod parser;
pub mod interpreter;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    let s = r#"
        if (1 == 0) {
            print("ok");
        } else {
            print("a");
        };
    "#;
    println!("running:\n{}\n", s);
    let lexer = Lexer::new(&s);
    // dbg!(&s[100..=101]);
    let mut prog = parse(lexer).unwrap();
    interp(&mut prog, &mut HashMap::new());

    
}
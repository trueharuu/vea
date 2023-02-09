#![allow(unused_braces)]
#![feature(proc_macro_hygiene)]
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
    let s = "env obj; obj.a = 1; obj.a.b = 2; print obj.a.b;".to_owned();
    println!("running \"{s}\"");
    let lexer = Lexer::new(&s);
    let prog = parse(lexer).unwrap();
    interp(&prog);
}


#![allow(unused_braces)]
#![feature(proc_macro_hygiene, iter_next_chunk)]
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
    let s = "0u8 == 0u16;".to_owned();
    println!("running \"{s}\"");
    let lexer = Lexer::new(&s);
    let mut prog = parse(lexer).unwrap();
    interp(&mut prog);

    
}
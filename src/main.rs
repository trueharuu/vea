#![feature(is_some_and)]
pub mod expr;
pub mod literal;
pub mod lox;
pub mod scanner;
pub mod token;
fn main() {
    let mut args = vec![">="].into_iter().peekable();
    lox::Lox::new().exe(&mut args);
}

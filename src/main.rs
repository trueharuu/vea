#![allow(dead_code)]
#![feature(is_some_and)]

use ast::expr::Expr;
use ast_printer::AstPrinter;
use literal::Literal;
use token::{Token, TokenType};
pub mod ast;
pub mod ast_printer;
pub mod literal;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;
fn main() {
    let expr = Expr::Binary(
        Box::new(Expr::Unary(
            Token::new(TokenType::Minus, "-".to_string(), Literal::None, 1),
            Box::new(Expr::Literal(Literal::Number(123.0))),
        )),
        Token::new(TokenType::Star, "*".to_string(), Literal::None, 1),
        Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
            45.67,
        ))))),
    );

    println!("{}", AstPrinter.print(expr))
}

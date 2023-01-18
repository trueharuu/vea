// use std::fmt::Debug;

// use crate::{ ast::expr::Expr, interpreter::{ Interpreter, Value, RuntimeError } };

// pub struct Callable(pub Box<Expr>, pub u8);
// impl Callable {
//     pub fn new(expr: Box<Expr>, arity: u8) -> Self {
//         Self(expr, arity)
//     }
//     pub fn call(&self, interpreter: &Interpreter, argv: Vec<Value>) -> Result<Value, RuntimeError> {
//         todo!();
//     }

//     pub fn arity(&self) -> u8 {
//       self.1
//     }
// }
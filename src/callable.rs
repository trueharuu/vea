use crate::{interpreter::{Interpreter, RuntimeError}, literal::Literal, env::Env};

pub trait Callable {
  fn call(&self, interpreter: &mut Interpreter, argv: Vec<Literal>) -> Result<Literal, RuntimeError>;
  fn arity(&self) -> Result<u8, RuntimeError>;
}
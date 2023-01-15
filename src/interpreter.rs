use std::{ops::BitOr, error::Error, fmt::Display};

use crate::{
    ast::expr::{Expr, Visitor},
    literal::Literal,
    token::{TokenKind, Token},
};

pub struct Interpreter();

impl Interpreter {
    pub fn eval(&self, expr: &Expr) -> Value {
        expr.accept(self)
    }

    pub fn collapse(&self, expr: &Value) -> Literal {
        match expr {
            Value::Literal(x) => *x,
            Value::Expr(e) => self.collapse(&self.eval(e)),
        }
    }

    fn check_number_operand(&self, operator: Token, operand: Literal) {
      if !matches!(operand, Literal::Number(x)) {
        panic!("{}", RuntimeError::new(operator, "operand must be of type number".to_string()));
      }
    }
}

enum Value {
    Literal(Literal),
    Expr(Expr),
}

impl BitOr<Value> for &Interpreter {
    type Output = Literal;
    fn bitor(self, rhs: Value) -> Self::Output {
        self.collapse(&rhs)
    }
}

impl Visitor<Value> for Interpreter {
    fn visit_literal_expr(&self, expr: &Expr) -> Value {
        if let Expr::Literal(x) = expr {
            Value::Literal(*x)
        } else {
            unreachable!();
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Value {
        if let Expr::Grouping(x) = expr {
            self.eval(x)
        } else {
            unreachable!();
        }
    }

    fn visit_unary_expr(&self, expr: &Expr) -> Value {
        if let Expr::Unary(op, r) = expr {
            let right = self.eval(r);

            match op.kind {
                TokenKind::Minus => {
                    self.check_number_operand(*op, self|right);
                    Value::Literal(Literal::Number(-self.collapse(&right)))
                }
                TokenKind::Bang => Value::Literal(Literal::Boolean(!self.collapse(&right))),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }

    fn visit_binary_expr(&self, expr: &Expr) -> Value {
        if let Expr::Binary(l, op, r) = expr {
            let left = self.eval(l);
            let right = self.eval(r);

            match op.kind {
                TokenKind::Minus => {
                  Value::Literal(Literal::Number(
                    self.collapse(&left) - self.collapse(&right),
                ))},
                TokenKind::Slash => Value::Literal(Literal::Number(
                    self.collapse(&left) / self.collapse(&right),
                )),
                TokenKind::Star => Value::Literal(Literal::Number(
                    self.collapse(&left) * self.collapse(&right),
                )),
                TokenKind::Plus => if let Literal::String(x) = self.collapse(&left) && let Literal::String(y) = self.collapse(&left) {
                  Value::Literal(Literal::String(x + &y))
                } else {
                  Value::Literal(Literal::Number(self.collapse(&left).into_number() + self.collapse(&right).into_number()))
                },

                TokenKind::Gt => Value::Literal(Literal::Boolean(self|left > self|right)),
                TokenKind::Ge => Value::Literal(Literal::Boolean(self|left >= self|right)),
                TokenKind::Lt => Value::Literal(Literal::Boolean(self|left < self|right)),
                TokenKind::Le => Value::Literal(Literal::Boolean(self|left <= self|right)),

                TokenKind::Ne =>Value::Literal(Literal::Boolean(self|left != self|right)),
                TokenKind::Eq =>Value::Literal(Literal::Boolean(self|left == self|right)),

                _ => unreachable!(),
            }
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError(Token, String);

impl RuntimeError {
  pub fn new(token: Token, message: String) -> Self {
    Self(token, message)
  }
}

impl Display for RuntimeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{self:?}")
  }
}

impl Error for RuntimeError {}
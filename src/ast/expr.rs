use std::ops::Not;

use crate::{literal::Literal, token::Token};

#[derive(Clone, Debug)]
pub enum Expr {
    // name, value
    Assign(Token, Box<Expr>),
    // left, operator, right
    Binary(Box<Expr>, Token, Box<Expr>),
    // callee, paren, arguments
    Call(Box<Expr>, Token, Vec<Box<Expr>>),
    // source, name
    Get(Box<Expr>, Token),
    // source
    Grouping(Box<Expr>),
    // source
    Literal(Literal),
    // left, operator, right
    Logical(Box<Expr>, Token, Box<Expr>),
    // source, name, value
    Set(Box<Expr>, Token, Box<Expr>),
    // keyword, method
    Super(Token, Token),
    // keyword
    This(Token),
    // operator, right
    Unary(Token, Box<Expr>),
    // name
    Variable(Token),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        match self {
            Expr::Assign(_, _) => visitor.visit_assign_expr(self),
            Expr::Binary(_, _, _) => visitor.visit_binary_expr(self),
            Expr::Call(_, _, _) => visitor.visit_call_expr(self),
            Expr::Get(_, _) => visitor.visit_get_expr(self),
            Expr::Grouping(_) => visitor.visit_grouping_expr(self),
            Expr::Literal(_) => visitor.visit_literal_expr(self),
            Expr::Logical(_, _, _) => visitor.visit_logical_expr(self),
            Expr::Set(_, _, _) => visitor.visit_set_expr(self),
            Expr::Super(_, _) => visitor.visit_super_expr(self),
            Expr::This(_) => visitor.visit_this_expr(self),
            Expr::Unary(_, _) => visitor.visit_unary_expr(self),
            Expr::Variable(_) => visitor.visit_variable_expr(self),
        }
    }
}

pub trait Visitor<R> {
    fn visit_assign_expr(&self, expr: &Expr) -> R;
    fn visit_binary_expr(&self, expr: &Expr) -> R;
    fn visit_call_expr(&self, expr: &Expr) -> R;
    fn visit_get_expr(&self, expr: &Expr) -> R;
    fn visit_grouping_expr(&self, expr: &Expr) -> R;
    fn visit_literal_expr(&self, expr: &Expr) -> R;
    fn visit_logical_expr(&self, expr: &Expr) -> R;
    fn visit_set_expr(&self, expr: &Expr) -> R;
    fn visit_super_expr(&self, expr: &Expr) -> R;
    fn visit_this_expr(&self, expr: &Expr) -> R;
    fn visit_unary_expr(&self, expr: &Expr) -> R;
    fn visit_variable_expr(&self, expr: &Expr) -> R;
}

impl Not for Expr {
  type Output = Box<Self>;
  fn not(self) -> Self::Output {
      Box::new(self)
  }
}
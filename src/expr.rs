use std::ops::Deref;

use crate::token::Token;

pub struct Expr;

pub struct Binary {
    base: Expr,
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
            base: Expr,
        }
    }
}

impl Deref for Binary {
    type Target = Expr;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

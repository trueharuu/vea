use std::ops::Not;

use crate::{ literal::Literal, token::Token };

use super::expr::Expr;

#[derive(Clone, Debug)]
pub enum Stmt {
    // statements
    Block(Vec<Stmt>),
    // name, superclass, methods
    Class(Token, Expr, Vec<Stmt>),
    // expression
    Expression(Expr),
    // name, params, body
    Fn(Token, Vec<Token>, Vec<Stmt>),
    // condition, then, else
    If(Expr, Box<Stmt>, Box<Stmt>),
    // expr
    Print(Expr),
    // keyword, value
    Return(Token, Expr),
    // name, initializer
    Var(Token, Expr),
    // condition, body
    While(Expr, Box<Stmt>),
}

impl Stmt {
    pub fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        match self {
            Self::Block(_) => visitor.visit_block_stmt(self),
            Self::Class(_, _, _) => visitor.visit_class_stmt(self),
            Self::Expression(_) => visitor.visit_expression_stmt(self),
            Self::Fn(_, _, _) => visitor.visit_fn_stmt(self),
            Self::If(_, _, _) => visitor.visit_if_stmt(self),
            Self::Print(_) => visitor.visit_print_stmt(self),
            Self::Return(_, _) => visitor.visit_return_stmt(self),
            Self::Var(_, _) => visitor.visit_var_stmt(self),
            Self::While(_, _) => visitor.visit_while_stmt(self),
            _ => unreachable!(),
        }
    }
}

pub trait StmtVisitor<R: ?Sized> {
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_class_stmt(&self, stmt: &Stmt) -> R;
    fn visit_expression_stmt(&self, stmt: &Stmt) -> R;
    fn visit_fn_stmt(&self, stmt: &Stmt) -> R;
    fn visit_if_stmt(&self, stmt: &Stmt) -> R;
    fn visit_print_stmt(&self, stmt: &Stmt) -> R;
    fn visit_return_stmt(&self, stmt: &Stmt) -> R;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_while_stmt(&self, stmt: &Stmt) -> R;
}

impl Not for Stmt {
    type Output = Box<Self>;
    fn not(self) -> Self::Output {
        Box::new(self)
    }
}

impl Not for &Stmt {
    type Output = Box<Stmt>;
    fn not(self) -> Self::Output {
        Box::new(self.clone())
    }
}
use std::fmt::{Debug, Display};

use crate::{interpreter::Env, lexer::Span};

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Expr(pub Span, pub Node);

pub type Bex = Box<Expr>;

#[derive(Debug, Clone)]
pub enum Node {
    Add(Bex, Bex), // a + b
    Sub(Bex, Bex), // a - b
    Mul(Bex, Bex), // a * b
    Div(Bex, Bex), // a / b
    Rem(Bex, Bex), // a % b

    Eq(Bex, Bex), // a == b
    Ne(Bex, Bex), // a != b
    Gt(Bex, Bex), // a > b
    Ge(Bex, Bex), // a >= b
    Lt(Bex, Bex), // a < b
    Le(Bex, Bex), // a <= b

    Inv(Bex), // !x
    Not(Bex), // ~x
    Neg(Bex), // -x

    Or(Bex, Bex),  // a | b
    And(Bex, Bex), // a & b
    Xor(Bex, Bex), // a ^ b
    Shl(Bex, Bex), // a << b
    Shr(Bex, Bex), // a >> b

    Pair(Bex, Bex), // a, b

    Var(String),      // x
    Let(String, Bex), // let a = b

    Print(Bex),  // print x
    Typeof(Bex), // typeof x

    Literal(Literal), // 123

    Set(Bex, Bex),    // a = b
    Get(Vec<String>), // a.b

    If(Bex, Vec<Expr>, Option<Vec<Expr>>), //    if (cond) { x } else { y }
    While(Bex, Vec<Expr>),                 // while (cond) { x } else { y }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    String(String),
    Boolean(bool),
    Object(Env),
    Never,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Integer(b) => write!(f, "{b}"),
            Self::String(b) => write!(f, "{b}"),
            Self::Object(_) => write!(f, "object"),
            Self::Never => write!(f, "!"),
        }
    }
}

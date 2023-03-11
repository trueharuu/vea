use std::fmt::Debug;

use crate::{lexer::Span, literal::Literal, tools::Named};

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

    Pair(Bex, Bex),     // a, b
    Array(Option<Bex>), // [a, b]
    List(Option<Bex>),  // { a, b }

    Var(String),      // x
    Let(String, Bex), // let a = b

    Print(Bex),  // print x
    Typeof(Bex), // typeof x

    Literal(Literal), // 123

    Set(Bex, Bex),    // a = b
    Get(Vec<String>), // a.b

    If(Bex, Vec<Expr>, Option<Vec<Expr>>), //    if (cond) { x } else { y }
    While(Bex, Vec<Expr>),                 // while (cond) { x } else { y }

    Block(Vec<Expr>, Option<Box<Expr>>), // { x; y }
}

impl Named for Node {
    fn name(&self) -> String {
        match self {
            Self::Add(..) => "Add".to_string(),
            Self::Sub(..) => "Sub".to_string(),
            Self::Mul(..) => "Mul".to_string(),
            Self::Div(..) => "Div".to_string(),
            Self::Rem(..) => "Rem".to_string(),

            Self::Eq(..) => "Eq".to_string(),
            Self::Ne(..) => "Ne".to_string(),
            Self::Gt(..) => "Gt".to_string(),
            Self::Ge(..) => "Ge".to_string(),
            Self::Lt(..) => "Lt".to_string(),
            Self::Le(..) => "Le".to_string(),

            Self::Inv(..) => "Inv".to_string(),
            Self::Not(..) => "Not".to_string(),
            Self::Neg(..) => "Neg".to_string(),

            Self::Or(..) => "Or".to_string(),
            Self::And(..) => "And".to_string(),
            Self::Xor(..) => "Xor".to_string(),
            Self::Shl(..) => "Shl".to_string(),
            Self::Shr(..) => "Shr".to_string(),

            Self::Pair(..) => "Pair".to_string(),

            Self::Var(..) => "Var".to_string(),
            Self::Let(..) => "Let".to_string(),

            Self::Print(..) => "Print".to_string(),
            Self::Typeof(..) => "Typeof".to_string(),

            Self::Literal(..) => "Literal".to_string(),

            Self::Set(..) => "Set".to_string(),
            Self::Get(..) => "Get".to_string(),

            Self::If(..) => "If".to_string(),
            Self::While(..) => "While".to_string(),

            Self::Block(..) => "Block".to_string(),

            Self::Array(..) => "Array".to_string(),
            Self::List(..) => "List".to_string(),

            _ => format!("{self:?}"),
        }
    }
}

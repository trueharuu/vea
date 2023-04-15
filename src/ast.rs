use crate::common::Span;
use crate::common::Spanned;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Add(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Sub(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Mul(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Div(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Gt(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Ge(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Lt(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Le(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Eq(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Ne(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Let(&'a str, Box<Spanned<Self>>),
    Literal(Spanned<Literal>),
    Variable(&'a str),
    List(Vec<Spanned<Self>>),
    Set(Vec<Spanned<Self>>),
    Print(Box<Spanned<Self>>),
    Neg(Box<Spanned<Self>>),
    Not(Box<Spanned<Self>>),
    None,
}

impl<'a> Expr<'a> {
    pub fn span(&self) -> Span {
        match self {
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Gt(a, b)
            | Expr::Ge(a, b)
            | Expr::Lt(a, b)
            | Expr::Le(a, b)
            | Expr::Eq(a, b)
            | Expr::Ne(a, b) => (a.1.start..b.1.end).into(),

            Expr::Literal(a) => a.1,

            _ => (0..0).into(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Add(..) => "Add",
            Self::Sub(..) => "Sub",
            Self::Mul(..) => "Mul",
            Self::Div(..) => "Div",
            Self::Gt(..) => "Gt",
            Self::Ge(..) => "Ge",
            Self::Lt(..) => "Lt",
            Self::Le(..) => "Le",
            Self::Eq(..) => "Eq",
            Self::Ne(..) => "Ne",
            Self::Let(..) => "Let",
            Self::Literal(..) => "Literal",
            Self::Variable(..) => "Variable",
            Self::List(..) => "List",
            Self::Set(..) => "Set",
            Self::Print(..) => "Print",
            Self::Neg(..) => "Neg",
            Self::Not(..) => "Not",
            Self::None => "None",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
    // never created in user code
    None,
}

impl Literal {
    pub fn type_of(&self) -> String {
        match self {
            Self::Number(_) => "int",
            Self::String(_) => "str",
            Self::Boolean(_) => "bool",
            _ => "",
        }
        .to_string()
    }

    pub fn ee(&self, rhs: &Self) -> Result<Self, String> {
        // dbg!("1");
        if self.type_of() != rhs.type_of() {
            Err(format!(
                "implementation of `{} == {}` does not exist",
                self.type_of(),
                rhs.type_of()
            ))
        } else {
            Ok(Literal::Boolean(self == rhs))
        }
    }
}

impl ::std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean(b) => b.to_string(),
                Self::Number(b) => b.to_string(),
                Self::String(b) => b.to_string(),
                _ => "".to_string(),
            }
        )
    }
}

impl ::std::ops::Add for Literal {
    type Output = Result<Self, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::String(a), Literal::String(b)) => Ok(Literal::String(a + &b)),
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a + b)),
            (a, b) => Err(format!(
                "implementation for `{} + {}` does not exist",
                a.type_of(),
                b.type_of()
            )),
        }
    }
}

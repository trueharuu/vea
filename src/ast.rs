use std::{ fmt::Display, collections::HashMap, ops::{ Add, Sub, Mul, Div } };

use crate::{ lexer::Span, token::Integer, b };

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub node: Node,
}

#[derive(Debug, Clone)]
pub enum Node {
    Add(b![Expr], b![Expr]),
    Sub(b![Expr], b![Expr]),
    Mul(b![Expr], b![Expr]),
    Div(b![Expr], b![Expr]),
    Pair(b![Expr], b![Expr]),
    Eq(b![Expr], b![Expr]),
    Ne(b![Expr], b![Expr]),
    Gt(b![Expr], b![Expr]),
    Lt(b![Expr], b![Expr]),
    Ge(b![Expr], b![Expr]),
    Le(b![Expr], b![Expr]),

    Var(String),
    Assign(String, b![Expr]),
    Print(b![Expr]),
    Typeof(b![Expr]),
    Literal(Literal),
    Env(String),
    InnerEnv(b![Expr]),
    Set(b![Expr], b![Expr]),
    Get(String, Vec<String>),

    List(b![Expr]),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(Integer),
    String(String),
    Boolean(bool),
    Array(Vec<Literal>),
    Set(Vec<Literal>),
    Object(HashMap<String, Literal>),
    Never,
}

impl Default for Literal {
    fn default() -> Self {
        Self::Never
    }
}

impl Literal {
    pub fn assert_object(&self) -> &HashMap<String, Literal> {
        if let Self::Object(v) = self { v } else { panic!("assertion failed: typeof x == object") }
    }
    pub fn type_of(&self) -> String {
        match self {
            Self::Boolean(_) => "bool".to_owned(),
            Self::Integer(Integer::I8(_)) => "i8".to_owned(),
            Self::Integer(Integer::I16(_)) => "i16".to_owned(),
            Self::Integer(Integer::I32(_)) => "i32".to_owned(),
            Self::Integer(Integer::I64(_)) => "i64".to_owned(),
            Self::Integer(Integer::I128(_)) => "i128".to_owned(),
            Self::Integer(Integer::ISize(_)) => "isize".to_owned(),
            Self::Integer(Integer::U8(_)) => "u8".to_owned(),
            Self::Integer(Integer::U16(_)) => "u16".to_owned(),
            Self::Integer(Integer::U32(_)) => "u32".to_owned(),
            Self::Integer(Integer::U64(_)) => "u64".to_owned(),
            Self::Integer(Integer::U128(_)) => "u128".to_owned(),
            Self::Integer(Integer::USize(_)) => "usize".to_owned(),
            Self::String(_) => "str".to_owned(),
            Self::Array(v) => {
                if v.is_empty() {
                    "Array()".to_owned()
                } else {
                    let mut types = Vec::new();
                    for i in v {
                        if !types.contains(&i.type_of()) {
                            types.push(i.type_of());
                        }
                    }

                    format!("Array({})", types.join(" | "))
                }
            }
            Self::Set(v) => {
                if v.is_empty() {
                    "Set()".to_owned()
                } else {
                    let mut types = Vec::new();
                    for i in v {
                        if !types.contains(&i.type_of()) {
                            types.push(i.type_of());
                        }
                    }

                    format!("Set({})", types.join(" | "))
                }
            }

            Self::Object(_) => "object".to_owned(),
            Self::Never => "never".to_owned(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Boolean(b) => b.to_string(),
            Self::String(s) => s.to_string(),
            Self::Array(a) => format!("{a:?}"),
            Self::Object(o) => format!("{o:?}"),
            Self::Never => "None".to_string(),
            i => i.to_string(),
        })
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(i), Self::String(u)) => i == u,
            (Self::Boolean(i), Self::Boolean(u)) => i == u,
            (Self::Array(i), Self::Array(u)) => i == u,
            (Self::Never, Self::Never) => true,
            (Self::Integer(Integer::I8(l)), Self::Integer(Integer::I8(r))) => l == r,
            (Self::Integer(Integer::I16(l)), Self::Integer(Integer::I16(r))) => l == r,
            (Self::Integer(Integer::I32(l)), Self::Integer(Integer::I32(r))) => l == r,
            (Self::Integer(Integer::I64(l)), Self::Integer(Integer::I64(r))) => l == r,
            (Self::Integer(Integer::I128(l)), Self::Integer(Integer::I128(r))) => l == r,
            (Self::Integer(Integer::ISize(l)), Self::Integer(Integer::ISize(r))) => l == r,
            (Self::Integer(Integer::U8(l)), Self::Integer(Integer::U8(r))) => l == r,
            (Self::Integer(Integer::U16(l)), Self::Integer(Integer::U16(r))) => l == r,
            (Self::Integer(Integer::U32(l)), Self::Integer(Integer::U32(r))) => l == r,
            (Self::Integer(Integer::U64(l)), Self::Integer(Integer::U64(r))) => l == r,
            (Self::Integer(Integer::U128(l)), Self::Integer(Integer::U128(r))) => l == r,
            (Self::Integer(Integer::USize(l)), Self::Integer(Integer::USize(r))) => l == r,
            (i, u) => panic!("cannot compare `{} == {}`", i.type_of(), u.type_of()),
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(Integer::I8(l)), Self::Integer(Integer::I8(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::I16(l)), Self::Integer(Integer::I16(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::I32(l)), Self::Integer(Integer::I32(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::I64(l)), Self::Integer(Integer::I64(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::I128(l)), Self::Integer(Integer::I128(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::ISize(l)), Self::Integer(Integer::ISize(r))) =>
                l.partial_cmp(r),
            (Self::Integer(Integer::U8(l)), Self::Integer(Integer::U8(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::U16(l)), Self::Integer(Integer::U16(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::U32(l)), Self::Integer(Integer::U32(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::U64(l)), Self::Integer(Integer::U64(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::U128(l)), Self::Integer(Integer::U128(r))) => l.partial_cmp(r),
            (Self::Integer(Integer::USize(l)), Self::Integer(Integer::USize(r))) =>
                l.partial_cmp(r),
            (_, _) => None,
        }
    }
}

impl Add for Literal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Array(v), Self::Array(b)) => Self::Array(vec![v, b].concat()),
            (Self::Integer(Integer::I8(l)), Self::Integer(Integer::I8(r))) =>
                Self::Integer(Integer::I8(l + r)),
            (Self::Integer(Integer::I16(l)), Self::Integer(Integer::I16(r))) =>
                Self::Integer(Integer::I16(l + r)),
            (Self::Integer(Integer::I32(l)), Self::Integer(Integer::I32(r))) =>
                Self::Integer(Integer::I32(l + r)),
            (Self::Integer(Integer::I64(l)), Self::Integer(Integer::I64(r))) =>
                Self::Integer(Integer::I64(l + r)),
            (Self::Integer(Integer::I128(l)), Self::Integer(Integer::I128(r))) =>
                Self::Integer(Integer::I128(l + r)),
            (Self::Integer(Integer::ISize(l)), Self::Integer(Integer::ISize(r))) =>
                Self::Integer(Integer::ISize(l + r)),
            (Self::Integer(Integer::U8(l)), Self::Integer(Integer::U8(r))) =>
                Self::Integer(Integer::U8(l + r)),
            (Self::Integer(Integer::U16(l)), Self::Integer(Integer::U16(r))) =>
                Self::Integer(Integer::U16(l + r)),
            (Self::Integer(Integer::U32(l)), Self::Integer(Integer::U32(r))) =>
                Self::Integer(Integer::U32(l + r)),
            (Self::Integer(Integer::U64(l)), Self::Integer(Integer::U64(r))) =>
                Self::Integer(Integer::U64(l + r)),
            (Self::Integer(Integer::U128(l)), Self::Integer(Integer::U128(r))) =>
                Self::Integer(Integer::U128(l + r)),
            (Self::Integer(Integer::USize(l)), Self::Integer(Integer::USize(r))) =>
                Self::Integer(Integer::USize(l + r)),
            (Self::String(l), Self::String(r)) => Self::String(l + r.as_str()),
            (i, o) =>
                panic!(
                    "operation `{0} + {1}` failed: Add({1}) not implemented for {0}",
                    i.type_of(),
                    o.type_of()
                ),
        }
    }
}

impl Sub for Literal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(Integer::I8(l)), Self::Integer(Integer::I8(r))) =>
                Self::Integer(Integer::I8(l - r)),
            (Self::Integer(Integer::I16(l)), Self::Integer(Integer::I16(r))) =>
                Self::Integer(Integer::I16(l - r)),
            (Self::Integer(Integer::I32(l)), Self::Integer(Integer::I32(r))) =>
                Self::Integer(Integer::I32(l - r)),
            (Self::Integer(Integer::I64(l)), Self::Integer(Integer::I64(r))) =>
                Self::Integer(Integer::I64(l - r)),
            (Self::Integer(Integer::I128(l)), Self::Integer(Integer::I128(r))) =>
                Self::Integer(Integer::I128(l - r)),
            (Self::Integer(Integer::ISize(l)), Self::Integer(Integer::ISize(r))) =>
                Self::Integer(Integer::ISize(l - r)),
            (Self::Integer(Integer::U8(l)), Self::Integer(Integer::U8(r))) =>
                Self::Integer(Integer::U8(l - r)),
            (Self::Integer(Integer::U16(l)), Self::Integer(Integer::U16(r))) =>
                Self::Integer(Integer::U16(l - r)),
            (Self::Integer(Integer::U32(l)), Self::Integer(Integer::U32(r))) =>
                Self::Integer(Integer::U32(l - r)),
            (Self::Integer(Integer::U64(l)), Self::Integer(Integer::U64(r))) =>
                Self::Integer(Integer::U64(l - r)),
            (Self::Integer(Integer::U128(l)), Self::Integer(Integer::U128(r))) =>
                Self::Integer(Integer::U128(l - r)),
            (Self::Integer(Integer::USize(l)), Self::Integer(Integer::USize(r))) =>
                Self::Integer(Integer::USize(l - r)),
            (i, o) =>
                panic!(
                    "operation `{0} - {1}` failed: Sub({1}) not implemented for {0}",
                    i.type_of(),
                    o.type_of()
                ),
        }
    }
}

impl Mul for Literal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(Integer::I8(l)), Self::Integer(Integer::I8(r))) =>
                Self::Integer(Integer::I8(l * r)),
            (Self::Integer(Integer::I16(l)), Self::Integer(Integer::I16(r))) =>
                Self::Integer(Integer::I16(l * r)),
            (Self::Integer(Integer::I32(l)), Self::Integer(Integer::I32(r))) =>
                Self::Integer(Integer::I32(l * r)),
            (Self::Integer(Integer::I64(l)), Self::Integer(Integer::I64(r))) =>
                Self::Integer(Integer::I64(l * r)),
            (Self::Integer(Integer::I128(l)), Self::Integer(Integer::I128(r))) =>
                Self::Integer(Integer::I128(l * r)),
            (Self::Integer(Integer::ISize(l)), Self::Integer(Integer::ISize(r))) =>
                Self::Integer(Integer::ISize(l * r)),
            (Self::Integer(Integer::U8(l)), Self::Integer(Integer::U8(r))) =>
                Self::Integer(Integer::U8(l * r)),
            (Self::Integer(Integer::U16(l)), Self::Integer(Integer::U16(r))) =>
                Self::Integer(Integer::U16(l * r)),
            (Self::Integer(Integer::U32(l)), Self::Integer(Integer::U32(r))) =>
                Self::Integer(Integer::U32(l * r)),
            (Self::Integer(Integer::U64(l)), Self::Integer(Integer::U64(r))) =>
                Self::Integer(Integer::U64(l * r)),
            (Self::Integer(Integer::U128(l)), Self::Integer(Integer::U128(r))) =>
                Self::Integer(Integer::U128(l * r)),
            (Self::Integer(Integer::USize(l)), Self::Integer(Integer::USize(r))) =>
                Self::Integer(Integer::USize(l * r)),
            (i, o) =>
                panic!(
                    "operation `{0} * {1}` failed: Mul({1}) not implemented for {0}",
                    i.type_of(),
                    o.type_of()
                ),
        }
    }
}
impl Div for Literal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(Integer::I8(l)), Self::Integer(Integer::I8(r))) =>
                Self::Integer(Integer::I8(l / r)),
            (Self::Integer(Integer::I16(l)), Self::Integer(Integer::I16(r))) =>
                Self::Integer(Integer::I16(l / r)),
            (Self::Integer(Integer::I32(l)), Self::Integer(Integer::I32(r))) =>
                Self::Integer(Integer::I32(l / r)),
            (Self::Integer(Integer::I64(l)), Self::Integer(Integer::I64(r))) =>
                Self::Integer(Integer::I64(l / r)),
            (Self::Integer(Integer::I128(l)), Self::Integer(Integer::I128(r))) =>
                Self::Integer(Integer::I128(l / r)),
            (Self::Integer(Integer::ISize(l)), Self::Integer(Integer::ISize(r))) =>
                Self::Integer(Integer::ISize(l / r)),
            (Self::Integer(Integer::U8(l)), Self::Integer(Integer::U8(r))) =>
                Self::Integer(Integer::U8(l / r)),
            (Self::Integer(Integer::U16(l)), Self::Integer(Integer::U16(r))) =>
                Self::Integer(Integer::U16(l / r)),
            (Self::Integer(Integer::U32(l)), Self::Integer(Integer::U32(r))) =>
                Self::Integer(Integer::U32(l / r)),
            (Self::Integer(Integer::U64(l)), Self::Integer(Integer::U64(r))) =>
                Self::Integer(Integer::U64(l / r)),
            (Self::Integer(Integer::U128(l)), Self::Integer(Integer::U128(r))) =>
                Self::Integer(Integer::U128(l / r)),
            (Self::Integer(Integer::USize(l)), Self::Integer(Integer::USize(r))) =>
                Self::Integer(Integer::USize(l / r)),
            (i, o) =>
                panic!(
                    "operation `{0} / {1}` failed: Div({1}) not implemented for {0}",
                    i.type_of(),
                    o.type_of()
                ),
        }
    }
}
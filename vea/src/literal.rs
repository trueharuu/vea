use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
    rc::Rc,
};

use crate::{ast::Expr, span::Span};

#[derive(Clone, Debug)]
pub enum Literal<'a> {
    Bool(bool),
    Integer(i64),
    String(String),
    // name, args, body
    Fn(Span<&'a str>, Vec<Span<&'a str>>, Box<Span<Expr<'a>>>),
    Object(HashMap<&'a str, Rc<RefCell<Self>>>),
    Set(Vec<Rc<RefCell<Self>>>),
    None,
}

impl<'a> Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(z) => z.to_string(),
                Self::Integer(z) => z.to_string(),
                Self::String(z) => z.to_string(),
                Self::Fn(z, a, ..) => format!("fn {}({})", z.0, a.len()),
                Self::Object(z) => format!(
                    "object {{ {} }}",
                    z.keys().copied().collect::<Vec<_>>().join(", ")
                ),
                Self::Set(p) => format!(
                    "set {{ {} }}",
                    p.iter()
                        .map(|x| x.borrow().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Self::None => "_".to_string(),
            }
        )
    }
}

impl<'a> Literal<'a> {
    #[must_use]
    pub fn type_of(&self) -> String {
        match self {
            Self::Bool(..) => "bool",
            Self::Integer(..) => "int",
            Self::String(..) => "str",
            Self::Fn(..) => "fn",
            Self::Object(..) => "object",
            Self::None => "_",
            Self::Set(..) => "set",
        }
        .to_owned()
    }

    pub fn req(self, rhs: Self) -> Result<Self, String> {
        match (self, rhs) {
            (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs == rhs)),
            (Self::String(lhs), Self::String(rhs)) => Ok(Self::Bool(lhs == rhs)),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Bool(lhs == rhs)),

            (lhs, rhs) => Err(format!(
                "cannot test equality `{} == {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }

    pub fn rne(self, rhs: Self) -> Result<Self, String> {
        match (self, rhs) {
            (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs != rhs)),
            (Self::String(lhs), Self::String(rhs)) => Ok(Self::Bool(lhs != rhs)),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Bool(lhs != rhs)),
            (lhs, rhs) => Err(format!(
                "cannot test inequality `{} != {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }

    pub fn rgt(self, rhs: Self) -> Result<Self, String> {
        match (self, rhs) {
            // (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs > rhs)),
            // (Self::String(lhs), Self::String(rhs)) => Ok(Self::Bool(lhs > rhs)),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Bool(lhs > rhs)),
            (lhs, rhs) => Err(format!(
                "cannot test inequality `{} > {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }

    pub fn rge(self, rhs: Self) -> Result<Self, String> {
        match (self, rhs) {
            // (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs >= rhs)),
            // (Self::String(lhs), Self::String(rhs)) => Ok(Self::Bool(lhs >= rhs)),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Bool(lhs >= rhs)),
            (lhs, rhs) => Err(format!(
                "cannot test inequality `{} >= {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }

    pub fn rlt(self, rhs: Self) -> Result<Self, String> {
        match (self, rhs) {
            // (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs < rhs)),
            // (Self::String(lhs), Self::String(rhs)) => Ok(Self::Bool(lhs < rhs)),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Bool(lhs < rhs)),
            (lhs, rhs) => Err(format!(
                "cannot test inequality `{} < {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }

    pub fn rle(self, rhs: Self) -> Result<Self, String> {
        match (self, rhs) {
            // (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs <= rhs)),
            // (Self::String(lhs), Self::String(rhs)) => Ok(Self::Bool(lhs <= rhs)),
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Bool(lhs <= rhs)),
            (lhs, rhs) => Err(format!(
                "cannot test inequality `{} <= {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Add for Literal<'a> {
    type Output = Result<Self, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs + rhs)),
            (Self::String(lhs), Self::String(rhs)) => Ok(Self::String(lhs + &rhs)),
            (lhs, rhs) => Err(format!(
                "cannot add `{} + {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Sub for Literal<'a> {
    type Output = Result<Self, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs - rhs)),

            (lhs, rhs) => Err(format!(
                "cannot subtract `{} - {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Mul for Literal<'a> {
    type Output = Result<Self, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs * rhs)),

            (lhs, rhs) => Err(format!(
                "cannot multiply `{} * {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Div for Literal<'a> {
    type Output = Result<Self, String>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => {
                if rhs == 0 {
                    return Err("cannot divide by zero".to_string());
                }

                Ok(Self::Integer(lhs / rhs))
            }

            (lhs, rhs) => Err(format!(
                "cannot divide `{} / {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Rem for Literal<'a> {
    type Output = Result<Self, String>;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => {
                if rhs == 0 {
                    return Err("cannot divide by zero".to_string());
                }

                Ok(Self::Integer(lhs % rhs))
            }

            (lhs, rhs) => Err(format!(
                "cannot get remainder for `{} % {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Shl for Literal<'a> {
    type Output = Result<Self, String>;
    fn shl(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs << rhs)),

            (lhs, rhs) => Err(format!(
                "cannot bit-shift-left `{} << {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Shr for Literal<'a> {
    type Output = Result<Self, String>;
    fn shr(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs >> rhs)),

            (lhs, rhs) => Err(format!(
                "cannot bit-shift-right `{} >> {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> BitAnd for Literal<'a> {
    type Output = Result<Self, String>;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs & rhs)),
            (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs & rhs)),

            (lhs, rhs) => Err(format!(
                "cannot bit-and `{} & {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> BitOr for Literal<'a> {
    type Output = Result<Self, String>;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs | rhs)),
            (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs | rhs)),

            (lhs, rhs) => Err(format!(
                "cannot bit-or `{} | {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> BitXor for Literal<'a> {
    type Output = Result<Self, String>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs ^ rhs)),
            (Self::Bool(lhs), Self::Bool(rhs)) => Ok(Self::Bool(lhs ^ rhs)),

            (lhs, rhs) => Err(format!(
                "cannot bit-xor `{} ^ {}`",
                lhs.type_of(),
                rhs.type_of()
            )),
        }
    }
}

impl<'a> Neg for Literal<'a> {
    type Output = Result<Self, String>;
    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(lhs) => Ok(Self::Integer(-lhs)),

            lhs => Err(format!("cannot negate `-{}`", lhs.type_of(),)),
        }
    }
}

impl<'a> Not for Literal<'a> {
    type Output = Result<Self, String>;
    fn not(self) -> Self::Output {
        match self {
            Self::Bool(lhs) => Ok(Self::Bool(!lhs)),

            lhs => Err(format!("cannot get inverse `!{}`", lhs.type_of(),)),
        }
    }
}

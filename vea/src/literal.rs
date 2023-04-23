use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Sub;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    Bool(bool),
    Integer(i64),
    String(String),
    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(z) => z.to_string(),
                Self::Integer(z) => z.to_string(),
                Self::String(z) => z.to_string(),
                Self::None => "_".to_string(),
            }
        )
    }
}

impl Literal {
    #[must_use]
    pub fn type_of(&self) -> String {
        match self {
            Self::Bool(..) => "bool",
            Self::Integer(..) => "int",
            Self::String(..) => "str",
            Self::None => "_",
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

impl Add for Literal {
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

impl Sub for Literal {
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

impl Mul for Literal {
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

impl Div for Literal {
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

impl Neg for Literal {
    type Output = Result<Self, String>;
    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(lhs) => Ok(Self::Integer(-lhs)),

            lhs => Err(format!("cannot negate `-{}`", lhs.type_of(),)),
        }
    }
}

impl Not for Literal {
    type Output = Result<Self, String>;
    fn not(self) -> Self::Output {
        match self {
            Self::Bool(lhs) => Ok(Self::Bool(!lhs)),

            lhs => Err(format!("cannot get inverse `!{}`", lhs.type_of(),)),
        }
    }
}

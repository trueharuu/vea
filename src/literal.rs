use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::interpreter::Env;

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    String(String),
    Boolean(bool),
    Object(Env),
    List(Vec<Literal>),
    Set(Vec<Literal>),
    Never,
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Object(a), Self::Object(b)) => a.values == b.values,
            (a, b) => a == b,
        }
    }
}

impl Literal {
    pub fn type_of(&self) -> String {
        match self {
            Self::Boolean(..) => "bool".to_owned(),
            Self::Integer(..) => "integer".to_owned(),
            Self::Never => "!".to_owned(),
            Self::Object(..) => "object".to_owned(),
            Self::String(..) => "str".to_owned(),
            Self::List(i) => {
                let mut types = Vec::new();

                for p in i {
                    if !types.contains(&p.type_of()) {
                        types.push(p.type_of());
                    }
                }

                if !types.is_empty() {
                    format!("list({})", types.join(" | "))
                } else {
                    "list".to_string()
                }
            }
            Self::Set(i) => {
                if let Some(p) = i.first() {
                    format!("set({})", p.type_of())
                } else {
                    "set".to_string()
                }
            }
        }
    }

    pub fn not(&self) -> Result<Self, String> {
        // ~x
        match self {
            Self::Boolean(z) => Ok(Self::Boolean(!z)),
            Self::Integer(p) => Ok(Self::Integer(!p)),
            p => Err(format!(
                "Failed to evaluate `~{p}`: Not not implemented for `{}`",
                self.type_of()
            )),
        }
    }

    pub fn inv(&self) -> Result<Self, String> {
        // !x
        match self {
            Self::Boolean(z) => Ok(Self::Boolean(!z)),
            Self::Integer(p) => Ok(Self::Boolean(*p != 0)),
            p => Err(format!(
                "Failed to evaluate `!{p}`: Inv not implemented for `{}`",
                self.type_of()
            )),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // println!("hit fmt!");
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Integer(b) => write!(f, "{b}"),
            Self::String(b) => write!(f, "{b}"),
            Self::Object(_) => write!(f, "object"),
            Self::Never => write!(f, "!"),
            Self::List(i) => write!(
                f,
                "[ {} ]",
                i.iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Set(i) => write!(
                f,
                "{{ {} }}",
                i.iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl Add for Literal {
    type Output = Result<Self, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // 1 + 2 = 3
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a + b)),
            // "a" + "b" = "ab"
            (Self::String(a), Self::String(b)) => Ok(Self::String(a + &b)),
            // [1, 2] + [2, 3] = [1, 2, 2, 3]
            (Self::List(a), Self::List(b)) => Ok(Self::List(vec![a, b].concat())),
            // [1, 2] + 3 = [1, 2, 3]
            (Self::List(a), b) => Ok(Self::List(vec![a, vec![b]].concat())),
            // { 1, 2 } + { 2, 3 } = { 1, 2, 3 }
            (Self::Set(a), Self::Set(b)) => {
                if a.is_empty() {
                    return Ok(Self::Set(b));
                }

                if b.is_empty() {
                    return Ok(Self::Set(a));
                }

                if Self::Set(a.clone()).type_of() != Self::Set(b.clone()).type_of() {
                    return Err(format!(
                        "Failed to evaluate `{} + {}`: Cannot join sets `{}` and `{}`",
                        Self::Set(a.clone()),
                        Self::Set(b.clone()),
                        Self::Set(a).type_of(),
                        Self::Set(b).type_of(),
                    ));
                } else {
                    let mut v = Vec::from_iter(a);
                    for i in b {
                        if !v.contains(&i) {
                            v.push(i);
                        }
                    }

                    Ok(Self::Set(v))
                }
            }
            // { 1, 2, 3 } + 3 = { 1, 2, 3 }
            (Self::Set(a), b) => {
                if a.is_empty() {
                    return Ok(Self::Set(vec![b]));
                }

                if a.first().map(|x| x.type_of()) != Some(b.type_of()) {
                    return Err(format!(
                        "Failed to evaluate `{} + {}`: Cannot add item of type `{}` to a `{}`",
                        Self::Set(a.clone()),
                        b,
                        Self::Set(a).type_of(),
                        b.type_of(),
                    ));
                } else {
                    Ok(Self::Set(vec![a, vec![b]].concat()))
                }
            }
            (a, b) => Err(format!(
                "Failed to evaluate `{} + {}`: Add not implemented for {}, {}",
                a,
                b,
                a.type_of(),
                b.type_of()
            )),
        }
    }
}
impl Sub for Literal {
    type Output = Result<Self, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a - b)),
            (Self::List(a), b) => {
                let mut v = Vec::new();

                let mut removed = false;
                for i in a {
                    if i == b && !removed {
                        removed = true;
                        continue;
                    }

                    v.push(i);

                    // item is there, not removed once (skip)
                    // item is not there, not removed once (dont skip)
                    // item is there, removed once (dont skip)
                    // item is not there, not removed once (dont skip)
                }

                Ok(Self::List(v))
            }
            (Self::Set(a), Self::Set(b)) => {
                let mut v = Vec::new();

                for i in a {
                    if !b.contains(&i) {
                        v.push(i);
                    }
                }

                Ok(Self::Set(v))
            }
            (Self::Set(a), b) => {
                let mut v = Vec::new();

                for i in a {
                    if b == i {
                        v.push(i);
                    }
                }

                Ok(Self::Set(v))
            }
            (a, b) => Err(format!(
                "Failed to evaluate `{} - {}`: Sub not implemented for {}, {}",
                a,
                b,
                a.type_of(),
                b.type_of()
            )),
        }
    }
}
impl Mul for Literal {
    type Output = Result<Self, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a * b)),
            (Self::Set(a), Self::Set(b)) => {
                let mut v = Vec::new();

                for i in a {
                    for j in b.clone() {
                        if i != j {
                            let pair = Self::Set(vec![i.clone(), j.clone()]);

                            if !v.contains(&pair) {
                                v.push(pair)
                            }
                        }
                    }
                }

                Ok(Self::Set(v))
            }
            (a, b) => Err(format!(
                "Failed to evaluate `{} * {}`: Mul not implemented for {}, {}",
                a,
                b,
                a.type_of(),
                b.type_of()
            )),
        }
    }
}
impl Div for Literal {
    type Output = Result<Self, String>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a / b)),

            (a, b) => Err(format!(
                "Failed to evaluate `{} / {}`: Div not implemented for {}, {}",
                a,
                b,
                a.type_of(),
                b.type_of()
            )),
        }
    }
}
impl Rem for Literal {
    type Output = Result<Self, String>;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a % b)),

            (a, b) => Err(format!(
                "Failed to evaluate `{} / {}`: Div not implemented for {}, {}",
                a,
                b,
                a.type_of(),
                b.type_of()
            )),
        }
    }
}
impl Neg for Literal {
    type Output = Result<Self, String>;
    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(a) => Ok(Self::Integer(-a)),

            a => Err(format!(
                "Failed to evaluate `-{}`: Neg not implemented for {}",
                a,
                a.type_of(),
            )),
        }
    }
}
impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Boolean(a), Self::Boolean(b)) => a.partial_cmp(b),
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => a.partial_cmp(b),
            (Self::Set(a), Self::Set(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

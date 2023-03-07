use std::{fmt::Display, ops::Add};

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
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                "Failed to evaluate `{} + {}`: Add not implemented for {} {}",
                a,
                b,
                a.type_of(),
                b.type_of()
            )),
        }
    }
}
// impl Sub for Literal {
//     type Output = Result<Self, String>;
//     fn sub(self, rhs: Self) -> Self::Output {
//         match self {}
//     }
// }

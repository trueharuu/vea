use std::{ fmt::Display, collections::HashMap };

use crate::{ lexer::Span, b };

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
    Set(String, String, b![Expr]),
    Get(String, String),
    Fn(String, b![Expr], Vec<Expr>)
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Vec<Literal>),
    Object(HashMap<String, Literal>),
    Fn(String, Vec<String>, Vec<Expr>),
    None,
}

impl Literal {
    pub fn assert_integer(&self) -> &i64 {
        if let Self::Integer(i) = self { i } else { panic!("assertion failed: not an Integer") }
    }

    pub fn assert_string(&self) -> &String {
        if let Self::String(i) = self { i } else { panic!("assertion failed: not a String") }
    }

    pub fn assert_boolean(&self) -> &bool {
        if let Self::Boolean(i) = self { i } else { panic!("assertion failed: not a Boolean") }
    }

    pub fn assert_object(&self) -> &HashMap<String, Literal> {
        if let Self::Object(i) = self { i } else { panic!("assertion failed: not an Object") }
    }

    pub fn type_of(&self) -> String {
        match self {
            Self::Boolean(_) => "bool".to_owned(),
            Self::Integer(_) => "int".to_owned(),
            Self::String(_) => "str".to_owned(),
            Self::Array(v) => {
                if v.is_empty() {
                    "none[]".to_owned()
                } else {
                    let first = v.first().unwrap();
                    first.type_of() + "[]"
                }
            }

            Self::Object(_) => "object".to_owned(),
            Self::None => "None".to_owned(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Boolean(b) => b.to_string(),
            Self::Integer(i) => i.to_string(),
            Self::String(s) => s.to_string(),
            Self::Array(a) => format!("{a:?}"),
            Self::Object(o) => format!("{o:?}"),
            Self::None => "None".to_string(),
        })
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(i), Self::Integer(u)) => i == u,
            (Self::String(i), Self::String(u)) => i == u,
            (Self::Boolean(i), Self::Boolean(u)) => i == u,
            (Self::Array(i), Self::Array(u)) => i == u,
            (Self::None, Self::None) => true,
            (i, u) => panic!("cannot compare `{} == {}`", i.type_of(), u.type_of()),
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(u)) => i.partial_cmp(u),
            (_, _) => None,
        }
    }
}
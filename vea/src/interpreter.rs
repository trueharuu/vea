use std::collections::HashMap;

use crate::ast::Expr;
use crate::literal::Literal;
use crate::span::RawSpanned;
use crate::span::Span;

pub struct Env {
    pub values: HashMap<String, Literal>,
    pub stdout: String,
    pub err: Vec<RawSpanned<String>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            err: Default::default(),
            stdout: Default::default(),
            values: Default::default(),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

pub fn interp(_program: &mut Env, one: Span<Expr<'_>>) -> Result<Literal, Span<String>> {
    let Span(one, _s) = one;
    // dbg!(&one);
    match one {
        _ => todo!(),
    }
}

pub fn exec(many: Vec<Span<Expr<'_>>>) -> Result<Env, Span<String>> {
    let mut env = Env::new();

    for i in many {
        // println!("{i:?}");
        interp(&mut env, i)?;
    }

    Ok(env)
}

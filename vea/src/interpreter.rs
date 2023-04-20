use std::collections::HashMap;

use crate::ast::Expr;
use crate::literal::Literal;
use crate::span::RawSpanned;
use crate::span::Span;

pub struct Env<'a> {
    pub values: HashMap<String, Literal<'a>>,
    pub stdout: String,
    pub err: Vec<RawSpanned<String>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Self {
            err: Default::default(),
            stdout: Default::default(),
            values: Default::default(),
        }
    }
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn interp<'t>(_program: &mut Env, one: Span<Expr<'_>>) -> Result<Literal<'t>, Span<String>> {
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

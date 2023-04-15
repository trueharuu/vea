use std::collections::HashMap;

use ariadne::Color;
use ariadne::Fmt;

use crate::ast::Expr;
use crate::ast::Literal;
use crate::common::Spanned;
use crate::common::Tag;

pub struct Env {
    pub values: HashMap<String, Literal>,
    pub stdout: String,
    pub err: Vec<Spanned<String>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

pub fn interp(program: &mut Env, one: Spanned<Expr<'_>>) -> Result<Literal, Spanned<String>> {
    let (one, s) = one;
    dbg!(&one);
    match one {
        Expr::Add(a, b) => (interp(program, *a)? + interp(program, *b)?).map_err(|x| (x, s)),
        // Expr::Eq(a, b) => (interp(program, *a)?.ee(&interp(program, *b)?)).map_err(|x| (x, s)),
        Expr::Print(a) => {
            let z = interp(program, *a)?.to_string();
            program.stdout += &z;
            Ok(Literal::None)
        }
        Expr::Let(name, expr) => {
            let v = interp(program, *expr)?;
            program.values.insert(name.to_owned(), v);
            Ok(Literal::None)
        }
        Expr::Literal((a, _)) => Ok(a),

        a => Err(format!("{} not implemented", a.name().fg(Color::Green)).tag(a.span())),
    }
}

pub fn exec(many: Vec<Spanned<Expr<'_>>>) -> Result<Env, Spanned<String>> {
    let mut env = Env::new();

    for i in many {
        // println!("{i:?}");
        interp(&mut env, i)?;
    }

    Ok(env)
}

use std::collections::HashMap;

use crate::ast::Expr;
use crate::common::Tag;
use crate::literal::Literal;

use crate::span::Span;

#[derive(Debug)]
pub struct Env {
    pub values: HashMap<String, Literal>,
    pub stdout: String,
    pub err: Vec<Span<String>>,
}

impl Env {
    #[must_use]
    pub fn new() -> Self {
        Self {
            err: Vec::new(),
            stdout: String::new(),
            values: HashMap::default(),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

pub fn interp(program: &mut Env, one: Span<Expr<'_>>) -> Result<Literal, Span<String>> {
    let Span(one, full_span) = one;
    dbg!(&one);
    match one {
        Expr::Access { ident } => {
            let value = program.values.get(ident.0);

            value.map_or_else(
                || Err(format!("variable {} does not exist", ident.0).t(ident.1)),
                |v| Ok(v.clone()),
            )
        }

        Expr::Group { expr, .. } => interp(program, *expr),
        Expr::Add { lhs, rhs, .. } => {
            (interp(program, *lhs)? + interp(program, *rhs)?).map_err(|x| x.t(full_span))
        }

        Expr::Sub { lhs, rhs, .. } => {
            (interp(program, *lhs)? - interp(program, *rhs)?).map_err(|x| x.t(full_span))
        }

        Expr::Mul { lhs, rhs, .. } => {
            (interp(program, *lhs)? * interp(program, *rhs)?).map_err(|x| x.t(full_span))
        }

        Expr::Div { lhs, rhs, .. } => {
            (interp(program, *lhs)? / interp(program, *rhs)?).map_err(|x| x.t(full_span))
        }

        Expr::Eq { lhs, rhs, .. } => interp(program, *lhs)?
            .req(interp(program, *rhs)?)
            .map_err(|x| x.t(full_span)),

        Expr::Ne { lhs, rhs, .. } => interp(program, *lhs)?
            .rne(interp(program, *rhs)?)
            .map_err(|x| x.t(full_span)),

        Expr::Gt { lhs, rhs, .. } => interp(program, *lhs)?
            .rgt(interp(program, *rhs)?)
            .map_err(|x| x.t(full_span)),

        Expr::Lt { lhs, rhs, .. } => interp(program, *lhs)?
            .rlt(interp(program, *rhs)?)
            .map_err(|x| x.t(full_span)),

        Expr::Ge { lhs, rhs, .. } => interp(program, *lhs)?
            .rge(interp(program, *rhs)?)
            .map_err(|x| x.t(full_span)),

        Expr::Le { lhs, rhs, .. } => interp(program, *lhs)?
            .rle(interp(program, *rhs)?)
            .map_err(|x| x.t(full_span)),

        Expr::Block { exprs, .. } => {
            let last = exprs.last();
            if exprs.is_empty() {
                return Ok(Literal::None);
            }

            for i in &exprs[..exprs.len() - 1] {
                interp(program, i.clone())?;
            }

            interp(program, last.unwrap().clone())
        }

        Expr::None => Ok(Literal::None),
        Expr::Literal { value } => Ok(value),

        Expr::If {
            condition,
            then,
            other,
            ..
        } => {
            let cond = interp(program, *condition)?;
            if let Literal::Bool(real_cond) = cond {
                if real_cond {
                    interp(program, *then)
                } else if let Some(else_cond) = other {
                    interp(program, *else_cond)
                } else {
                    Ok(Literal::None)
                }
            } else {
                return Err(format!(
                    "condition in `if` statement must be of type `bool`, recieved `{}`",
                    cond.type_of()
                )
                .t(full_span));
            }
        }

        Expr::Let { ident, expr, .. } => {
            let value = program.values.get(ident.0);

            if value.is_some() {
                Err(format!("variable {} does already exists", ident.0).t(ident.1))
            } else {
                let value = interp(program, *expr)?;
                program.values.insert(ident.0.to_owned(), value);

                Ok(Literal::None)
            }
        }

        Expr::Neg { expr, .. } => (-interp(program, *expr)?).map_err(|x| x.t(full_span)),

        Expr::Not { expr, .. } => (-interp(program, *expr)?).map_err(|x| x.t(full_span)),

        Expr::Print { value, .. } => {
            let output = interp(program, *value)?;

            program.stdout += &output.to_string();
            Ok(output)
        } // _ => Err("todo".to_owned().t(full_span)),
    }
}

pub fn exec<'a>(many: Vec<Span<Expr<'_>>>, env: &'a mut Env) -> Result<&'a mut Env, Span<String>> {
    println!("env: {env:?}");

    for i in many {
        println!("{i:?}");
        interp(env, i)?;
    }

    Ok(env)
}

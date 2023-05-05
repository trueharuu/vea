use std::collections::HashMap;

use crate::ast::Expr;
use crate::common::Tag;
use crate::common::VeaErr;
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
    // dbg!(&one);

    macro_rules! short {
        (assign $T:tt $ident:expr, $expr:expr) => {
        {
            if !program.values.contains_key($ident.0) {
                return Err(format!("variable {} does not exist", $ident.0).t($ident.1));
            }

            let cpy = program.values.clone();
            let current = cpy.get($ident.0);
            let rhs = interp(program, *$expr)?;
            let after = (current.unwrap().clone() $T rhs).map_err(|x| x.t(full_span))?;

            program.values.insert($ident.0.to_owned(), after);
            Ok(Literal::None)
        }
        };

        (just $T:tt $lhs:expr, $rhs:expr) => {
            {(interp(program, *$lhs)? $T interp(program, *$rhs)?).map_err(|x| x.t(full_span))}
        }
    }

    match one {
        Expr::Error(e) => Err(match e {
            VeaErr::IntegerOverflow => "this value cannot be stored as an integer"
                .to_string()
                .t(full_span),
            VeaErr::InvalidStringEscape => "invalid string escape".to_string().t(full_span),
            // VeaErr::InvalidQuotationMark(q) => format!("").t(full_span),
        }),
        Expr::Access { ident } => {
            let value = program.values.get(ident.0);

            value.map_or_else(
                || Err(format!("variable {} does not exist", ident.0).t(ident.1)),
                |v| Ok(v.clone()),
            )
        }

        Expr::Group { expr, .. } => interp(program, *expr),

        Expr::Add { lhs, rhs, .. } => short!(just + lhs, rhs),
        Expr::Sub { lhs, rhs, .. } => short!(just - lhs, rhs),
        Expr::Mul { lhs, rhs, .. } => short!(just * lhs, rhs),
        Expr::Div { lhs, rhs, .. } => short!(just / lhs, rhs),
        Expr::Rem { lhs, rhs, .. } => short!(just % lhs, rhs),
        Expr::And { lhs, rhs, .. } => short!(just & lhs, rhs),
        Expr::Xor { lhs, rhs, .. } => short!(just ^ lhs, rhs),
        Expr::Shl { lhs, rhs, .. } => short!(just << lhs, rhs),
        Expr::Shr { lhs, rhs, .. } => short!(just >> lhs, rhs),
        Expr::Or { lhs, rhs, .. } => short!(just | lhs, rhs),

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
            let cond = interp(program, *condition.clone())?;
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
                .t(condition.1));
            }
        }

        Expr::While {
            condition, then, ..
        } => {
            let mut single = interp(program, *condition.clone())?;
            let mut ctr = 0;

            dbg!(&then);

            loop {
                // dbg!(&ctr);
                if ctr > 2000 {
                    return Err("maximum loop count reached".to_string().t(full_span));
                }

                if let Literal::Bool(rcond) = single {
                    if rcond {
                        interp(program, *then.clone())?;
                        single = interp(program, *condition.clone())?;
                        ctr += 1;
                    } else {
                        break;
                    }
                } else {
                    return Err(format!(
                        "condition in `while` statement must be of type `bool`, recieved `{}`\n\t= note: marked on iteration {}",
                        single.type_of(), ctr
                    )
                    .t(condition.1));
                }
            }

            Ok(Literal::None)
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
        }

        Expr::Assign { ident, expr, .. } => {
            if !program.values.contains_key(ident.0) {
                return Err(format!("variable {} does not exist", ident.0).t(ident.1));
            }

            let value = interp(program, *expr)?;
            program.values.insert(ident.0.to_owned(), value);
            Ok(Literal::None)
        }

        Expr::AddAssign { ident, expr, .. } => short!(assign + ident, expr),
        Expr::SubAssign { ident, expr, .. } => short!(assign - ident, expr),
        Expr::MulAssign { ident, expr, .. } => short!(assign * ident, expr),
        Expr::DivAssign { ident, expr, .. } => short!(assign / ident, expr),
        Expr::RemAssign { ident, expr, .. } => short!(assign % ident, expr),
        Expr::AndAssign { ident, expr, .. } => short!(assign & ident, expr),
        Expr::XorAssign { ident, expr, .. } => short!(assign ^ ident, expr),
        Expr::ShlAssign { ident, expr, .. } => short!(assign << ident, expr),
        Expr::ShrAssign { ident, expr, .. } => short!(assign >> ident, expr),
        Expr::OrAssign { ident, expr, .. } => short!(assign | ident, expr),
        // _ => Err("todo".to_owned().t(full_span)),
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

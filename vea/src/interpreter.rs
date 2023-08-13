use chumsky::span::SimpleSpan;

use crate::{ast::Expr, common::Tag, env::Env, literal::Literal, rc_cell, span::Span};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[must_use]
#[inline]
pub fn none<'a>() -> Rc<RefCell<Literal<'a>>> {
    Rc::new(RefCell::new(Literal::None))
}

pub fn interp<'a>(
    program: &mut Env<'a>,
    one: Span<Expr<'a>>,
) -> Result<Rc<RefCell<Literal<'a>>>, Span<String>> {
    let Span(one, full_span) = one;

    macro_rules! mm {
        ($t:tt $program:ident, $full_span:ident, $lhs:ident, $rhs:ident) => {
            (interp($program, *$lhs)?.borrow().clone() $t interp($program, *$rhs)?.borrow().clone())
        .map(|x| Rc::new(RefCell::new(x)))
        .map_err(|x| x.t($full_span))
        };

        (* $t:tt $program:ident, $full_span:ident, $ident:ident, $expr:ident) => {{
            let lhs = program.get(&$ident.0.to_string());

            if let Some(lh) = lhs {
                let lhv = lh.borrow().clone();
                let rhv = interp(program, *$expr)?;
                let out = (lhv $t rhv.borrow().clone()).map_err(|x| x.t($full_span))?;
                program.set(&$ident.0.to_string(), Rc::new(RefCell::new(out)));
            } else {
                return Err(format!("variable `{}` does not exist", $ident.0).t($ident.1));
            }

            Ok(none())
        }};

        (= $id:ident, $program:ident, $full_span:ident, $lhs:ident, $rhs:ident) => {{
            let lhv = interp($program, *$lhs)?.borrow().clone();
            let rhv = interp($program, *$rhs)?.borrow().clone();

            lhv.$id(rhv).map(|x| Rc::new(RefCell::new(x))).map_err(|x| x.t($full_span))
        }}
    }

    // current branch is GONE
    if program.retyet {
        return Err("statement after `return` is never reached"
            .to_string()
            .t(full_span));
    }

    match one {
        Expr::Literal { value } => Ok(Rc::new(RefCell::new(value))),
        Expr::Access { ident } => {
            let value = program.get(ident.0);
            value.ok_or_else(|| format!("variable `{}` does not exist", ident.0).t(ident.1))
        }

        Expr::Let { ident, expr, .. } => {
            if program.has(ident.0) {
                return Err(format!("variable `{}` already exists", ident.0).t(ident.1));
            }

            let value = interp(program, *expr)?;

            program
                .assign(ident.0, value)
                .map(|_| none())
                .map_err(|x| x.t(full_span))
        }

        Expr::Print { value, .. } => {
            let val = interp(program, *value)?;
            program.print(&val.borrow().to_string());
            Ok(none())
        }

        Expr::Assign { ident, expr, .. } => {
            if !program.has(ident.0) {
                return Err(format!("variable `{}` does not exist", ident.0).t(ident.1));
            }

            let value = interp(program, *expr)?;
            program.set(ident.0, value);

            Ok(none())
        }

        Expr::Add { lhs, rhs, .. } => mm! { + program, full_span, lhs, rhs },
        Expr::Sub { lhs, rhs, .. } => mm! { - program, full_span, lhs, rhs },
        Expr::Mul { lhs, rhs, .. } => mm! { * program, full_span, lhs, rhs },
        Expr::Div { lhs, rhs, .. } => mm! { / program, full_span, lhs, rhs },
        Expr::Rem { lhs, rhs, .. } => mm! { % program, full_span, lhs, rhs },
        Expr::And { lhs, rhs, .. } => mm! { & program, full_span, lhs, rhs },
        Expr::Or { lhs, rhs, .. } => mm! { | program, full_span, lhs, rhs },
        Expr::Xor { lhs, rhs, .. } => mm! { ^ program, full_span, lhs, rhs },
        Expr::Shl { lhs, rhs, .. } => mm! { << program, full_span, lhs, rhs },
        Expr::Shr { lhs, rhs, .. } => mm! { >> program, full_span, lhs, rhs },

        Expr::AddAssign { ident, expr, .. } => mm! { * + program, full_span, ident, expr },
        Expr::SubAssign { ident, expr, .. } => mm! { * - program, full_span, ident, expr },
        Expr::MulAssign { ident, expr, .. } => mm! { * * program, full_span, ident, expr },
        Expr::DivAssign { ident, expr, .. } => mm! { * / program, full_span, ident, expr },
        Expr::RemAssign { ident, expr, .. } => mm! { * % program, full_span, ident, expr },
        Expr::AndAssign { ident, expr, .. } => mm! { * & program, full_span, ident, expr },
        Expr::OrAssign { ident, expr, .. } => mm! { * | program, full_span, ident, expr },
        Expr::XorAssign { ident, expr, .. } => mm! { * ^ program, full_span, ident, expr },
        Expr::ShlAssign { ident, expr, .. } => mm! { * << program, full_span, ident, expr },
        Expr::ShrAssign { ident, expr, .. } => mm! { * >> program, full_span, ident, expr },

        Expr::Eq { lhs, rhs, .. } => mm! { = req, program, full_span, lhs, rhs },
        Expr::Ne { lhs, rhs, .. } => mm! { = rne, program, full_span, lhs, rhs },
        Expr::Gt { lhs, rhs, .. } => mm! { = rgt, program, full_span, lhs, rhs },
        Expr::Ge { lhs, rhs, .. } => mm! { = rge, program, full_span, lhs, rhs },
        Expr::Lt { lhs, rhs, .. } => mm! { = rlt, program, full_span, lhs, rhs },
        Expr::Le { lhs, rhs, .. } => mm! { = rle, program, full_span, lhs, rhs },

        Expr::Neg { expr, .. } => (-interp(program, *expr)?.borrow().clone())
            .map_err(|x| x.t(full_span))
            .map(|x| Rc::new(RefCell::new(x))),

        Expr::Not { expr, .. } => (!interp(program, *expr)?.borrow().clone())
            .map_err(|x| x.t(full_span))
            .map(|x| Rc::new(RefCell::new(x))),

        Expr::Block { exprs, .. } => {
            exec(
                exprs,
                &Rc::new(RefCell::new(Env::with_parent(
                    None,
                    Rc::new(RefCell::new(program.clone())),
                ))),
            )?;
            Ok(none())
        }

        Expr::Group { expr, .. } => interp(program, *expr),
        Expr::Error(value) => Err(value.to_string().t(full_span)),

        Expr::FnCall {
            access, arguments, ..
        } => {
            let value = interp(program, *access.clone())?.borrow().clone();

            if let Literal::Fn(name, argv, bloc) = value {
                let local = Rc::new(RefCell::new(Env::with_parent(
                    Some(name.0.to_string()),
                    Rc::new(RefCell::new(program.clone())),
                )));

                if arguments.len() > argv.len() {
                    return Err(format!(
                        "fn `{}` expected {} arguments but got {}",
                        name.0,
                        argv.len(),
                        arguments.len()
                    )
                    .t(arguments
                        .get(argv.len())
                        .map(|x| SimpleSpan::new(x.1.start, arguments.last().unwrap().1.end))
                        .unwrap()));
                }

                for (i, arg) in argv.iter().enumerate() {
                    let argr = arguments.get(i);

                    if argr.is_none() {
                        return Err(format!(
                            "fn `{}` expected {} arguments but got {}",
                            name.0,
                            argv.len(),
                            arguments.len()
                        )
                        .t(name.1));
                    }

                    let actual = interp(program, argr.unwrap().clone())?;

                    local.borrow_mut().set(arg.0, actual);
                }

                if let Expr::Block { exprs, .. } = bloc.0 {
                    exec(exprs, &local)?;

                    if !local.borrow().retyet {
                        return Err(format!("fn `{}` doesn't return anything", &name.0).t(name.1));
                    }
                } else {
                    return Err(format!("fn `{}` has a magic non-block body", &name.0).t(bloc.1));
                }
                Ok(program.get_ret(name.0).unwrap_or_else(none))
            } else {
                Err("value is not a function".to_string().t(access.1))
            }
        }

        Expr::FnDecl {
            name,
            arguments,
            block,
            ..
        } => program
            .assign(
                name.0,
                Rc::new(RefCell::new(Literal::Fn(name, arguments, block))),
            )
            .map(|_| none())
            .map_err(|x| x.t(full_span)),

        Expr::If {
            condition,
            then,
            other,
            ..
        } => {
            let cond = interp(program, *condition.clone())?.borrow().clone();

            if let Literal::Bool(b) = cond {
                if b {
                    interp(program, *then)?;
                } else if let Some(o) = other {
                    interp(program, *o)?;
                }

                Ok(none())
            } else {
                Err("condition of an `if` statement must be of type `bool`"
                    .to_string()
                    .t(condition.1))
            }
        }

        Expr::None => Ok(none()),

        Expr::While {
            condition, then, ..
        } => {
            loop {
                let cond = interp(program, *condition.clone())?.borrow().clone();

                if let Literal::Bool(b) = cond {
                    if b {
                        interp(program, *then.clone())?;
                    } else {
                        break;
                    }
                } else {
                    return Err("condition of an `while` statement must be of type `bool`"
                        .to_string()
                        .t(condition.1));
                }
            }
            Ok(none())
        }

        Expr::Return { value, .. } => {
            let v = interp(program, *value)?;

            if let Some(mut parent) = program.parent.as_ref().map(|x| x.borrow_mut()) {
                println!("return {:?} within {:?}", v, &program.name);
                program.retyet = true;
                let name = program.name.as_ref().cloned().unwrap();
                parent.set_ret(&name, v);
                Ok(none())
            } else {
                Err("used `return` statement outside of a `fn` block"
                    .to_string()
                    .t(full_span))
            }
        }

        Expr::Object { exprs, .. } => {
            let mut obj = HashMap::new();

            for Span(expr, s) in exprs {
                match expr {
                    Expr::Let { ident, expr, .. } => {
                        obj.insert(ident.0, interp(program, *expr)?);
                    }
                    Expr::FnDecl {
                        name,
                        arguments,
                        block,
                        ..
                    } => {
                        obj.insert(
                            name.0,
                            Rc::new(RefCell::new(Literal::Fn(name, arguments, block))),
                        );
                    }
                    _ => return Err("unreachable state".to_string().t(s)),
                }
            }

            Ok(rc_cell!(Literal::Object(obj)))
        }

        Expr::Set { exprs, .. } => {
            let mut set = vec![];

            for expr in exprs {
                let value = interp(program, expr.clone())?;

                if set.iter().any(|x: &Rc<RefCell<Literal<'_>>>| {
                    matches!(
                        x.borrow().clone().req(value.borrow().clone()),
                        Ok(Literal::Bool(true))
                    )
                }) {
                    return Err(
                        format!("value {} is already in this set", value.borrow()).t(expr.1)
                    );
                }

                set.push(value);
            }

            Ok(rc_cell!(Literal::Set(set)))
        }

        // Expr::Chain { path } => {
        //     let mut value: Option<Rc<RefCell<Literal<'_>>>> = None;

        //     for part in path {
        //         if let Some(v) = value {
        //             match v.borrow().clone() {
        //                 Literal::Object(v) => {
        //                     let key = interp(program, part.clone())?.borrow().clone();

        //                     match key {
        //                         Literal::String(s) => {
        //                             let kv = v.get(&*s);

        //                             if let Some(kvv) = kv {
        //                                 value = Some(kvv.clone());
        //                             } else {
        //                                 return Err(format!("unknown key `{s}`").t(part.1));
        //                             }
        //                         }

        //                         c => {
        //                             return Err(format!(
        //                                 "cannot index with `{c}` as it is not a string"
        //                             )
        //                             .t(part.1));
        //                         }
        //                     }
        //                 }
        //                 // TODO: add builtin primitive methods and properties
        //                 c => {
        //                     return Err(format!(
        //                         "cannot index into a value of type `{}`",
        //                         c.type_of()
        //                     )
        //                     .t(part.1))
        //                 }
        //             }
        //         } else {
        //             value = Some(interp(program, part)?);
        //         }
        //     }

        //     value.ok_or_else(|| "path is empty".to_string().t(full_span))
        // }
        Expr::Chain { parent, child } => {
            let p = interp(program, *parent.clone())?.borrow().clone();

            match p {
                Literal::Object(v) => {
                    let c = interp(program, *child.clone())?.borrow().clone();
                    match c {
                        Literal::String(s) => {
                            return v.get(&*s).cloned().ok_or_else(|| {
                                format!("value does not have an index `{}`", s).t(child.1)
                            })
                        }
                        c => Err(
                            format!("cannot index with a value of type `{}`", c.type_of())
                                .t(child.1),
                        ),
                    }
                }
                c => Err(
                    format!("cannot index into a value with type `{}`", c.type_of()).t(parent.1),
                ),
            }
        }
    }
}

pub fn exec<'a>(
    many: Vec<Span<Expr<'a>>>,
    env: &Rc<RefCell<Env<'a>>>,
) -> Result<Env<'a>, Span<String>> {
    for i in many {
        interp(&mut env.borrow_mut(), i)?;
    }

    Ok(env.borrow().clone())
}

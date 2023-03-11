use crate::{ast::*, literal::Literal, tools::Named};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Write,
    rc::Rc,
    // sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub values: RefCell<HashMap<String, Literal>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            parent: None,
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_parent(v: Rc<RefCell<Env>>) -> Self {
        Env {
            parent: Some(v),
            ..Default::default()
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

pub fn interp<'a>(
    p: &'a mut Program,
    env: &mut Rc<RefCell<Env>>,
    stdout: &'a mut String,
) -> Result<&'a mut String, String> {
    for expr in &mut p.stmts {
        interp_expr(env, expr, stdout)?;
    }

    Ok(stdout)
}

fn interp_expr<'a>(
    env: &'a mut Rc<RefCell<Env>>,
    expr: &'a Expr,
    stdout: &mut String,
) -> Result<Literal, String> {
    // println!("{expr:#?}");

    match &expr.1 {
        Node::Literal(a) => Ok(a.clone()),
        Node::Print(a) => {
            let val = interp_expr(env, a, stdout);

            val.clone()?;

            write!(stdout, "{}", val.unwrap()).unwrap();

            Ok(Literal::Never)
        }

        Node::Block(stmts, f) => {
            let mut p = Program {
                stmts: stmts.to_vec(),
            };

            let mut e = Rc::new(RefCell::new(Env::with_parent(env.clone())));
            interp(&mut p, &mut e, stdout)?;

            if let Some(p) = f {
                interp_expr(&mut e, p, stdout)
            } else {
                Ok(Literal::Never)
            }
        }

        Node::Var(i) => {
            let mut e = env.borrow_mut();
            let val = e.values.get_mut().get(i);

            if let Some(v) = val {
                Ok(v.clone())
            } else {
                Err(format!("Unknown variable '{i}'"))
            }
        }

        Node::Let(name, value) => {
            if env.borrow_mut().values.get_mut().contains_key(name) {
                Err(format!("Variable '{name}' already exists in this environment\n\t= note: use `{name} = ...` to reassign it instead"))
            } else {
                let val = interp_expr(env, value, stdout)?;
                let e = env.borrow_mut();

                e.values.borrow_mut().insert(name.clone(), val);

                Ok(Literal::Never)
            }
        }

        Node::Array(p) => {
            if let Some(s) = p {
                let list = parse_pairs(s.clone())
                    .into_iter()
                    .map(|x| interp_expr(env, &x, stdout))
                    .collect::<Vec<_>>();
                for i in &list {
                    if let Err(e) = i {
                        return Err(e.clone());
                    }
                }

                Ok(Literal::List(
                    list.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>(),
                ))
            } else {
                Ok(Literal::List(vec![]))
            }
        }

        Node::List(p) => {
            if let Some(s) = p {
                println!("{s:?}");

                let values = parse_pairs(s.clone());
                println!("{values:?}");

                let mut set = Vec::new();
                println!("set.contains(&Literal::Never)");
                set.contains(&Literal::Never);
                println!("passed");

                for i in values {
                    let value = interp_expr(env, &i, stdout);

                    if let Ok(v) = value {
                        if !set.contains(&v) {
                            set.push(v);
                        }
                    } else {
                        return Err(value.unwrap_err());
                    }
                }

                Ok(Literal::Set(set))
            } else {
                Ok(Literal::Set(vec![]))
            }
        }

        // ops
        Node::Add(a, b) => interp_expr(env, a, stdout)? + interp_expr(env, b, stdout)?,
        Node::Sub(a, b) => interp_expr(env, a, stdout)? - interp_expr(env, b, stdout)?,
        Node::Mul(a, b) => interp_expr(env, a, stdout)? * interp_expr(env, b, stdout)?,
        Node::Div(a, b) => interp_expr(env, a, stdout)? / interp_expr(env, b, stdout)?,
        Node::Rem(a, b) => interp_expr(env, a, stdout)? % interp_expr(env, b, stdout)?,

        Node::Eq(a, b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout)? == interp_expr(env, b, stdout)?,
        )),
        Node::Ne(a, b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout)? != interp_expr(env, b, stdout)?,
        )),
        Node::Gt(a, b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout)? > interp_expr(env, b, stdout)?,
        )),
        Node::Ge(a, b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout)? >= interp_expr(env, b, stdout)?,
        )),
        Node::Lt(a, b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout)? < interp_expr(env, b, stdout)?,
        )),
        Node::Le(a, b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout)? <= interp_expr(env, b, stdout)?,
        )),
        Node::Inv(a) => interp_expr(env, a, stdout)?.inv(),
        Node::Not(a) => interp_expr(env, a, stdout)?.not(),
        Node::Neg(a) => interp_expr(env, a, stdout).and_then(|x| -x),
        t => Err(format!(
            "Failed to evaulate `{}`: not implemented",
            t.name()
        )),
    }
}

fn parse_pairs(t: Box<Expr>) -> Vec<Expr> {
    if let Node::Pair(p, v) = t.1 {
        println!("pair: {p:?}, {v:?}");
        vec![parse_pairs(p), parse_pairs(v)].concat()
    } else {
        vec![*t]
    }
}

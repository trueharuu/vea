use crate::ast::*;
use std::collections::HashMap;

pub fn interp<'a>(
    p: &'a mut Program,
    env: &mut HashMap<String, Literal>,
    stdout: &'a mut String,
) -> Result<&'a mut String, String> {
    for expr in &mut p.stmts {
        if let Err(i) = interp_expr(env, expr, stdout) {
            return Err(i);
        };
    }

    Ok(stdout)
}
macro_rules! propagate {
    [$name:ident] => {
        if let Err(i) = &$name {
            return Err(i.clone());
        }
    }
}
fn interp_expr<'a>(
    env: &mut HashMap<String, Literal>,
    expr: &'a Expr,
    stdout: &mut String,
) -> Result<Literal, String> {
    use crate::ast::Node::{
        Add, Array, Assign, Call, Div, Env, Eq, Fn, Ge, Get, Gt, If, InnerEnv, Le, List, Lt, Mul,
        Ne, Not, Pair, Print, Rem, Set, Sub, Throw, Typeof, Var, While,
    };
    // println!("{expr:#?}");
    match &expr.node {
        Add(ref a, ref b) => Ok(interp_expr(env, a, stdout)? + interp_expr(env, b, stdout)?),
        Sub(ref a, ref b) => Ok(interp_expr(env, a, stdout)? - interp_expr(env, b, stdout)?),
        Mul(ref a, ref b) => Ok(interp_expr(env, a, stdout)? * interp_expr(env, b, stdout)?),
        Div(ref a, ref b) => Ok(interp_expr(env, a, stdout)? / interp_expr(env, b, stdout)?),
        Assign(ref var, ref b) => {
            let val = interp_expr(env, b, stdout);

            if let Err(i) = &val {
                return Err(i.clone());
            }

            env.insert(var.to_owned(), val.clone().unwrap());
            val
        }
        Var(ref var) => {
            let a = env.get(&var[..]).cloned();

            if a.is_none() {
                return Err(format!("variable {} is undefined", var));
            }

            Ok(a.unwrap())
        }
        Node::Literal(ref lit) => Ok(lit.clone()),
        Print(ref e) => {
            let val = interp_expr(env, e, stdout);

            if let Err(i) = &val {
                return Err(i.clone());
            }

            *stdout += &format!("{:?}", val.clone()?);
            val
        }

        Throw(ref e) => {
            let val = interp_expr(env, e, stdout);

            propagate!(val);

            Err(format!("{:?}", val?))
        }

        Typeof(ref e) => {
            let value = interp_expr(env, e, stdout);
            Ok(Literal::String(value?.type_of()))
        }
        Pair(_, ref e) => interp_expr(env, e, stdout),
        Eq(ref a, ref b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout) == interp_expr(env, b, stdout),
        )),
        Ne(ref a, ref b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout) != interp_expr(env, b, stdout),
        )),
        Gt(ref a, ref b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout) > interp_expr(env, b, stdout),
        )),
        Lt(ref a, ref b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout) < interp_expr(env, b, stdout),
        )),
        Ge(ref a, ref b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout) >= interp_expr(env, b, stdout),
        )),
        Le(ref a, ref b) => Ok(Literal::Boolean(
            interp_expr(env, a, stdout) <= interp_expr(env, b, stdout),
        )),
        Env(ref var) => {
            // let val = Literal::Object(HashMap::new());
            env.insert(var.to_owned(), Literal::Object(HashMap::new()));
            Ok(Literal::Never)
        }
        InnerEnv(ref o) => {
            if let Get(obj, prop) = &o.node {
                let mut v = env.get_mut(obj.as_str()).unwrap();

                for i in &prop[0..prop.len() - 1] {
                    if let Literal::Object(p) = v {
                        v = p.get_mut(i).unwrap();
                    }
                }

                if let Literal::Object(p) = v {
                    p.insert(
                        prop.last().unwrap().to_string(),
                        Literal::Object(HashMap::new()),
                    );
                }
            }

            Ok(Literal::Never)
        }

        Set(ref o, ref value) => {
            if let Get(obj, prop) = &o.node {
                let r = interp_expr(env, value, stdout);
                let mut v = env.get_mut(obj.as_str()).unwrap();

                for i in &prop[0..prop.len() - 1] {
                    if let Literal::Object(p) = v {
                        v = if let Some(s) = p.get_mut(i) {
                            s
                        } else {
                            return Ok(Literal::Never);
                        };
                    } else if let Literal::Array(a) = v {
                        let idx = i.parse::<usize>().unwrap();
                        v = if let Some(s) = a.get_mut(idx) {
                            s
                        } else {
                            return Ok(Literal::Never);
                        }
                    }
                }

                if let Literal::Object(p) = v {
                    p.insert(prop.last().unwrap().to_string(), r?);
                } else if let Literal::Array(a) = v {
                    let idx = prop.last().unwrap().to_string().parse::<usize>().unwrap();
                    while a.len() < idx {
                        a.push(Literal::Never);
                    }

                    a.push(r?);
                }
            }

            Ok(Literal::Never)
        }

        Get(ref obj, ref prop) => {
            let mut v = env.get(obj.as_str()).unwrap();

            let mut if_str: Option<&str> = None;
            for i in prop {
                if let Literal::Object(o) = v {
                    v = o.get(i).unwrap_or(&Literal::Never);
                } else if let Literal::Array(o) = v {
                    v = o
                        .get(i.parse::<usize>().unwrap())
                        .unwrap_or(&Literal::Never);
                } else if let Some(s) = if_str {
                    let idx = i.parse::<usize>().unwrap();
                    if_str = Some(&s[idx..(idx + 1)])
                } else if let Literal::String(s) = v {
                    let idx = i.parse::<usize>().unwrap();
                    if_str = Some(&s[idx..(idx + 1)])
                }
            }

            if let Some(s) = if_str {
                return Ok(Literal::String(s.to_owned()));
            }

            Ok(v.clone())
        }

        Array(ref p) => {
            if let Some(e) = p {
                let mut value = Vec::new();
                let mut node = &e.node;
                let mut last = None;
                let mut m = false;

                while let Pair(a, b) = node {
                    m = true;
                    value.push(interp_expr(env, a, stdout)?);
                    node = &b.node;
                    last = Some(b);
                }

                if !m {
                    last = Some(e);
                }

                if let Some(l) = last {
                    value.push(interp_expr(env, &**l, stdout)?);
                }

                Ok(Literal::Array(value))
            } else {
                Ok(Literal::Array(Vec::new()))
            }
        }

        List(ref p) => {
            if let Some(e) = p {
                let mut value = Vec::new();
                let mut node = &e.node;
                let mut last = None;
                let mut m = false;

                while let Pair(a, b) = node {
                    m = true;
                    let v = interp_expr(env, a, stdout);
                    if let Err(i) = &v {
                        return Err(i.clone());
                    }
                    let bump = v?;
                    let tt = value.first();
                    if tt.is_some_and(|x: &Literal| x.type_of() != bump.type_of()) {
                        return Err(format!(
                            "tried to add item of type {} to a Set({})",
                            bump.type_of(),
                            tt.unwrap().type_of()
                        ));
                    }

                    if value.contains(&bump) {
                        return Err(format!("tried to add duplicate item into set: {:?}", bump));
                    }

                    value.push(bump);
                    node = &b.node;
                    last = Some(b);
                }

                if !m {
                    last = Some(e);
                }

                if let Some(a) = last {
                    let v = interp_expr(env, a, stdout);
                    if let Err(i) = &v {
                        return Err(i.clone());
                    }
                    let bump = v?;
                    let tt = value.first();
                    if tt.is_some_and(|x: &Literal| x.type_of() != bump.type_of()) {
                        return Err(format!(
                            "tried to add item of type {} to a Set({})",
                            bump.type_of(),
                            tt.unwrap().type_of()
                        ));
                    }

                    if value.contains(&bump) {
                        return Err(format!("tried to add duplicate item into set: {:?}", bump));
                    }

                    value.push(bump);
                };

                Ok(Literal::Set(value))
            } else {
                Ok(Literal::Set(Vec::new()))
            }
        }

        If(ref e, ref s, ref otherwise) => {
            let bool = *interp_expr(env, e, stdout)?.assert_bool();
            if bool {
                let mut m = Program { stmts: s.clone() };
                let a = interp(&mut m, env, stdout);
                propagate!(a);
            } else if let Some(o) = otherwise {
                let mut m = Program { stmts: o.clone() };
                let a = interp(&mut m, env, stdout);
                propagate!(a);
            }

            Ok(Literal::Never)
        }

        Not(ref e) => Ok(Literal::Boolean(
            !interp_expr(env, e, stdout)?.assert_bool(),
        )),
        Rem(ref lhs, ref rhs) => {
            Ok(interp_expr(env, lhs, stdout)? % interp_expr(env, rhs, stdout)?)
        }

        While(ref e, ref s) => {
            while *interp_expr(env, e, stdout)
                .inspect(|x| println!("{x:?}"))?
                .assert_bool()
            {
                println!("running {:?}", e.node);
                let mut m = Program { stmts: s.clone() };
                let a = interp(&mut m, env, stdout);
                propagate!(a);
            }

            Ok(Literal::Never)
        }

        Fn(ref name, ref a, ref s) => {
            let mut names = Vec::new();
            let mut node = &a.node;

            while let Pair(a, b) = node {
                println!("{a:?} {b:?}");
                if let Node::Var(i) = &a.node {
                    names.push(i.clone());
                }
                node = &b.node;
            }

            if let Node::Var(i) = &node {
                names.push(i.clone());
            }

            env.insert(name.clone(), Literal::Fn(names, s.clone()));

            Ok(Literal::Never)
        }

        Call(ref id, ref kvar) => {
            let f = interp_expr(env, id, stdout);

            propagate!(f);

            if let Ok(Literal::Fn(args, body)) = f {
                let mut value = Vec::new();
                let mut node = &kvar.node;
                let mut last = None;
                let mut m = false;

                while let Pair(a, b) = node {
                    m = true;
                    value.push(interp_expr(env, a, stdout)?);
                    node = &b.node;
                    last = Some(b);
                }

                if !m {
                    last = Some(kvar);
                }

                if let Some(l) = last {
                    value.push(interp_expr(env, &**l, stdout)?);
                }

                if args.len() != value.len() {
                    return Err(format!(
                        "cannot call fn({}) with {} arguments",
                        args.len(),
                        value.len()
                    ));
                }

                let mut env2 = HashMap::from_iter(env.clone());

                for (k, v) in args.iter().zip(value) {
                    env2.insert(k.to_string(), v);
                }

                let mut v = Program { stmts: body };
                let r = interp(&mut v, &mut env2, stdout);
                propagate!(r);

                return Ok(Literal::Never);
            } else {
                return Err("not a fn".to_owned());
            }
        }
    }
}

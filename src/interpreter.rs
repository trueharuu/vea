use crate::ast::*;
use std::collections::HashMap;

pub fn interp<'a>(p: &'a mut Program) {
    let mut env = HashMap::new();
    for expr in &mut p.stmts {
        interp_expr(&mut env, expr);
    }
}
fn interp_expr<'a>(env: &mut HashMap<&'a str, Literal>, expr: &'a mut Expr) -> Literal {
    use crate::ast::Node::{
        Add,
        Assign,
        Div,
        Eq,
        Ge,
        Gt,
        Le,
        Lt,
        Mul,
        Ne,
        Pair,
        Print,
        Sub,
        Typeof,
        Var,
        Env,
        Set,
        Get,
        InnerEnv,
    };
    // println!("{expr:#?}");
    match &mut expr.node {
        Add(ref mut a, ref mut b) => interp_expr(env, a) + interp_expr(env, b),
        Sub(ref mut a, ref mut b) => interp_expr(env, a) - interp_expr(env, b),
        Mul(ref mut a, ref mut b) => interp_expr(env, a) * interp_expr(env, b),
        Div(ref mut a, ref mut b) => interp_expr(env, a) / interp_expr(env, b),
        Assign(ref mut var, ref mut b) => {
            let val = interp_expr(env, b);
            env.insert(var, val.clone());
            val
        }
        Var(ref mut var) =>
            env
                .get(&var[..])
                .unwrap()
                .clone(),
        Node::Literal(ref lit) => lit.clone(),
        Print(ref mut e) => {
            let val = interp_expr(env, e);
            println!("{}", val);
            val
        }
        Typeof(ref mut e) => {
            let value = interp_expr(env, e);
            Literal::String(value.type_of())
        }
        Pair(_, ref mut e) => interp_expr(env, e),
        Eq(ref mut a, ref mut b) => Literal::Boolean(interp_expr(env, a) == interp_expr(env, b)),
        Ne(ref mut a, ref mut b) => Literal::Boolean(interp_expr(env, a) != interp_expr(env, b)),
        Gt(ref mut a, ref mut b) => Literal::Boolean(interp_expr(env, a) > interp_expr(env, b)),
        Lt(ref mut a, ref mut b) => Literal::Boolean(interp_expr(env, a) < interp_expr(env, b)),
        Ge(ref mut a, ref mut b) => Literal::Boolean(interp_expr(env, a) >= interp_expr(env, b)),
        Le(ref mut a, ref mut b) => Literal::Boolean(interp_expr(env, a) <= interp_expr(env, b)),
        Env(ref var) => {
            // let val = Literal::Object(HashMap::new());
            env.insert(var, Literal::Object(HashMap::new()));
            Literal::None
        }
        InnerEnv(ref mut o) => {
            if let Get(obj, prop) = &mut o.node {
                let mut v = env.get_mut(obj.as_str()).unwrap();

                for i in &prop[0..prop.len() - 1] {
                    if let Literal::Object(p) = v {
                        v = p.get_mut(i).unwrap();
                    }
                }

                if let Literal::Object(p) = v {
                    p.insert(prop.last().unwrap().to_string(), Literal::Object(HashMap::new()));
                }
            }

            Literal::None
        }

        Set(ref mut o, ref mut value) => {
            if let Get(obj, prop) = &mut o.node {
                let r = interp_expr(env, value);
                let mut v = env.get_mut(obj.as_str()).unwrap();

                for i in &prop[0..prop.len() - 1] {
                    if let Literal::Object(p) = v {
                        v = if let Some(s) = p.get_mut(i) {
                            s
                        } else {
                            return Literal::None;
                        };
                    }
                }

                if let Literal::Object(p) = v {
                    p.insert(prop.last().unwrap().to_string(), r);
                }
            }

            Literal::None
        }

        Get(ref obj, ref prop) => {
            let mut v = env.get(obj.as_str()).unwrap();

            for i in prop {
                v = v.assert_object().get(i).unwrap_or(&Literal::None);
            }

            v.clone()
        }
    }
}
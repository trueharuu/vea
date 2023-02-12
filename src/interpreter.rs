use crate::ast::*;
use std::{collections::HashMap};

pub fn interp<'a>(p: &'a mut Program) {
    let mut env = HashMap::new();
    for expr in &mut p.stmts {
        interp_expr(&mut env, expr);
    }
}
fn interp_expr<'a>(env: &mut HashMap<&'a str, Literal>, expr: &'a Expr) -> Literal {
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
        List,
    };
    // println!("{expr:#?}");
    match &expr.node {
        Add(ref a, ref b) => interp_expr(env, a) + interp_expr(env, b),
        Sub(ref a, ref b) => interp_expr(env, a) - interp_expr(env, b),
        Mul(ref a, ref b) => interp_expr(env, a) * interp_expr(env, b),
        Div(ref a, ref b) => interp_expr(env, a) / interp_expr(env, b),
        Assign(ref var, ref b) => {
            let val = interp_expr(env, b);
            env.insert(var, val.clone());
            val
        }
        Var(ref var) =>
            env
                .get(&var[..])
                .unwrap()
                .clone(),
        Node::Literal(ref lit) => lit.clone(),
        Print(ref e) => {
            let val = interp_expr(env, e);
            println!("{}", val);
            val
        }
        Typeof(ref e) => {
            let value = interp_expr(env, e);
            Literal::String(value.type_of())
        }
        Pair(_, ref e) => interp_expr(env, e),
        Eq(ref a, ref b) => Literal::Boolean(interp_expr(env, a) == interp_expr(env, b)),
        Ne(ref a, ref b) => Literal::Boolean(interp_expr(env, a) != interp_expr(env, b)),
        Gt(ref a, ref b) => Literal::Boolean(interp_expr(env, a) > interp_expr(env, b)),
        Lt(ref a, ref b) => Literal::Boolean(interp_expr(env, a) < interp_expr(env, b)),
        Ge(ref a, ref b) => Literal::Boolean(interp_expr(env, a) >= interp_expr(env, b)),
        Le(ref a, ref b) => Literal::Boolean(interp_expr(env, a) <= interp_expr(env, b)),
        Env(ref var) => {
            // let val = Literal::Object(HashMap::new());
            env.insert(var, Literal::Object(HashMap::new()));
            Literal::Never
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
                    p.insert(prop.last().unwrap().to_string(), Literal::Object(HashMap::new()));
                }
            }

            Literal::Never
        }

        Set(ref o, ref value) => {
            if let Get(obj, prop) = &o.node {
                let r = interp_expr(env, value);
                let mut v = env.get_mut(obj.as_str()).unwrap();

                for i in &prop[0..prop.len() - 1] {
                    if let Literal::Object(p) = v {
                        v = if let Some(s) = p.get_mut(i) {
                            s
                        } else {
                            return Literal::Never;
                        };
                    }
                }

                if let Literal::Object(p) = v {
                    p.insert(prop.last().unwrap().to_string(), r);
                }
            }

            Literal::Never
        }

        Get(ref obj, ref prop) => {
            let mut v = env.get(obj.as_str()).unwrap();

            for i in prop {
                v = v.assert_object().get(i).unwrap_or(&Literal::Never);
            }

            v.clone()
        },

        List(ref e) => {
            let mut value = Vec::new();
            let mut node = &e.node;
            
            while let Pair(a, b) = node {
                value.push(interp_expr(env, a));
                node = &b.node;
            }


            Literal::Array(value)
        }
    }
}
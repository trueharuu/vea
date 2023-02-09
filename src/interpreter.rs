use crate::ast::*;
use std::collections::HashMap;

pub fn interp<'a>(p: &'a Program) {
    let mut env = HashMap::new();
    for expr in &p.stmts {
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
    };
    // println!("{expr:#?}");
    match &expr.node {
        Add(ref a, ref b) =>
            match (interp_expr(env, a), interp_expr(env, b)) {
                (Literal::Integer(i), Literal::Integer(o)) => Literal::Integer(i + o),
                (Literal::String(i), Literal::String(o)) => Literal::String(i + o.as_str()),
                (i, u) => panic!("cannot calculate `{i:?} + {u:?}`"),
            }
        Sub(ref a, ref b) =>
            Literal::Integer(
                interp_expr(env, a).assert_integer() - interp_expr(env, b).assert_integer()
            ),
        Mul(ref a, ref b) =>
            Literal::Integer(
                interp_expr(env, a).assert_integer() * interp_expr(env, b).assert_integer()
            ),
        Div(ref a, ref b) =>
            Literal::Integer(
                interp_expr(env, a).assert_integer() / interp_expr(env, b).assert_integer()
            ),
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
        Node::Literal(lit) => lit.clone(),
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
            Literal::None
        }
        Set(ref obj, ref prop, ref value) => {
            let mut e = env.clone();
            println!("{e:?}");
            let val: &mut Literal = env.get_mut(obj.as_str()).unwrap();
            let into = interp_expr(&mut e, value);
            if let Literal::Object(ref mut h) = val {
                h.insert(prop.to_string(), into);
                Literal::None
            } else {
                panic!("value {obj} is not an object");
            }
        }

        Get(ref obj, ref prop) => {
            env.get(obj.as_str())
                .unwrap()
                .assert_object()
                .get(prop)
                .unwrap_or(&&Literal::None)
                .clone()
        }
    }
}
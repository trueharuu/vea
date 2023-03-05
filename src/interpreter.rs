use crate::ast::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Write,
    rc::Rc,
    // sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub values: RefCell<HashMap<String, Literal>>,
}

pub fn interp<'a>(
    p: &'a mut Program,
    env: &mut Rc<Env>,
    stdout: &'a mut String,
) -> Result<&'a mut String, String> {
    for expr in &mut p.stmts {
        if let Err(i) = interp_expr(env, expr, stdout) {
            return Err(i);
        };
    }

    Ok(stdout)
}

fn interp_expr<'a>(
    env: &'a mut Rc<Env>,
    expr: &'a Expr,
    stdout: &mut String,
) -> Result<Literal, String> {
    // println!("{expr:#?}");

    match &expr.1 {
        Node::Literal(a) => Ok(a.clone()),
        Node::Print(a) => {
            let val = interp_expr(env, a, stdout);

            if let Err(v) = val {
                return Err(v);
            }

            write!(stdout, "{}", val.unwrap()).unwrap();

            return Ok(Literal::Never);
        }
        _ => todo!(),
    }
}

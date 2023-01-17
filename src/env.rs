use std::{ collections::HashMap, rc::Rc, cell::RefCell };

use crate::{ literal::Literal, token::Token, interpreter::RuntimeError };

#[derive(Clone)]
pub struct Env {
    values: Rc<RefCell<HashMap<String, Literal>>>,
    enclosing: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self { values: Rc::new(RefCell::new(HashMap::default())), enclosing: None }
    }

    pub fn with_parent(enclosing: Box<Env>) -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::default())),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&self, name: String, value: Literal) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        let b = self.values.borrow();
        let value = b.get(&name.lexeme);
        if let Some(v) = value {
            return Ok(v.clone());
        }

        if let Some(e) = &self.enclosing {
            return e.get(name);
        }

        return Err(RuntimeError::new(name.clone(), format!("undefined var '{name}'")));
    }

    pub fn assign(&self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.borrow().contains_key(&name.lexeme) {
            self.values.borrow_mut().insert(name.lexeme, value);
            return Ok(());
        }

        if let Some(e) = &self.enclosing {
            e.assign(name, value);
            return Ok(());
        }

        Err(RuntimeError(name.clone(), format!("variable '{name}' is not defined")))
    }
}
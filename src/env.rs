use std::{ collections::HashMap, rc::Rc, cell::RefCell };

use crate::{ literal::Literal, token::Token, interpreter::RuntimeError };

#[derive(Clone)]
pub struct Env {
    values: Rc<RefCell<HashMap<String, Literal>>>,
}

impl Env {
    pub fn new() -> Self {
        Self { values: Rc::new(RefCell::new(HashMap::default())) }
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

        return Err(RuntimeError::new(name.clone(), format!("undefined var '{}'", name.clone())));
    }

    pub fn assign(&self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.borrow().contains_key(&name.lexeme) {
            self.values.borrow_mut().insert(name.lexeme, value);
            Ok(())
        } else {
            Err(RuntimeError(name.clone(), format!("variable '{}' is not defined", name.clone())))
        }
    }
}
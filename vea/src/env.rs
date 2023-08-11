use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::literal::Literal;

#[derive(Debug, Clone)]
pub struct Env<'a> {
    pub name: Option<String>,
    pub values: Rc<RefCell<HashMap<String, Rc<RefCell<Literal<'a>>>>>>,
    pub retval: Rc<RefCell<HashMap<String, Rc<RefCell<Literal<'a>>>>>>,
    pub stdout: String,
    pub parent: Option<Rc<RefCell<Self>>>,
    pub retyet: bool,
}

impl<'a> Env<'a> {
    #[must_use]
    pub fn new(name: Option<String>) -> Self {
        Self {
            name,
            parent: None,
            stdout: String::default(),
            values: Rc::new(RefCell::new(HashMap::default())),
            retval: Rc::new(RefCell::new(HashMap::default())),
            retyet: false,
        }
    }
    pub fn with_parent(name: Option<String>, parent: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent),
            ..Self::new(name)
        }
    }

    #[must_use]
    pub fn get(&self, k: &str) -> Option<Rc<RefCell<Literal<'a>>>> {
        if let Some(v) = self.values.borrow().get(k) {
            return Some(v.clone());
        }

        if let Some(p) = &self.parent {
            return p.borrow().get(k);
        }

        None
    }

    #[must_use]
    pub fn get_ret(&self, k: &str) -> Option<Rc<RefCell<Literal<'a>>>> {
        if let Some(v) = self.retval.borrow().get(k) {
            return Some(v.clone());
        }

        if let Some(p) = &self.parent {
            return p.borrow().get_ret(k);
        }

        None
    }

    pub fn assign(&mut self, k: &str, v: Rc<RefCell<Literal<'a>>>) -> Result<(), String> {
        if self.get(k).is_some() {
            return Err(format!("variable `{k}` already exists"))
        }

        if let Some(p) = &self.parent {
            if p.borrow().has(k) {
                return Err(format!("variable `{k}` shadows an outer variable with the same name"));
            }
        }

        self.values.borrow_mut().insert(k.to_owned(), v);
        Ok(())
    }

    pub fn set(&mut self, k: &str, v: Rc<RefCell<Literal<'a>>>) -> bool {
        self.values.borrow_mut().insert(k.to_owned(), v);
        true
    }

    pub fn set_ret(&mut self, k: &str, v: Rc<RefCell<Literal<'a>>>) -> bool {
        self.retval.borrow_mut().insert(k.to_owned(), v);
        true
    }

    #[must_use]
    pub fn has(&self, k: &str) -> bool {
        if self.values.borrow().get(k).is_some() {
            return true;
        }

        if let Some(p) = &self.parent {
            return p.borrow().has(k);
        }

        false
    }

    pub fn print(&mut self, text: &str) -> &mut Self {
        self.stdout += text;
        self
    }

    pub fn println(&mut self, text: &str) -> &mut Self {
        self.print(text).print("\n")
    }
}

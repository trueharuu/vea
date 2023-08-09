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
    pub fn get(&self, k: &String) -> Option<Rc<RefCell<Literal<'a>>>> {
        if let Some(v) = self.values.borrow().get(k) {
            return Some(v.clone());
        }

        if let Some(p) = &self.parent {
            return p.borrow().get(k);
        }

        None
    }

    #[must_use]
    pub fn get_ret(&self, k: &String) -> Option<Rc<RefCell<Literal<'a>>>> {
        if let Some(v) = self.retval.borrow().get(k) {
            return Some(v.clone());
        }

        if let Some(p) = &self.parent {
            return p.borrow().get_ret(k);
        }

        None
    }

    pub fn assign(&mut self, k: &String, v: Rc<RefCell<Literal<'a>>>) -> bool {
        if self.get(k).is_some() {
            return false;
        }

        self.values.borrow_mut().insert(k.clone(), v);
        true
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
    pub fn has(&self, k: &String) -> bool {
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

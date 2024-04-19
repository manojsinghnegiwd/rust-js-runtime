use std::{collections::HashMap, rc::Rc};
use std::cell::RefCell;
use crate::types::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    variables: RefCell<HashMap<String, Value>>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Self {
        match parent {
            Some(parent) => Scope {
                parent: Some(Rc::new(RefCell::new(parent))),
                variables: RefCell::new(HashMap::new()),
            },
            None => Scope {
                parent: None,
                variables: RefCell::new(HashMap::new()),
            }
        }
    }

    pub fn with_rc(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        match parent {
            Some(parent) => Scope {
                parent: Some(parent),
                variables: RefCell::new(HashMap::new()),
            },
            None => Scope {
                parent: None,
                variables: RefCell::new(HashMap::new()),
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let variables = self.variables.borrow();
        if variables.contains_key(name) {
            match variables.get(name) {
                Some(value) => return Some(value.clone()),
                None => (),
            }
        }

        match &self.parent {
            Some(parent) => parent.borrow().get(name),
            None => None,
        }
    }

    pub fn contains_key_local(&self, name: &str) -> bool {
        self.variables.borrow().contains_key(name)
    }

    pub fn contains_key(&self, name: &str) -> bool {
        if self.variables.borrow().contains_key(name) {
            return true
        }

        match &self.parent {
            Some(parent) => parent.borrow().contains_key(name),
            None => false,
        }
    }

    pub fn assign (&mut self, name: String, value: Value) {
        if self.variables.borrow().contains_key(&name) {
            self.variables.borrow_mut().insert(name, value);
            return
        }

        match &mut self.parent {
            Some(parent) => parent.borrow_mut().assign(name, value),
            None => panic!("Variable not found: {}", name)
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.borrow_mut().insert(name, value);
    }
}
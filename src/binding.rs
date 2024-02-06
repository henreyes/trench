use crate::parser::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AList {
    bindings: RefCell<HashMap<String, Atom>>,
}

impl AList {
    pub fn new() -> Self {
        AList {
            bindings: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_binding(&self, key: String, value: Atom) {
        self.bindings.borrow_mut().insert(key, value);
    }

    pub fn get_binding(&self, key: &str) -> Option<Atom> {
        self.bindings.borrow().get(key).cloned()
    }
}

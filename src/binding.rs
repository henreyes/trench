use crate::parser::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AList {
    bindings: RefCell<HashMap<String, Atom>>,
    parent: Option<Rc<RefCell<AList>>>, 
}

impl AList {
    pub fn new() -> Self {
        AList {
            bindings: RefCell::new(HashMap::new()),
            parent: None, 
        }
    }

    pub fn new_with_parent(parent: &Rc<RefCell<Self>>) -> Rc<RefCell<AList>> {
        Rc::new(RefCell::new(AList {
            bindings: RefCell::new(HashMap::new()),
            parent: Some(parent.clone())
        }))
    }

    pub fn get_binding(&self, key: &str) -> Option<Atom> {
        self.bindings.borrow().get(key).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().get_binding(key))
        })
    }

    pub fn set_binding(&self, key: String, value: Atom) {
        self.bindings.borrow_mut().insert(key, value);
    }
}


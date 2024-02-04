use std::collections::HashMap;

enum Expr {
    Symbol(String),
    Number(f64),
   
}

struct Environment {
    bindings: HashMap<String, Expr>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
        }
    }
    fn add_binding(&mut self, key: String, value: Expr) {
        self.bindings.insert(key, value);
    }

    fn get_value(&self, key: &str) -> Option<&Expr> {
        self.bindings.get(key)
    }
}

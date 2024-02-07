use crate::parser::*;
use crate::binding::*;
use core::cell::RefCell;
use std::rc::Rc;

pub fn assoc(symbol: &str, a_list:  &Rc<RefCell<AList>>) -> Result<Atom, String> {
    match a_list.borrow().get_binding(symbol) {
        Some(atom) => Ok(atom),
        None => Err(format!("Symbol '{}' not found", symbol)),
    }
}

pub fn apply_atom(list: &[Atom]) -> Result<Atom, String> {
    if let Some(Atom::Symbol(s)) = list.first() {
        match s.as_str() {
            "+" | "-" | "*" => {
                if list.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }

                let mut ops = Vec::new();
                for atom in &list[1..] {
                    match atom {
                        Atom::Integer(n) => ops.push(*n),
                        _ => return Err("unsupported operand for operation".to_string()),
                    }
                }

                let result = match s.as_str() {
                    "+" => ops.iter().sum(),
                    "-" => ops.iter().skip(1).fold(ops[0], |acc, &val| acc - val),
                    "*" => ops.iter().product(),
                    _ => unreachable!(), 
                };

                return Ok(Atom::Integer(result));


            },
            "defun" => println!("defun symbol"),
            "cond" => println!("cond symbol"),
            _ => println!("nothing here")
        }
        Ok(Atom::Void)  
    } else {
        Err("Expected a symbol".to_string())
    }
}

pub fn eval(parsed: &Atom, a_list: &Rc<RefCell<AList>>) -> Result<Atom, String> {
    match parsed {
        Atom::List(expr) => apply_atom(expr),
        Atom::Symbol(atom) => assoc(atom, a_list),
        Atom::Integer(int) => Ok(Atom::Integer(*int)),
        _ => Err("Unhandled Atom variant".to_string()),
    }
}

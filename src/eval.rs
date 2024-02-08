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

pub fn apply_atom(list: &[Atom], a_list: &Rc<RefCell<AList>>) -> Result<Atom, String> {
    if let Some(Atom::Symbol(s)) = list.first() {
        match s.as_str() {
            "car" => {
                if list.len() != 2 {
                    return Err("car expects exactly one list argument".to_string());
                }
                match eval(&list[1], a_list)? {
                    Atom::List(ref l) if !l.is_empty() => Ok(l[0].clone()),
                    Atom::List(_) => Err("car cannot operate on an empty list".to_string()),
                    _ => Err("car expects a list argument".to_string()),
                }
            },
            "cdr" => {
                if list.len() != 2 {
                    return Err("cdr expects exactly one list argument".to_string());
                }
                match eval(&list[1], a_list)? {
                    Atom::List(ref l) if !l.is_empty() => Ok(Atom::List(l[1..].to_vec())),
                    Atom::List(_) => Err("cdr cannot operate on an empty list".to_string()),
                    _ => Err("cdr expects a list argument".to_string()),
                }
            },
            "+" | "-" | "*" => {
                if list.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
    
                let mut ops = Vec::new();
                for atom in &list[1..] {
                    let evaluated_atom = eval(atom, a_list)?;
                    match evaluated_atom {
                        Atom::Integer(n) => ops.push(n), 
                        _ => return Err(format!("Expected a numerical value, found {:?}", evaluated_atom)),
                    }
                }
    
                let result: f64 = match s.as_str() {
                    "+" => ops.iter().sum(),
                    "-" => ops.iter().skip(1).fold(ops[0], |acc, &val| acc - val),
                    "*" => ops.iter().product(),
                    _ => unreachable!(), 
                };
                return Ok(Atom::Integer(result));
            },
            "defun" => {
                if list.len() < 4 {
                    return Err("Invalid function definition: expected at least 4 elements".to_string());
                }
            
                let defun = match &list[1] {
                    Atom::Symbol(s) => s,
                    _ => return Err("Invalid function name".to_string()),
                };
            
                let params_list = match &list[2] {
                    Atom::List(params) => params.iter().map(|param| {
                        match param {
                            Atom::Symbol(s) => Ok(s.clone()),
                            _ => Err("Invalid parameter: expected a symbol".to_string()),
                        }
                    }).collect::<Result<Vec<String>, String>>()?,
                    _ => return Err("Invalid function definition: expected a list of parameters".to_string()),
                };
            
                let body = list[3..].to_vec();
                let function = Atom::Function { params: params_list, body };
                a_list.borrow_mut().set_binding(defun.clone(), function);
            
                Ok(Atom::Void)
            },
            
            
            "cond" =>  return Ok(Atom::Void),
            _ => {
                match a_list.borrow().get_binding(s){
                    Some(Atom::Function {params, body}) => {
                        let args: Vec<Atom> = list[1..]
                            .iter()
                            .map(|arg| eval(arg, a_list))
                            .collect::<Result<Vec<Atom>, String>>()?;

                        let func_env = AList::new_with_parent(&a_list);
                    
                        for(param, arg) in params.iter().zip(args.iter()) {
                            func_env.borrow_mut().set_binding(param.clone(), arg.clone());
                        }
                        
                        if let Some(first) = body.first() {
                            return eval(first, &func_env);
                        } else {
                            return Err("Function body is empty".to_string());
                        }
                    },
                    _ => Err("symbol is not recognized".to_string()),
                }
            }
        }
    } else {
        Err("Expected a symbol".to_string())
    }
}

pub fn eval(parsed: &Atom, a_list: &Rc<RefCell<AList>>) -> Result<Atom, String> {
    match parsed {
        Atom::List(expr) => apply_atom(expr, a_list),
        Atom::Symbol(atom) => assoc(atom, a_list),
        Atom::Integer(int) => Ok(Atom::Integer(*int)),
        Atom::Quote(inner) => Ok((**inner).clone()),
        _ => Err("Unhandled Atom variant".to_string()),
    }
}

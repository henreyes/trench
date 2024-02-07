mod tokenize;
mod parser;
mod binding;
mod eval;
use crate::parser::*;
use crate::tokenize::*;
use crate::binding::*;
use crate::eval::*;
use core::cell::RefCell;
use std::rc::Rc;


fn main() {
    // "(defun my_function (x) (+ x 1))"
    let input = "(+ 1 5)";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens); 

    let a_list = Rc::new(RefCell::new(AList::new()));
    let result = (|| -> Result<(), String> {
        let parsed_atom = parse(&tokens)?;
        let eval_parsed = eval(&parsed_atom, &a_list)?;
        println!("Eval: {:?}", eval_parsed);
        Ok(())
    })();



}

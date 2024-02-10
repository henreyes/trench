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
use std::io::{self, Write};


fn main() {
    let environment = Rc::new(RefCell::new(AList::new())); 

    loop {
        print!("lisp> "); 
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().eq_ignore_ascii_case("quit") || input.trim().eq_ignore_ascii_case("exit") {
                    break; 
                }

                let tokens = tokenize(&input);
                match parse(&tokens) {
                    Ok(parsed_expr) => {
                        match eval(&parsed_expr, &environment) {
                            Ok(result) => println!("Result: {:?}", result),
                            Err(e) => println!("Error: {}", e),
                        }
                    },
                    Err(e) => println!("Parse Error: {}", e),
                }
            },
            Err(error) => println!("Error reading line: {}", error),
        }
    }
}
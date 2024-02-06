mod tokenize;
mod parser;
use crate::parser::*;
use crate::tokenize::*;


pub fn assoc(symbol: &str) -> Result<Atom, String> {
    println!("assoc to be defined");
    Ok(Atom::Void)
}

pub fn apply_atom(list: &[Atom]) -> Result<Atom, String> {
    if let Some(Atom::Symbol(s)) = list.first() {
        match s.as_str() {
            "+" => println!("add symbol"), 
            "-" => println!("subtract symbol"),
            "*" => println!("multiply symbol"),
            "defun" => println!("defun symbol"),
            "cond" => println!("cond symbol"),
            _ => println!("nothing here")
        }
        Ok(Atom::Void)  
    } else {
        Err("Expected a symbol".to_string())
    }
}

pub fn eval(parsed: &Atom) -> Result<Atom, String> {
    match parsed {
        Atom::List(expr) => apply_atom(expr),
        Atom::Symbol(atom) => assoc(atom),
        Atom::Integer(int) => Ok(Atom::Integer(*int)),
        _ => Err("Unhandled Atom variant".to_string()),
    }
}


fn main() {
    // "(defun my_function (x) (+ x 1))"
    let input = "(+ 1 5)";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens); 

   
    let parsed_result = parse(&tokens);

    match parsed_result {
        Ok(parsed_atom) => {
            println!("Parsed Atom: {:?}", parsed_atom);
        },
        Err(e) => println!("Parsing Error: {}", e),
    }


}

mod tokenize;
mod parser;
use crate::parser::*;
use crate::tokenize::*;






fn main() {
    // "(defun my_function (x) (+ x 1))"
    let input = "(1 2 3 4 5)";
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

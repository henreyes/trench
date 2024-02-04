use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("lisp> "); 
        io::stdout().flush().unwrap();

        input.clear(); 
        match stdin.read_line(&mut input) {
            Ok(n) => {
                if n == 0 { 
                    break;
                }
                let input_trimmed = input.trim(); 
                if input_trimmed.is_empty() {
                    continue;
                }
                println!("You entered: {}", input_trimmed); 
            },
            Err(error) => {
                println!("Error reading input: {}", error); 
                break;
            },
        }
    }
}

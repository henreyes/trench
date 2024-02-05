
use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Symbol(String),
    Number(f64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
        }
    }
}

struct TokenVec(Vec<Token>);
impl fmt::Display for TokenVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, token) in self.0.iter().enumerate() {
            if index > 0 { write!(f, " ")?; } 
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}



#[derive(Clone, Debug)]
pub enum Atom {
    Void,
    Integer(f64),
    Bool(bool),
    Symbol(String),
    List(Vec<Atom>),
    Lambda(LambdaFunction),
}

#[derive(Clone, Debug)]
pub struct LambdaFunction {
    params: Vec<String>,
    body: Vec<Atom>,
    closure: Rc<Environment>, 
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            },
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            },
            ' ' | '\n' | '\t' => {
                chars.next();
            },
            _ => { 
                while let Some(&nc) = chars.peek() {
                    if nc == '(' || nc == ')' || nc.is_whitespace() {
                        break;
                    } else {
                        current_token.push(nc);
                        chars.next();
                    }
                }
                if let Ok(n) = current_token.parse::<f64>() {
                    tokens.push(Token::Number(n));
                } else {
                    tokens.push(Token::Symbol(current_token.clone()));
                }
                current_token.clear();
            }
        }
    }

    tokens
}

pub fn parse_list(tokens: &[Token]) -> Result<(Atom, &[Token]), String> {
    match tokens.split_first() {
        Some((Token::OpenParen, rest)) => {
            let mut list = Vec::new();
            let mut current_tokens = rest;
            while let Some((first, rest)) = current_tokens.split_first() {
                match first {
                    Token::CloseParen => return Ok((Atom::List(list), rest)), 
                    _ => {
                        let (parsed, new_rest) = parse_expr(current_tokens)?;
                        list.push(parsed);
                        current_tokens = new_rest;
                    }
                }
            }
            Err("Unclosed list".to_string())
        },
        _ => Err(format!("Expected open parenthesis, '(', found {:?}", tokens.first())),
    }
}

pub fn parse_expr(tokens: &[Token]) -> Result<(Atom, &[Token]), String> {
    match tokens.split_first() {
        Some((Token::OpenParen, _)) => parse_list(tokens),
        Some((Token::Number(n), rest)) => Ok((Atom::Integer(*n), rest)),
        Some((Token::Symbol(s), rest)) => Ok((Atom::Symbol(s.clone()), rest)),
        Some((Token::CloseParen, _)) => Err("Unexpected ')'".to_string()),
        None => Err("Empty expression".to_string()),
        _ => Err(format!("Unexpected token: {:?}", tokens.first())),
    }
}

pub fn parse(tokens: &[Token]) -> Result<Atom, String> {
    let (parsed, remaining) = parse_expr(tokens)?;
    if !remaining.is_empty() {
        Err("Extra tokens after parse".to_string())
    } else {
        Ok(parsed)
    }
}




#[derive(Clone, Debug)]
pub struct Environment {
    bindings: RefCell<HashMap<String, Atom>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
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

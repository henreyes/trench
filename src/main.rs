
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



pub fn parse_list(tokens: &[Token], pos: &mut usize) -> Result<Atom, String> {
    if tokens.get(*pos) != Some(&Token::OpenParen) {
        return Err(format!("Expected '(', found {:?}", tokens.get(*pos)));
    }

    *pos += 1; 

    let mut list: Vec<Atom> = Vec::new();
    while *pos < tokens.len() && tokens[*pos] != Token::CloseParen {
        match &tokens[*pos] {
            Token::Number(n) => {
                list.push(Atom::Integer(*n));
                *pos += 1;
            },
            Token::Symbol(s) => {
                list.push(Atom::Symbol(s.clone()));
                *pos += 1; 
            },
            Token::OpenParen => {
                // Recursive call to handle nested lists
                let sublist = parse_list(tokens, pos)?;
                list.push(sublist);
       
            },
            _ => return Err(format!("Unexpected token: {:?}", tokens[*pos])),
        }
    }

    if *pos >= tokens.len() {
        return Err("Unclosed list".to_string());
    }

    *pos += 1; 
    Ok(Atom::List(list))
}

pub fn parse(tokens: &[Token]) -> Result<Atom, String> {
    let mut pos = 0;
    let result = if tokens.len() > pos {
        match &tokens[pos] {
            Token::OpenParen => parse_list(tokens, &mut pos),
            Token::Number(n) => Ok(Atom::Integer(*n)),
            Token::Symbol(s) => Ok(Atom::Symbol(s.clone())),
            _ => Err(format!("Unexpected token: {:?}", tokens[pos])),
        }
    } else {
        Err("No tokens to parse".to_string())
    };

    if pos < tokens.len() {
        Err("Extra tokens after parse".to_string())
    } else {
        result
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
    let input = "(defun my-function (x) (+ x 1))";
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

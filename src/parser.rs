use crate::tokenize::*;

#[derive(Clone, Debug)]
pub enum Atom {
    Void,
    Integer(f64),
    Bool(bool),
    Symbol(String),
    List(Vec<Atom>),
    Function {
        params: Vec<String>,
        body: Vec<Atom>, 
    },
    Quote(Box<Atom>),
    Nil,
   
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
        Some((Token::Quote, rest)) => {
            let (quoted_expr, remaining_tokens) = parse_expr(rest)?;
            Ok((Atom::Quote(Box::new(quoted_expr)), remaining_tokens))
        },
        Some((Token::OpenParen, _)) => parse_list(tokens),
        Some((Token::Number(n), rest)) => Ok((Atom::Integer(*n), rest)),
        Some((Token::Symbol(s), rest)) => Ok((Atom::Symbol(s.clone()), rest)),
        Some((Token::CloseParen, _)) => Err("Unexpected ')'".to_string()),
        Some((Token::Nil, rest)) => Ok((Atom::Nil, rest)),
        None => Err("Empty expression".to_string()),
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


